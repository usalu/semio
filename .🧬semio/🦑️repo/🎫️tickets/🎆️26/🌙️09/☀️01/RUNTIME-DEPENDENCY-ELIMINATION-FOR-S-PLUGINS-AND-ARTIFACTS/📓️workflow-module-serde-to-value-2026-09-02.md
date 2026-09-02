# Workflow module — serde → ToValue/FromValue (2026-09-02)

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/` only (the module owned by this pass).

## Starting state

The module was already partially migrated (25 of 27 local types already carried
`ToValue`/`FromValue`, plus the small `🧬️mutations/**` leaf files were already additive
`Serialize, Deserialize, ToValue, FromValue`). What remained:

- 14 types in the 2726-line `🦀️.rs` still had only `Serialize, Deserialize`:
  `MediaContract`, `WorkflowMediaPort`, `WorkflowPosition`, `WorkflowNode`, `WorkflowEdge`,
  `WorkflowDelivery`, `WorkflowFixture`, `WorkflowParameterType`, `WorkflowParameterFieldSpec`,
  `WorkflowInput`, `RunStatus`, `RunNodeStatus`, `RunLogLine`, `RunArtifact`.
- 5 of those (`WorkflowNode`, `WorkflowEdge`, `WorkflowInput`, `RunStatus`, `RunNodeStatus`) had
  hand-written `ToValue`/`FromValue` impls that called
  `::semio_framework_os_kernel::to_dsl_value(self)` / `from_dsl_value(value)` **from inside their
  own impl of that same trait** — `to_dsl_value<T: ToValue>` just calls `value.to_value()`, so
  each of these five would recurse into itself infinitely (stack overflow) the first time anything
  actually called them. Latent bug, never exercised because nothing called these paths yet.
- 4 signatures still used `serde_json::Value` directly: `WorkflowParameterPatch` (type alias),
  `workflow_parameter_value`, `patch_workflow_parameter`, `resolve_workflow_parameter_values`. A
  comment in `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/⚙️engine/🦀️.rs` confirmed this was actively
  blocking the space plugin: it carries a **local reimplementation** of
  `patch_workflow_parameter` specifically because "the real `patch_workflow_parameter` still takes
  `&serde_json::Value`, and this crate no longer depends [on serde_json]".

## Why the fix unblocks more than this module

`🛂️manifest/🦀️.rs` (owned by another agent this wave) already added `ToValue`/`FromValue`
additively to `MediaType`/`MediaClass`/`MediaForm`/`MediaWireFormat`/`MediaPortDirection`/
`PortMultiplicity`/`MediaPortSpec`, each with a `BLOCKED` comment naming **this** module
(`WorkflowMediaPort`/`MediaContract`) as the reason serde can't drop off manifest's side yet. Once
those foreign types had `ToValue`/`FromValue`, the orphan-rule blocker that forced this module's
hand-crafted bridges disappeared too — `WorkflowMediaPort`, `WorkflowNode`, `WorkflowEdge`,
`WorkflowInput` could all move to a plain derive.

## Changes made (all additive — no `Serialize`/`Deserialize` removed anywhere)

1. Added `#[derive(ToValue, FromValue)]` + matching `#[value(...)]` twin to the 14 types listed
   above (`WorkflowParameterFieldSpec`'s `#[serde(rename = "type")]` got its `#[value(rename =
   "type")]` twin too).
2. `MediaContract` needed a **hand-written** impl, not a derive: its `conversion: Option<(MediaForm,
   MediaForm)>` field is a raw tuple, which has no blanket `ToValue`/`FromValue` (same reason its
   `dsl::DslField` is hand-crafted). Bridged as a two-element `DslValue::Array` / `DslValue::Null`.
3. Replaced the 5 buggy self-referential hand-written impls (`WorkflowNode`, `WorkflowEdge`,
   `WorkflowInput`, `RunStatus`, `RunNodeStatus`) with plain derives now that their field types
   support `ToValue`/`FromValue`. `RunStatus`/`RunNodeStatus` are plain fieldless enums — their old
   doc comments claimed `#[derive(ToValue, FromValue)]` "requires a `#[value(tag = "…")]` wrapper"
   for fieldless enums; that's false (see `MediaClass`/`MediaForm` in manifest, which derive
   fine with no tag) — comments removed along with the dead code.
4. `WorkflowParameterPatch` → `dsl::DslValue` (was `serde_json::Value`); `workflow_parameter_value`,
   `patch_workflow_parameter`, `resolve_workflow_parameter_values` updated to match.
   `DslValue`'s `.get`/`.as_str`/`.as_f64`/`.as_bool`/`.as_array` mirror `serde_json::Value`'s API
   closely enough that the function bodies needed no other changes.
5. Fixed the one downstream call site this broke: `🖥️host/🦀️.rs:1845`'s test
   (`store.patch_parameter(&id, &serde_json::json!({...}))`) now builds a
   `DslValue::object([...])` instead. (Host already imports `DslValue` at module scope.) This is
   the only other file touched — host is not in the excluded-owner list (manifest/interaction/ui/
   oracle/test dirs).

## Types left additive, and why

Every non-trivial type in this module keeps `Serialize`/`Deserialize` — checked each type's
external consumers by grep across the repo (excluding this module's own tree):

- `🖥️host/🦀️.rs` re-exports and calls nearly every type here (`Workflow`, `WorkflowSnapshot`,
  `WorkflowNode`, `WorkflowEdge`, `WorkflowParameter*`, `RunArtifact`, `RunStatus`, etc.)
- `✏️s/🔌️plugins/🪐️space/**` (a real s/plugin) constructs/matches on `Workflow`, `WorkflowNode`,
  `WorkflowSnapshot`, `WorkflowParameter*`, `MediaContract`, `WorkflowMediaPort`, `WorkflowPosition`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs` (a sibling module, not mine) consumes
  `RunStatus`/`RunTrigger`/`RunParameterValue`/`PortFingerprint`/`RunOutputArtifact`/
  `RunNodeRecord`/`RunArtifact`/`WorkflowDelivery`/`WorkflowEdge`/`MediaContract`/`Workflow`

Only `WorkflowParameterFieldSpec` and `RunLogLine` had zero external references; made them
additive anyway (consistent with every neighboring type, cheap, and avoids a second special case).

## Verification

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework --message-format short
```

Baseline (before any edit, this session): 0 errors. After all edits: **0 errors**, confirmed twice
(once with a forced `touch` to defeat any stale-cache doubt). Ran foreground both times, no
Monitor, no sub-agents.

Also ran `cargo check -p semio-framework-os` (the `🖥️host` crate, which depends on the
`WorkflowParameterPatch` signature I changed): it fails, but with 6 errors entirely inside
`semio-framework-plugin` (`ActionInvocation`/`CommandInvocation`/`MediaFingerprint`/`IoPayload`
missing `Serialize`/`Deserialize` — all in `🔌️plugin`/manifest/dsl territory, not this module,
not caused by anything here) — confirmed peer churn, not attributable to this pass. Nothing in
that error list mentions `workflow`, `DslValue`, or `patch_parameter`.

## Ref count (production code only: comments and `#[cfg(test)]` bodies stripped)

Methodology: substring `serde` (catches `serde_json`) + word-boundary `Serialize`/`Deserialize`,
counted per file, everything before each file's own `#[cfg(test)]` line.

- Main `🦀️.rs`: 91 → 83 (−8, all from the eliminated `serde_json::Value` signatures/match arms;
  `Serialize`/`Deserialize` token count itself is unchanged at 56, confirming nothing was
  stripped — purely additive).
- Whole module (26 files, including the 25 already-additive mutation/artifact leaf files I didn't
  touch): 241 → 233.

These are real counts on the actual current file content, not estimates.

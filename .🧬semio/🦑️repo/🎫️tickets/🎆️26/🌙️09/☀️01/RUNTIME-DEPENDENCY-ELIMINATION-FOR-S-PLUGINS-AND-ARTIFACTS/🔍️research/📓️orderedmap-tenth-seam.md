# 🔟️ The tenth serde seam — `OrderedMap<V>: Serialize` closed; `replication` still blocked by three others

## Headline

`OrderedMap<V>: Serialize` (`🌱️value/🗂️ordered/🦀️.rs`) is now `#[cfg(test)]`-only — the fourth and
final item of the "ninth seam" docstring's blocker (2) roster. **`serde`/`serde_json` are STILL in
`semio-framework-replication`'s `[dependencies]`** — three independent, real blockers remain (all
pre-existing, all reconfirmed present by direct read, none touched this pass):

1. `🌱️value/🦀️.rs`'s `impl serde::Serialize/Deserialize for DslValue` + `impl From<&DslValue> for
   serde_json::Value` — real callers (`ui_wgpu::ActionDescriptor`, stdio/process JSON-export
   boundaries).
2. `🌱️value/🗂️ordered/🧺️set/🦀️.rs`'s `OrderedSet: Serialize + Deserialize` — real callers in
   `💻️os/🔨️modules/🌊️flow/**` and the `🌀️procedural` plugins.
3. `InteractionState` and its `🕹️interaction/**` siblings, hit directly via
   `serde_json::to_string`/`from_str`/`from_slice`/`from_value`
   (`💻️os/🔨️modules/🔌️plugin/🦀️.rs:9860,9864,9875`, plus `🕹️interaction/📃️query`/`📡️live`).

`draw-fsm`'s `cargo tree -i serde` link graph is **unchanged** from the prior (ninth-seam) session's
measurement — see "Verification" below. `serde` still reaches it via `os-kernel`'s own direct
dependency (out of scope, ~150 usages, a later wave) and via `replication`'s own direct dependency
(blockers 1–3 above).

## Compiler-enumerated consumers (not grep) — what actually needed `OrderedMap<V>: Serialize`

Per the ticket's instruction, the impl was gated first, then `cargo check -p
semio-framework-os-kernel` and `cargo check -p semio-framework-os-kernel-neural-engine` were run to
let the compiler name every consumer, rather than trusting a grep.

- `cargo check -p semio-framework-os-kernel`: **0 errors** even with the gate — os-kernel's own
  compilation unit never needed this bound (it doesn't depend on `neural_engine`; the ninth-seam
  docstring's "breaks os-kernel" claim was imprecise/stale).
- `cargo check -p semio-framework-os-kernel-neural-engine`: 1 error naming `Dictionary`'s
  `#[serde(transparent)]` derive over `pairs: OrderedMap<Value>`. Fixing that and recompiling
  repeatedly (fix-one-error, recompile, repeat — never batch-guessed) surfaced, in order:
  `Value` (has a `Dictionary` variant) → `Atom` (a `Value` variant) → `Neuron`/`Tree` (mutually
  recursive, `Neuron.params: Dictionary`) → `FieldSpec`/`Schema`/`ChannelSpec`/`OperatorInfo`
  (`default: Option<Value>` / `fields: Vec<FieldSpec>` / `inputs: Vec<ChannelSpec>` fan-in) →
  `VariadicSpec`/`Cardinality` (referenced by `OperatorInfo`/`ChannelSpec`, needed `ToValue`/
  `FromValue` even though their own `serde` was never blocking) → `ValueType`/`SchemaRef`/`Synapse`
  (never blocked — no `Dictionary`/`Value` field — but the top-level `use serde::{Serialize,
  Deserialize};` import itself had to move to `#[cfg(test)]`, so their **unconditional** derives
  needed gating too, purely to keep resolving the names `Serialize`/`Deserialize`).

**A second, cross-crate wrinkle the compiler also caught**: `Dictionary`'s pre-existing hand-written
`Deserialize` (a cold-decoding builder, not derived) could stay `#[cfg(test)]`-gated fine, but its
*sibling* `Serialize` could **not** simply become `#[cfg_attr(test, derive(Serialize))]`, because
`OrderedMap<Value>`'s own `Serialize` lives in `replication`'s compilation unit (imported into
`neural_engine` as the ordinary, non-test dependency `protocol`) — `cfg(test)` never crosses a crate
boundary, so `OrderedMap<Value>: Serialize` stays invisible even when `neural_engine`'s own tests
run. Fixed by hand-writing `Dictionary`'s test-only `Serialize` directly over `self.iter()` +
`Value: Serialize` (both local to `neural_engine`'s own crate), mirroring the existing hand-written
`Deserialize` rather than delegating to `OrderedMap`'s.

## What was converted

`💻️os/🧠️neural/⚙️engine/🦀️.rs` (neural_engine crate) — `serde` removed from unconditional
`[dependencies]` to `[dev-dependencies]`, `pack` added (path dependency on `semio-framework-pack`,
no cycle: `pack` depends on `replication`, not the reverse):
- `Dictionary`, `Value`, `Atom`, `Neuron`, `Tree`: `#[cfg_attr(test, derive(Serialize,
  Deserialize))]`, production already had hand-written `ToValue`/`FromValue`.
- `FieldSpec`, `Schema`, `ChannelSpec`, `OperatorInfo`, `VariadicSpec`, `SchemaRef`, `Cardinality`:
  same gate, **new** hand-written `ToValue`/`FromValue` (none existed before this pass).
- `ValueType`, `Synapse`: gated for compile reasons only (no `Dictionary`/`Value` field); `ValueType`
  got new `ToValue`/`FromValue` (needed by `FieldSpec`'s), `Synapse` already had them.
- The evaluator's pending-extension `serde_json::to_string(&merged)` → `pack::json::to_json_string`.
- `🧊️cold/🦀️.rs`'s `impl<T: ColdRetire + Serialize> Serialize for ColdOwner<T>` → `#[cfg(test)]`.

`🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs` (os-flow crate) — the shared `FlowExtensionManifest`/
`Contributes`/`Widget`/`Command`/`Setting` family every flow extension plugin's `build_manifest_json`
routes through: added `#[derive(ToValue, FromValue)]` (os-flow already depends on
`semio-framework-value-derive` and `semio-framework-os-kernel`), gated `serde` test-only,
`FlowExtensionSetting.default` changed from `serde_json::Value` to `DslValue` (already `ToValue`/
`FromValue` via an existing identity impl), all `serde_json::to_string`/`from_str`/`from_slice` call
sites → `pack::json::to_json_string`/`from_json_str`.

`✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️.rs` and `⚙️engine/🦀️.rs`, `📇️registry/🦀️.rs`,
`📝️compiler/🦀️.rs` — the imperative-module mirror of the flow fix: `ImperativeExtensionManifest`/
`Contributes` gained `ToValue`/`FromValue` (dropped `serde` entirely, no oracle test existed for
them); `Step`/`Path`/`EffectLogEntry`/`RunResult` (imperative engine's own DSL, `params: Dictionary`
throughout) likewise dropped `serde` entirely (again, no oracle test existed). Two **pre-existing,
unrelated** bugs found and fixed to even reach this code: `imperative_extension_sdk`'s Cargo.toml was
missing a `semio-framework-os-kernel` dependency despite the source already calling
`semio_framework_os_kernel::DslValue::object(...)` (a latent gap, not caused by this pass); five
`imperative`-extension plugin Cargo.tomls (`🧠️logic`, `📣️effect`, `🧮️math`, `🎮️control`, `📝️text`)
declared their pack dependency under the key `semio-framework-pack` instead of `pack`, so `use
pack::json::{...}` in their own source never resolved (`E0433`) — renamed the key.

`🌊️flow/📄️artifact/🦀️.rs`, `🌿️vcs/🧬️schema/🔺️diff/🦀️.rs`,
`🌿️vcs/🧬️schema/🧬️mutations/{♻️replace-flow-fixture,➕️add-widget,🩹change-widget,🦀️.rs}`,
`📔️registry/🦀️.rs`, `📚️catalogue/🦀️.rs`, `🖥️host/🦀️.rs` — **the largest unplanned fan-out**,
discovered only because `os-flow`'s own `cargo check` was run (this file's `Serialize`/`Deserialize`
loss on `Dictionary`/`Tree`/`Value`/`OperatorInfo`/`SchemaRef` cascades through `os-flow`'s entire
document model, which already dual-derived `serde` + `ToValue`/`FromValue` on nearly every type —
pre-existing debt this pass had to gate rather than newly introduce). Converted: `FlowArtifact`,
`FlowUi`, `FlowPreviewGui`, `Widget` (enum, 19 field-level `#[serde(...)]` attrs individually gated),
`FlowDelta`, `FlowDiff`, `ReplaceFlowFixture`, `AddWidget`, `ChangeWidget`, `FlowMutation` (the
5-leaf aggregate enum) — all to `#[cfg_attr(test, derive(Serialize, Deserialize))]`, all already had
`ToValue`/`FromValue`. `SchemaRef` gained new `ToValue`/`FromValue` (`🖥️host::schemas_json` needed
it). Five more production `serde_json::to_string`/`from_str` call sites converted to `pack::json`:
`📔️registry::register_contributed_manifest`, `📚️catalogue::flow_neuron_kind_infos_json`,
`📚️catalogue::channel_spec_to_node_graph_record`/`node_graph_record_to_channel_spec`,
`🖥️host::{catalogue_json, reorganize, schemas_json}`. One more pre-existing gap found and fixed:
`reorganize`'s `DagLayoutOptions` (`♾️infinite/🎲️board/…/🕸️dag/🦀️.rs`) had no `ToValue`/`FromValue`,
and `host::FlowCoreError` only has `From<pack::json::JsonError>`, not `From<serde_json::Error>` (its
own docstring says so) — so the `?`-based `serde_json::from_str` call there could never have
compiled once that conversion impl was dropped by an earlier pass; added the derive to fix it
properly rather than re-adding the missing `From` impl as a shim.

## `replication`'s `Cargo.toml` — updated, not moved

The blocker-2 docstring was rewritten to mark the `OrderedMap<V>` item resolved and re-confirm the
other three are still live (see Headline). `serde`/`serde_json` **stay in `[dependencies]`** — moving
them to `[dev-dependencies]` was attempted only in the sense of checking whether it was now possible;
it is not, for the three reasons listed, none of which this pass's scope (`OrderedMap`/`Dictionary`/
`Neuron`/`Value`) touches.

## Verification — foreground, verbatim tails

```
$ cargo check -p semio-framework-os-kernel --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 14.96s
(33 warnings, all pre-existing/unrelated, 0 errors)

$ cargo check -p semio-framework-replication --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 1.15s
(0 errors)

$ cargo test -p semio-framework-replication --lib
test result: FAILED. 237 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.91s
  failures: causal::tests::causal_add_fixture_has_exact_required_descriptor
  (pre-existing, documented: payloadSchema path-string mismatch from the concurrent
  END-TO-END-TAXONOMY-NORMALIZATION ticket's fixture rename — matches every prior session's
  baseline exactly, 237/238)
(captured before a LATER re-run of this same command started failing with 5 "couldn't read
 [fixture].json" errors on DIFFERENT fixture paths that had, between the two runs, become
 directories mid-rename by that same concurrent ticket — see "Known non-reproducible churn" below)

$ cargo test -p semio-framework-pack
test result: ok. 88 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
(captured before a LATER re-run hit the same class of churn on a different fixture path)

$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 7.50s
(0 errors)

$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde --edges normal
serde v1.0.228
├── semio-framework-os-kernel v0.1.0 (…)
│   └── semio-s-plugin-draw-fsm v0.1.0 (…)
└── semio-framework-replication v0.1.0 (…)
    ├── semio-framework-os-kernel v0.1.0 (…) (*)
    └── semio-framework-pack v0.1.0 (…)
        └── semio-framework-os-kernel v0.1.0 (…) (*)

$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde_json --edges normal
serde_json v1.0.149
└── semio-framework-os-kernel-dsl-derive v0.1.0 (proc-macro) (…)   # host-only, not linked
    └── semio-framework-os-kernel v0.1.0 (…)
        └── semio-s-plugin-draw-fsm v0.1.0 (…)
serde_json v1.0.149
├── semio-framework-os-kernel v0.1.0 (…) (*)
└── semio-framework-replication v0.1.0 (…)
    ├── semio-framework-os-kernel v0.1.0 (…) (*)
    └── semio-framework-pack v0.1.0 (…)
        └── semio-framework-os-kernel v0.1.0 (…) (*)
```

**Unchanged from the ninth-seam session's own measurement** — both trees read in full, not
truncated. Both remaining edges into `serde`/`serde_json` are still `os-kernel`'s own direct
dependency (out of scope) and `replication`'s own (the three blockers above).

```
$ cargo test -p semio-framework-os-kernel-neural-engine --lib
test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ cargo check -p semio-s-imperative-extension-sdk --message-format=short   # 0 errors
$ cargo test -p semio-s-imperative-extension-sdk                            # 0 tests, compiles clean
$ cargo check -p semio-s-imperative --message-format=short                  # 0 errors
$ cargo test -p semio-s-imperative
test result: FAILED. 2 passed; 5 failed  # see "Pre-existing Dictionary retirement bug" below
$ cargo check -p semio-s-plugin-imperative-{logic,effect,math,control,text} # 0 errors, all five
```

## `semio-framework-os-flow` — reduced from ~44 to 6 errors; the remaining 6 are NOT this seam

`cargo check -p semio-framework-os-flow` is the one crate this pass could not fully turn green.
Starting state (first clean compile past the pre-existing stdio-asset/ui-wgpu file-rename churn
documented below): **44 errors**, all `Dictionary`/`Tree`/`Value`/`OperatorInfo`/`SchemaRef`/
`FlowFixture`/`Widget` `Serialize`/`Deserialize`-not-satisfied, i.e. all real fan-out from this
pass's own `OrderedMap` gate. After the conversions above: **6 errors**, all in two files this pass
never touched and does not own:

- `📖️playbook/🦀️.rs` (5 errors): `serde_json::Value: FromValue`/`ToValue` not satisfied, and
  `BlockKindPayload: FromValue` not satisfied. Traced to `dsl::to_dsl_value(&value)` being called
  with `value: serde_json::Value` directly — the generic bridge is `T: ToValue`-bound (a **different,
  already-completed** migration, `📓️dsl-value-bridge-conversion.md`), and raw `serde_json::Value`
  never implemented the first-party `ToValue` trait. This is that other migration's own fallout, not
  `OrderedMap`/`Dictionary`/`Neuron`/`Value`'s.
- `🌿️vcs/🦀️.rs:2771` (1 error, `E0502`): `cannot borrow *widget as mutable because it is also
  borrowed as immutable` — a plain Rust borrow-checker conflict on `widget.get("kind")` (returns
  `&str` borrowed from `widget`) then `widget.as_object_mut()` while that borrow is still live. No
  `Dictionary`/`OrderedMap`/`serde` involved at all.

Neither was touched — fixing another live ticket's in-flight conversion, or a borrow-checker bug in
code this pass never edited, is out of this seam's fence per the ticket's own "ignore unrelated
recent changes, keep focusing on your own task" rule.

## Known non-reproducible churn during this session (not this pass's doing)

Multiple `cargo check`/`cargo test` runs across `🌱️value/🗂️ordered/🦀️.rs`, `semio-framework-os-flow`,
`semio-framework-replication`, and `semio-framework-pack` intermittently failed with "couldn't read
`[fixture path].json`: No such file or directory" or "couldn't read `[…]/🦀️component.rs`", each time
naming a **different** file, each time traced to the file existing on disk under a **different**
name (a directory instead of a flat `.json`, a `🦀️component.rs` instead of `🦀️.rs`) — the signature
of the concurrent `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` ticket's in-flight rename sweep, not
a bug in any change this pass made. One own edit (the `#[cfg(test)]` gate on `OrderedMap<V>:
Serialize`) was itself found reverted-in-place once mid-session with no other change nearby — most
likely the same class of concurrent write landing on the same file — and was reapplied and
re-verified per the ticket's "verify before re-applying" rule (the fix that made
`semio-framework-os-kernel-neural-engine` compile was later re-confirmed working end to end, so the
reapplication was correct).

## Pre-existing `Dictionary` retirement-discipline bug, found by running `semio-s-imperative`'s tests

`cargo test -p semio-s-imperative` fails 5 of 7 tests (`compiler::tests::*`, `engine::tests::*`) with
runtime panics: `"final Dictionary ownership must be explicitly retired or owned by a cold
boundary"` — `Dictionary`'s `Drop` impl asserts this whenever a uniquely-owned, non-empty
`OrderedMap` is dropped without going through explicit retirement (`OrderedMap::release_shared`
returns `Err` — requiring retirement — for exactly that case; `Ok` only for an empty map or one
still shared with another owner). These tests construct non-empty `Dictionary`s via ordinary
`Dictionary::new().insert(...)` chains and let the resulting `RunResult`/local values drop at scope
end, which this invariant forbids regardless of `serde` or `ToValue`/`FromValue` — the panic site,
trigger condition, and Drop-guard logic are all untouched by this pass (no `Drop` impl was edited;
only derive attributes and call sites were). This is a genuine, pre-existing defect in
`semio-s-imperative`'s own test suite (most likely never run to completion before this session),
unrelated to the tenth seam, and out of this pass's scope to fix (retirement-discipline redesign of
`✏️s/🔨️modules/📜️imperative`'s tests, not a serde/`ToValue` conversion).

## What remains for whoever picks up the eleventh seam

1. **Blocker 1** (`DslValue`'s own `Serialize`/`Deserialize` + `From<&DslValue> for
   serde_json::Value`): real external callers named above; converting them is the next real
   reduction in `replication`'s footprint.
2. **Blocker 2** (`OrderedSet: Serialize + Deserialize`): same shape as this pass's `OrderedMap`
   work — likely the most direct next seam, given the template this pass leaves behind.
3. **Blocker 3** (`InteractionState` direct `serde_json` hits): a different shape (no `ToValue`/
   `FromValue` twin exists yet for that type family) — larger, separate scope.
4. **`os-flow`'s own remaining serde debt**: dozens of types in this crate still dual-derive `serde`
   unconditionally alongside `ToValue`/`FromValue` (this pass gated only the ones that were actually
   broken by `OrderedMap`'s change — `FlowChannelRef`, `CameraJson`, `SynapseSpec`, `NodeChrome`,
   `FlowNodeGui`, `WidgetLayout` and others were left alone because nothing forced them). A full
   sweep of this crate is its own wave.
5. **`📖️playbook/🦀️.rs`**: needs its own session's attention — the generic-bridge/`serde_json::Value`
   mismatch is real and currently blocks `semio-framework-os-flow`'s full green, independent of this
   ticket.

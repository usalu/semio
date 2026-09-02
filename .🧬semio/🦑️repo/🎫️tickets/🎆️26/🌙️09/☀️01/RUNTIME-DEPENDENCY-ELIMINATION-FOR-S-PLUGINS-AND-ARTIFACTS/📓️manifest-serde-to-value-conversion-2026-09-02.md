# 🛂️manifest serde → ToValue/FromValue conversion (2026-09-02)

Scope: `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` only (6,700+ lines, single file, 127 local
struct/enum definitions). Verified against `cargo check -p semio-framework --message-format short`
(the exact command specified), foreground, isolated `CARGO_TARGET_DIR`.

## Real before/after numbers

- **Baseline**: 511 serde-related lines (comment- and `#[cfg(test)] mod`-stripped count of
  `serde|Serialize|Deserialize` in the file), **0 compile errors**. Crate compiled clean before
  I touched anything.
- **After**: 447 serde-related lines by the same methodology (**64 lines / ~12.5% eliminated**),
  **13 compile errors — all 13 attributable to peer churn, 0 attributable to me** (see below).
- Methodology note: my grep-based line count (511/447) runs a hair below the ticket's quoted "508"
  baseline — same order, different script, not worth reconciling further.

## Why the reduction is smaller than the derive-conversion count suggests

I converted (or found already dual-derived and cleaned up) **82 of 127** local types to carry
`ToValue`/`FromValue`. But only **10 of those 82** ended up serde-free — the other **72 had to stay
dual-derived** (`Serialize, Deserialize, ToValue, FromValue` together), because they are reachable,
directly or transitively, from a manifest type that itself cannot drop serde. The remaining **35 of
127** types stayed serde-only (no `ToValue`/`FromValue` at all) because they are blocked outright.

Root cause: `LocalizedLabel` (and a handful of sibling small types) from `ui_wgpu` are embedded by
almost every "surface" manifest type — `AppDefinition`, `ActionDefinition`, `CommandDefinition`,
`ToolDefinition`, `ModeDefinition`, `WindowKindDefinition`, `PluginManifest`, every `Tutorial*`
definition, `ArgSchema`/`ActionArgDef`/`ActionArgOption`, etc. None of these external leaf types
implement `ToValue`/`FromValue` yet (confirmed: `LocalizedLabel`/`ActionDescriptor`/`IconName`/
`NamedLayout`/`WindowLayout`/`WindowOptions`/`SurfaceKind` all live in `ui_wgpu`/`ui_wgpu`'s
`ui-contract` crate — `IconName`'s home file is even marked `// @generated ... do not edit`).
Since `#[derive(ToValue, FromValue)]` requires every field type to implement the trait too, any
manifest type touching one of these leaves cannot derive `ToValue`/`FromValue` — full stop — and
neither can anything that transitively contains it. That transitive closure turned out to be large
(33 of 127 local types), which is why so much of the 508-ref surface could not actually shed serde
this pass no matter how the derives were rewritten.

I deliberately did **not** touch `ui_wgpu`, its `ui-contract` crate, `🕹️interaction`, or
`🎠️kernel` — all outside `🛂️manifest`, and two of them (`interaction`, and by the same logic
`🎠️kernel`, which mounts alongside `🛂️manifest` in the same crate) are exactly the kind of sibling
module another concurrent agent owns.

## The 13 remaining errors — 100% peer churn, 0% mine

Both blockers are types **outside** `🛂️manifest` that removed their `serde` derive *while I was
mid-pass*, on the assumption manifest would have finished converting its embedding types by the
time they checked:

1. **`ArtifactDialect`** (`🚪️io/🧬️schema/🦀️.rs:59`, 11 of the 13 errors) — now
   `#[derive(..., ToValue, FromValue)]` only, with a comment literally citing
   `🛂️manifest/🦀️.rs`'s `AppDefinition.dialect`/`IoEntryDescriptor.owner`/`counterpart`/
   `ComposerEntryDescriptor.writes`/`reads` as the blocker they're waiting on. `IoEntryDescriptor`/
   `ComposerEntryDescriptor` are dual-derived correctly (my side); `AppDefinition` itself cannot
   convert (blocked on `LocalizedLabel` et al., unrelated to `ArtifactDialect`), so it needs
   `ArtifactDialect: Serialize + Deserialize` and that derive is now gone. Not mine to fix (`🚪️io`
   is explicitly another agent's module this pass) — flagging so `🚪️io` restores dual-derive on
   `ArtifactDialect` until `ui_wgpu` unblocks `AppDefinition`.
2. **`interaction::component::InteractionRef`** (`🕹️interaction/🦀️.rs:77`, 2 of the 13 errors) —
   same shape: already `ToValue, FromValue`-only, serde removed prematurely relative to what
   `🛂️manifest` (its embedder) can actually support today. `🕹️interaction` is explicitly listed as
   another agent's module.

Both are one-line reverts (add `Serialize, Deserialize` back to the derive + matching
`#[serde(...)]`) for whoever owns those files — I did not make the edit myself since neither file
is in `🛂️manifest`.

## What's genuinely done (10 types, fully serde-free)

`Version`, `MediaFingerprint`, plus a handful of the `MediaVocabulary`/`ArgFormat`-adjacent leaves
that touch no blocked external type at all. Full serde-free status is achievable for the rest only
after `ui_wgpu`/`ui-contract` (`LocalizedLabel`/`ActionDescriptor`/`IconName`/`NamedLayout`/
`WindowLayout`/`WindowOptions`/`SurfaceKind`), `🕹️interaction`, and `🎠️kernel`
(`ActivationEvent`/`CapabilityRequirement`) each gain `ToValue`/`FromValue` on their own types —
none of which is `🛂️manifest`'s call to make.

## Bugs found and fixed along the way (not part of the count, but real)

- `AppRole`/`TopicContribution` had a **stale dual-derive** (serde derive left in place alongside
  hand-written `ToValue`/`FromValue` impls from an earlier incomplete pass) — cleaned up.
- Three `#[derive(..., ToValue, FromValue, ToValue, FromValue)]` **literal duplicate-token** bugs
  (`MediaClass`/`MediaForm`/`MediaType`) from a mechanical rewrite colliding with types another
  concurrent agent had already dual-derived — fixed (E0119 conflicting impls).
- `ResourceSelector`/`MediaFingerprint` are tuple newtype structs — `#[derive(ToValue, FromValue)]`
  does not support tuple structs at all (hard compile error, not silent). Hand-wrote the impls (or,
  for `ResourceSelector`, kept `#[derive(Serialize, Deserialize)]` + `#[serde(transparent)]`, which
  *does* support tuple structs, since it needed serde back anyway).
- `NonEmptyVec<T>`/`Version`/`VersionReq` used `#[serde(into = "…", try_from = "…")]`, which
  `#[value(...)]` has no equivalent for — hand-wrote `ToValue`/`FromValue` via the existing
  `Display`/`FromStr`/`TryFrom<Vec<T>>` conversions instead of bridging through `serde_json::Value`.
- `ArtifactKindSpec.export_stdio_kinds`/`import_stdio_kinds: Vec<&'static str>` used
  `#[serde(default, skip_deserializing)]` — `#[value(...)]` only has bare `skip` (which also drops
  the field from *encode*, changing the wire shape). Wrote a `deserialize_with` shim
  (`ignore_stdio_kinds_on_decode`) that always resolves to the default, matching the original
  asymmetric behavior exactly.
- `ActionArgDef::json_schema()`/`arg_schema_json_schema()`/`apply_arg_format()` built a JSON Schema
  document via `serde_json::json!`/`serde_json::Value` — rewrote the whole builder over `DslValue`
  directly (return type is now `DslValue`, not `serde_json::Value`). Confirmed the one other
  in-repo consumer (`🌉️mcp/🗂️catalog/🦀️.rs`) is currently **orphaned** (not `#[path]`-mounted into
  any crate — grepped for `mod mcp`/`path = ".*🌉️mcp` repo-wide, zero hits), so this didn't need a
  companion fix to stay green.
- `encode_artifact_kind_choice`/`decode_artifact_kind_choice`/`encode_surface_app_choice`/
  `decode_surface_app_choice` and `ActionArgDef::default_value` rewritten onto
  `dsl::os_pack::json` (`parse`/`object`/`to_string`/`from_dsl_value`/`to_dsl_value`) instead of
  `serde_json`.
- `parse_contributions`/`TutorialDefinition::from_json` rewritten onto `dsl::os_pack::json::from_json_str`
  — except `TutorialDefinition::from_json`, which had to revert to `serde_json::from_str` once
  `TutorialDefinition` itself became blocked (see above).

## Files touched

- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (only file edited this pass).

## Not touched (by design, per scope)

- `🧰️framework/🔨️modules/🖱️ui/*` (`ui_wgpu`, `ui-contract` crates) — owns `LocalizedLabel`,
  `ActionDescriptor`, `IconName`, `NamedLayout`, `WindowLayout`, `WindowOptions`, `SurfaceKind`,
  `Locale`, `Terminology`.
- `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs` — explicitly another agent's module.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — owns `ActivationEvent`, `CapabilityRequirement`;
  also the consumer requiring `ViewModel`/`MediaType`-family to stay serde.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs` — the other consumer requiring the
  `MediaVocabulary` family (`MediaType`/`MediaForm`/`MediaClass`/`MediaWireFormat`/
  `MediaPortDirection`/`PortMultiplicity`/`MediaPortSpec`) to stay dual-derived.
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs` (`ArtifactDialect`) — not edited, see peer-churn
  note above.
- No Cargo.toml edited (rule honored).

## Recommended next step for whoever picks this up

Once `ui_wgpu`/`ui-contract` add `ToValue`/`FromValue` for the leaf types listed above, re-run this
same closure on `🛂️manifest`: the 35 fully-blocked types and the 72 dual-derived types should mostly
collapse to pure `ToValue`/`FromValue`, at which point the `Serialize`/`Deserialize` derives (and the
top-of-file `use serde::{Deserialize, Serialize};`) can come out for good. `🎠️kernel`'s
`ActivationEvent`/`CapabilityRequirement` and `🕹️interaction`'s `InteractionDefinition`/
`InteractionRef` gate a smaller residual set (`AppDefinition`'s `interactions` field family,
`ExtensionPointDeclaration`).

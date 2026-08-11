# Wave 4a — Agent 6: Framework Core (Closed `Contribution` Deletion)

## Task
Delete the closed `Contribution` enum shape and its fallback paths entirely from the manifest core
type definitions and the framework-owned files that construct/consume `PluginManifest`/
`ExtensionManifest`/`ProgramContributionEntry`. The open `TopicContribution`/`topic_contributions`
shape becomes the sole path.

## Files edited

### 1. `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
- Deleted the `Contribution` enum (7 variants: `PlaybookBlockKind`, `SourcingModule`,
  `ProcessMachines`, `FlowExtension`, `FormsQuestionKind`, `CadComputer`, `ImperativeModule`).
- Deleted `contributions: Vec<Contribution>` from `PluginManifest` (kept `topic_contributions`).
- Deleted `contribution: Contribution` from `ProgramContributionEntry` (kept
  `topic_contribution: Option<TopicContribution>` — left it `Option` rather than making it
  non-optional, per the task's guidance, since verifying every construction site workspace-wide
  is out of this agent's file ownership).
- `parse_contributions` had no direct dependency on the removed field (pure `serde_json::from_str`)
  — left as-is.
- Updated `TopicContribution`'s doc comment to drop the "coexists with `Contribution` during
  migration" framing; migration is done, it's now just "the open contribution shape".
- Removed the `crate::ui::Contribution::export().unwrap();` line from the `typegen`
  `#[test] fn generates_all_types()`-style export block (kept `ProgramContributionEntry::export()`).

### 2. `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`
- Deleted the `PluginContribution` closed union type (7-variant discriminated union mirroring the
  Rust enum).
- `ProgramContributionEntry`: dropped `contribution: PluginContribution`, added
  `topicContribution?: TopicContribution` (mirrors the Rust struct's field — previously absent
  from the TS mirror, added per the "keep topicContribution" instruction).
- `PluginManifest`: dropped `contributions?: readonly PluginContribution[]` (kept
  `topicContributions?`).
- Updated `TopicContribution`'s doc comment to drop migration-coexistence framing.

### 3. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- Removed `Contribution` from the manifest-types `use` list (top of file, `app` module).
- `Plugin` struct (the OS-side plugin builder, distinct from `ExtensionBundle`): removed its
  `contributes(Contribution) -> Self` builder method — it pushed into
  `PluginManifest.contributions`, which no longer exists. This method wasn't explicitly named in
  the task brief (only `ExtensionBundle::contributes` was), but it became dead/non-compiling the
  moment the framework-core field was deleted, so removal was required. No callers of
  `Plugin::contributes` exist in this file.
- Fixed `Plugin::new()`'s `PluginManifest { ... }` initializer to drop `contributions: Vec::new()`.
- Fixed `plugin_manifest()`'s fallback `PluginManifest { ... }` literal to drop `contributions: vec![]`.
- Removed the now-unused `Contribution` import from `plugin_runtime`'s `use semio_framework::{...}`.
- `ExtensionManifest`: deleted `contributions: Vec<Contribution>` field (kept `topic_contributions`).
- `ExtensionBundle::new()`: dropped `contributions: Vec::new()` from its `ExtensionManifest` literal.
- `ExtensionBundle::contributes(Contribution) -> Self`: deleted entirely (kept
  `contributes_topic`, whose doc comment was trimmed to drop the reference to the now-deleted
  `Self::contributes`/closed `Contribution` enum).
- `extension_manifest()`'s fallback `ExtensionManifest { ... }` literal: dropped
  `contributions: Vec::new()`.

### 4. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- Host-side `ExtensionManifest` mirror struct: deleted `contributions: Vec<Contribution>` field
  (kept `topic_contributions`).
- Its one `PluginManifest { ... }` construction site (test-support `unknown`/`Unknown` fallback):
  dropped `contributions: vec![]`.
- Removed the now-unused `Contribution` import from the top-of-file `use semio_framework::{...}`.

### 5. `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- Removed `Contribution` from the top `use semio_framework::{...}` import, added `TopicContribution`.
- This file's own local `ProgramContributionEntry` struct (a host-side wire type, distinct from the
  manifest crate's, used for `contributions_json()`): changed
  `contribution: Contribution` → `topic_contribution: TopicContribution`. This wasn't spelled out
  in the per-file instructions (which only mentioned `PluginManifest {...}` sites), but it's a
  producer/consumer pair that reads `PluginManifest.contributions` — dead the moment that field is
  gone — so it had to move to the open shape to keep the file compiling and functionally
  equivalent.
- `PluginHost::contributions()`: rewrote to iterate `loaded.manifest.topic_contributions` instead
  of `loaded.manifest.contributions`, building `ProgramContributionEntry { plugin_id,
  topic_contribution }`.
- Fixed 7 `PluginManifest { ... }` construction sites (all `#[cfg(test)] mod tests`) to drop their
  `contributions: vec![]`/`contributions: []`-style line:
  - `loads_plugin_apps_into_registry`
  - `hot_swap_bumps_instance_generation_and_tracks_app_changes` (×2 manifests)
  - `hot_swap_rollback_on_invalid_manifest_keeps_old_plugin` (×2 manifests)
  - `contributions_track_plugin_load_and_hot_swap` (×2 manifests)
- `contributions_track_plugin_load_and_hot_swap`: this test specifically exercised the closed
  `Contribution::PlaybookBlockKind` shape end-to-end (construct → `load_plugin` →
  `host.contributions()` → assert). Rewrote it in place (not deleted — it still tests real,
  live behavior, just via the open shape) to build a `TopicContribution::new("playbook.blockKind",
  json!({...}))` instead of `Contribution::PlaybookBlockKind {...}`, and to assert against
  `host.contributions()[0].plugin_id` same as before. The `topic_contribution.clone()` payload
  mirrors the same field names the closed variant had (appId/blockKind/label/iconId/
  defaultValueJson/paramsBodyKey/previewBodyKey) as freeform JSON, since `TopicContribution`'s
  payload is intentionally untyped.

### 6. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- `contributions_json_from_plugins`: the local `ProgramContributionEntry<'a>` serialize-only struct
  had `contribution: &'a semio_framework::Contribution` reading `program.manifest.contributions`.
  Same dead-on-field-deletion situation as file 5 — rewrote to
  `topic_contribution: &'a semio_framework::TopicContribution` reading
  `program.manifest.topic_contributions`.
- Fixed the one `PluginManifest { ... }` test-fixture site (`resolve_commands_tags_every_source`)
  to drop `contributions: vec![]`.

## Deviations from the literal per-file instructions (and why)

The task text for files 5 and 6 said only "remove `contributions: vec![]`/`Vec::new()` lines from
`PluginManifest {...}` sites." In both files there was additionally a local
producer/consumer pattern (a host-local `ProgramContributionEntry` struct + a function building it
from `manifest.contributions`) that directly depended on the field deleted in file 1. Since that
field no longer exists, those functions could not be left as-is — they were updated to read
`topic_contributions`/`TopicContribution` instead, consistent with the wave's overall directive
("every consumer now reads the open shape... this wave removes the closed shape... entirely").
This was the smallest change that keeps both files compiling and their behavior (contribution
tracking / `contributionsJson` wire payload) intact.

`Plugin::contributes` (file 3, the OS-side plugin builder — not `ExtensionBundle::contributes`,
which *was* explicitly named) was deleted for the same reason: it directly pushed into
`PluginManifest.contributions`, so it became non-compiling dead code the moment the field was
deleted in file 1. No in-file callers existed.

## Out of scope (left untouched, belongs to other wave-4a agents)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏗️builder/🦀️component.rs` — calls
  `plugin.contributes(contribution)` (a producer site building `Contribution::X{...}` literals) and
  imports `semio_framework::Contribution`. This is a different directory from all six assigned
  files/dirs, so per the "never touch a file outside your assigned directories" rule it was left
  alone. `cargo check -p semio-framework-plugin` and `-p semio-framework-os` both surface exactly
  the two expected errors from this file (`E0432 unresolved import Contribution`, `E0599 no method
  contributes`) — confirmed these are the *only* errors in those crates and both point into
  `🏗️builder/🦀️component.rs`, not into any file this agent owns.

## Verification

- `cargo check -p semio-framework` — **0 errors.** Compiles clean in isolation (only pre-existing
  unrelated warnings in os-kernel/etc., nothing touched by this wave).
- `cargo check -p semio-framework-plugin-host` — **0 errors.** Compiles clean.
- `cargo check -p semio-framework-plugin` — 2 errors, both in `🏗️builder/🦀️component.rs` (not my
  file — parallel-agent producer site, expected per task brief).
- `cargo check -p semio-framework-os` (the real crate name for the `💻️os/🖥️host` package —
  `project.json`/`Cargo.toml` calls it `semio-framework-os`, distinct from
  `semio-framework-os-kernel`) — build halts on the same `semio-framework-plugin` dependency error
  above (upstream of `semio-framework-os` in the dependency graph) before it can typecheck
  `semio-framework-os` itself; this is the same parallel-agent blocker, not something introduced by
  my edits. `cargo check -p semio-framework-os-kernel` (checked separately since it's the crate
  most of the workspace glue lives in) — **0 errors**, and it doesn't even touch any of my 6 files
  (confirmed no `Contribution` reference in its output).

No `[DEBUG]` logs were added (none were needed — this was a pure type/field deletion + call-site
fixup, verified via `cargo check`, not runtime behavior).

## Grep sweep confirming zero closed-shape references remain in all 6 assigned files

```
grep -n "Contribution\b" <each file>
```
- File 1: only `TopicContribution` remains.
- File 2 (`.ts`): only `TopicContribution`/`topicContribution`/`topicContributions` remain (plus
  the unrelated wire-string field `contributionsJson: string` on `PluginViewState`, out of scope —
  that's a JSON-blob field name, not the closed enum).
- File 3: only `TopicContribution` remains.
- File 4: only `TopicContribution` remains.
- File 5: only `TopicContribution`/`topic_contribution` remain.
- File 6: only `TopicContribution`/`topic_contribution` remain.

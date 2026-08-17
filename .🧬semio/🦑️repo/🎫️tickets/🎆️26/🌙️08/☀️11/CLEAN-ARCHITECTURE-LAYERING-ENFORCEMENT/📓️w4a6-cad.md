# w4a6 — cad plugin: delete closed `Contribution::CadComputer` path

## Scope
`✏️s/🔌️plugins/📐️cad/` subtree: plugin root, its 4 `🧩️extensions/`, and the engine consumer
`🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`.

## Files edited

1. `🧩️extensions/🏢️aec-building/🦀️component.rs`
   - Removed `use semio_framework::Contribution;`.
   - Removed `.contributes(Contribution::CadComputer{...})` from `bundle()`; kept `.contributes_topic("cad.computer", ...)` as sole contribution.
   - Rewrote `bundle_contributes_building_import_profile` test to read `manifest.topic_contributions[0]` (topic + `payload["moduleId"]` / `payload["computersJson"]`) instead of destructuring `Contribution::CadComputer`.

2. `🧩️extensions/📐️spatial-shape/🦀️component.rs`
   - Same producer-site removal.
   - Rewrote `bundle_contributes_spatial_shape_for_cad_play` test: `manifest.contributions.len()` → `manifest.topic_contributions.len()`, closed-enum destructure → `topic_contributions[0]` + payload field reads.

3. `🧩️extensions/🏛️aec-building-structure/🦀️component.rs`
   - Same producer-site removal.
   - Rewrote `bundle_contributes_structure_manifest` test to use `topic_contributions[0].payload["computersJson"]`.

4. `🧩️extensions/🔥️aec-building-energy/🦀️component.rs`
   - Same producer-site removal.
   - Rewrote `bundle_contributes_energy_computers` test to use `topic_contributions[0].payload["computersJson"]`.

5. `🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
   - `cad_computer_fields`: deleted the closed `Contribution::CadComputer` fallback branch (the second half of the function, gated on `entry.contribution`). The function now short-circuits on `entry.topic_contribution` alone (`"cad.computer"` topic, `CadComputerTopicPayload` decode) — no behavior change for the already-migrated producers, since every producer now only emits the open shape.
   - Removed `Contribution` from `use semio_framework::{parse_contributions, Contribution, MeshImporter};` (now unused).
   - `sync_cad_computer_contributions` unchanged (already called through `cad_computer_fields`, only the helper's internals changed).
   - No `#[cfg(test)]` in this file referenced `cad_computer_fields`/`Contribution::CadComputer`, so no test fixtures needed updating here.

## Grep verification
`grep -n "Contribution\b"` across all 5 files after edits: zero matches for the type/enum (one remaining docstring mentions `Contribution::CadComputer` only as historical context in a comment, harmless).

No other test fixtures constructing `Contribution::CadComputer{...}` exist anywhere else under the assigned directories (checked via `grep -rl "CadComputer"` across the whole `📐️cad` plugin tree before editing — the only hits outside my 5 files are
`🎛️apps/📐️cad/🎮️commands/🧩️contribution/🦀️component.rs` and `🔨️modules/🏃️runtime/🟦️component.ts`, both **out of scope** per the assignment boundary — apps/commands and modules/runtime are not in the 4 extensions dirs or the named engine file, so left untouched for their owning agent).

## Verify: `cargo check -p semio-s-plugin-cad -p semio-s-plugin-cad-aec-building -p semio-s-plugin-cad-aec-building-structure -p semio-s-plugin-cad-aec-building-energy -p semio-s-plugin-cad-spatial-shape`

Result: **fails**, but not because of anything in my assigned files. Both errors are inside
`semio-framework-plugin`'s own builder module, a dependency crate outside my assignment:

```
error[E0432]: unresolved import `semio_framework::Contribution`
 --> 🧰️framework/…/🔌️plugin/🏗️builder/🦀️component.rs:4:73
  | use semio_framework::{kernel::CapabilityRequirement, CommandDefinition, Contribution};
  |                                                                         ^^^^^^^^^^^^ no `Contribution` in the root

error[E0599]: no method named `contributes` found for struct `component::app::Plugin`
 --> 🧰️framework/…/🔌️plugin/🏗️builder/🦀️component.rs:166:29
  | plugin = plugin.contributes(contribution);
```

`semio_framework`'s manifest module (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`) has already had the
`Contribution` enum and `PluginManifest.contributions`/`ProgramContributionEntry.contribution` fields fully
deleted (confirms this wave's premise — the framework-side type deletion has landed there). But
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` still imports `Contribution` and calls
a `Plugin::contributes` method that no longer exists on `component::app::Plugin` — that file is evidently
mid-edit by a different concurrent agent (framework/plugin builder, not `📐️cad`). Grepped to confirm: zero
occurrences of `Contribution`/`CadComputer` remain in any of my 5 assigned files; the compile error's file
path is entirely outside `✏️s/🔌️plugins/📐️cad/`. Per operating rules, noted and left alone rather than fixed.

## Summary of files touched (created/updated/removed)
- Updated: `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- No files created or removed.

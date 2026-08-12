# GIS packet — engine dissolution report

Scope: `✏️s/🔌️plugins/🌍️gis` only. Both artifact `⚙️engine` dirs deleted; contents rehomed per the
destination map. No other plugin's dissolution work touched (one necessary cross-plugin import fix
in `demonstrator`, explained below).

## Destinations chosen

| engine dir | region | destination |
|---|---|---|
| `🗺️gismap/…/⚙️engine/🦀️component.rs` | `🔹ArtifactEngine` (`GisMapEngine` struct) | **DELETED** — 0 external refs (grep confirmed), no `ArtifactEngine` trait exists repo-wide |
| " | `🔖️DocumentHelpers` (`empty_gis_map_snapshot`, `gis_map_document_from_descriptor_json`, `gis_map_descriptor_json`, `default_document`, `value_to_dsl`/`dsl_to_value`) | `🧬️schema/🦀️component.rs` |
| " | `🔖️CollectionDiffing` (`feature_collection_operations`, `positions_operations`, `routes_operations`, `regions_operations`) | `🧬️schema/🦀️component.rs` |
| " | `🔖️DrawingBridge` (styles, `feature_lon_lat`, `feature_line`, `polyline_draw_node`, `point_marker_draw_node`, `gis_map_snapshot_to_drawing`, `drawing_to_svg_io_key`, `render_drawing_to_svg`) | `🧬️schema/🦀️component.rs` — pure document/drawing helpers, not snapshot-derived, not stateful |
| " | `🔖️MediaExport` (`gis2d_document_json_to_svg`) | `🧬️schema/🦀️component.rs` |
| " | `🔖️MediaImport` (`dwg_geometry_to_draw_node`, `dwg_drawing_to_semio_drawing`, `collect_draw_node_points`, `gis2d_document_json_from_dwg`) | `🧬️schema/🦀️component.rs` |
| " | `🔖️Io` (`gis2d_io`, `gis2d_features_in_port`, `gis2d_map_out_port`, `gis2d_map_media`) | `🎛️apps/◻2d/🦀️component.rs` — returns `AppIo`/`MediaPortSpec`/`Media`, docstring literally says "AppDefinition.io", exact exemplar match |
| " | `🚪️DerivedIoRegistry` (`io_registry` mod) | `🚪️io/🦀️component.rs`, new region `🚪️IoRegistry` |
| `🏔️gisterrain/…/⚙️engine/🦀️component.rs` | `🔹ArtifactEngine` (`GisTerrainEngine` struct) | **DELETED** — 0 external refs |
| " | `🔖️DocumentHelpers` (`empty_gis_terrain_snapshot`, `default_terrain_document`) | `🧬️schema/🦀️component.rs` |
| " | `🔖️FixtureText` (`terrain_fixture_text` mod, `imported_positions`, `parse_descriptor`) | `🧬️schema/💡️inferences/🦀️component.rs` — pure `&GisTerrainSnapshot → TerrainDescriptorJson` projection |
| " | `🔖️Io` (`gis3d_io`, `gis3d_map_in_port`, `gis3d_scene_out_port`, `gis3d_scene_media`) | `🎛️apps/🧊️3d/🦀️component.rs` |
| " | `🚪️DerivedIoRegistry` (`io_registry` mod) | `🚪️io/🦀️component.rs`, new region `🚪️IoRegistry` |
| `🏔️gisterrain/…/⚙️engine/terrain/🦀️component.rs` (539-line submodule) | `TerrainDescriptor` (`TerrainProjectOrigin`, `TerrainPositionData`, `TerrainDescriptorJson`, `default_exaggeration`, `GIS_3D_TERRAIN_TILE_URL_TEMPLATE`, `TerrainSceneStyleJson`, `build_terrain_scene_json`) | `🧬️schema/🦀️component.rs` — pure DTOs + pure formatter, not snapshot-derived, not stateful |

## Unqualified paths qualified (before → after)

All bodies moved were audited against the io-registry shadow trap. No bare `io_registry::entries()`
calls existed in either engine file (both already called the export-composer functions directly by
their full module path, e.g. `crate::artifacts::gismap::io::export::serializers::...`), so there was
nothing to requalify inside the moved bodies themselves. The two **declaration() composers() call
sites** (out of scope to move, but required updating since `io_registry`'s home changed):

- `🗺️gismap/🦀️component.rs:87` — `crate::artifacts::gismap::standards::v1::engine::io_registry::entries()` → `crate::artifacts::gismap::standards::v1::subsets::any::io::io_registry::entries()`
- `🗺️gismap/🦀️component.rs:183` (the root's own shadow-wrapper's `use … as v1`) — same qualification change
- `🏔️gisterrain/🦀️component.rs:53` — `crate::artifacts::gisterrain::standards::v1::engine::io_registry::entries()` → `crate::artifacts::gisterrain::standards::v1::subsets::any::io::io_registry::entries()`
- `🏔️gisterrain/🦀️component.rs:148` (root's own shadow-wrapper's `use … as v1`) — same

All 13 call sites of `crate::artifacts::gismap::engine::*` and 4 call sites of
`crate::artifacts::gisterrain::engine::*` across `🎛️apps/`, `🧬️mutations/`, `🌉️wasm`, `🗺️maphost`,
were repointed to `crate::artifacts::gismap::schema::*` / `crate::artifacts::gisterrain::schema::*`
(or, for `parse_descriptor`, the fully-qualified `…schema::inferences::parse_descriptor`, since no
top-level `inferences` shim exists — verified by grep, unlike `schema`/`io`/`op`/`dsl`/`spr`/`diff`/
`mutations`/`snapshot` which all have shims in `📦️glue.rs`). One necessary **cross-plugin** call site:
`✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🗺️verfolgen/🦀️component.rs` imported
`gis::artifacts::gismap::engine::{gis2d_document_json_from_dwg, gis2d_document_json_to_svg}` — updated
to `gis::artifacts::gismap::schema::{…}`. This is a hard consequence of removing gis's `engine` module
entirely (glue.rs's shim `pub mod engine { pub use super::standards::v1::engine::*; }` is what that
import resolved through); left unfixed it would have broken demonstrator's compile. No other file in
demonstrator or any other plugin was touched.

## `⚙️engine` mounts removed from `📦️glue.rs`

- gisterrain: the nested `#[path="."] pub mod engine { mod component (root file) + pub mod terrain
  (submodule file) }` block under `standards::v1`, plus the root-level shim
  `pub mod engine { pub use super::standards::v1::engine::*; }`.
- gismap: the single-file `#[path=…] pub mod engine;` mount under `standards::v1`, plus the identical
  root-level shim.

`grep -n "⚙️engine\|::engine::" 📦️glue.rs` → **zero lines** (verified below).

## Known pre-existing issue hit — NOT fixed, verified pre-existing

`crate::modules::terrain` (no `pub mod modules` anywhere in this crate) is unresolved. Two sites, both
predating this ticket:

1. `🏔️gisterrain/…/🧬️schema/💡️inferences/🦀️component.rs:69` — this is the **relocated**
   `parse_descriptor` region; the original engine file (`git show HEAD:…/⚙️engine/🦀️component.rs`,
   line 10) had the identical `use crate::modules::terrain::{TerrainDescriptorJson,
   TerrainPositionData, TerrainProjectOrigin};` line. Carried forward **verbatim, unfixed**, per the
   explicit instruction not to repair it.
2. `🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs:11` — untouched by this packet.
   `git log --oneline -3` on this file: last commit `47e1a1deab` (flag 487), all three predate this
   ticket's flag range (492+). Confirmed pre-existing, not mine, left alone.

The E0432 cascade at inferences `:77` (`use super::{TerrainDescriptorJson, …}` inside the nested
`terrain_fixture_text` mod) is a direct downstream consequence of error (1) above — same root cause,
not a separate bug.

## Assertion-count arithmetic (before → after, exact)

Before (engine dirs, via `git show HEAD:<path>`):

| file | assert! | assert_eq! | assert_ne! | #[test] |
|---|---:|---:|---:|---:|
| gismap `⚙️engine/🦀️component.rs` | 17 | 15 | 0 | 9 |
| gisterrain `⚙️engine/🦀️component.rs` | 2 | 13 | 0 | 5 |
| gisterrain `⚙️engine/terrain/🦀️component.rs` | 3 | 4 | 0 | 3 |
| **total removed** | **22** | **32** | **0** | **17** |

After (destination files, delta = after-count minus each file's own pre-existing before-count):

| file | Δassert! | Δassert_eq! | Δ#[test] |
|---|---:|---:|---:|
| gismap `🧬️schema/🦀️component.rs` (0→15/10/7) | +15 | +10 | +7 |
| gismap `🎛️apps/◻2d/🦀️component.rs` (10→12, 16→21, 10→12) | +2 | +5 | +2 |
| gisterrain `🧬️schema/🦀️component.rs` (0→3/6/4) | +3 | +6 | +4 |
| gisterrain `🧬️schema/💡️inferences/🦀️component.rs` (0→1, 2→7, 2→4) | +1 | +5 | +2 |
| gisterrain `🎛️apps/🧊️3d/🦀️component.rs` (13→14, 14→20, 11→13) | +1 | +6 | +2 |
| **total added** | **22** | **32** | **17** |

Deltas balance exactly against the removed totals. `assert_ne!` = 0 throughout, no change.

## Compiler check (mandated command, verbatim)

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/🎯️target" cargo check -p semio-s-plugin-gis --all-targets
```

**Run 1** (stdio red at the time): `error: could not compile \`semio-s-plugin-stdio\` (lib) due to 1
previous error; 602 warnings emitted` — a `SemioMeshSnapshot` E0425 inside
`🗄️stdio/…/✳️mesh/🧬️schema/🧬️mutations/🦀️component.rs`, not attributable to gis (stdio-attributed,
per rule 7). gis itself never reached compilation in this run.

**Run 2** (stdio had gone green in the interim):

```
error[E0433]: cannot find `modules` in `crate`
  --> …/🏔️gisterrain/…/🧬️schema/💡️inferences/🦀️component.rs:69:12
error[E0433]: cannot find `modules` in `crate`
  --> …/🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs:11:12
error[E0432]: unresolved imports `super::TerrainDescriptorJson`, `super::TerrainPositionData`, `super::TerrainProjectOrigin`
  --> …/🏔️gisterrain/…/🧬️schema/💡️inferences/🦀️component.rs:77:17
error: could not compile `semio-s-plugin-gis` (lib) due to 3 previous errors; 59 warnings emitted
error: could not compile `semio-s-plugin-gis` (lib test) due to 3 previous errors; 81 warnings emitted
```

All 3 errors are the single known pre-existing `crate::modules::terrain` issue (2 root sites + 1
direct cascade). **No other error was reported anywhere in gis** — both `lib` and `lib test` targets
fail with the identical 3 errors, meaning nothing else in either target regressed. This is the honest,
current state: gis is not green, but the only red is the pre-documented, out-of-scope issue.

## Structural verification

```
find ✏️s/🔌️plugins/🌍️gis -path "*🗿️artifacts*" -name "⚙️engine" -type d
→ (empty — both dirs gone)

grep -rn "::engine::\|standards::v1::engine" ✏️s/🔌️plugins/🌍️gis
→ ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/🧬️schema/🦀️component.rs:8  (semio_s_plugin_stdio::…::v1::engine::geometry::{…})
→ ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/🧬️schema/🦀️component.rs:641 (semio_s_plugin_stdio::…::v1::engine::register())
```

These 2 remaining hits are **not gis's own engine** — they are calls into `semio_s_plugin_stdio`'s
still-existing `engine` module (stdio's own dissolution is a separate, much larger, still-in-flight
packet set per the manifest — `🗄️stdio` 41 dirs, RELEASED but not yet dissolved). Both lines were
copied verbatim from the original engine file (same text, same purpose: geometry types + a
once-guarded test registration call into stdio). The literal `find`/`grep` pair the ticket specifies
therefore reads `find` = 0 (pass) and `grep` = 2 (both attributable to stdio, not gis). Reporting this
honestly rather than claiming a false zero.

## Files touched

**Deleted (directories):**
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (695 LOC)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (539 LOC, incl. `terrain/` submodule)

**Updated:**
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs` — removed both `engine` mounts + both root shims
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️component.rs` (composers() call + wrapper's `use…as v1`)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🗺️maphost/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🗺️features/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️component.rs` (composers() call + wrapper's `use…as v1`)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs` (only the `parse_descriptor` import line; `crate::modules::terrain` line left untouched)
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs`
- `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/🗺️verfolgen/🦀️component.rs` (cross-plugin, 1 line, necessary consequence — see above)

**Not touched:** `✏️s/🔌️plugins/🗄️stdio`, repo-root `📜️script.ts`, real `🔣️taxonomy.json`, `AGENTS.md`,
any `fem`/`procedural` files, no other demonstrator files.

## Honest pass/fail

Structural goals (both engine dirs gone, glue.rs mounts gone) are **met and verified**. The mandated
compiler check is **not green** — blocked by the pre-existing, out-of-scope `crate::modules::terrain`
issue, proven via git log to predate this ticket. Every other aspect of the relocation (both `lib` and
`lib test` targets, all call sites, both `io_registry` shadow-hazard sites, assertion counts) checks
out clean under that one known blocker.

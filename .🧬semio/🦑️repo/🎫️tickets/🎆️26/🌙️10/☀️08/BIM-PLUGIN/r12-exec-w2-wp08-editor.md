# r12-exec-w2-wp08-editor

Status: **WRITTEN, NOT COMPILED, NOT TESTED.** The framework crates (`semio-framework-plugin`, `os-infinite`, owner commit 677 in flight) did not
compile during the whole session. A gate poll ran every ~5 minutes for about an hour; the last run (`cargo check -p semio-s-artifact-bim-model --lib --message-format=short`, exit 101) showed
15 errors, all in `🧰️framework/.../🔌️plugin` (`WindowConfigSnapshot` / `WindowTransientSnapshot` without `clone`, 2-vs-1 argument calls) and none under `✏️s/`.
The BIM crate itself was therefore never reached. No test was run, so no pass count exists.

S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, A = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`.

## Files created (under S/✏️editor)
- `🧩️entities/🧷️wall-sweeps/🦀️.rs`: `wall-sweep` kind rows (`SWEEP_FIELDS`: name, host, side, profile, height, inset, material), `create_wall_sweep` (baseboard, left face, first wall/material), pickers
  (`wall_choices`, `slab_choices`, `surface_choices`), wall rows `top_attach` / `base_slab` (read and write fns), opening rows `reveal_depth` / `reveal_material`.
- `🧩️entities/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs`: 9 tests (create defaults, tree parent and storey, sparse writes and read-back, profile text, inferred rows, top attach with offset kept and free, base slab, pickers en/de, reveal rows).
- `🧵️gestures/🧷️sweep/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`: sweep tool (`Sweeping`, `nearest_wall`), 6 tests (side from the pointer, library profile and material, ghost band, no wall / no material, wall search, inferred length).
- `🎮️commands/🔗️attach-walls/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`: `attachWalls` (`AttachWalls { ids, target }`), 5 tests.

## Files edited
- `🧩️entities/🦀️.rs`: `top_text` / `parse_top` for roof, slab, ceiling (`roof r-main 0.1`); `profile_text` / `parse_profile` now speak `rect w x d` (also `rectangle w × d`), `circle d`, `i w x d web t flange t`, `custom x, y; …`, `family f`;
  module mount; wall rows `top_attach`, `base_slab`; opening rows `reveal_depth`, `reveal_material`; `wall-sweep` table row; `storey_of` arm; wall inferred rows `elevation` and `top_elevation`.
- `🧩️entities/🧪️tests/🔬️unit/🦀️.rs`: stale kind counts replaced, parent arm `wall-sweep`, 2 new tests (top text, profile text).
- `🦀️.rs` (editor root): command rows `armSweep`, `attachWalls`, bridge arms, `bim_action_args`, 4 fault notices (`bim.attach.wall-missing`, `bim.attach.target-missing`, `bim.create.wall-missing`, `bim.create.material-missing`), key `shift+r`, import.
- `🪛️utilities/🦀️.rs`: utility `sweep` (icon `baseline`, group `structure`, `PLAN_WORLD`, `shift+b`, `armSweep`); `🎮️commands/🛠️arm-utility/🦀️.rs`: `ArmSweep`.
- `🧵️gestures/🦀️.rs`: `sweep` mount and `build_tool` arm; `🧵️gestures/🧪️tests/🔬️unit/🦀️.rs`: `"sweep"` in the no-preview list, missing `Surface::Sheet` arm of `Rig::on` added (existing test-build break).
- `🗣️terminology/🦀️.rs`: 27 labels en+de (kind, group, fields, utility, 2 commands + describes, `arg_target`, 2 actions, 4 faults, 2 diagnostic categories).
- `🎮️commands/🏗️create-entity/🦀️.rs` (`wall-sweep` container is a wall), `🎮️commands/🗑️delete-selection/🦀️.rs` (sweeps leave with their host), `🕹️interaction/🦀️.rs` (sweeps are element-domain nodes under their wall),
  `📌️panels/🌳️outliner/🦀️.rs` (sweeps nested under their wall, no storey group), `📌️panels/🔍️properties/🦀️.rs` (action rows "Attach to roof" when the selection also holds a roof/slab/ceiling, "Free the top" for an attached wall;
  both via the existing commands), `📌️panels/🚨️diagnostics/🦀️.rs` (categories `wall`, `wall-sweep`).
- Tests extended: properties (2), outliner (1), interaction (1 + granularity counts now derived from `ENTITIES`), completeness (1 reachability + labels + notices), unit (`every_command()` rows, bridge asserts).
- A: mount row `attach_walls` in `🦀️.rs` (command mount block).

## Decisions and open items
- The properties panel already prepends a "None" choice to every picker, so the attach/base pickers list only the targets (no extra "free" entry). The choice fns cannot see the entity, so the lists hold every slab/roof/ceiling of the model, not only of the wall's building; the model refuses a foreign building (`mutation.invalid`).
- "Free the top" is `setField top_attach ""`, which writes `SetWallTop(StoreyTop{0})`; attaching keeps the offset of an attached wall, else 0.
- The sweep tool takes the profile from a selected beam or column type of the library (else the baseboard) and the material from the selected material (else the first).
- Stale tests I met and fixed minimally: entity kind counts (14/8), granularity counts, `Rig::on` missing `Surface::Sheet`. `every_command()` in the unit tests still lacks other peers' rows (ramp, ceiling, annotation arms ...) and will fail until they add them.
- `diag_cat_*` test needs labels for `column`, `family`, `classification`, `property` (peers' categories); I added only `wall` and `wall-sweep` (also `curtain-wall` appeared meanwhile).
- To do once the framework compiles: `cargo check -p semio-s-artifact-bim-model --lib`, fix any type error in the files above (all code was written blind), then `cargo test --lib editor::` and the wasm32-wasip2 check.
- Max path length of the editor tree: 175 characters.

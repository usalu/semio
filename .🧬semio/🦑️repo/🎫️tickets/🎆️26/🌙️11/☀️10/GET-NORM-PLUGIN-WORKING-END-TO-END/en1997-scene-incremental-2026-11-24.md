# EN 1997 scene: incremental correctness and rebuild cost (2026-11-24)

## 1. Incremental differs from full (correctness)
- Cause: the checks of a family without subject (for example `en1997.gi-01.*`, `en1997.layer.model`) belong to a row only while that row is the sole row of its collection (`owner_entity`: `rows.len() == 1`). Adding the clone `added-1` flips `footing-F1` from owner to non-owner, but the fragment key of `footing-F1` did not move: its row, its pose and the report rows of its subjects are unchanged. The cached fragment kept the owned annotation (`global-checks` style highlights). The unfiled report hid it, since its checks have no subject.
- Fix: the sole-row flag joins the layout stamp (the part of the fingerprint that names what an entity derives from its siblings): `layout_stamps` of the en1997 kit appends `|{sole}` to every row and to the layers.
- Same defect pattern in two more families (`owner_entity` with a `len() == 1` rule): en1990 (`kit::sole_rows`, stamps in the scene `layout_stamps`) and en1993 (`layout_stamps` of the members kit). Fixed the same way. The other families do not route by row count.

## 2. Unrelated edit rebuilt 31 fragments (limit 8)
- Cause: two coupled causes.
  1. Soil layers carried the site bounds (`layout.min/max/depth`) in their layout stamp. A placement moves the bounds, so every layer fragment rebuilt (legitimate, a layer draws the site), but the stamp is also what dependents hash.
  2. `entity_dependencies` of a layer listed lab tests, investigations, layer waters, bases, footings and slopes (a proxy for "the bounds depend on them"), so footings, piles, slopes, pile groups and labs (which read layers) formed a cycle with the layers and every one of them read the moved footing transitively. Result: footings, pile, slope, group, lab, water and both units rebuilt.
- Fix:
  - New `SceneKit::site_stamps` / `StructuralProjection::site_stamps` (framework): what only the entity's own fragment reads of the whole site. It joins the entity's fragment key, never the fingerprint dependents read. It is computed for all entities on every pass (one `Layout::of`), so delta passes cannot go stale.
  - en1997: layers, layer waters, characteristic bases and `unit-groundwater` take the bounds from `site_stamps`; `layout_stamps` keeps the layer top, the pose and the sole flag.
  - `entity_dependencies` of a layer is empty (a layer fragment reads its own row, the site bounds and its own checks). The two unit-test assertions that required the reverse edges now assert an empty dependency list.
- Result: `an_unrelated_edit_rebuilds_only_the_edited_fragment` passes (misses within 8).

## 3. Drag latency
- Profile (`sample` of the test process, 2498 samples): 73 % of a drag step in `assemble_with` -> `StructuralScene::validate` -> `SceneEntity::check` -> `compute_hash` (canonical JSON of every entity of the scene), 49 % of it in the hash alone. After item 2 only 2 to 3 fragments rebuilt per step; the rest was re-hashing 60 cached footings.
- Fix: the seal law (`contentHash` matches the content) is checked once when a fragment is built (`Fragment::seal_violations` in the miss closure of the projector); the per-pass validation of the assembled scene runs every other law (`StructuralScene::validate_sealed`, `assemble_sealed_with`). `validate()` and `assemble_with` keep checking the seal of every entity. No budget changed and no gating added (gating is not the convention here; the budget is met).
- Numbers (debug, prototype profile, run `scene-live-4`): scene-drag p50 123 ms, p95 168 ms (budget 240 ms; before: 214/245 after items 1 and 2, 1659 before); scene-commit p95 714 ms (budget 8000 ms); scene-cold 1290 ms under load (budget 20 s).

## Python oracle tests
- `SEMIO_PYTHON` is set by `withProvisionedPythonV1(env, workspaceRoot)` in the Cargo test admission, from the environment `.cache/b/d/m/python/environments/workspace-3b97251b6599d245` (exists, numpy/scipy/shapely import). In my plain `repo cargo ... test` runs (`scene-full-5`) the three oracle tests still reported `SEMIO_PYTHON is unset and no ancestor .venv exists`, so the wrapper did not inject it (the resident repo process may still run the pre-fix TypeScript, or its workspace root resolves outside the checkout; not investigated further).
- With `SEMIO_PYTHON=<that interpreter>` exported before `repo cargo`, the oracle tests run and pass: `contact_pressure_stations_integrate_to_the_vertical_action_like_numpy`, `numerics_match_the_numpy_scipy_shapely_oracle`.
- `flow_net_lines_exist_only_with_a_head_and_carry_the_finite_difference_exit_gradient` needs no interpreter; it compares against a hard-coded scipy value that was stale (`0.14230...`). A scipy sparse solve of the identical stencil (head 3, embedment 6, thickness 16, 8 cells per wall, 102 x 21 cells) gives `0.1468406970622835`, equal to the Rust solver (`0.147`, 2142 cells). The constant in the test is updated to that value.

## Changed files
- Framework (`✏️s/🔌️plugins/📕️norm/🎬️structural-scene`): `🎯️projection/🦀️.rs` (`site_stamps`, `sites`, sealed assembly), `🎯️projection/🧲️scene-kit/🦀️.rs` (`SceneKit::site_stamps`), `🎯️projection/🧱️scene-builder/🦀️.rs` (`Fragment::seal_violations`, `assemble_sealed_with`), `🧬️schema/🧩️entities/🦀️.rs` (`check_content`, `check_seal`, `validate_sealed`).
- en1997 scene: `🧰️kit/🦀️.rs` (`layout_stamps`, `site_stamps`, layer dependencies), `🧱️core/🦀️.rs` (`site_stamps`), `🧪️tests/🔬️unit/🦀️.rs` (two dependency assertions, flow-net oracle constant).
- en1990 scene: `🧰️kit/🦀️.rs` (`sole_rows`), `🦀️.rs` (`layout_stamps`). en1993: `🎬️scene-members/🧰️kit/🦀️.rs` (`layout_stamps`).
- Logs under `🗑️generated/scene-*.log`, `rerun-*.log`, `sample.txt` (delete on close). Script `run-scene-reruns.sh` kept.

## Test lines
- en1997 scene live law (`scene-live-4`): 3 passed (`incremental_equals_full_on_random_edit_sequences`, `an_unrelated_edit_rebuilds_only_the_edited_fragment`, `a_drag_preview_stays_within_the_budget_on_a_large_document`).
- en1997 scene full (`scene-full-5`, before the oracle constant): 57 passed, 3 failed (2 no interpreter, flow net stale constant); with `SEMIO_PYTHON` (`rerun-en1997`): 59 passed, 1 failed (flow net, now fixed). Final run: `scene-final-7`, see the last section.
- Shared code reran: `semio-s-plugin-norm-structural-scene` 75 + 19 passed, 0 failed. en1990 scene: library compiles with the change, but its unit tests do not compile for reasons outside this work (`Series` not imported, moves out of a shared reference in `🧪️tests/🔬️unit`). en1993 scene: refused before compiling ("exceeds its source boundary: 8 of 32 files, 396338 of 262144 bytes"), not caused by this change. Other kit families (en1991, en1992, en1994 to en1999, din*) use the sealed assembly through the shared projector; their suites were not rerun here.

## Final run (`scene-final-7`, `SEMIO_PYTHON` exported)
- `semio-s-artifact-norm-en1997-scene`: 60 passed, 0 failed (including the three Python oracle tests).
- Latency (debug): scene-drag p50 100 ms, p95 148 ms (budget 240 ms); scene-commit p95 1094 ms (budget 8000 ms); scene-cold 2123 ms (budget 20000 ms, under load).

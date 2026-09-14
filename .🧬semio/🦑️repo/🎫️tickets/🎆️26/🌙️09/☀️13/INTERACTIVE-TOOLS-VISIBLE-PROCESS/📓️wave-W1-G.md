# 🧥️ Wave W1-G: Fill Planner Own-Mesh Collisions

This is lane W1-G. It follows up `📓️wave-W2-C.md` §7.2.

**Status: landed.** The fill suite is green at threads 4. At threads 1 it had one timing failure under load, and that test then passed twice when run alone (§3). Native and wasm32-wasip2 checks pass.

Paths:
- `P` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill`
- `T` = this ticket folder

Logs are in `T/🗑️generated/W1-G/`.

## 1. Result

- **The defect.** The fill planner looked up each document body's collision mesh by the body's *kind*, not by its own `meshUrl`. The revalidate job did the same. So a body whose own mesh differs from its kind's mesh was tested with the wrong shape, and candidates docking into it read `fits`.
- **The fix was already in place when I checked.** Before my first test run, a concurrent peer had switched both sites to W2-C's `resolve_placed_object_mesh_url`. That peer edit is uncommitted and landed around 10:51, together with W0-I's `retarget`/`rebind`/`first_key` changes:
  - `P/🦀️.rs:1468`, `FillBuilder::prepare_entry_one`
  - `P/🦀️.rs:3076`, `FillRevalidateJob::prepare_head_one`
- **My source change is one line of logic.** The revalidate job still had a `kind_catalogs == None` branch that used the raw `object.mesh_url`. That skipped the resolver's trim and fixture-kind fallback. It now calls the same resolver with an empty `KindCatalogBundle`, which is the planner's own rule when no catalogs exist (`FixedCatalogOwner` is empty).
  - `P/🦀️.rs:3075-3076`
  - import `KindCatalogBundle` at `P/🦀️.rs:18`
- **Revalidate uses the same resolver as the run job.** Both jobs get the same roots from `🛠️tools/🪣️fill/🦀️.rs:212-216`.
- **Other kind-based lookups are correct.** The only remaining `resolve_object_kind_mesh_url` calls are for *new* candidates, where the kind's mesh is right (`P/🦀️.rs:1970`, brush `:526`, `:543`). The precompute broad-phase (`⏳️precompute/🦀️.rs:646`) already used the resolver.

## 2. Tests (red proof first)

### 2.1 Language-neutral fixture

The fixture `P/🧫️fixtures/🎞️fill-run.json` gains a new law, `laws.ownMesh`:

- **Scene:** one host vortex and two compatible kinds.
- **Blocker:** a body of the host's kind, carrying its own cube mesh, placed where both candidates dock.
- **Kind mesh:** the same cube shifted `kindMeshOffset` = 40 along x.
- **Case `clear`:** `success:fits`, 1 locked, revalidation conflicts `[false]`.
- **Case `own-mesh-blocker`:** `danger:solid-overlap` ×2, 0 locked, stall `no-free-placement`, revalidation conflicts `[true]`.

### 2.2 New test

`fill_run_and_revalidation_collide_with_a_placed_body_carrying_its_own_mesh` (`P/🧪️tests/🔬️unit/🦀️.rs:1497`).

- It covers both the run job and `FillRevalidateJob`. The revalidation re-tests the clear run's provisional placement against each case's head.
- Helpers: `own_mesh_cube`, `own_mesh_kind`, `own_mesh_roots`, `own_mesh_lane` (`:1436-1487`).

### 2.3 Parry3d oracle extended

- The oracle body is now `fill_run_parry3d_tally` (`:925`), returning a `FillRunOracleTally` (`:909`).
- It builds each document body's hull from the mesh that body renders (own `meshUrl` first). It does this itself, not from the planner's `placed` list, which had agreed with the defect.
- It asserts that every document body is a collision body.
- The test `fill_run_job_collision_verdicts_agree_with_the_parry3d_oracle` (`:984`) now runs three documents:

| Document | Seed | Requested | Own-mesh scale | Records checked |
|---|---|---|---|---|
| Concrete Forest (shipped law, unchanged) | 7 | 60 | none | all |
| Concrete Forest own-mesh variant | 7 | 60 | 1.5 | first 2000 |
| Nakagin own-mesh variant | 1 | 12 | 0.5 | first 600 |

- In the variants, every second body carries `/own-mesh/body.glb` at the given scale of its kind's box (`own_mesh_fill_roots`, `:889`). The scale and record limits are in `parryOracle.ownMeshVariants`.
- Assertions: 0 disagreements for each document, at most 10 % ambiguous for each document, and collisions and fits both decided across all documents.

### 2.4 Red proof

`T/🐍️w1g-red-proof-swap.py revert` put the two sites back to kind-only lookup; `restore` put the fix back.

| Test | Result on the reverted code | Log |
|---|---|---|
| Fixture test | `own-mesh-blocker` read `success:fits`, locked 1, revalidation `[false]` | `test-red-3.txt` |
| Oracle | `concrete-forest seed 7 own-mesh: 2 of 204 decisive verdicts disagree with parry3d` | `test-red-3.txt` |

After `restore`, both pass.

## 3. Commands run (foreground, repo root)

| Command | Result | Log |
|---|---|---|
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- fill --test-threads=4` | **50 passed, 0 failed**, including the 2 ms step law and the oracle | `test-fill-t4.txt` |
| same with `--test-threads=1` | 49 passed, 1 failed: `fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin`, worst step 2.56 ms at load average 35 | `test-fill-t1.txt` |
| `… -- fill_run_job_step_and_overlay_append_stay_below --test-threads=1`, run twice | **1 passed** each time | `test-step-law-t1-1.txt`, `-2.txt` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --lib --tests` | ok, 130 crate warnings, none in `P` | `check-native.txt` |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --target wasm32-wasip2` | ok, 94 crate warnings, none in `P` | `check-wasip2.txt` |

**The threads-1 failure is timing under machine load, not this change.** My edit only touches the once-per-body preparation stage, not per-candidate stepping. The law passes alone, and it passed in the threads-4 run.

## 4. Commands to register in launch.json

- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- fill_run_and_revalidation_collide_with_a_placed_body_carrying_its_own_mesh`
- `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- fill_run_job_collision_verdicts_agree_with_the_parry3d_oracle --test-threads=1` (parry3d oracle, own-mesh variants)

## 5. Deviations

1. **I did not author the two resolver call-site swaps.** A peer landed them just before my first test run. I kept them unchanged, wrote the tests, proved them red against kind-only lookup, and unified the remaining `None` branch.
2. **The Nakagin variant uses scale 0.5, not 1.5.** At 1.5 the Nakagin run stalls silently with 0 verdicts (see open item 1), so the oracle would have nothing to check.

## 6. Foreign edits

- `P/🦀️.rs` (W0-I is adopting edits in the same file): the revalidate `None` branch plus one import, and nothing else.
- `P/🧫️fixtures/🎞️fill-run.json` and `P/🧪️tests/🔬️unit/🦀️.rs` (W1-A): additive law, helpers and the oracle refactor. Existing cases are unchanged.
- The red-proof swap was temporary (about 3 minutes, exact-string replacement) and has been restored.

## 7. Open items

1. **Silent stall on Nakagin with 1.5× own meshes.** `own_mesh_fill_roots("nakagin", 1, 1.5)` ends with counters `[0,0,0,0]`, no steps and no rejection, while `stalled = true`.
   - The likely cause is a spatial-index `CollisionMutationStep::Stale` (or a cell reservation limit) during `PrepareSpatial`. That path sets `stalled` without publishing a warning step.
   - A user would see a fill run that "completes" having tested nothing, with no reason shown. This needs a fill-planner owner.
2. **Nakagin is a weak discriminator.** With box fallbacks its counters are the same at own-mesh scales 0.5, 0.75 and 1.25, because almost every candidate collides with its neighbours. Concrete Forest (one document body) is the variant that actually went red.

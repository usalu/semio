# 📓️ Wave 5G — puzzle 🖐️5d target volumes + Volume Brush

Ported the one fill-adjacent 3d feature family 5d lacked entirely: the `target-volume` mutation family,
the five app verbs, the Volume Brush utility, both panes' painting, and the planner-bridge arm that makes
a volume actually constrain a fill run. Schema-first and test-driven throughout.

Paths below are relative to the repo root; `SUBSET` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any`.

---

## 1. Verdicts (real, with commands)

| what | command | verdict |
|---|---|---|
| native type-check | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --message-format=short -j 4` | **GREEN** — `EXIT=0`, `Finished dev profile … in 11m 11s`, **0 errors**, 3 crate warnings (all pre-existing, none in 5G files: `✏️editor/🦀️.rs:9412` unnecessary qualification, `✏️editor/🪟️window/🦀️.rs:275` unnecessary qualification, `✏️editor/🦀️.rs:137 parse_example_dsl` never used). Output: `🗑️generated/5G/check-native-short.txt` |
| slice unit + fixture laws | `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- target_volume --test-threads=1` | **GREEN** — `144 passed; 0 failed` (21 fixture-vector modules + 12 editor laws + the shared harness tests that match the filter). Output: `🗑️generated/5G/test-target-volumes.txt` |
| Python second implementation | `.venv/bin/python <scratchpad>/run_py_oracle.py` (loads `SUBSET/🧪️tests/🖐️mutate-puzzle-5d-1/🐍️.py` with a stubbed `semio_repo_test`, replays every committed 5d vector) | **GREEN** — `kinds declared: 35; cases ok: 49; failed: 0` (forward + committed-after law on all 49 vectors, own-inverse law on the 42 non-rejection ones). Output: `🗑️generated/5G/python-second-implementation.txt` |
| publication audit | `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit Puzzle5dPlayApp` | **GREEN** — exits 0; admitted list contains `addTargetVolume, deleteTargetVolume, relocateTargetVolume, setTargetVolumeFlag` (artifact) and `setVoxelDims` (window-config); `windowOwnershipCases=7`. Fixed one Ajv neutral fixture in `📜️script.ts:128-145` (the `Puzzle5dWorldWindowConfig` case had to gain `voxelDims`). |
| `wasm32-wasip2` check | — | **NOT RUN — owed to integration.** The fleet addendum caps wait time; a fresh wasm target build under 19 concurrent cargos was not affordable. Nothing in 5G is `cfg(wasm32)`-gated, so the native check covers every line it added, but the wasm target is unproven. |

### What is NOT mine and still red in the crate
A full `--lib` run (no filter) shows ~30+ failures in `editor::puzzle5d::component::unit_tests::*` and
`puzzle5d_retained_retirement_laws::*`. Root cause, isolated:
`unit_tests::initial_snapshot_is_the_concrete_forest_document` asserts the seeded document's `schema` and gets
`Some("puzzle.3d.fixture")` where `Some("puzzle.5d")` is expected. The string `puzzle.3d.fixture` does not occur
anywhere in the 5d tree (`grep` over `🗿️artifacts/🖐️5d`: 0 hits) and the example DSL correctly says
`schema=puzzle.5d`, so the 5d app is being seeded from the **3d** app's initial snapshot by something outside this
slice. Every one of those failures drives that seeded app. 5G's own laws pass because they assert on what they
themselves dispatch, not on the seeded identity. **Hand-off: whoever owns the app-seeding regression must fix it
before the crate can be called green.**

---

## 2. Schema + mutations (landed)

**New value type** `Puzzle5dTargetVolume` — `🗿️artifacts/🖐️5d/🦀️.rs:364` (id, `origin: [f64;3]` `#[dsl(coord)]`,
`orientation: Option<[f64;4]>`, `scale: Option<Puzzle5dScale>`, `hidden`, `locked`) — field-for-field the same shape
as `Puzzle3dTargetVolume`, so the planner bridge renames the type and nothing else.

**Document/artifact/diff**
- `SUBSET/🧬️schema/📸️snapshot/🦀️.rs:58` — `target_volumes: Vec<Puzzle5dTargetVolume>`, `#[value(default, skip_serializing_if = "Vec::is_empty")] #[dsl(table)]`. The `skip_serializing_if` is deliberate and copied from 2d's `target_regions`, not from 3d: it keeps the empty table out of the printed DSL so the three committed 5d `.dsl.semio` example assets (incl. the 2.96 MB `capsule-dream`) round-trip unchanged.
- `SUBSET/🧬️schema/🦀️.rs:34` + `to_snapshot`/`from_snapshot`/`set_snapshot`.
- `SUBSET/🧬️schema/🔺️diff/🦀️.rs:36` — `target_volumes: Option<Puzzle5dTargetVolumesDelta>` plus `Puzzle5dTargetVolumesDelta` / `…PatchEntry` / `…Patch`.
- `SUBSET/🧬️schema/🔺️diff/📝️text/🦀️.rs` — `apply_target_volumes_delta` + both `apply` arms (snapshot and artifact) + the `absorb` merge.

**Seven mutation leaves** under `SUBSET/🧬️schema/🧬️mutations/` (dir names identical to 3d's, all already in the
taxonomy vocabulary): `🌍create-target-volume`, `🪦delete-target-volume`, `🚀move-target-volume`,
`🌀rotate-target-volume`, `📐scale-target-volume`, `🔐change-target-volume-locked`, `🙈change-target-volume-hidden`.
Each carries `🔣️.json` descriptor, `🦀️.rs` leaf + builder + `MutationKind`, `↩️inverse/🦀️.rs`, `🔺️diff/🦀️.rs`,
`🧬️schema/🔣️.json` payload schema, and its three test cases.

One deliberate divergence from 3d: `delete-target-volume`'s inverse restores the volume **at its own index**
(`🪦delete-target-volume/↩️inverse/🦀️.rs`), the way 5d's `delete-part` already does — 3d appends, which only
survives its own fixture because that base holds a single volume. The 5d `🚫️removes-volume-1` vector has two and
caught it.

**Union + registries**: `🧬️mutations/🦀️.rs` — 7 enum variants (appended, so binary variant tags stay stable),
7 `KINDS` entries, 7 `pub use` rows, and the target-volume arm of `puzzle5d_snapshot_mutations` (create / delete /
move / rotate / scale / hidden / locked). Twins updated by hand: `🧬️mutations/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`,
`📸️snapshot/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`, `🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`,
`🔺️diff/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`. Crate-root `#[path]` mounts for all 7 leaves and their 21 test cases:
`🗿️artifacts/🖐️5d/🦀️.rs` (after the `replace_kind_catalogs` block).

**Fixtures** `SUBSET/🧫️fixtures/🧬️mutations/<kind>/<case>` — 21 cases, 105 files, three per mutation:
1. a handcrafted vector (3d's own case names reused: `🧊️appends-volume-2`, `🚫️removes-volume-1`, `⬆️lifts-volume-1`, `🔄️half-turn-about-z`, `📏️per-axis-to-uniform`, `🙈️hides-volume-1`, `🔒️locks-volume-1`);
2. a real-world vector built from the `🌲️concrete-forest` example's own `seed-left-001` part, grips `v0`/`v7`, mesh url and coordinates (`🌲️paints-the-seed-bay`, `🌲️clears-the-seed-bay`, `🌲️shifts-the-seed-bay-north`, `🌲️squares-the-seed-bay`, `🌲️widens-the-seed-bay`, `🌲️hides-the-seed-bay`, `🌲️locks-the-seed-bay`);
3. a rejection vector with the D6 `🔺️diff/🚫️.absent` sentinel (`🚫️rejects-a-volume-id-the-model-already-holds` is FATAL `mutation.duplicate-id`; the six addressing verbs raise Error-level `mutation.target-missing`).

All 28 pre-existing 5d diff fixtures gained `"targetVolumes": null` (the diff codec always states its `Option`
fields). Every committed target volume states `hidden`/`locked` explicitly — unlike a part's flags the volume codec
carries no `skip_serializing_if` on them, which the `committed_json_is_canonical` laws pinned.

**Oracle registration** `SUBSET/🔮️oracles/🔣️.json`: 7 rows added to `mutationCatalogs[0].vectors` (with all three
scenarios each), 7 to `mutationCatalogs[0].kinds` (now 35) and 7 to `mutationManifests[0].mutations`
(capability `puzzle-5d-1-mutate`, `qualifyingKind: verified-native-second-implementation`).

**Language-neutral test** `SUBSET/🧪️tests/🖐️mutate-puzzle-5d-1`: `🦀️.rs` KINDS +7, `🥒️.feature` +7 rows in both
Scenario Outlines, `🐍️.py` taught the family — `MEMBERS` gains `targetVolumes` (with a new `OMITTED_WHEN_EMPTY`
rule, because a document that constrains nothing writes no key at all), `volumes_of`/`volume_at`/`written_volume`/
`with_volumes`, `validate` duplicate-id + always-stated-flags rules, apply + own-inverse for all seven verbs.

**Taxonomy** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`:
`semanticDirectoryMemberKinds["members-of-tests"].memberNames` +14 (the forest and rejection case names; the seven
mutation directory names and the seven handcrafted case names were already registered by 3d).
`🔣️schema-catalog.json` is generated (`bun ./📜️script.ts schema generate`) and was **not** hand-edited — it owes a
regen for the seven new payload scopes.

---

## 3. Commands + registries (all five verbs, Artifact/WindowConfig, `Migrated`)

New command modules under `SUBSET/✏️editor/🎮️commands/`: `➕️add-target-volume`, `🪦️delete-target-volume`,
`🚚️relocate-target-volume`, `🚩️set-target-volume-flag`, `📐️set-voxel-dims`.

Registries touched in `SUBSET/✏️editor/🦀️.rs` (every one the rules file lists):
- `puzzle5d_command_variants!` — `AddTargetVolume`/`DeleteTargetVolume`/`RelocateTargetVolume`/`SetTargetVolumeFlag`/`SetVoxelDims` (`:4080-4084`), which also gives `command_from_action` and `TOOL_JOB_IDS` for free.
- `dispatch_puzzle5d_action` — five arms (`:4466-4470`).
- `PUZZLE5D_RETAINED_TOOL_IDS` (`:4481`) and `PUZZLE5D_WINDOW_TOOL_IDS` (`:4558` — `addTargetVolume` and `setVoxelDims` need the addressed pane's runtime, exactly as 3d routes them through `Puzzle3dWindowCommandWork`).
- `bounded_first_step_tool_proofs!` tools list (`:8915-8919`).
- `build_tool_job` — `"relocateTargetVolume" => Puzzle5dRelocateVolumeWork::default()`; delete/flag fall to `BoundedFirstStepCommandWork`.
- `PUBLICATION_CONTRACTS` (`:8541-8545`) — `addTargetVolume`/`deleteTargetVolume`/`relocateTargetVolume`/`setTargetVolumeFlag` = `[Artifact]`, `setVoxelDims` = `[WindowConfig]`.
- `create_puzzle5d_app` — `.mutation("addTargetVolume")`, two `bounded_catalog` mutations (`deleteTargetVolume`, `setTargetVolumeFlag`), `.mutation("relocateTargetVolume")`, a View `setVoxelDims`, five `.action_interactive_job(…, InteractiveJobClassification::Migrated)` (`:9711-9715`), and `.utility(world3d::utilities::volume_brush::definition(…))`.
- `puzzle5d_retained_extent` now counts `targetVolumes` — an honest extent for the verbs that scan them.
- Terminology `SUBSET/✏️editor/🗣️terminology/🦀️.rs`: `target_volumes`, `target_volume`, `volume_brush`, `voxel`, `width`, `depth`, `height`, `target_volume_origin_required` — all four locale×terminology cells each.
- `SUBSET/🧫️fixtures/🗄️retained-jobs/🔣️.json`: five ids at the head of `toolIds`/`evidenceToolIds` (the law compares this list to `PUZZLE5D_RETAINED_TOOL_IDS` **in order**), five `semanticCursors` entries, and 8 new vectors (grid-snap, missing-origin, delete, full-pose relocate, locked-refusal, flag, exact voxel dim, clamped voxel dim).
- `✏️s/🔌️plugins/🧩️puzzle/🧫️fixtures/🔏️publication-authority/🔣️.json` already listed all five routes under `Puzzle5dPlayApp` with exactly the lanes above — no edit needed, and the audit confirms it.

`Puzzle5dRelocateVolumeWork` — `SUBSET/✏️editor/🦀️.rs:5250` — is the port of `Puzzle3dRelocateVolumeWork`: stages
Search → Origin → Orientation → Scale, one bounded mutation per step, `extent = volumes + 4`, incremental
`close_step`, `terminal_is_empty`. A locked volume is skipped in Search, so the gumball refuses it.

**Window config lane**: `voxel_dims: [u32; 3]` added to `Puzzle5dRuntime`
(`SUBSET/✏️editor/🎚️config/🦀️.rs`, `default_voxel_dims`), to the merged `Puzzle5dWindowConfig` and to the persisted
`Puzzle5dWorldWindowConfig` (`SUBSET/✏️editor/🪟️window/🦀️.rs:27,114`) with the `runtime`/`config_from_runtime`/
`window_config_from_world`/`addressed_config` wiring, and to all four `🪟️window/🧬️schema` twins
(`voxelDims`, integer, `[1,64]`, exactly 3 items). Band constants `PUZZLE5D_VOXEL_DIM_MIN/MAX`,
`PUZZLE5D_DEFAULT_VOXEL_DIMS = [4,4,4]`, `PUZZLE5D_TARGET_VOLUME_COLOR`, `PUZZLE5D_TARGET_VOLUME_ID_STEM` at
`SUBSET/✏️editor/🦀️.rs:103-111`; `Puzzle5dFreshIds::next_target_volume` at `:202`.

---

## 4. Utility + both panes

- `SUBSET/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🧊️volume-brush/🦀️.rs` — `UTILITY_ID = "volumeBrush"`, `definition`, `voxel_dim_measures` (three sliders → `setVoxelDims{axis}`), `options` group keyed to the utility. Bound by the world window only (`…/🧊️3d/🦀️.rs` `utilities` list) and pushed into `window_measures`.
- World scene: `world_target_volumes_json` (`…/🧊️3d/🦀️.rs:244`) publishes the **same record shape puzzle 3d does**
  (`id/origin/orientation/scale/color/hidden/locked`) on `scene.target_volumes_json` (`:273`), so the framework's
  generic `targetVolumesJson` lane and `World3dHost` paint 5d unchanged.
- Board scene: `board_target_regions_value` (`…/◻️2d/🦀️.rs:127`) publishes the projected flat rectangles under the
  same `targetRegions` key the 2d artifact uses (`:160`); hidden volumes are dropped rather than painted
  transparent. The projection is `target_volume_flat_rect` (`SUBSET/✏️editor/🦀️.rs:760`), which uses the ONE
  board↔world map this artifact already places paired parts with (`PUZZLE5D_FLAT_TO_WORLD`, world XY ground plane,
  board Y inverted) — i.e. the linear inverse of `🧬️schema/💡️inferences/🎛️flat-position`. No second persisted pose.

## 5. Fill constraint (the planner bridge)

- `SUBSET/✏️editor/🧠️precompute/🦀️.rs:78` — `puzzle3d_snapshot` no longer hands the planner `target_volumes: Vec::new()`; it maps the 5d document's volumes through `puzzle3d_target_volume` (`:189`). Hidden volumes are included on purpose: `hidden` is a paint flag, and dropping them would silently widen the fill region.
- `SUBSET/✏️editor/🧠️precompute/🪣️fill/🦀️.rs:60` — `puzzle3d_ops` gained all seven arms, so a run whose provisional list ever carries a volume edit translates it instead of faulting `puzzle5d-fill-run-provisional`.
- Law `target_volumes_reach_the_planner_snapshot` pins both (uniform and per-axis scale, hidden+locked volume still bridged).

**NOT verified:** an actual end-to-end fill run on concrete-forest constrained to one volume, placing parts only
inside it. That needs the boot chain (5F) and the battery; it is a coordinator-owned lane. What is proven here is
that the constraint reaches the real 3d `FillBuilder` input unchanged.

## 6. Laws

`SUBSET/✏️editor/🧪️tests/🔬️target-volumes/🦀️.rs` (new topic, mounted at `SUBSET/✏️editor/🦀️.rs:9811`), 12 laws:
one-Alt+click-is-one-grid-snapped-box-and-undo-removes-it · missing-origin-answers-a-notice ·
voxel-dims-clamp-and-size-the-next-paint · flag-writes-one-flag-at-a-time (unknown flag writes nothing) ·
relocate-writes-the-whole-pose-and-a-locked-volume-refuses · delete-removes-only-the-addressed-one ·
both-panes-paint-the-same-volume · the-flat-rectangle-is-the-footprint-under-the-shared-map ·
target-volumes-reach-the-planner-snapshot · the-volume-brush-is-bound-to-the-world-window-with-three-sliders ·
a-painted-volume-is-one-create-target-volume-in-the-delta. Plus the 21 fixture modules (each: applies-to-after,
inverse-restores-before, canonical JSON, declared outcome, produced diff == committed diff, canonical diff,
diff-applies-to-after; rejection cases instead pin the refusal code/level/target and the D6 sentinel).
Also fixed `SUBSET/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs:131` (world-window round-trip fixture needed `voxel_dims`).

---

## 7. Hand-offs

**→ 5C (panels).** Outliner/inspector rows for volumes are yours. Accessors, all `pub`:
- document: `crate::editor::puzzle5d::Puzzle5dDocument::target_volumes: Vec<Puzzle5dTargetVolume>` (fields `id`, `origin: [f64;3]`, `orientation: Option<[f64;4]>`, `scale: Option<serde_json::Value>`, `hidden`, `locked`).
- `crate::editor::puzzle5d::target_volume_scale_json(&volume) -> [f64; 3]` — the extent, either arm of the scale union.
- `crate::editor::puzzle5d::target_volume_flat_rect(&volume) -> [f64; 4]` — `[x, y, width, height]` in board units, for a board-side inspector row.
- verbs to bind from rows: `setTargetVolumeFlag {id, flag: "hidden"|"locked", value}` (show/hide + lock/unlock, group them the way 3d's `targets` group does) and `deleteTargetVolume {id}` (destructive, last in the menu). Both are `Migrated`, Artifact lane, already registered — you only need the row.
- a `📌️panels/…` outliner section label: `labels.target_volumes` (EN "Target Volumes" / DE "Zielvolumina"); singular `labels.target_volume`.

**→ 5A2 (`scaleSelection`).** `scaleSelection` must also scale target volumes, the way 3d's `Puzzle3dScaleWork`
does. Your `Puzzle5dTransformWork` (`SUBSET/✏️editor/🦀️.rs`, `Puzzle5dTransformStage::Parts`) currently walks
`projection["parts"]` only. The arm you need, after the part cursor is exhausted, is a second cursor over
`projection["targetVolumes"]` that, for each **unlocked** selected volume, pushes
`crate::standards::v1::subsets::any::schema::mutations::scale_target_volume(id, Some(crate::Puzzle5dScale::Vec3([sx*current[0], sy*current[1], sz*current[2]])))`,
reading `current` from the row's `scale` (number → uniform, array → per-axis, absent → `[1,1,1]`) exactly as your
part arm already does. Remember to widen `extent` by the volume count and to keep the `gumball-scale` coalesce key.
I deliberately did not touch `Puzzle5dTransformWork` — it is your file and was in flight.

**→ integration (wave 2).**
1. `wasm32-wasip2` check of `semio-s-artifact-puzzle-5d` — owed, never run by 5G.
2. `bun ./📜️script.ts schema generate` — `🔣️schema-catalog.json` owes seven new `s.puzzle.puzzle5d.mutation.*-target-volume` scopes.
3. The app-seeding regression (`initial_snapshot` answers `puzzle.3d.fixture`) blocks ~30 shared 5d `unit_tests`; not 5G's.
4. A real constrained fill run on concrete-forest (§5) once the 5d boot chain is up.

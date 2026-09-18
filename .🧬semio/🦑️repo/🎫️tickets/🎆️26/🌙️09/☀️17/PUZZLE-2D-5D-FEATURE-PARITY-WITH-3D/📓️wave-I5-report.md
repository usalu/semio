# 📓️ Wave I5 — puzzle 🖐️5d integration

Integrator for `semio-s-artifact-puzzle-5d` after the eight 5-slices landed. Repo root
`/Users/ueli/Documents/semio`. EDITOR5 =
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.
All logs in `TICKET/🗑️generated/I5/`.

---

## 1. Verdicts

| gate | command | verdict |
|---|---|---|
| native, all targets | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --all-targets --message-format=short` | **EXIT 0**, 0 errors. `check-native-5.txt` |
| wasm component | same `--target wasm32-wasip2` | **EXIT 0**, 0 errors. `check-wasm-3.txt` |
| warnings | — | `semio-s-artifact-puzzle-5d (lib)` **3** → see §6; `(lib test)` 35 (all `unnecessary qualification` / one unused import in 5G's and 5E's test files) |
| publication authority, ALL owners | `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts publication-authority-audit` | **EXIT 0** — `owners=Puzzle2dPlayApp,Puzzle3dPlayApp,Puzzle5dPlayApp; windowOwnershipCases=7; schema=Ajv; oracle=independent` |
| retained-jobs fixture | mechanical compare of `🧫️fixtures/🗄️retained-jobs/🔣️.json` `toolIds` against `PUZZLE5D_RETAINED_TOOL_IDS` | **EQUAL, in order, 70 ids**; `evidenceToolIds` == the same set |
| full lib suite | `CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- --test-threads=4` | see §5 |

---

## 2. The regression that was blocking everything (found + fixed)

### 2.1 The 5d examples were not puzzle-5d documents at all

Between 19:23 and 19:37 — inside the fleet's second outage, so by a peer, not by a wave-1 slice —
`document_json()` in **all seven** puzzle example modules (2d ×2, 3d ×2, 5d ×3) was rewritten from the
committed DSL-derived form to `include_str!("🖼️assets/…/📄️document.json")`, with hand-made,
**git-untracked** `📄️document.json` files dropped beside each asset. The three 5d ones were wrong:
concrete-forest and nakagin held **puzzle 3d** documents (`"schema":"puzzle.3d.fixture"`, `objects`,
`attractions`), capsule-dream a 121-byte empty stub. `document_from_json` answered `empty_document()`
on the serde failure, so the 5d app booted a ZERO-part document whose schema read `puzzle.3d.fixture`.

That single edit is the whole of: 5G's "~30 shared unit tests fail on `puzzle.3d.fixture`", 5A2's
"nakagin and capsule-dream load as EMPTY documents / `clipboard_verbs_cover_every_part_of_the_largest_example`
left 0 right 2880", and most of 5B's nine blocked fill laws.

**Fixed:**
- `📚️examples/{🌙️capsule-dream,🌲️concrete-forest,🏗️nakagin-capsule-tower}/🦀️.rs` — `document_json()` back on
  `parse_dsl(DSL_TEXT)` + `dsl::json::to_json_string`, byte-identical to HEAD.
- the three bogus `📄️document.json` deleted (copies kept in `🗑️generated/I5/salvaged-example-json/`).
- EDITOR5 `🦀️.rs:403` `document_from_json` now **fails loudly**
  (`panic!("puzzle5d example document decodes: {error}")`) instead of silently answering
  `empty_document()`. It only ever decodes SHIPPED assets; user JSON arrives through
  `📥️import-fixture`, which refuses with a notice. The now-dead `parse_example_dsl` helper was removed
  (it was the source of the `function parse_example_dsl is never used` warning three slices reported).
- I2 did the 2d half and the coordinator the 3d half after I wrote it into `📌️important.md`.

### 2.2 `#[dsl(table)] target_volumes` — TWO separate breaks, both fixed

**(a) the asset text.** `FieldKind::VecTable` (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:1044`)
answers `expected List, found Absent`: a DSL table field is never optional, `#[value(default)]` or not.
Puzzle 3d has always shipped an explicit EMPTY `target-volumes [...] {}` table in its example assets for
exactly this reason. Mirrored in all three 5d assets
(`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`, appended at the end — `target_volumes` is the last field of
`Puzzle5dSnapshot`). The shared derive was deliberately NOT changed; it is one framework crate away from
every artifact in the repo and 3d's precedent settles the question.

**(b) the table SCHEMA — the descriptor-probe guest panic.** After (a) the activation still died with
`table column 'scale' has a non-self-delimiting shape (TUPLE) and cannot be a table column at 1:1`.
Root cause: `impl dsl::DslField for Puzzle5dScale` (`🗿️artifacts/🖐️5d/🦀️.rs:148`) bound
`Shape::Tuple(Float, None)` — legal while `scale` was only reached through the nested keyed `part-3d`
record, illegal the moment `target_volumes` became a table whose `scale` is a BARE positional column.
Puzzle 3d hit this exact wall when it added `targetVolumes` and documents it at `🧊️3d/🦀️.rs:137-143`.
5d now binds the same bracketed `Shape::List(Float)` — `scale=[2]` uniform / `scale=[2 3 4]` per-axis,
self-delimiting at any item count. `orientation` stays `TUPLE` and is fine: `[f64; 4]` is a BOUNDED
tuple, which IS self-delimiting. No shipped 5d asset carries a `scale=` literal, so no asset text moved.

**Result — the coordinator's blocker is cleared:**
`examples::puzzle5d::{concrete_forest,nakagin_capsule_tower,capsule_dream}_tests::dsl_asset_parses_and_round_trips`
and all three `inference_determinism_law` **pass**. `materialize-dev` can be re-run.

### 2.3 The census the loader now really carries

| example | parts | fasteners | kindCatalogs |
|---|---|---|---|
| 🌲️concrete-forest | **1** | **0** | no |
| 🏗️nakagin-capsule-tower | **180** | **179** | yes |
| 🌙️capsule-dream | **2 880** | **2 864** | yes |

Concrete forest is a ONE-SEED FILL PLAYGROUND (`seed-left-001` + 17 compatibility rows), not a populated
document — that is what its asset has shipped since 08-09 and what 5G's own fixtures address. Several
laws written while the examples were empty had invented a richer shape for it; they are repointed at
nakagin (§4). `every_shipped_example_document_really_carries_its_content` (EDITOR5
`🧪️tests/🔬️unit/🦀️.rs:1898`) now pins this exact census per example instead of `!is_empty()`, so the
next time an example asset regresses the failure names the number.

---

## 3. Capacity: the capsule-dream example switch

`Puzzle5dSetActiveExampleWork` declared ONE unit per replayed row, so switching to capsule-dream cost
≈5 749 units (≈11 500 reloading it over itself) against the shared
`crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS = 4 096`, and could only ever refuse.
**The shared constant was NOT raised** (2d, 3d and three fixtures pin it). Instead the replay is chunked,
3d's own shape (`PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK`):

- `PUZZLE5D_SET_ACTIVE_EXAMPLE_CHUNK = 64`, `…_FIXED_STEPS = 13`,
  `…_MUTATIONS = PUZZLE_COMMAND_WORK_ITEMS * CHUNK` (EDITOR5 `🦀️.rs`, just above
  `enum Puzzle5dSetActiveExampleStage`). The mutation ceiling is now exactly what the declared extent can
  legitimately produce, so `push` can no longer refuse work `extent` already admitted.
- `units()` counts `len.div_ceil(CHUNK)` per cursored stage; all six cursored arms (`ClearFasteners`,
  `ClearParts`, `ClearCompatibility`, `AddCompatibility`, `AddParts`, `AddFasteners`) take a chunk per
  step through a new `take_chunk`. A fresh capsule-dream switch now declares ≈**103** units.
- The chunk is 8× 3d's on purpose and the constant's docstring says why: 3d steps over a TYPED snapshot,
  while every 5d work re-derives `puzzle5d_projection_value` per step, so a step's cost is dominated by
  that one O(document) projection rather than by the rows it replays. Fewer, fatter steps is strictly
  cheaper here (capsule-dream ≈720 steps → ≈100).
- The per-step capacity re-check was hoisted behind a new `admitted` flag: `Self::units` builds a full
  projection of its own, so re-running it every step tripled the per-step cost for nothing.

---

## 4. Everything else that landed

| # | what | where |
|---|---|---|
| 1 | **5C hand-off — label stamping at the RETAINED creation sites.** `puzzle5d_next_part_label_from_projection(projection, kind)` (new, EDITOR5 `🦀️.rs`, `🏷️Labels` region) is the projection-side twin of `puzzle5d_next_part_label` — a retained Work only ever holds `Puzzle5dPlaySnapshot`'s projection, never a typed document. Called from `Puzzle5dAddNodeWork`'s and `Puzzle5dAddBrushPartWork`'s `Create` stages, which both stamped `part_3d.label: None`. | EDITOR5 `🦀️.rs` |
| 2 | **Fill placements get labels too.** `Puzzle5dPlannerBoard::next_label` (new) + `adopt()` now stamps `part_3d.label` (`object.label` when the 3d planner already named it, else the board's own numbering). `adopt_suggestion` delegates to `adopt`, so accept-suggestion is covered by the same call. | `🧠️precompute/🦀️.rs:293,321` |
| 3 | **5G hand-off — `scaleSelection` scales target volumes.** New `Puzzle5dTransformStage::Volumes` + `volume_cursor`; `scaleSelection` walks `projection["targetVolumes"]` after the part cursor, skips LOCKED volumes, and pushes `scale_target_volume(id, Vec3(current × s))`. `extent` widened by the volume count; the terminal `Emit` (coalesce key, locked-refusal notice) was extracted into one `complete()` so both cursors end on the same rung. A shared `scale_row` helper reads either arm of the `Puzzle5dScale` union. | EDITOR5 `🦀️.rs` `Puzzle5dTransformWork` |
| 4 | **5G hand-off — outliner rows for target volumes.** New `TARGET_VOLUMES_SECTION` between parts and fasteners, `target_volume_row` with the same two inline toggles a part row carries, dispatching **`setTargetVolumeFlag {flag, id, value}`** (NOT `setSelectionFlag`, which writes the part slice only). New granularity `PUZZLE5D_GRANULARITY_TARGET_VOLUME = "targetVolume"`, 3d's own spelling. | `📌️panels/🗿️artifact/🦀️.rs`, EDITOR5 `🦀️.rs:87` |
| 5 | **5G hand-off — inspector group for target volumes.** `target_volume_fields` (id, origin, orientation, scale via `target_volume_scale_json`, the board footprint via `target_volume_flat_rect`, hidden, locked) + a `volume_flag_row`, and a `PUZZLE5D_GRANULARITY_TARGET_VOLUME` branch plus an id-only fallback in `selected_section`. New `Puzzle5dInteractionSnapshot::selected_target_volume_ids`. | `📌️panels/🔍️inspection/🦀️.rs`, EDITOR5 `🦀️.rs` |
| 6 | **Lock refusals.** `Puzzle5dActionCtx::refuse_when_locked` existed with ZERO callers. Wired into `delete_selection` (locked parts are never removed; an all-locked selection refuses with ONE `selection_locked` notice), `patch_part` (same refusal + locked parts skipped in the loop) and `patch_grip` (the OWNING part's lock refuses/skips). The gumball arms and the board drag already refused; `Puzzle5dCutJob` already skipped locked parts (5D). | `🎮️commands/{🗑️delete-selection,🩹️patch-part,✊️patch-grip}/🦀️.rs` |
| 7 | **`interactive-job.step-overrun` on `copy`/`cut`.** `Puzzle5dSelectionScan::rows()` re-derived the WHOLE projection **and cloned the whole row array** on every one of a selection's ~2N+M cursor steps: `framework route 'copy' overran the 8 ms step ceiling for 4 consecutive steps, worst 70521us` on the 180-part Nakagin (`cut`: 50 407 µs). Replaced by a memoised `projection` field + a single-row `row(key, index)` accessor, with its own rung on the close ladder and in `terminal_is_empty`. | EDITOR5 `🦀️.rs:1570-1610`, `1955`, `1975` |
| 8 | **The committed-fixture harness refused 5G's rejection scenarios.** `committed_mutation_fixtures_equal_their_canonical_regeneration` hard-asserted `status == "applied"` for EVERY scenario; 5G's seven `🚫️…` vectors are `"rejected"`. It now has a rejection branch: the mutation must leave its own before-snapshot untouched and the scenario must state the `🔺️diff/🚫️.absent` sentinel. | `🧬️schema/🧬️mutations/🧪️tests/🔬️committed-fixtures/🦀️.rs:26` |
| 9 | Registry/count drift closed: `dispatch_registers_semantic_descriptors` 28 → **35** mutation kinds (5G's +7); `app_pack_and_spr_exclude_window_and_transient_fields` 4 → **6** shared-config keys (5C's `proximityRadius` + `chunkSize`). | `🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:140`, `🪟️window/🧪️tests/🔬️unit/🦀️.rs:19` |
| 10 | **Probe reconciliation** (`TICKET/🔍️browser-probe-5d.ts`): `setObjectKindWeight\|setVortexKindWeight` → `puzzle5d-play-fill-distribution → setPartKindWeight\|setGripKindWeight` (5A1/5B), `focusSelection\|zoomToSelection` → `focusSelection` (5A2), `Puzzle5dClipboardJob` → `Puzzle5dCopyJob\|Puzzle5dCutJob\|Puzzle5dPasteJob` (5D), and the §15 doc comment now records that `focusSelection`/`selectSameKindSelection` are the surviving ids. A grep of the probe and of the whole 5d production tree for `zoomToSelection`, `addBrushObject`, `setObjectKindWeight`, `setVortexKindWeight`, `selectSameKind"`, `importComposeKit`, `setFixtureJson` returns **only** the laws that assert those ids are gone. |

---

## 5. The suite

`CARGO_INCREMENTAL=0 cargo test -p semio-s-artifact-puzzle-5d --features component-app-assembly --lib -- --test-threads=4`

| run | passed | failed | log |
|---|---|---|---|
| before I5 (the state the slices left) | 535 | **32** | `test-lib-3.txt` |
| after the example + census wave | 543 | 24 | `test-lib-4.txt` |
| after the fixture/harness wave | 548 | 19 | `test-lib-5.txt` |
| final | _see below_ | | `test-lib-6.txt` |

Two tests are EXCLUDED from the timed runs with `--skip` and are documented as SLOW, not hung:
`set_active_example_switches_the_document_and_never_faults_on_capacity` and
`kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode`. Both now drive a real
2 880-part capsule-dream through the retained factory; at ~100 job steps each costing one full
O(document) projection in a debug build they take tens of minutes. That is the §7 performance finding,
not a deadlock — the rest of the suite runs in 75–137 s.

---

## 6. Warnings

`semio-s-artifact-puzzle-5d (lib)`: 3, none introduced by I5 —
`🦀️.rs:9551` and `🪟️window/🦀️.rs:275` `unnecessary qualification` (5E's), and one more of the same class.
(The `parse_example_dsl is never used` warning three slices reported is GONE — the function was the dead
half of the example regression and was removed.)
`(lib test)`: 35, all `unnecessary qualification` plus one `unused import: semio_framework_plugin::PluginApp`
in 5G's `🧪️tests/🔬️target-volumes/🦀️.rs:7`.

---

## 7. Still red, with evidence and ownership

_(filled in below from the final run)_

---

## 8. Not verified / owed

- **No browser, Nx or battery evidence.** Source + laws only; the coordinator owns activate/serve/Playwright.
- **`[DEBUG] maintenance idle probe` is FRAMEWORK code, not 5d, and I deliberately did not remove it.**
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29240`, an unconditional `eprintln!` on the
  maintenance hot path. `git log -S` dates it to today's 12:02 auto-commit (`0b460ed19f`); it is in HEAD,
  it is not attributable to a live peer ticket, and it floods every 5d test run (hundreds of lines per
  test). Removing it invalidates `semio_framework_plugin` and every crate under it — a full framework
  rebuild for the whole fleet in the middle of integration, and it would also invalidate the coordinator's
  in-flight component build. **Coordinator: drop that one line in the quiescent window before the final
  batteries.** Until then, filter runs with `grep -v "maintenance idle probe"`.
- **Comments inside definitions** (AGENTS.md rule 6): 149 in the 5d tree — against 208 in the 3d reference
  artifact. This is a repo-wide pre-existing pattern, not a regression of this ticket; hoisting 149
  rationale comments into docstrings across six slices' files is a churn/risk trade the integrator
  declined. Nothing I5 added carries an in-definition comment.
- **`🔣️schema-catalog.json`** still owes the regen 5G asked for (§7 of its report):
  `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript && bun ./📜️script.ts schema generate`
  — seven new `s.puzzle.puzzle5d.mutation.*-target-volume` scopes. It is a GENERATED file; I5 did not
  hand-edit it. **This is the descriptor/catalog regeneration command the coordinator asked for.**
- **`🧊️3d/🦀️.rs:139`** still says the sibling `Puzzle5dScale` "uses Tuple" — stale since §2.2(b). Editing
  3d would have forced a 3d + 5d rebuild for a comment; left for whoever next touches that file.
- **The retained-jobs fixture's `capacities`** describes the SHARED puzzle band (8 192 / 512 / 4 096 …)
  while 5d's factory runs on `PUZZLE5D_RETAINED_RAW_BYTES = 262 144` / `…_DECODED_ITEMS = 16 384`.
  Both the shared law and the 5d factory are currently green because the law pins the shared constants,
  but the fixture does not describe 5d's real band. 5D's report argues for parameterising
  `retained_command_test_catalog()` per owner; that changes 2d's and 3d's fixtures too and is a
  coordinator call.

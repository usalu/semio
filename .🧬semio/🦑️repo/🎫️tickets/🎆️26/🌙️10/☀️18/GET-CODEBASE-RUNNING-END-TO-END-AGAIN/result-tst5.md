# Result tst5: re-run of the 916 bun suites under `✏️s/**` and `💻️os`, regressions and code-fixable reds

Executor `tst5`, 2026-10-19. Foreground, `bun test ./<explicit file>` only (2 workers), no sub-agents, no cargo, no `package.json`/`bun.lock` edit, no modifying git command.
Tools (kept): `tst5-tools/` = tst3b's `runbun.py`/`classify.py`/`t.sh` (output dir `tst5-out`), plus `fixplace.ts` (fixtures outside `🧪️tests`), `fixmove.py` (tst3c's mover, with `./` prefix for TS), `treerf.py`/`treeall.py` (switch literal `.rs` readers to `readModuleTreeSource`), `pairdiff.ts` (JSON vs pack descriptor diff), `rw.ts`, `prod.ts`, `report.py`. Generated output (`🗑️generated/tst5-*`) is deleted.

## Totals

Scope: 916 suites (tst3b: 915; one added since). Emulated harness identical to tst3b (`SEMIO_SCRIPT_PROCESS_INVOCATION`, `SEMIO_SCRIPT_PROVIDERS`, `REPO_ROOT`, `SEMIO_REPO_ROOT`, `SEMIO_TICKET_DIR`, short `SEMIO_TEST_ARTIFACT_DIR`, `SEMIO_BUILD_MODE=prototype`, `--timeout 120000`, tsconfig override for `tree-sitter-wasms`/`d3-dsv`, `perl alarm 300`).

| run | green suites | tests pass / fail |
|---|---|---|
| tst3b final (915) | 845 | 9,763 / 131 |
| tst5 start (916, after the splits `split-s`, `rg-plug`, `rg-fem3d`, `rust-mounts`, `vcs-bound`, `fixups3`) | 846 | 9,779 / 117 |
| tst5 second full pass (after my fixes and peers' work) | 875 | 9,826 / 80 |
| tst5 final (second pass + targeted re-run of the 41 reds and 59 recently edited suites) | **876** | see table |

Net: 846 -> 876. The aggregate was flat at the start (846 vs 845) because 4 suites regressed or were never listed while peers' work (`vcs-bound`, store worker cycle, `rust-mounts`) turned others green; every one of the 70 reds was individually re-diagnosed.

## Regressions found (not in tst3b's red list), fixed at the root

- `♾️infinite/🎲️board/…/➕️normal/🗃️model/🧪️tests/🎟️admission`: `🪣️fill-job` was split into `➕️normal/🪣️fill/**`; reader -> tree reader, `step(scene,style,brush,cx)` signature, `next_step_demands(style)`.
- `🏪️store/👷️worker/🧪️tests/📥️values`: `validateBrowserActorUiValues` moved into `🧩️actor-wire-helpers`.
- `📤️output` (plugin reactor): dashboard `run-script` tool gained `--dry-run={dry-run}` (the other tests of that law were already updated by the dashboard work).
- `💻️os/📦️packages/🟦️typescript/📜️script.ts` (vitest runner): `⚙️config-diff-twin` read config mutation fixtures from the old `🧬️schema/**/🧫️fixtures`; they now live in `🎚️config/🧪️tests/🧫️fixtures/🧬️mutations` (reader re-pointed). Still red: `retry` is not installed anywhere and `cockatiel` 3.2.1 is in the store but not hoisted (deps owner).
- `🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit`: `require("@semio-tech/framework-plugin/**/schema.json")` resolves to a json facade `.ts` (tst3c facades); the law compiled the module namespace (accepts everything). New `schemaOf` unwraps `default` (5 call sites). The same pattern probably hides silent passes elsewhere.
- `🪶️sqlite` workflow suites, `semantic-source-pairs`, `descriptor-verification` (240 s timeout under 2-worker load, green later), `local-hub/🎫️session` (2 s loan deadline, flaky under load): classified in the table.

## Source-law drift resolved by re-deriving the needle from the present code (owners should confirm intent)

Pattern counts over the 41 laws touched: mount roots read as plain files (13), `pub(crate)` -> `pub` after crate splits (5), signature changes (`advance(.., receiving)`, `advance(body, grant)`, `header.advance(.., body, ..)`, `try_serialize_cold`) (7), moved definitions (`ProgramBridge`, `AppFramePreparation`, kernel runtime root, native asset stream, `RendererAssetDecodeJob`) (6), rustfmt spacing (`source.turn(grant, |cx| ..)`) (1), law slice unbounded after tree concatenation (4). Per-suite detail in the table.

Code (not law) edits: `🛎️services/🚪️native-io/🦀️.rs` removed the dead `#[doc(hidden)] pub fn native_io_close_grant` (the law forbids it; no other user); `🔋️energy/…/⚡️electrical/🧪️tests/🔆️pv-plane/🟦️.ts` exports `solarPvFixtureCases` and the energy package `📜️script.ts` `solar-source capture` reads it instead of importing a fixture (law: TS imports of fixtures only from `🧪️tests`).

## Fixture moves

- Energy (13) and puzzle (2) fixtures outside `🧪️tests` moved to `D/🧪️tests/🧫️fixtures/**` with 32 references re-pointed (`fixmove.py`), plus the two schema-ancestor energy fixtures (`🧬️schema/📸️snapshot/🧳️state`, `🧬️schema/🧬️mutations/🪟️optics`) to `✳️any/🧪️tests/🧫️fixtures/**` with 4 readers.
- Trinity rewriting: 6 committed `🔺️diff` fixtures converted from `entries:[{key,..}]` to the keyed object the framework map schema/Rust `BTreeMap` use; rewriting diff schema `entries` refinement array -> object (`additionalProperties`).
- NOT moved: 533 FEM fixtures (plus 12 energy-model ones now moved) that violate `fixture-placement` rule 1/2. `tst5-tools/fixmove.py apply <offender list>` plus `fixplace.ts` plan it (545 moves, 99 files, 1,137 edits, dry run clean), but FEM's `🏭️generator/📜️script.ts` (edited 17:51) holds path tables `🕸️mesh/🧫️fixtures/…` that must move in the same step, and the FEM fleet is mid-migration (both `🧫️fixtures` and `🧪️tests/🧫️fixtures` hold different mutation families). Owner action.

## Still red (40 suites), by owner

- Deps / build: 9 NO-DIST suites (`<pkg>:build` through the cache pin), `retry`/`cockatiel` for the os vitest runner.
- Needs cargo / native output / browser / live launcher: brep x3, geometry-service (fixture `{}` since HEAD 675), fem surface x2 (`SEMIO_FEM_NEUTRAL_RESULTS`), mounted-owner, retained-command unit, BIM `components`/`ramps` blessed outputs (`BIM_BLESS=1`), `semantic-source-pairs` (14 hub descriptors: JSON regenerated, pack stale), Playwright chromium 1248, `🏗️preparation`, `🧩️native-prerequisites`, `⌨️native-entrypoint/📇️catalog`, `command-ownership`, `Paint2dHost/ownership`.
- Toolchain: tsv (tree-sitter-rust grammar vs async closures), workflow `🏃️run` and `🔁️workflow` sqlite snapshot (Bun 1.3.14 NUL-text byte accounting vs the committed corpus), space-history (tree-sitter refuses the Rust test).
- Owner decisions: puzzle-3d wasm (`Puzzle3dArtifactVcs` still declared), canonical-edit oracle (`pub fn seal_prepared_owned` is required by cross-crate callers; law wants `pub(crate)`), `create-app/🎟️source` (`ProgramFault` derives `Clone, PartialEq` while carrying the non-Clone `create_app_source`; the law wants Debug-only; compile risk, unverified), `🧪️tests/📦️units` (hash pins of Rust bodies: needs re-bless), `runtime-close-budget` (4 aggregate source-text laws), `graph 🤝️group` (standalone rustc isolation), `fixture-placement` (FEM 533 + BIM blessed).
- Peer work in flight (do not touch, retry later): `crate-boundaries` 126/1 (a Cargo edit gives `semio-framework-mutation-diff` a `semio-framework-value` edge again), vite `📥️artifacts` 39/4 (peer `taxonomy.json` `pathBudgetReserve` edit at 19:18, `🎡️play/📦️site/📜️script.ts` edited 19:07, bounded inventory), bim `🪶️sqlite` (peer changes `runArtifactRustPackageMain` to `adopt()` a ScriptRouter; the bim package script still passes a record), `service-composition/📥️checked` (peer split of the gis `💡️inference-port` test, 969 lines removed 18:34).

## Verification

- `python3 tst5-tools/runbun.py <916 list> r1.jsonl 540 300` x2 (start state), same for `r2.jsonl` (after fixes) and `r3.jsonl` (100 suites: reds + recently edited), classify with `classify.py`. Every edited suite was re-run alone with `tst5-tools/t.sh ./<file>` (full harness env) before moving on.
- Nothing built, no cargo; Rust edits are limited to one dead-function removal (`native-io/🦀️.rs`, uncompiled, grep shows no user).
- `tryfix.py` (kept from tst3b) temporarily copies and restores test files: it touched mtimes of ~20 tests (restored byte-identical); one run kept a no-op treeify.

## Table (suite -> tst3b -> now -> after fix)

`tst3b` = status in tst3b's report ("red" = listed in its still-red/partly-repaired text, "(not listed)" = not named although red when tst5 started); "now" = first tst5 pass; "after" = final.

| suite | tst3b | now (start of tst5) | after fix | cause / fix |
|---|---|---|---|---|
| `P/✒️writer/🗿️artifacts/✒️writer/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | red | red 0/1 | red 0/1 | NO-DIST: package exports ./dist, not built (needs <pkg>:build) |
| `P/🀄️wfc/🗿️artifacts/🖼️bitmap/S1/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | red | red 7/1 | red 7/1 | NO-DIST self-reference of wfc bitmap |
| `P/🌀️procedural/🗿️artifacts/🧊️generation3d/S1/🧬️schema/💡️inferences/📐️geometry/✂️brep-intersect/🧪️tests/🔬️unit/🟦️.ts` | red | red 4/1 | red 4/1 | pinned meshes absent (cargo-produced fixtures) |
| `P/🌀️procedural/🗿️artifacts/🧊️generation3d/S1/🧬️schema/💡️inferences/📐️geometry/🐚️brep-topology/🧪️tests/🔬️unit/🟦️.ts` | red | red 4/1 | red 4/1 | pinned meshes absent (cargo-produced fixtures) |
| `P/🌀️procedural/🗿️artifacts/🧊️generation3d/S1/🧬️schema/💡️inferences/📐️geometry/🛠️brep-feature/🧪️tests/🔬️unit/🟦️.ts` | red | red 3/2 | red 3/2 | pinned meshes absent (cargo-produced fixtures) |
| `P/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/📐️geometry-service/🧪️tests/🔬️unit/🟦️.ts` | red | red 0/7 | red 0/7 | geometry-service fixture is {} since HEAD 675; needs cargo service run |
| `P/🌊️flow/🗿️artifacts/🌊️flow/S1/✏️editor/🎮️commands/♻️retirement/🧪️tests/🟦️.ts` | red | red 0/1 | green 1/0 | Flow retirement law: `retirement::shared` -> `arc_custody`, ToolJobFactory `create_job_from_wire_pages*` -> `admission_demands`/`admit_job` |
| `P/🌍️gis/🗿️artifacts/🏔️gisterrain/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | red | red 0/1 | red 0/1 | NO-DIST |
| `P/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | red | red 0/1 | red 0/1 | NO-DIST |
| `P/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | red | red 0/1 | red 0/1 | NO-DIST |
| `P/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | red | red 0/1 | red 0/1 | NO-DIST |
| `P/📋️forms/🗿️artifacts/📋️forms/S1/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts` | red | red 16/2 | red 17/1 | derived child handles are now rebuilt by parseFormsChild: `assert.equal` -> `deepEqual`; remaining failure is NO-DIST forms-js |
| `P/🔱️trinity/🗿️artifacts/♻️rewriting/S1/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | red | red 48/2 | green 50/0 | map delta wire is a keyed object (framework map schema): test input, 6 committed `🔺️diff` fixtures and the rewriting diff schema `entries` array -> object |
| `P/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🟦️.test.ts` | red | red 54/1 | green 55/0 | `advance(&mut self, body:usize, grant:` signature of fem 2d/3d editors accepted by the law regex |
| `P/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | red | red 18/1 | red 18/1 | tree-sitter-rust grammar 0.1.13 cannot parse async closures `async |x|` (toolchain) |
| `P/🧩️puzzle/🗿️artifacts/◻️2d/S1/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | red | red 61/2 | green 63/0 | tests dir moved one level down (`io/sqlite/snapshot/tests`): 5 -> 4 `../` |
| `P/🧩️puzzle/🗿️artifacts/🧊️3d/S1/✏️editor/⏳️precompute/🪣️fill/🎟️caller/🧪️tests/🟦️.ts` | red | red 3/1 | green 4/0 | unit root `🧪️tests/🔬️unit/🦀️.rs` is a mount root: tree reader |
| `P/🧩️puzzle/🗿️artifacts/🧊️3d/S1/✏️editor/🌉️wasm/🧪️tests/🟦️.ts` | red | red 0/1 | red 0/1 | law forbids `ArtifactVcs` in puzzle-3d wasm bridge; code still declares `Puzzle3dArtifactVcs` (VCS removal pending) |
| `P/🧩️puzzle/🗿️artifacts/🧊️3d/S1/✏️editor/🧵️retained/🎬️action/🧪️tests/🟦️.ts` | red | red 1/2 | green 3/0 | Rust readers -> tree readers; `OriginalFixtureScope` / `close_registered_fixture_app` needles |
| `P/🧪️tests/📁fixture-placement/🟦️.ts` | red | red 35/4 | red 35/4 | 545 fixtures outside `🧪️tests` (533 FEM): 13 energy + 2 puzzle + 2 schema-ancestor moved, pv-plane TS import via test module; FEM left (generator path tables); BIM glb/ifc blessed output needs cargo |
| `P/🪐️space/🗿️artifacts/🏠️home/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | red | red 0/1 | red 0/1 | NO-DIST |
| `P/🪐️space/🗿️artifacts/🪐️space/📦️packages/🟦️typescript/🧪️tests/🪶️sqlite/🟦️.ts` | red | red 0/1 | red 0/1 | NO-DIST |
| `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🔬️neutral/🟦️.ts` | red | red 1/1 | red 1/1 | needs SEMIO_FEM_NEUTRAL_RESULTS (cargo native output) |
| `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🟦️.ts` | red | red 4/1 | red 4/1 | needs SEMIO_FEM_NEUTRAL_RESULTS (cargo native output) |
| `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🧪️tests/❄️cold/🟦️.ts` | red | red 1/1 | green 2/0 | `pub fn pcg(` lives in a split module: tree reader |
| `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts` | red | red 0/1 | red 0/1 | live launcher (playground preparation invocation) |
| `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧩️native-prerequisites/🟦️.ts` | red | red 0/5 | red 0/5 | live launcher (bounded Nx namespace) |
| `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/🏃️command-ownership/🟦️.ts` | red | red 0/3 | red 0/3 | owned capture limit (live launcher) |
| `OS/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🗃️model/🧪️tests/🎟️admission/🟦️.ts` | (not listed) regression | red 0/1 | green 1/0 | `🪣️fill-job` split into `➕️normal/🪣️fill/**`: tree reader; `step(scene,style,brush,cx)` signature |
| `OS/🔨️modules/🏪️store/👷️worker/🧪️tests/📥️values/🟦️.ts` | (not listed) regression | red 0/1 | green 1/0 | `validateBrowserActorUiValues` moved to `🧩️actor-wire-helpers` |
| `OS/🔨️modules/🏪️store/👷️worker/🧪️tests/🔁️acyclic-import-graph/🟦️.ts` | red | red 1/1 | green 2/0 | store worker import cycle fixed by peers |
| `OS/📦️packages/🟦️typescript/📜️script.ts` | (not listed) regression | red 0/1 | red 0/1 | config-diff-twin fixtures moved to `🎚️config/🧪️tests/🧫️fixtures/🧬️mutations`; remaining: `retry` not installed, `cockatiel` not hoisted |
| `OS/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | red | red 22/4 | red 22/4 | tree-sitter refuses Rust source + source-law drift |
| `OS/🔨️modules/🏪️store/🕰️history/📐️planning/🧪️tests/🟦️.ts` | red | red 5/1 | green 6/0 | completed-preview needle re-derived from `step_preview`/`settle` |
| `OS/🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🟦️.ts` | red | red 4/1 | red 4/1 | standalone rustc isolation no longer links (module imports carved crates) |
| `OS/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/📄️catalog-retirement/🟦️.ts` | red | red 13/1 | green 14/0 | slice bounded to the `artifact_retirement_box_*` fns (tree order pulled a later trait) |
| `OS/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts` | red | red 24/1 | green 25/0 | `.max(grant.maximum_capacity_bytes)` form accepted; bare `maximum_capacity_bytes: demand.capacity_bytes` still refused |
| `OS/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts` | red | red 9/2 | green 11/0 | `advance_demands`/`next_demand` replace four per-lane `next_advance_*_demand`; `semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES` |
| `OS/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🔮️oracle/🟦️.ts` | red | red 14/2 | red 15/1 | `pub fn seal_prepared_owned` (cross-crate callers) vs law `pub(crate)`: law/crate-layout conflict, owner decision |
| `OS/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts` | red | red 14/1 | green 15/0 | `advance(grant, receiving)` signature |
| `OS/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source/🎟️control/🧪️tests/🟦️.ts` | red | red 1/1 | green 2/0 | fixture `requiredReceivingPorts` advance signature |
| `OS/🔨️modules/🏪️store/🪆️child/🛬️receive/🧪️tests/🟦️.ts` | red | red 0/1 | green 1/0 | `header.advance(original,control,body,grant)` |
| `OS/🔨️modules/📡️spr/📜️history/🛬️decode/🧪️tests/🟦️.ts` | red | red 2/1 | green 3/0 | History driver test is a mount root: tree reader |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🎟️original/🧪️tests/🟦️.ts` | red | red 2/1 | green 3/0 | `pub struct RendererAssetDecodeJob` (was pub(crate)); slice to EOF |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🧪️tests/📇️catalog/🟦️.ts` | red | red 0/1 | red 0/1 | live launcher (Original process issuance) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🎟️source/🧪️tests/🟦️.ts` | red | red 1/1 | red 1/1 | tree readers, `map_err(ProgramFault::from)`, Drop-based `begin_close`, ProgramBridge lifted to contracts crate; remaining: `ProgramFault` derives Clone/PartialEq with a non-Clone `create_app_source` field (law wants Debug only) |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🧩️compile/🧪️tests/🟦️.ts` | red | red 1/1 | green 2/0 | `pub fn poll_compiler`/`dispatch`; tree readers |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🚪️native-io/🎟️source/🧪️tests/🟦️.ts` | red | red 0/1 | green 1/0 | kernel runtime root lifted to kernel-protocol, asset stream moved to presentation-raster, `pub fn pump_renderer_source_io_sessions` |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/📬️outcome/🧪️tests/🟦️.ts` | red | red 5/2 | green 7/0 | `AppFramePreparation` moved to frame-build; `retire_for_component_surface_close_step` -> `retire_original_frame_step`; mailbox `pub fn close_engine_surface_step` + registry path |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧲️engine-surface-retention/🟦️.ts` | red | red 8/4 | red 8/4 | Playwright chromium 1248 not installed |
| `OS/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🧭️ownership/🟦️.ts` | red | red 3/1 | red 3/1 | child script invocation missing (live launcher) |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | red | red 55/1 | red 55/1 | Bun 1.3.14: byte accounting of NUL-containing TEXT differs by 1 per NUL string from the committed corpus (owner decision) |
| `OS/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | red | red 41/1 | red 41/1 | same as above |
| `OS/🔨️modules/🔌️plugin/♻️cancellation/📦️lease/🧪️tests/🟦️.ts` | red | red 3/1 | green 4/0 | `try_serialize_cold` |
| `OS/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/💼️job/📤️output/🧪️tests/🟦️.ts` | (not listed) regression | red 2/1 | green 3/0 | dashboard `run-script` command gained `--dry-run={dry-run}` |
| `OS/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🎟️receipt/🧪️tests/🟦️.ts` | red | red 0/1 | green 1/0 | receipts routed before `for frame_bytes in output.frames` (typed_operation_results loop gone) |
| `OS/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🧪️tests/🟦️.ts` | red | red 0/2 | green 2/0 | authority text scoped to `🤝️authority`; whitespace-insensitive close-step needle |
| `OS/🔨️modules/🔌️plugin/🏇️mounted-owner/🧪️tests/🟦️.ts` | red | red 13/5 | red 13/5 | needs native result artifact (cargo) |
| `OS/🔨️modules/🔌️plugin/📇️registry/📥️runtime/🧪️tests/🟦️.ts` | red | red 2/1 | green 3/0 | cargo-test output name `native_catalog_empty_alternate_and_refused_owners_match_portable_corpus ... ok` |
| `OS/🔨️modules/🔌️plugin/📇️registry/🛂️descriptor-verification/🧪️tests/🟦️.ts` | (not listed) regression | red 20/1 | green 21/0 | 240 s timeout under 2-worker load (green in the later runs) |
| `OS/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧬️semantic-source-pairs/🟦️.ts` | (not listed) red | red 0/1 | red 0/1 | committed JSON/pack descriptor pairs disagree for 14 hub components (JSON regenerated 10-15, pack 10-13): needs cargo/wasm regeneration |
| `OS/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🟦️.ts` | red | red 3/1 | green 4/0 | bootstrap uses `readScriptProcessEnvelope(JSON.parse(env[SCRIPT_PROCESS_INVOCATION_ENV]!),Date.now())` |
| `OS/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🟦️.ts` | red | red 7/2 | green 9/0 | body slice bounded to its fn; fixture dispatch checked in its own module; tree readers |
| `OS/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts` | red | red 7/6 | red 7/6 | hash pins of Rust bodies / mount lists (needs re-bless by Rust owners) |
| `OS/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts` | red | red 26/4 | red 26/4 | aggregate `includes` laws on unrelated Rust (SOURCE-LAW-DRIFT) |
| `OS/🔨️modules/🔌️plugin/🧵️retained-command/🔗️read/🧪️tests/🟦️.ts` | red | red 0/1 | green 1/0 | `source.matches(cx)` replaces `cx.operation().0`; tree reader |
| `OS/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️unit/🟦️.ts` | red | red 0/1 | red 0/1 | cargo-running unit (native caller capabilities) |
| `OS/🔨️modules/🛎️services/🚪️native-io/🎟️original/🧪️tests/🟦️.ts` | red | red 0/1 | green 1/0 | removed dead `#[doc(hidden)] pub fn native_io_close_grant` |
| `OS/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/🟦️.ts` | red | red 42/1 | red 39/4 | `buildAdmittedWasmWebV1` accepted besides `buildRepositoryWasmWebV1`; remaining: peer taxonomy edit (`pathBudgetReserve`), play site script, bounded browser inventory |
| `OS/🧪️tests/📦️crate-boundaries/🟦️.ts` | red | red 112/5 | red 126/1 | peers fixed to 127/0; a later peer Cargo edit (mutation-diff -> semio-framework-value) fails 1 |
| `P/🏙️bim/🗿️artifacts/🏢️model/S1/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | green (not red in tst3b) | green 13/0 | red 12/1 | peer in flight: `runArtifactRustPackageMain` now `adopt()`s a ScriptRouter, the bim package script still passes a record |
| `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/📥️checked/🟦️.ts` | green (not red in tst3b) | green 3/0 | red 2/1 | peer split of gis `💡️inference-port` test (fixture-count law) |
| `OS/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🟦️.ts` | green (not red in tst3b) | green 34/0 | green 34/0 | `require("@semio-tech/framework-plugin/**/schema.json")` now resolves to a json facade `.ts` (namespace, not schema): `schemaOf` unwraps `default` |
| `OS/🔨️modules/🧑‍💻dev/🚀️local-hub/🎫️session/🧪️tests/🟦️.ts` | green (not red in tst3b) | green 16/0 | green 16/0 | 2 s resource-loan deadline under 2-worker load (flaky) |

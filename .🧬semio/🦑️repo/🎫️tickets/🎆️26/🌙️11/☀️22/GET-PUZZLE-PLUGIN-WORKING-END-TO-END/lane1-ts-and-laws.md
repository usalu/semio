# Lane 1 — TypeScript Wiring and Source Laws

Ticket: `GET-PUZZLE-PLUGIN-WORKING-END-TO-END`. Lane: `ts-wiring-and-source-laws`.
Runner: native Bun `1.4.2` through the ticket `launch-script.ts`. No `cargo` invocation from this lane.

## Result

Source and TypeScript checks that this lane can run are green after the fixes below. Three graph wire-checks and five Puzzle 5D verifiers are cargo tests; they were recorded and not executed. The hub `fixtures lint` command is repo-wide and fails on writer and mathematical artifacts, with zero Puzzle findings.

## 1. Graph generation and wire checks

| Command | Package | Exit | Result |
|---|---|---|---|
| `graph-generate` | `@semio-tech/puzzle-2d-rs` | 0 | Wrote 2 manifests under `🗿️artifacts/◻️2d/🤖️generated` (package manifest and the Nakagin example). |
| `graph-wire-check` | `@semio-tech/puzzle-2d-rs` | 1 | Not a manifest defect. The check runs `cargo test … owner_wire_law`. rustc's wrapper refuses: "Original compiler ingress requires the repository launcher and authentic enforcing store". |
| `graph-generate` | `@semio-tech/puzzle-3d-rs` | 0 | Wrote 1 manifest. |
| `graph-wire-check` | `@semio-tech/puzzle-3d-rs` | 1 | Same compiler-ingress refusal. |
| `graph-generate` | `@semio-tech/puzzle-5d-rs` | 0 | Wrote 1 manifest. |
| `graph-wire-check` | `@semio-tech/puzzle-5d-rs` | 1 | Same compiler-ingress refusal. |

## 2. Dead `store-close` targets

Commit `1f0dfd57f85` ("Retire cold bounds…") deleted `🔨️modules/🏠️host/🧰️owned/🧪️tests/♻️store-close/🟦️.ts` and its Rust twin. Nothing replaced `standalone_history_close_resumes_past_the_original_ledger_page`.

Removed from `◻️2d` `📋️project.json`: targets `test-store-close-source` and `test-store-close-native`, and the `nativeSources` glob for the deleted test. Removed `StoreCloseScript` and its registration from `📜️script.ts`. Deleted the unread fixture `♻️store-close/🧫️fixtures/🔣️.json`. Left `🧬️schema/♻️store-close` in place; it is a separate schema, not a consumer of that fixture.

## 3. TypeScript and source-law suites

| Check | Exit | Count |
|---|---|---|
| `@semio-tech/puzzle-js` vitest (`🧪️tests/🎚️config/🟦️.ts`) | 0 | 14 files, 54 tests |
| `publication-authority-audit` on the puzzle TypeScript package | 0 | Owners `Puzzle2dPlayApp`, `Puzzle3dPlayApp`, `Puzzle5dPlayApp`; `windowOwnershipCases=7`; oracle independent |
| `@semio-tech/puzzle-5d-react` vitest | 0 | 1 file, 1 test |
| `verify puzzle-fill-policy-self-tests` | 0 | 73 fill checks |
| `verify browser-contribution` | 0 | Owner entry, package, engine, and independent compiler/schema authority |
| 2D `verify snapshot-sqlite-source` | 0 | `tsc --noEmit` of the schema snapshot plus the io sqlite module and its test |
| 2D `test-presence source` | 0 | rustfmt check only. The native branch is cargo and was not run |
| 3D `verify snapshot-sqlite-source` | 0 | `tsc --noEmit` |
| 3D `verify original-work-construction` | 0 | 4 tests, 49 expects. Final rerun after the story-path fix |
| 5D `verify snapshot-sqlite-source` | 0 | `tsc --noEmit` |
| 5D `verify original-work-construction` | 0 | 1 test (catalog weights plus the current cursor source law) |
| Hub retained `test-ownership` | 0 | 9 tests |

### Cargo verifiers not run

These 5D commands call `runArtifactRustPackagesTests` (`cargo test -p semio-s-artifact-puzzle-5d-v1-unit-tests --lib`):

- `verify optional-fields` → `optional_part_and_grip_fields_round_trip_against_independent_json`
- `verify inline-capacity` → `inline_import_matches_independent_json_capacity_boundary`
- `verify catalog-fields` → `catalog_extensions_round_trip_against_independent_json`
- `verify edit-acceptance` → `created_part_board_and_world_edits_reopen_with_history`
- `verify import-acceptance` → `import_identity_matches_neutral_fixtures_and_independent_json`

### Hub `fixtures lint`

Exit 1. The puzzle composition script walks every mutation tree in the repository. Puzzle trees contributed no findings. The failure is 40 missing case files (25 writer, 15 mathematical) plus 6632 derived-encoding gaps reported as 13095 errors. Those trees belong to other plugins and were left unchanged.

### Fixes

- 2D `verify snapshot-sqlite-source` typechecked a deleted test under `schema/snapshot/tests/sqlite`. It now typechecks the schema snapshot module plus `io/sqlite/snapshot` and its test.
- 3D close-custody law expected `PluginApp::maintenance_step(raw, cx, loan)`. The fixture now calls `raw.mounted_owner_maintenance_step(&mut turn)` after `MountedOwnerTurnV1::admit`. The law asserts that call.
- 3D and 5D parser laws require the hashed `puzzle*ParseDslJson` function and forbid `ArtifactVcs`, `funded_turn`, and `StepContext::new` in the file that owns the parser. Parser bodies were not edited. Hashes stay `c47ec76d…` (3D) and `15558fd9…` (5D). The envelope handle moved into a sibling `envelope.rs` (`mod envelope; pub use envelope::*;`). The 5D parser stays in `wasm/bridge` because its body uses `crate::Puzzle5dSnapshot` of the host-admission crate. The law reads the bridge when the wasm facade does not contain the function. The story URL now points at `s/dev/puzzle/stories`, which is where `3d-world` and `5d-timeline` live.
- 5D scene splice test indexed `Splice` with a `Table` key (`TS7053`: `fasteners` is not on every splice alternative). Rows are read through a table switch that checks the key is present.
- 5D catalog source law expected `cursor.next_advance_demands`, `admit_original_tool_construction_scoped`, and `insert: self.insert.take()`. The gesture module now uses `cursor.advance_turn`, `cx.with_original_source_effect`, and `from_original_construction`. The law asserts those.
- Retained ownership: the 5D leaf `editor/tests/retained-wiring` did not exist. Added that test, mounted it from the 5D unit tests, and pointed it at `super::editor_source()` plus the framework factory. The 3D and 5D laws now accept `framework.contains` as well as `source.contains`, and the contract strings match the factory text (`from_wire_with_checkpoint(*payload,input,checkpoint)`, `from_wire(*payload,input)`, `bytes>super::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES`).

## 4. Storybook 2D "Lasso Select"

No file change. `LassoSelect` in `s/dev/puzzle/stories/2d-board` already sets `initialRuntime.selectionMethod` to `"lasso"`. `buildStorySceneNode` copies that onto `board2d`. `Board2dHost` reads `node.board2d` and the lifecycle effect calls `session.setSelectionOptions(scene.selectionMethod, …)`. The wasm binding forwards that to `host.focus.set_selection_options`. The default engine method is `"rectangle"`. A closed lasso that returns to its start is a degenerate rectangle and selects nothing unless the method is `"lasso"`. The storybook host path already passes `"lasso"`. `sync_host_runtime_state` in the puzzle 2D play app still hardcodes `"rectangle"`; that path is not the storybook `BoardSession`.

## 5. Files changed

- `plugins/puzzle/artifacts/2d/packages/rust/project.json` — removed dead store-close targets and the nativeSources glob.
- `plugins/puzzle/artifacts/2d/packages/rust/script.ts` — removed `StoreCloseScript`.
- `plugins/puzzle/artifacts/2d/tests/rust-package-commands` — sqlite verifier paths.
- Deleted the unread 2D store-close fixture json.
- `plugins/puzzle/artifacts/3d/editor/wasm/rs` — parser module only.
- `plugins/puzzle/artifacts/3d/editor/wasm/envelope.rs` — envelope handle moved here.
- `plugins/puzzle/artifacts/3d/editor/wasm/tests` — read the 5D bridge when the facade has no parser; stories resolve under dev.
- `plugins/puzzle/artifacts/3d/editor/retained/action/tests` — mounted-owner maintenance assertion.
- `plugins/puzzle/artifacts/5d/editor/wasm/bridge/rs` — parser only.
- `plugins/puzzle/artifacts/5d/editor/wasm/bridge/envelope.rs` — envelope handle moved here.
- `plugins/puzzle/artifacts/5d/tests/scene` — typed splice row access.
- `plugins/puzzle/artifacts/5d/editor/catalog/tests` — current cursor source law.
- `plugins/puzzle/artifacts/5d/editor/tests/unit/rs` — mounts `retained_wiring`.
- `plugins/puzzle/artifacts/5d/editor/tests/retained-wiring/rs` — new wiring law.
- `hub/compositions/puzzle/retained/tests/ownership` — accepts framework or source assertions.
- `hub/compositions/puzzle/retained/tests/fixtures/ownership/json` — current factory strings.

`schema/store-close` was not removed. No `AGENTS.md` edit. No git mutation.

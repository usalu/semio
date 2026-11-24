# 🧩️ Curation lib, remaining failures root-caused (2026-11-23)

Crate `semio-s-artifact-sourcing-curation`, `--lib --test-threads=1`, native cargo (`native-cargo.sh`).
Start 80 pass / 16 fail. Final counts: see "Result" at the bottom.

## 1. Add / count / remove / drop / sort / set-contributions published nothing

Not a framework drop and not a missing effect. Instrumented the retained reducer (`sourcing_curation_retained_reduce`):
the command it received was `CurationAdd { object_id: "" }` while the snapshot had 12 stock rows, so
`curation_decision_for_delta` resolved `NoOp` and the emit was empty (`muts=0 cfg=0 effects=0`). An empty emit is
exactly the `[Ui, Terminal]` receipt: the typed-operation unit falls through to the UI/terminal page branch.

Cause: the typed fixtures and the generic `command -> args` projection (`original_fixture_command_arguments`) go through
the command's `ToValue`, which keeps Rust field names (`object_id`, `column_id`, `module_id`, `example_id`).
The app's action bridge `sourcing_curation_command_from_action` (and the manifest, and the TS windows) address camelCase
(`objectId`, `columnId`, `moduleId`, `exampleId`). The key mismatch silently produced empty strings.
Every other plugin with multi-word command fields declares `#[value(rename_all = "camelCase")]`; the eight sourcing
command payloads with multi-word fields did not.

Fix: `#[value(rename_all = "camelCase")]` on `CurationAdd`, `CurationRemove`, `CurationSetCount`, `DropOnPool`,
`DropOnCurated`, `SortTable`, `SetFilterModule`, `SetActiveExample`. One-word payloads (`json`, `value`, `path`, `delta`) are unaffected.
This also fixed `setContributions`/sort/example/grid/pool tests that were downstream of the empty payloads (the grid "no unit box mesh",
"demo stock empty" and "captured shared original changed" symptoms all disappeared).

Browser note: the browser sends camelCase args (`{objectId}`), so this key mismatch is not what the browser hit; the browser
"does nothing" symptom was the one-item stall already fixed in `seal_prepared` (foreign-step presence). Not re-verified in a browser here.

## 2. `JSON parse work quote was not admitted`

`AuthoringWallet::run` reuses one Work scope across runs. `OriginalWorkState::prepare` quotes
`observed + additional <= remaining` where `observed = actual - baseline`, and the wallet's `remaining` is already net of what earlier
runs spent, so work spent by earlier runs was counted twice. A multi-step parse of anything past the first 4096-unit step was refused
(`pack_json::parse`, schema validation of an action argument, the schema/inferences unit test).

Fix (framework, general): `AuthoringWallet::run_with` starts each run as its own turn (`scope.begin_turn()`), the same way a host turn
resets the baseline. File `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️authoring/🦀️.rs`.

## 3. History acceptance and archive/example loads

Archive reload and the example load died on `initialization requires its original installed native semantic producer`,
history on `original report requires its installed semantic and retirement issuers`: the document owner catalog carried no replay issuer.
Sourcing hand-implemented only the one-item publication half (`SourcingCurationArtifactPreparationFactory`) and used the default catalog.

Fixes (production, wasm):
- `🗄️artifact-store-preparation/🦀️.rs` rewritten: the hand-written preparation (about 180 lines) is replaced by the shared `ColdMutationApplyFactory`
  with `ColdPreparationBounds` (same 4096 items, 256 byte ids, 512 KiB envelope), installed as ONE shared issuer through
  `bounded_artifact_store_owners_with_shared_semantic_preparation`; `ArtifactEditor` overrides `document_store_owners_source_demands` / `build_document_store_owners`
  (`🧩️sourcing-app/🦀️.rs`). Operation wire: `with_original_receiving()` plus `whole_document_target` as conflict target.
- `NativeReceive` + `DslReceiving`/`DslEnum` derives on `SourcingMutation`, its three leaves and `CuratedItem` (required by `with_original_receiving`).
- Conflict target is the whole document (conservative). Per-object targets (`curated/<objectId>`) would need an `ArtifactOperationTargets` impl and a dependency on `semio-os-store-operation-wire`; left as a multi-user refinement.

Open: `history_edits_end_to_end` still fails with `faulted original JSON receiving requires cleanup` (a JSON mutation receive in the acceptance law faults, then the
cleanup masks the first cause). `conflict_history_edits_end_to_end` and the archive tests now pass. Not root-caused further.

## 4. `retained_example_load` doc id

The test compared the load's document id with the controller id string; the stamp (`stamp_load_document_identity`) correctly names the live store.
The test now compares with the live store's own archive id.

## 5. `renders_pool_table_scene` "captured shared original changed its Work or destination"

No longer reproduces: it was a consequence of operations wedged by the empty payloads (item 1) leaving a half-closed shared custody.

## Result
Curation lib `--test-threads=1`: 96 passed, 1 failed (`history_edits_end_to_end`).

## Coordinator question: why is a tests crate in the windows component closure
`semio-s-artifact-sourcing-curation-tests` is a mirror crate that `#[path]`-includes the editor sources (`🪞️mirror`); the lib compiles into whatever links the
sourcing editor, so a compile error in editor code shows in both. My intermediate `with_original_receiving()` (E0599: `SourcingMutation: NativeReceive`) was the cause and is fixed
by the `NativeReceive` derives; the final tree compiles natively, wasm32 not re-run here.

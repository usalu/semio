# Duplicate Layer Placement

Source inspection confirmed the native command always inserted at root, losing the inherited coordinate frame of nested sources. The shared fixture specifies adjacent insertion in the same sibling stack, including a translated, rotated, nonuniformly scaled parent, own-locked sources, an inherited lock refusal and a missing target refusal. Copying retains source metadata, transforms, asset references and lock state with fresh recursive identities. Native mutation/inverse assertions require exact restoration.

The TypeScript planner is independently compared with d3-hierarchy parent/sibling discovery. Initial TypeScript run failed because the planner implementation was absent; this is an API red check, not a behavior failure. Native behavior red run is pending. Existing Nx test routes are reused; no new executable command is introduced.

Layer protection native run 6 completed: 297 passed, zero skipped. Its log is under generated/raster-lock-native-6.log. This is native test evidence, not live end-user acceptance. The following native run also covers final assertion/source edits made during that build.


Inspector source inspection found no selected-layer Duplicate or Delete controls, despite registered command handlers. Neutral selection fixtures and native tests now require both controls for one selected layer, EN/DE labels, exact command/target bindings, inherited-parent protection on duplication and structural protection on deletion. Empty/multiple selection must not expose a single-target action. Controls remain unimplemented while the native behavior-red build is running. Shared command fixture additionally contains a nondefault unlinked/inverted transformed mask and shared image references; native checks compare all copied metadata recursively.


The native handler now uses a shared placement/admission plan and inserts after the source under the same parent. The Inspector exposes localized Duplicate and Delete controls for a single layer and applies matching protection policy. Tests were authored first; TypeScript had an observed missing-implementation failure followed by 84 passing tests. The native run named red-1 was still compiling shared dependencies when the implementation was applied, so it must not be described as an observed native behavior-red run. Its eventual outcome may include the implementation; a final-source native pass is required.


Remaining interaction follow-through: framework selection stays on the source after duplication; Delete Layer source comments document Flat-domain deleted-selection pruning as a framework gap. This change intentionally does not reintroduce app-owned selection state. A supported framework-owned selection transition/pruning path still needs verification for the complete end-user workflow. Large subtree duplication also still uses recursive cloning and needs a bounded-work review.


Native run red-1 compiled the new implementation and ran 299 tests: 298 passed, one failed in the new recursive metadata test. The test itself used mutable JSON indexing of a missing children property, which inserted children:null into pixel expectations. Replaced that access with get_mut; production copy fields were unchanged. Both-language Inspector action/disabled/binding coverage passed in this run. Final native revalidation is running after the test-helper repair.


## Files Touched in This Continuation

Under the Raster any-subset editor: commands/duplicate-layer native handler, TypeScript planner, shared fixture, native tests and TypeScript tests; Inspector native render, selection fixture and native tests; terminology native labels. The existing Raster TypeScript package script adds the duplication tests. Ticket notes updated: complete-editing, acceptance, layer-protection, duplicate-placement and interactive-export. No scripts or executable targets were added, and no Git mutation was performed.

Final TypeScript source run 3: 84 passed, zero failed (`raster-duplicate-ts-green-3.log`). Native green-1 remains running after the JSON test-helper repair. Activation 11 succeeded, but this is build/activation evidence only; live UI acceptance remains unverified.


Duplicate final native run green-1 completed successfully: 299 tests passed, zero skipped (`raster-duplicate-native-green-1.log`). This verifies same-parent duplication, metadata/inverse assertions and both-language Inspector action bindings/protection. Live browser acceptance remains open.
# Selection Follow-Up Investigation

Add Layer and Duplicate Layer currently leave selection unchanged. Their comments correctly prohibit writing the framework-owned selection through Raster config mutations, but do not rule out requesting the framework interaction verb. Layout's Add Frame emits an `interactionSelect` dispatch effect for its newly created identifier. The current shared typed publication ladder parks those effects and applies them only after artifact/config/child publications have drained, explicitly ensuring a selection names an existing document entity. This is the available mechanism for selecting a newly added or duplicated Raster layer without introducing a second selection store.

Before implementing this follow-up, add neutral action arguments and native retained-publication assertions covering the selected new identity, no host replay effect, and deletion/undo behavior. Current duplication placement/history verification does not prove selection follows the copy. No production selection change was made during this investigation.

## Selection Follow-Up Implementation

Added `editor/🖱️selection` with a neutral schema/fixture and Rust/TypeScript request builders. Add Layer, Drop Layer Kind and Duplicate Layer now emit one framework `interactionSelect` request for their new identifier. The framework's existing post-publication interaction lane owns the state change; no Raster config selection or extra document mutation was added.

The observed TypeScript red was the missing request-builder module, with 120 existing tests passing. After implementation, **124 tests pass, 0 fail**. The new cases validate exact domain/merge/method and preserve quoted, escaped and Unicode IDs. Authored native coverage checks the same neutral wire, creation/duplication focus, no host replay effect, pruning after delete, and four undos returning through the four document edits. Native verification is pending. Run 6 was already active during these changes; the full current expected Raster census is **313**, and the actual executed census/source must be checked before claiming native coverage.

Native run 6 failed at production compilation in the new selection helper: `dsl::json::object` requires `JsonValue`, while the action takes `DslValue`. The helper now builds JSON values and explicitly converts with the existing `to_dsl_value`; the neutral native test serializes the command value through `to_json_string`. Run 7 is active against this correction. Run 6 did not execute the export-disposer or editable-archive repairs.

## Selection Validation Follow-up

Native run 7 passed creation/drop/duplicate selection but failed automatic pruning after delete. Raster's Flat interaction declaration intentionally had no membership validation. It now declares Topology and provides the actual nested layer IDs, parent IDs and layer granularity to the framework. This allows framework validation to handle delete, undo/redo and remote document changes consistently; no config selection mirror or manual delete-only clearing was added. Source comments describing the old gap were corrected. Full native run 8 is pending.

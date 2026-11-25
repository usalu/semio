# Cancel receipt: stage 14 double debit, folder-reload and gesture laws

## Result
- `semio-plugin-tests-07-3`: 10 of 12 pass. That includes the new law `a_cancel_at_every_ladder_stage_receipts_each_step_exactly_once`, the cancel-at-any-point law, the cancelled-load law, the refused-load law and all 3 gesture laws. The 2 still failing are `a_folder_bound_document_reloads_...` and `two_programs_on_one_folder_converge_...` (see Open).
- `semio-plugin-tests-06` (needs `--features artifact-app-testing`): the operation law still fails at its heap assertion; the cause is not in code I own (see Open).
- `semio-plugin-native-identity-tests` closed-original registration law: still fails, same kind of cause (see Open).
- Unit law port (`tool-run/unit/tests`): ported to the current job API and type-checks. Not run, because its host crate `semio-plugin-tests-13` has about 70 unrelated compile errors (see Open).

## 1. Root cause of the stage-14 cancel failure
Stage 14 is `drive_store_replacement_jobs`. A cancel during store-replacement initialisation retires the rejected worker source through `RefusedWorkerSource::close_step`, which posts the reserved-return effect (1 item, 118 bytes) on the context. The ownership wrapper `original_lifecycle_step` qualifies a receipt against the effects posted without the reserved ones. A receipt that already includes the reserved return was therefore debited a second time. The debug trace showed (2 items, 236 bytes) posted against a reported (1, 118). It surfaced as "original lifecycle child receipt differs from its whole producer receipt", and on the close path as a phase-loan overflow (2 items on a loan of 1).

Three owners composed this the wrong way:
- `original_rejection_lifecycle` (the rejected source closure inside the initializer) ran under the plain `original_lifecycle_step`.
- `drive_store_replacement_jobs` composed only in two states (Initializing, OpeningMembers). The close path hit a third.
- The generic close wrapper in `plugin-app-impl` (`close_step`) wrapped the whole close ladder in the plain step, so the live cleanup after a cancel also failed. This is the live-cleanup fault the browser sees about 28 s after a cancelled Concrete Forest load.

## 2. Fixes
- New `original_composed_lifecycle_step` in `document/envelope-decode-and-ingress`, re-exported by `semio-plugin-document-media`. It is the former private ladder helper, moved down. The owner's receipt covers the reserved return effects, so they leave the unposted debit once and rejoin the returned receipt. I did not put it in `semio-plugin-ownership` because that crate is at its source-size limit (the carve is in progress); that crate is untouched by me.
- `original_rejection_lifecycle`, `drive_store_replacement_jobs` (now always composed) and the `plugin-app-impl` `close_step` wrapper all use it. The ladder's private copy is removed.
- Refused-load codes: the original admission (`OriginalDocumentArchiveAdmission::validate_step`) had lost the named `plugin.document-load.*` faults and answered a bare `InvalidValue`. It now refuses with `too-many-members`, `incomplete`, `too-large` and `member-invalid`, with the `count`, `maximum`, `bytes` and `ordinal` params the notices need. The refused-load law passes.

## 3. Laws and test ports
- New law `a_cancel_at_every_ladder_stage_receipts_each_step_exactly_once` (`folder-reload-route`). It cancels a load after every poll count. For each cancel it either pays the ladder turn by turn or, for every other cancel, goes straight to the live cleanup (`close`). Per turn it checks that the receipt equals the posted effects on items, copied bytes and capacity, and that no release is posted beyond it. It asserts that some cancel paid the ladder while a store replacement was still Initializing (the stage-14 state). It does not name stage numbers.
- `maintain_turn` (renewed host turn, issuer, one maintenance step) is shared by `poll_load` and the cancelled-load law. That law previously never renewed its turn and so never retired.
- `drive_archive_host` now uses the test's own caller context. It used a fresh caller per call, which could never retire next to the app's cancellation issuer.
- tests-06 `verb` now dispatches through `dispatch_fixture_action`. A selected input that cancels a live preview answers its documented "must settle" refusal, so `verb` pumps the history until idle and retries.
- Gesture laws: the render overlay is refolded in paid maintenance turns, so the laws pay it (`settle_overlay`) before reading the rendered document. Host-fact delivery reaches the app with the next action, so the history-edit law flushes it with a blur and counts the Frozen events. No assertion was weakened.
- `tool-run/unit/tests` is ported from the old job step API to `OriginalJobFixtureCaller`, `OriginalJobReturnOrigin` and `step(cx, claim)`. It keeps the original assertions, including heap equals receipt.

## Open (not fixed here, with evidence)
- Folder-bound reload and two-programs laws: after my changes they stop at "original operation has no authored conflict path issuer", then "installed operation wire has no original receiving capability". The instance ingress installs only `semantic().with_batch_digest()`; production apps never install `with_original_target` or `with_original_receiving`, and the generic app mutation is not bounded by `NativeReceive<DslValue>`. This belongs to the operation-wire work other agents have uncommitted right now. I tried adding both at the ingress and reverted it (the bound is not satisfied).
- tests-06 operation law (heap 392/0 reported vs 915/48 observed) and the registration law (heap 39,708/2,443 vs receipt 36,760/0, during construction): both are heap-versus-receipt mismatches inside Pack encoder turns. `pack/value/encode/borrowed/cursor` carries an uncommitted rewrite that adds an `order` PagedList to the symbol table; the receipts of that list's births and pops are the likely missing term. I did not edit another agent's in-flight file and did not confirm this by running without that diff.
- tests-13 does not compile for reasons outside this change (missing `artifact_app_laws::project_and_retire_fixture_tree`, `observe_and_retire_fixture_tree`, `built_surface_scene`, `TestMemberStoreProducer`, old-API calls in other mounted files). I added the `artifact-app-testing` feature on its tests-06 dependency. The unit-law port was checked by type only.

## Changed files
Under `framework/products/os/modules/plugin/`:
- `document/envelope-decode-and-ingress`: composed step, rejection lifecycle.
- `document/` root: re-export.
- `document/document-archive-load`: named admission refusals.
- `maintenance-ladder`: private helper removed, store replacement always composed.
- `instance/plugin-app-impl`: close wrapper composed.
- `tests/folder-reload-route`: new law, `maintain_turn`, host driver, refusal message.
- `tests/gesture`: `settle_overlay`, host-event flush.
- `tests/time-travel`: `verb` through the real dispatch with settle-and-retry.
- `tests/tool-run/unit/tests`: port.
- `companions/tests-07-3` Cargo.toml: instance-app dependency.
- `companions/tests-13` Cargo.toml: feature on tests-06.
- Not mine: the `semio-plugin-ownership` size and carve changes seen in the tree.

Ticket folder: `cancel-receipt.md`, `cancel-cargo.sh` (copy of `pt06-cargo.sh` for the private build dir). The private build dir and all `cancel-*` outputs under the generated folder are deleted. All `[DEBUG]` code I added is removed.

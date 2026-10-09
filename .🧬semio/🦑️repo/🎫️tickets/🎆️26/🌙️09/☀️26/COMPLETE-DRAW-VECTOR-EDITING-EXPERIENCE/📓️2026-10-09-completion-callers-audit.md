# Completion Caller Custody Audit

This read-only audit follows the actual sealed `CompletionCell` introduction. The cell never physically releases from ordinary Drop; callers must retain original `ArtifactToolCompletion<A>` handles and return them through their declared controlled retirement. No Weak/raw Arc capability exists.

## Production Callers

| Owner in Plugin root | Original owner still requiring integration |
| --- | --- |
| `build_artifact_reserved_action_job`, around 30667 | The scope-local completion remains on builder error/None and on any proof/read admission error after creation. Request clone must retain custody when the app builder refuses. Successful Some returns the original handle with the job. |
| `build_artifact_reserved_media_job`, around 30699 | Same local original handle and request clone on proof/read/builder refusal or absent job. |
| bounded-operation fixture factory around 26042 | Captures `completion.inner.clone()`, then uses raw Arc pointer identity. Must capture the whole completion handle and compare via a narrowly scoped borrowed identity helper, retaining original factory capture through explicit retirement. |
| `preview_addressed_action`, around 31870–31950 | Scope-local original completion is dropped after both `take()` calls and on every early error. This preview has no mounted original carrier. Returning only an emit and ephemeral counts also drops actual ephemeral values; unsupported outputs must retain those original values. |
| `ChildEmissionPreviewJob`, around 21285–21332 | Close removes emit/ephemeral/completion as one shallow item. Must retain actual values and use exact app completion factory plus controlled completion handle retirement; no empty surrogate is allowed. |
| typed dispatch around 33552 | On success the local completion moves into mounted state. Every request/builder/dispatch error before that mount still needs original input/completion/context custody. |
| `TypedCommandFullOperationJob`, output close around 22561 | `drop(self.output.take())` loses original completion alias. It requires controlled completion retirement and independent four currency demands. |
| `NaturalFileImportJob`, around 38711 | Owns original completion and pending rejection; old close must return both through genuine authority. Its opaque decoder is a separate retained frontier. |
| mounted typed operation around 22307 | Original completion is shallowly dropped after a strong-count check. Coordinator owns replacement by explicit controlled return. Strong count is only diagnostic. |

`typed_completion_outbox` contains `TypedOperationCompletionWitness`, not completion handles. The drop around 34080 is therefore a separate metadata frontier, not a cell alias.

## Fixture Callers

Existing OS fixtures create local originals in transaction-unit-command-close (91), surface (841 and 1182), and plugin-builder-command tests (3267, 3287, 3333, 3376, 3466, 3473 and 3488). They create external/consumer clones and must explicitly retire every original clone. Their mock apps need concrete payload factories or must intentionally preserve/refuse unsupported originals. Test drops must not bypass the production custody law.

Other app tests use `test_new` in stdio PNG (271) and ZIP (407 and 443). They require the same original handle return. Those app owners are outside this Draw lane; no edits were made to their tests.

The new Draw completion witness consumes all aliases through ControlledRetirement and retains original rejected values. It is authored, not yet compiled or executed.

## Verification Limits

This audit is source evidence only. Production Kernel reached Plugin in the coordinator's latest captured check; successful current Plugin/Draw compilation and actual preview/dispatch terminal cleanup are not established. A guarded ordinary Drop finding is not a native runtime receipt.

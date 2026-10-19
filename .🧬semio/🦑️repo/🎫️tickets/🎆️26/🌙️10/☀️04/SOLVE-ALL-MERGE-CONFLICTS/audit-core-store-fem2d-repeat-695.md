# Independent Core, Store and FEM2D Audit — Repeat 695

## Result

No concrete newly introduced defect was found in the defining receivers independently inspected below. This is a source-only audit, not compilation or runtime confirmation. No source, Git or index changes were made. Only this audit Markdown was written.

## Worker Authority and Whole Failure Custody

Actual mounted receiver: framework/modules/job/worker/mounted-and-batch-sessions Rust, pump_one_for_interactive_turn lines 87–97. It snapshots before and granted from the supplied cx, invokes try_step_on_original_caller(cx), stores the actual checked-out owner before verification, BORROWS issued/progress, and requires issued == granted, progress.fits(granted), and before.checked_add(progress) == cx.retained_progress(). There is no receipt take or consume_retained call in this receiver. The pending receipt stays with the checked-out original producer even on verification refusal.

Actual authority receiver: worker/authority NativeWorkerAuthorityState::step_on_original_caller. It validates operation, generation, exact cancellation node and issuer clock; drives the existing producer with the same cx; computes physical after-before receipt; records issued_retained and retained_step_progress; marks receipt pending only on successful outcome. drive_step failure returns the original failure value. Actual session try_step_on_original_caller restores authority into SESSION_CLOSE and returns that same failure, rather than flattening it to ValueError or Blocked. The mounted caller propagates it by ?. This confirms the claimed same-caller path in actual definitions, not a similar span elsewhere.

## Store Actual Receiving Owners

Bundle advance_apply_batch is a free function taking StoreParts. Its inserted registry check and diagnostic/lane publication use current store.owners/store.log/store.cursor receivers, not obsolete self ownership. publish_prepared_diagnostics is defined in the same bundle owner and transfers diagnostics into store.log.envelope.edit_messages, retains displaced diagnostics through store.owners.displaced_retirements, and sets pending report messages/worst. Lane insertion is preflighted with the diagnostic-adjusted grant and contributes physical receipt to the publication result.

Bundle actually mounts ../../ingress/original/Rust and imports OriginalReplayEdits and original_edits_for_ids. In ingest_remote_merge, replay reads the borrowed source at line 651; quarantined conflict inspection uses original_edits_for_ids at 692–693 and degraded inspection at 727–728; batch ownership is moved only at line 752. The original source implementation searches the incoming batch first and then the original ledger and returns Vec<&Edit<M>> in exact requested order, refusing missing identity. Thus these actual conflict hosts do not use copied operation maps. Current declare_edit_ids remains at 788.

## Frozen FEM2D Receivers

Independently recomputed SHA256 for all nine files listed in domain-fem2d-preservation-repeat-695.md; all nine match the final recorded hashes. This binds the inspected files to that 205-range report.

MountedState::close_step is defined in mounted-state-close at line 7 and returns Result<PluginLifecycleStep, PluginLifecycleFailure>. Graph, mesh, PCG construction and PCG closures at lines 37, 49, 105 and 122 all call cx.with_retained_loan(child, |same| ...close_step(same))?; the helper in mounted-state-and-preparation only classifies returned Blocked/Pending/Complete steps. Typed failures already propagate at ? and are not flattened by that helper. MountedState::step uses the actual same-cx retained loan for region mesh retirement at mounted-state-step line 174.

prepare_snapshot_admission is actually defined in job-bridge-and-live-mount at line 153. Its progress and ready bindings occur inside that function, immediately after zero-app guard; authored source lookup updates that progress. They are not hosted merely in the snapshot registry type or an unrelated step function. Parent mounts retain current child ownership; no monolithic parent restoration was needed for these compositions.

## Inherited Shapes

Immutable S directly contains both presence/rejection next_demand definitions, both durable_group_test_outcome signatures (four-argument public at old monolith line 1018 and three-argument helper at 1042), two MutationDiff SpaceHistoryDiff implementations (192 and 231), and both ItemDelta MutationDiff apply definitions inside its unit source. These are authored incoming defects/shapes, not new duplicate definitions introduced by relocation. No source fix is proposed under the explicitly source-only preservation scope. The reported malformed exact-S Pack JSON was not independently parsed in this pass.

## Ledger Scope and Limits

Read the current generated framework relocation ledger rather than accepting the older Markdown opening census (209 unmapped) as final. The ledger is authoritative for final disposition counts; its first current-equivalent-source entry explicitly explains the Hub fixture receiving split. This audit independently traced the high-risk worker, Store and FEM2D definitions above; it does not claim all 1,207 relocation bodies were independently re-proved. Active non-FEM2D domain owners were deliberately excluded.

Current ledger count independently read: 1,207 operations, zero status=unmapped; mapped 946, already-present 40, resolved-root 17, remaining 204 explicitly dispositioned by named/current owner categories (including 9 rejected-cross-owner-alignment). Disposition is not by itself proof of receiver correctness; high-risk receiver traces above provide the independent subset proof.

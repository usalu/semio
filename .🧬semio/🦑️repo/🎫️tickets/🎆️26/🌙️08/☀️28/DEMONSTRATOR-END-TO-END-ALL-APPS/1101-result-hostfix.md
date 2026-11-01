STATUS: RED/BLOCKED, no host test could run. Native test build of the host closure stops at `semio-framework-os-services` (lib, 14 errors) = the unported compute-pool, a flow redesign not a signature swap. Log `C/🗑️generated/1101/hostfix-e.txt` (rc=1, error lines 31285-31452). UNVERIFIED: BackboneDocument.transitions, store-ladder token, trapped-guest/seat/lending/begin_close tests.

## Runs (runner `C/1101-tools/hostfix/t.sh`, root workspace test)
- a: `semio-framework-os-store-paged-one-item` is not a package (real: `-os-store-one-item`, `-one-item-batch`, `-one-item-publication`). b: `semio-framework-actor-retained-turn` is a framework-workspace member (needs its own manifest run).
- c, d: foreign `value-retained-clone` paged-list errors (peer was mid-edit); gone after framework-green.
- e (after framework-green): everything upstream compiled (framework value/retirement etc. through ~31k log lines); first failure is `semio-framework-os-services` lib, file `🛍️products/💻️os/🔨️modules/🛎️services/🧰️compute-pool/🦀️.rs` (untouched since Oct 23, no owner): `WorkerJobSession::owned_admission_demand`, `MountedWorkerJobSession::{try_admit_owned,pump_one}` no longer exist; `acknowledge_checked_out_outcome` now takes `&mut StepContext`; `close_original_retained_step` closures must return `RetainedWorkOutcome`; `return_original_cancel_alias_step` returns `RetainedWorkOutcome` (needs `.result`).

## Why not ported by me
`1030-result-pjob.md` lists compute-pool as an open redesign: the worker model has no pool submit/pump any more (callbacks run only on the original caller via `WorkerJobSessionAdmission::advance(job,params,cx)` then `step_on_original_caller(cx)`). `ComputePool::run_job` has no StepContext, so porting changes its signature, the caller `🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs` (`drive_router_effect_job`) and 3 tests in `🛎️services/🧪️tests/🔬️component-unit/🧰️compute-pool`. Same open redesign class (per pjob, not re-measured): plugin `post-turn-relay` mounted-registry pump, plugin `👷️job/🏛️reserved/⚙️worker`, ui layout-stepping. These will block native plugin-host tests next. Needs a decision/owner (pjob successor); I did not blind-port.

## ArtifactEditor hook review
Already removed (pubtoken): zero hits for `admit_document_store_publication_authority` / `REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY` in `🧰️framework`, `🎓️teaching`. No edit.

## Rerun when unblocked
Root: `t.sh <tag> -p semio-framework-plugin-host-shard-laws -p semio-framework-os -p semio-framework-plugin-original-job-ownership -p semio-framework-plugin-host-shard -p semio-framework-os-store-one-item -p semio-framework-os-store-one-item-batch -p semio-framework-os-store-one-item-publication -p semio-framework-plugin-reactor-turn -- trapped_guest backbone_transitions_seat lending begin_close`; framework workspace: `-p semio-framework-actor-retained-turn`.

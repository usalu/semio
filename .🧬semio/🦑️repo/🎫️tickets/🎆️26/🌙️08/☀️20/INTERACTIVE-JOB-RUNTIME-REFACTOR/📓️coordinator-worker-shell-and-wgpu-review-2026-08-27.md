# Worker Closure And Mounted WGPU Review

## Exact Scheduler Completion

Root read the complete worker-closure receipt proposal and the actual native worker_loop, cooperative pump, WorkerJobSubmission and drive_worker_job_authority. The existing pool consumes Box<FnOnce> under catch_unwind but only exposes pool-wide occupancy; semantic WorkerJob outcome publication precedes final closure/captured Arc retirement. It cannot authorize native aggregate shell release.

The proposed extension stays in the existing pool queues: a preadmitted pool-owned completion slot, exact checked submission ticket, scheduler-only post-invocation receipt, original retained quarantine until that receipt, and separately measured final native shell release. No second scheduler or optional untracked compatibility route is accepted. Production async/job changes remain held until the actual Plugin baseline and scoped native tests are coherent.

Root also identified the existing inner catch in job::drive_worker_job_authority: its Err(_) arm discards the panic payload before the outer pool could retain it. The staged plan must include that same authority's actual panic handoff; fixing only the outer catch cannot prove retained fault ownership. submit_at, callback_at and shutdown ownership are separate entry points, not implicitly covered by try_submit. One real callback clock must include invocation and its tail; moving destruction after measurement or adding an unrelated watchdog is not a completion proof.

All findings are source review, not executed new native tests. The exact proposal and staged native/cooperative test design are in `📓️worker-closure-receipt-proposal-2026-08-27.md`.

## Mounted WGPU

Root read the complete `📓️wgpu-mounted-watchdog-verdict-review-2026-08-27.md`. Four Winit wrappers remain RAII-only. The void metrics helper does not prevent its two downstream enqueues after refusal; redraw performs GPU/window/snapshot/cursor effects before the outer verdict. Existing retained presenter state is the appropriate candidate owner, but snapshot-only gating cannot undo an already executed GPU submit/present.

Required work remains actual preadmitted event/mailbox/snapshot receivers and a measured final platform commit with explicit failure/retained ownership. A late platform-call overrun cannot be reported as zero external effect. No wrapper-only finish-after-publication patch, fake rollback, runtime quota increase, source repin or generated publication is approved. Current WGPU source remains explicitly unreleased.


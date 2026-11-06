# Renderer Worker Close Audit

The renderer drives its mounted worker through pump_one_for_interactive_turn(cx), which calls try_step_on_original_caller(cx); the worker authority close is also passed the caller's cx. This is the same-original-caller path, subject to the binding checking operation, generation, payload authority, clock and deadline.

Two source-level blockers were reported to root:

1. RendererAssetDecodeSession::pump_one attempts a checked-out take_probe, but if PNG is pending it calls session.begin_close anyway. MountedWorkerJobSession::begin_close takes checked_out and returns the worker owner to its underlying closing session. The renderer then lacks checked_out_job access to take the probe once the PNG closes; the close job remains blocked waiting for handoff. Keep custody accessible while draining PNG or provide a real close checkout.
2. RendererAssetDecodeJob::close_step calls the PNG close, which posts actual retained progress, and then returns Blocked. Worker authority receive_original_worker_close_effects subtracts already-posted progress from the reported Blocked progress, which is zero; a productive close produces its invariant refusal. Return the actual progress as Pending on productive close, and use Blocked only for a zero-spend handoff wait.

Native verification was not run.

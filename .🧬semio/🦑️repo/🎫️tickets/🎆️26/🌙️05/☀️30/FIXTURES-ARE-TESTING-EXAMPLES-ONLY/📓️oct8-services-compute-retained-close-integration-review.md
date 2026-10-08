# Services Compute Retained Close Integration

Actual ComputePool routes are in `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs`. Rejected admission cleanup at 718 loops synchronously until empty; session cleanup at 814 uses the old scalar call/unit Complete pattern; failure cleanup at 883 loops retained outcome synchronously then gives the session a fixed turn limit. These cannot be repaired by signature substitution alone.

Canonical Job rejection exposes `next_close_demands(body) -> Result<RetainedCloneGrant,ValueError>` at 2761 and `close_step(fullGrant)` at 2779. Mounted session exposes typed demands at 3251 with exclusive phase admission, restoration of checked-out authority, and distinct Contention/Refused. Its terminal witness at 3272 joins inner authority emptiness and retirement-slot release. Complete progress is not itself a substitute for that witness.

The existing Services scheduling pattern retains the same submitted closure on Contended/Saturated and uses `callback_at(now+1)` for another owned turn. This is the appropriate local model for bounded cleanup: retain rejection/outcome/session/permit across turns, publish exact demands, perform one admitted close, preserve refusal and blocked states, release only after terminal witness. Pool shutdown needs an explicitly retained cleanup owner on an available cleanup executor; merely reporting WorkerLost and dropping state is insufficient. No unbounded grant or synchronous terminal loop is justified.

An additional current ownership edge precedes admission: run_job cancellation at 703–705 returns while the original local `J` is still owned, causing its ordinary drop. That candidate must enter the same retained controlled cleanup path. Tests should exercise rejected admission and early cancellation with undersized release/depth, original owner identity, eventual physical page/slot release and permit return; existing component-unit tests at 255/272/285 cover normal/deadline/cancel routes to extend.

This is current source diagnosis only. No execution, source edit, or runtime positive is claimed.

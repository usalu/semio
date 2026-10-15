# Job Worker Runtime Independent Source Review

Read-only focused review, 2026-10-15. Read latest `job-crate-decomposition-current.md` first. No tests, builds, Originals, lock updates, or production edits. Root reports native successor 28383 terminated compiler RED in 18.6 seconds, before runtime, because a wake-undergrant edit touched two standalone tests without the required session/caller bindings. A subsequent surgical Source correction is **unexecuted**; this review does not imply compile/runtime success.

## Terminal combined frame

`🧰️framework/🔨️modules/🧵️job/♻️session-return/🦀️.rs` 8–19 now selects genuine authority-terminal `SESSION_EMPTY` while a session frame still exists. All other live phases refuse at 9; this is distinct from the stronger fully physically empty `WorkerJobSession::terminal_is_empty` at worker 1050. That distinction makes combined frame retirement reachable without permitting live authority removal.

Registered wake custody remains refused at 10 (wake guard, ordinary waker and retained wake). The quote includes exact frame plus separately admitted retirement-node extent at 11. Return refuses item-zero, release undergrant and depth-zero before consuming the session at 15. `SessionHandle::try_return` still decides actual shared-handle exclusion at 17–19 and restores the exact handle on refusal. No lowered release quote or early node release was found.

Node terminal marking at 18 follows removal from its owned slot and actual frame/node drops, then sets retirement_state=3. Since the successful path consumed the frame and separately removed/dropped the node, this is the rightful completed state; without it `Drop` would see unfinished retirement despite physical release. Mounted wrappers still refuse checked-out originals at 24–28.

## Lower authority and cancellation observations

Worker `👷️worker/🦀️.rs` 794 publishes `SESSION_EMPTY` only after actual authority close returns Complete and the opaque authority independently reports terminal empty. Lower `👷️worker/🎟️authority/🦀️.rs` 483–488 preserves all-axis undergrant refusal, blocks a nonempty ledger, releases ledger before authority backing, and asserts absence of producer, parameters, cancellation retirement, unpublished payload, faults and outcomes before freeing native storage. No terminal-state shortcut bypassing those checks was found.

Cancellation witness/return queries at worker 1012/1015 now return false/None once authority is already `SESSION_EMPTY`, or once the frame is gone. Remaining nonclosing phases still return contention; only SESSION_CLOSE wins the transition CAS and takes the authority. Lower authority 254–255 still limits alias return to close stage 3 with parameters gone and the actual matching original alias witness. This avoids trying to take an absent terminal authority while preserving live-phase refusal. Returning false/None describes the absence of alias custody; it is not a new grant or a claim of total physical emptiness.

Independent incremental frame close still checks complete demands at worker 1041–1047, actual uniqueness/registered waker, retained wake close, and handle try_return. Retirement-node release remains separately paid at 1087–1100. Combined retirement has no weakened undergrant allowance compared with those actual extents.

## Original caller and receipt correction

The test admission macro at worker 10–14 borrows the supplied existing caller, uses its remaining grant, executes its turn and checks `before.checked_add(actual_progress) == cx.retained_progress()`. It does not create a new WorkerJobAdmissionContext or fresh root. This is the correct cumulative-versus-local receipt distinction and preserves ownership transfer assertions for both original job and original parameters.

Public preparation law `👷️worker/🚪️admission/🏗️preparation/🧪️tests/🌍️public/🦀️.rs` 40–48 carries one once-admitted finite caller through alias admission, preparation, handoff, callback, repeated denial/close, alias return and final root retirement, comparing actual heap birth/release and final zero-effect Drop. No replacement caller is minted inside those turns. Stale unmounted predecessor test source still exists nearby; its existence does not establish execution or weaken the reviewed active macro.

## Finding and limitation

No concrete new production defect or weakened authority/wake/undergrant semantics was found in these focused changes. This is Source assessment only. The current successor still needs compilation and the original native runtime laws; compiler RED 28383 supplied no runtime evidence. In particular actual retained-wake undergrant, shared-handle refusal, paid combined release, live-phase refusal and same-caller conservation must not be declared passing from this review.

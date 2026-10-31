# Genuine Worker Session Birth and Output Custody

The current admit_ready consumes authority, then performs Arc::new and Box::new before a receipt callback; validation or receipt failure can lose the only owned session. The replacement is retained WorkerJobSessionAdmission state, not a Result wrapper. Its advance accepts caller-owned output Option and returns only Copy status.

Phases: real authority initialization; retained ready authority; slot scan (one actual compare_exchange attempt per paid turn, retained next index and exact reserved slot); fallible empty SessionHandle Arc birth using the canonical allocation-return backend; fallible fixed retirement-node Box birth; allocation-free authority/frame/node ownership handoff into the exact chosen Session/Mounted/Batch header; paid close of the original authority admission; then distinct funded output-slot move. Each partial allocation/slot/authority lives in this genuine owner before physical/work posting or qualification. Null work1 retains input/other partials; zero-item/capacity/depth denial0; no per-turn Source renewal.

A private sealed first-party from_session construction port replaces the consuming admit_ready family; it cannot run arbitrary external constructors. The old infallible SessionHandle::new and synchronous ready birth are removed. Slot scanning is resumable so quantum1 is not an opaque scan transaction or a hidden many-unit overrun. No bytes are copied for uniquely owned metadata moves.

Cancellation closes retained born session, or partial authority/frame/node and reserved slot under the SAME original Source, then publishes terminal header emptiness. No implicit Box/Arc drop owns live authority. Public output remains in its real caller slot on post-fold refusal; denied output leaves the born owner unchanged.

Owning laws must pin every null frontier, slot exhaustion and exact same slot on retry, real q1 phase progress, denied output, actual post-fold refusal custody, partial paid close and cached terminal0. Existing Worker/Batch cases and NativeIO original eleven remain mandatory; source-only schema/laws are not acceptance.

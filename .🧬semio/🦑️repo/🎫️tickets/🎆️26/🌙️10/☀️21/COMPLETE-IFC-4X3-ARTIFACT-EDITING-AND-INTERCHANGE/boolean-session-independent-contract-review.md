# Shared Boolean Session Independent Contract Review

Read-only, source-free review of boolean-session-contract.md during the active core rewrite. No Boolean source was reread/imported or modified. The proposed one private Control/session is the correct narrow reuse seam; no second evaluator, caller cost/refund or per-node fresh allowance is needed. Workspace remains a distinct unfinished stage.

## Concrete State and Authority Obligations

Use independent state dimensions for busy, entered, observer-active, failed-with-presence-bit, and closed. A thrown undefined is a latched failure, so failureValue!==undefined cannot be its guard. Session failure must be checked before every operation/next/checkpoint and again after observer return, before result publication. If an observer catches a reentrancy refusal, that caught refusal still poisons the original operation. If it subsequently throws another value, preserve whichever failure the session actually observed first. Do not replace existing exact external caller precedence accidentally while generalizing the latch.

Choose and test when generator ownership is acquired. Native JS generator bodies are lazy: creating two generator objects before either first next can bypass a busy check placed only inside a factory's entered body. If factories acquire busy immediately, return() before first next never enters a generator finally, so an outer explicit owner wrapper must release its reservation. Either strategy is coherent with a documented created/entered/closed state, but no leaked busy bit or two simultaneously resumable child cursors may result. Busy clearance must be tied to the exact cursor token; a stale return/next cannot release a newer cursor's ownership.

An observer can close over the actual session and current generator even though its this receiver is undefined. Guard checkpoint, child creation, child.next/return and close during observer execution. Calling a generator.next recursively normally raises an engine TypeError before the body executes; do not rely on that to set the session failure latch. A protected wrapper around public iterator methods is needed if reentrant next/return is in the declared API threat model. If only entry/checkpoint are guarded, explicitly preserve that narrower boundary rather than claim all public iterator reentrancy denied.

checkpoint() should always have exactly one internally fixed debit and no cost argument. Its initial/completion observer gates, if exposed, must be private named transitions; a public zero-cost observation route must not let actual source/leaf work escape the grant. Capture all policy/observer handles once, default only undefined, validate before birth/work and invoke callbacks unbound. Public work observation may be a readonly getter; no writable alias of original grants/control should escape.

## Sequential and Failure Lifetime

Successful first operation releases busy but retains monotonic spent work for the next operation. A failed or cancelled operation must not clear failure just because its generator closes; only bounded cleanup remains. close must be idempotent and terminal, never a reset. Work denial, original callback Error/undefined, callbackfalse cancellation and explicit reentrancy refusal need distinct expected first-failure tests. Repeated next after done may return done but must not mint another terminal result or repeat publication callbacks. A returned mesh is owned by the caller; the session work grant does not prove its stable future contents or IFC source authority.

Generator return/cleanup cannot overwrite an earlier primary failure. Since the private kernel has only known finite finally behavior, avoid inventing a general infinite-generator promise. If async host cooperation itself throws/rejects after a child turn, capture that actual operation failure, close the entered child and refuse session reuse. Preserve original standalone sync/steps/async and halfspace outcomes rather than silently changing all error precedence under a generic session exception wrapper.

## Minimum Discriminating Neutral Programs

1. Two sequential genuine Boolean operations share measured cumulative W; W-1 refuses second completion, W succeeds, and no caller reset is possible.
2. Boolean then halfspace and halfspace then Boolean preserve one counter; each standalone path still owns exactly one session.
3. Create two cursors before next; enter/close them in both orders; return an unentered cursor; stale cursor return after next operation. All preserve single-owner state and original spend.
4. Observer attempts session child creation, checkpoint, current next, current return and close; catches the refusal; original outer publication must still refuse wherever that path is promised guarded.
5. First failure values undefined/object/Error, then second different failure and later child creation/checkpoint/close: primary presence/identity remains exact.
6. Early return after entered work leaves cumulative spend unchanged and releases only that child's busy state; policy defines whether ordinary caller-abandonment makes the session reusable or terminal.
7. Host continuation rejection and finite cleanup failure cannot yield a successful session or reset grants. Completion observer failure prevents successful result publication.
8. Public callbacks receive no mutable session; null/noncallable/single-getter policy regression laws remain unchanged.

Use exact integer arithmetic/SQLite for the monotonic test program; actual kernel traversal must supply genuine observed W. These programs do not prove workspace bounds. The forthcoming maxIntermediateValues ledger still needs reserve-before-growth/retained siblings/dynamic kernel overlap, measured peak-1/peak, original handle provenance and cleanup tests before IFC nested-expression admission is complete.

# Guest Port Loan Current

Read-only 2026-10-09. Proposed split-field macro reviewed against actual control APIs; replacement macro was not yet mounted when read. Existing receiving module still used whole-owner RefCell. No compilation or execution.

The proposed split is appropriate: native allocation ports explicitly require Send (`🧰️framework/🔨️modules/🌱️value/🛫️encode/🛂️allocation/🦀️.rs:10–16`, decode sibling 8–13); ordinary callbacks do not. A closure borrowing only Send allocation Binding, original owned counter and fixed maximum can satisfy Send while observer separately borrows callback/work fields. AtomicUsize safely shares the admitted total without sharing the non-Send callback. Keep retirement recipient outside both loans, installed on the original owner.

Required event ordering:

1. Checkpoint actual original owner before deriving remaining ceiling or executing Guest work.
2. Validate child relative owned/next/maximum against remaining and original absolute `start + child_owned`; checked arithmetic before mutation.
3. Facade charge observes original cancellation before first/large allocation. At observation original absolute owned still excludes this prospective request.
4. Allocation closure sends actual original absolute request to installed original port before changing original owned or shared atomic. On refusal neither ledger advances.
5. After accepted port request, update original owned and shared observed owned, then return; facade updates its own relative ledger afterward. Observer must compare relative progress against atomic minus original start, while publishing absolute atomic owned to original callback.

Pitfalls to guard:

- Do not capture retirement recipient or callback transitively in allocator. A helper struct containing the whole owner recreates the Send failure even if closure only calls one method.
- Atomic sharing supports safe field separation, not concurrent Guest execution. Observer plus allocator must follow the same synchronous protocol; a concurrently moved allocator could interleave original port requests with observer receipts. Keep loan operation bounded and borrowed.
- Never call facade observer while holding allocator's mutable original fields, or invoke allocation through original full-control `charge` inside allocator. The split intentionally performs preflight callback in façade and actual port debit in allocator once.
- Original forwarded port receives original maximum and absolute prior owned, not child remaining/zero-origin values. Child request can have a restricted ceiling, but cannot exceed original remaining. `scope_maximum` stays intact; bridge must not call admit_turn_capacity or widen original maximum.
- Native façade charge checkpoints only first or >65536-byte allocation. Preserve this existing cadence; for small repeated reservations rely on original reported work/checkpoints rather than promising per-allocation cancellation callbacks.
- Manually borrowed work fields must increment/change original stage when starting child work, otherwise StageScope's conditional restoration may not restore changed fields. Prefer unconditional restoration of captured parent work fields on Drop. It must preserve owned counter, allocation port, recipient and maximum. Current encode StageScope is now RAII at encode root 95; confirm decode and manual macro use the same guarantee.
- Reject invalid finite completed/total before mutating original work; repeated monotonic same-stage reports advance delta, actual restart uses a new stage. Preserve first original observer error rather than generic canceled. At success require façade relative owned equals shared atomic minus original start.
- Failure after accepted allocation must retain admitted original ownership. Macro unwinding may restore work only, never owned bytes. Guest VM custody and native recipient custody remain separately attributable; no implicit drop-based qualification.

Meaningful verification: prefund original ledger; assert original port sees absolute start offsets; refuse a request and show both relative/absolute ledgers unchanged for that request; cancel before first allocation and after admitted interior work; unwind with parent work restored and cumulative ownership retained; use a real non-Send original observer to prove native Send port separation without unsafe Send assertions.

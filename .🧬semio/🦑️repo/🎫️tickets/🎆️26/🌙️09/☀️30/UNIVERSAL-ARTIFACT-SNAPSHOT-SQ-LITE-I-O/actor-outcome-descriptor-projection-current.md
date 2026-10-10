# Actor Outcome Descriptor Projection Audit

Read-only source audit; no builds or runtime qualification. Current concurrent Actor bridge has already replaced the previous owned producer payload projection with `JobOutcomeDescriptor`, borrowed `JobOutcomeView`, and cumulative original grant/progress.

## Original paid sequence

Create a named StepContext against the same external recipient, invoke actual `drive_step(job, context, site, stage, verdict)`, transfer its paid admission into the descriptor, and borrow exact producer payloads through `borrow_outcome`. Keep the descriptor pending while projecting; never close, move, or drop producer payloads. Consume every projection birth/copy through that same recipient. End all payload views before acknowledging the descriptor. A direct descriptor acknowledgement requires one item, one bool copy byte, and depth one; the worker descriptor slot separately requires acknowledgement and slot removal. Zero remaining authority preserves pending descriptor unchanged. Resume the producer only after acknowledgement completes.

## Current concrete concerns

| Location | Concern |
| --- | --- |
| Actor main Rust JobOutcomeProjection::step, around 1750–1774 | `try_reserve_exact` occurs before actual capacity birth receipt. Required capacity is preflighted, but actual allocator capacity may exceed it. `consume_retained` records actual excess and returns error after destination mutation. Custody is retained, but physical authority can already be exceeded. Use an actual capacity-limited original allocation authority or a genuinely pre-admitted destination whose existing capacity suffices. |
| Same pending projection branch | Borrow/copy/allocation/ack proceeds without checking StepContext cancellation, deadline, or fuel. The fresh producer path checks these through drive_step, but the pending projection path bypasses that gate. Check original temporal authority before any projection mutation, preserving pending descriptor on refusal. |
| JobTurnBridge::step final sequence update | `next_step_sequence += 1` remains unchecked and can overflow; checked admission should precede any payload/descriptor mutation where sequence publication is required. |

The current equality check `budget.retained == self.original` and StepContext borrowing `self.progress` conserve the same cumulative wallet across turns. Descriptive retirement demand is no longer minted into a fresh close grant. These are source observations, not evidence that the Actor libtest frontier compiles or executes.

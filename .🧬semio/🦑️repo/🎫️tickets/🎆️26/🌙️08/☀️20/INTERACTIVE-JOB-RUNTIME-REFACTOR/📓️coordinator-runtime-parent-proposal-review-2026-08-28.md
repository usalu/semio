# Runtime Parent Proposal Review

## Accepted Direction And Next Boundary

The coordinator read Dag's complete171-line proposal, both ticket declarations, Retained's complete98-line caller census, and the actual runtime registry/actor-authority and neutral consumer/record preparation code. One original composition root, charged original runtime/cell, source-owned construction, same Store FIFO and one inline Release remain the selected direction. No production implementation or native lease follows from this architecture review.

The next bounded packet is cut1: actual runtime constructor/layout and original-parent recovery schema/tests. Cuts2–3, bound Store release and Sync mailbox/channel retirement remain dependent work. No RuntimeAppCell identity may be assigned to the separately constructed SyncSession merely because its types match.

## Two Concrete Prerequisites

1. Current RuntimeActorAuthority contains a4096-byte array plus u16 length. Its source-level minimum already exceeds4096 before tuple/node/Option metadata. Partitioning only the1024-entry registry cannot make even one such unchanged element fit. Preserve the4096 semantic actor limit; declare bounded byte-storage/header representation inside the same registry and measure actual containing allocation/write/free Layouts before claiming a native phase fits. This is a source lower bound, not an executed layout measurement.
2. Neutral prepared_consumer is one latest pointer, cleared/replaced by later preparation. Recovering by type/latest pointer after losing the original facade is not original-parent recovery. The first-cut packet must specify a root-retained exact recovery key/slot and live check, with all metadata accounted. No unpriced external table, movable facade backlink, identity-as-liveness, or public numeric key can fill that gap. Desired tests must cover unrelated and same-type later registrations, cancellation, lost facade, and a foreign equal-capacity root.

The existing R11 Destroy/Free/Refund/Clear remains proven only for its25 standalone tests. Planned targeted live-child release adds private binding/unlink/residue obligations; it must reuse the same inline slot and cannot refund a record while the original binding still reads it. That dependent implementation is not yet authorized.

## Runtime Detach Is Still Separate

The caller census identifies no original RuntimeAppCell-owned SyncSession. Current mailbox publication can precede a wake-callback panic, and broadcast Receiver drop can retire unread event payloads. These source findings require original request/channel ownership, not a bare await removal or success flag. OS compile remains deferred while that known production join is unfinished.

[Dag proposal](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️runtime-opening-original-parent-funding-proposal-r1-2026-08-28.md>) · [Retained caller census](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️sync-detach-original-owner-caller-census-2026-08-28.md>).

This review changed reports only. No native test, layout result, timing acceptance, source hold, quota increase or cleanup occurred.


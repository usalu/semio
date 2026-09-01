# Mixed Worker Inbox Ownership Review

## Subsequent Independent Replay

Root independently executed the registered selector:2PASS/178skip180,1.24s,start03:55:02,Nx0; all8 selected pre/post hashes match. See [R1 Record](</Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️coordinator-inbox-inventory-r1-2026-08-28.md>). The two defective producer traces are deliberately characterized, not approved. The short inventory hold is released. The prior source-review/delegated boundaries below remain historical evidence.

## Evidence Read

Root read the complete205-line [peer report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📓️shared-worker-inbox-ownership-2026-08-28.md), both current ActorWorkerInboxInventory tests, the actual generated worker dispatcher/reply/replyError and host-shim producers, and Shard spawn/handleMessage/host-effect settlement. This is root source review, not an independent actor/NodeVM replay.

The peer's final executed boundary is178PASS/2FAIL180,3.00s,start03:44:02,Nx1,28 selected hashes identical. Its focused inventory is2PASS/178skip180,1.05s03:43:25,Nx0. Source/test reads corroborate what those tests attempt; counts remain delegated evidence. The two old copied payload/page failures remain unresolved.

## Findings And Decision

The census correctly separates result success/fault, heartbeat, trap, effect-request, ignored effect-emit/ui-patch-emit, and platform error/messageerror. Per-worker one-return serialization would not identify the next callback and would obstruct an awaited shim effect or another actor's accepted request. Existing per-actor concurrency must remain.

Current spawn reads event.data before any retained event owner; handleMessage discriminates and looks up a pending request before its optional output capture. The host-effect entry holds callback/controller identity but not the complete original frame/params root. replyError touches payload/formatting before a retained error owner; the outer worker catch can send a second fault after a successful post callback throws. The characterization tests explicitly expect those current defective traces with semanticallyAccepted=false. Their passing assertions are not one-response conformance.

No canonical binary receiver, new worker channel, object/binary compatibility branch, whole-result fallback, or worker-wide mutex is approved. Same numeric correlation and shape checks are not private producer/slot or whole-root retirement evidence. The raw4161 allowance cannot pay arbitrary ordinary results, host-effect graphs, platform events, Promise/reaction/closure graphs or bootstrap metadata.

## next Bounded Packet

Requested the peer's schema-first actual original ShardClient→ShardSlot/worker bootstrap participant: same explicit shared ledger, original pre-callback ingress and first-violation roots, actual handler/closure/registration inventory, and exact private transfer/alias observation before reuse. Price newly introduced fields and existing graph overlap without double-counting controller208 or PendingEntry160. Cover createWorker, handler installation and attach-SAB before/after faults; onmessage must retain the original event before extracting data, and onerror/messageerror retain original worker identity.

This is declaration/API/transfer design only until reviewed. It cannot authorize canonical dispatch by itself. Worker-side original guest-result/first-fault retention and one-response producer credit need a separately coupled contract, not a retry boolean. Existing accepted ordinary/retirement/effect traffic must remain progress-capable; finite first-violation containment is not a rogue-flood or platform-queue memory bound.

## Preservation

Imported source hold for independent page9 was released at its terminal. Root changed no actor/materializer/runtime source, compiled no native target and published no generated output. The peer says inbox400.995 is now in both launch files through taxonomy's distinct five-row receipt; that supersedes the earlier seed-only observation but is not a root launch revalidation here.

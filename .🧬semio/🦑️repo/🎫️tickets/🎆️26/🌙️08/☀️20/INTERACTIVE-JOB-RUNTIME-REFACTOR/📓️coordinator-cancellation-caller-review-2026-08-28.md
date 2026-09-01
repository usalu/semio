# Payload-Driven Actor Cancellation Review

Root read the complete [peer report](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📓️payload-driven-cancellation-caller-2026-08-28.md) and actual `OwnedKernelReturnInput advances no framing on unread or genuinely cancelled fragments` caller in [ShardClient](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts:3000).

The changed caller uses the original payload to admit genuine cancellation evidence and drive its source/registration handshake, then independently drives builder-binding and payload retirement. Its expectations explicitly retain the source page, original response envelope, content and field; the retired fragment cannot be read, the detached builder cannot resume source consumption, and no page-input ACK exists. Grants derive from released canonical fixtures rather than an increased arbitrary loop bound. The independent Immer accounting remains in the test.

The peer records actual focused RED then GREEN1, and a subsequent full actor176PASS/2FAIL178 with22 selected broad-run endpoints stable. Its focused GREEN pre-hash output was truncated, so that focused run is not a complete source capture. Root has reviewed these boundaries but has not rerun this new actor caller. Root's earlier independent combined UI20 is separate evidence for its underlying cancellation/binding components, not this new integration caller.

The remaining two actor failures are copied-content/page-boundary work. Current strict46 includes active UI page declarations and older UI/tutorial joins; this is not a strict pass. UI-owned page/reader admission, raw receiver custody, semantic streaming, InputAck and final composition retirement remain unfinished. No production source changed in this review.


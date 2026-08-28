# Coordinator Captured Disposal and Resident Roster Review — 2026-08-27

## Captured Transport Teardown

Earlier source inspection found that dispose(actorId) selected the current routing map, while lifecycle ownership could remain on the original slot after route reassignment/release. The Demonstrator owner reproduced this with actual RED tests and repaired the private disposal target to the captured activation/slot/generation. The coordinator subsequently read the implementation, its activation-already-owned guard, captured lifecycle dispose, and the owner's report. The peer executed actor108/108; the coordinator's separate full React R23 executed640/640 with the same Shard hash e0a3ef816cbebebe8a76f750c5dd8f4aec5763e7f558c5490e7f97dadec287ea unchanged. The React run does not re-execute all actor108 cases.

The captured path refuses worker loss and live instance/close owners. A failed post leaves the activation retained, and a successful-post marker makes a repeated old dispose inert before a same-name replacement can be touched. The activate guard prevents a new activation while the old available owner remains. Current non-instance effect cancellation and pending rejection still perform synchronous loops; their boundedness is not established here.

Successful posting is not a worker disposal receipt or a native/Wasm memory-release witness. The existing generated worker merely deletes actor/in-flight/budget map entries. The coordinator inspected the materializer's activation-keyed component imports and disposal branch; the peer independently inspected the actual GIS output and found module-scope memory/export roots. Those roots are not shown to disappear when a Map entry disappears. This is source evidence, not a measured leak. The peer owns an explicit-instantiation factory investigation with per-activation imports, preserving one WIT ABI; no production output or catalog pin was changed.

## Shared Resident Roster

The coordinator read the complete released resident implementation and canonical contract. Its pool strongly links lifetime scopes; each lifetime links payload scopes; each payload links page roots and primary writers; pages link captured readers. Capacity is explicit, shared and logically charged at 256 bytes per page, with independent handle accounting. Actual private instance/activation/lifetime matches and final instance retirement are joined. Parent close can recover an abandoned writer, and already-issued read aliases remain readable until explicitly retired. These laws are included in full React R23, not merely schema assertions.

The primitive still exposes bare page.capture without an exact privately branded consumer. Producer retention alone cannot recover an escaped reader whose caller throws before adoption; a strong producer reader list prevents garbage-collection loss but does not by itself assign a serviceable consumer. The live paged cutover must remove or narrow that bypass and atomically register the exact retained consumer before exposing a reader. The UI owner explicitly accepted that API-level requirement.

## Paged Adoption Boundary

The new builder plan correctly requires both private Field.matchesOwner and ResidentPayload.matchesOwner before binding, registers the concrete builder in the original instance before exposure, and retains a failed admitted builder for close. The peer's exact captured return-page parent and Field/Fragment/Release mint are still being implemented. No structural stand-in or format-valid receipt may substitute for them.

A copied-fragment witness requires all fragment bytes independently written and its source reader detached. It does not require sealing a final destination page: a partial page may span raw fragments. Exact input release, UI publication ACK and final lifetime retirement remain separate witnesses. Bare capture must not survive as a live bypass after the concrete consumer is mounted.

Root performed source/report review and independent React verification only. No generated publication, cleanup, guest memory measurement, all-app runtime or strict callback timing certificate is claimed.

# Retained Source Identity Registry Review

Read-only review of held code; no compiler, runtime, production join, or global activation credit.

Authority: [held provider](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/child-paged-complete-family/retained-source-identity-registry-provider-held.rs:21), [held owning law](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/child-paged-complete-family/retained-source-identity-registry-law-held.rs:21).

## Concrete Reuse Blocker

Provider lines21/25 address a source as identity minus one. Line32 pops tombstones whenever owners becomes zero, even when closing is false. The monotone next identity is unchanged. After an open abort and complete retired-owner drain, a fresh source appends at index zero with identity three (or later); borrow/take instead address index two, so the new accepted source cannot be published. This is a source-addressability defect, independent of byte disposal grants.

The authored law drains the first displaced owner while the second remains accepted, then calls begin_close after either abort or commit (lines36/42). It never inserts into an open registry after all owners/tombstones were drained. Add an explicit abort → open retirement drain → new Tick → Commit case pinning the original new-source pointer and all8194 bytes. Also cover commit → transferred publication → open metadata retirement → new Tick. Keep the original1-item/4096-byte physical bound and existing allocation control. Preserve monotone identities and genuine unique owners; retaining stable tombstones until explicit close or an explicit authored address base can fix indexing without source cloning.

## Confirmed Held Structure

Insertion admits exact page layout before taking the incoming owner. Allocation/checkpoint refusal retains incoming; a reserved-slot failure restores its owner. Source identity Clone copies only u64. Reconciliation consults fresh accepted provisional identities and permanently marks absent owners retired. Take transfers one original owner and leaves a tombstone. The law asserts genuine8194 source bytes, pointer preservation, refused insertion/reconciliation, accepted Tick replacement, displaced physical disposal, Commit extraction before empty-ledger reconciliation, duplicate take refusal, late Commit Idle, and host Abort.

The callback may refuse partway through reconciliation after earlier absent identities were marked retired; this is safe only when the callback reads the same immutable accepted ledger for the entire pass. The actual settle_press join must preserve that accepted snapshot and extract every committed source before reconciliation. The helper/law does not establish production caller timing.

Retirement delegates at most one item and caller bytes to each original owner; metadata page release reports actual released allocation bytes. The current law measures disposal through drain_retained, rather than proving a real parent handback at the eventual production exit. Keep parent-return qualification separate and require the actual retained publication carrier to implement that edge; no automatic drop/refund exemption follows from this registry.

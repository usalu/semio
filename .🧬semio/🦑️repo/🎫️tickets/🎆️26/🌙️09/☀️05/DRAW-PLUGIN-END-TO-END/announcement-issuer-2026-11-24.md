# Announcement issuer refusal on rectangle commit (r22, de)

## Root cause
`StoreReplay::flush_published_apply_batch` (store engine, `replay-invalidation-announcements-and-transactions`) checked
`publication.original_registry.ptr_eq(store.snapshot_read_leases)` BEFORE its phase/outbound gate. The typed-operation
pump (`mounted-run-and-pump`) calls the flush after every publication unit while `pending_artifact_publication` is still
held. After the ACK, the publication closes and moves `original_registry` into `registry_retirement` (it becomes `None`),
so any later flush on the closing/closed publication returned `VcsError::InverseRefused("batched announcement requires its
original Store issuer")` instead of the documented `Ok(false)` ("nothing left to announce"). The issuer was not replaced:
it was legitimately released by close.

## Fix
Issuer identity is still verified while the registry is held (foreign store -> Err, unchanged). Then the phase/outbound
gate runs (`Ok(false)` when not awaiting an announcement). A publication that is awaiting an announcement yet holds no
issuer is still refused.

## Files
- store engine `...store/⚙️engine/🎞️live/📥️lifted/🏪️store/📣️replay-invalidation-announcements-and-transactions/🦀️.rs`
- test `...store/🧪️tests/📤️outbound-announcement/🦀️.rs` (flush after close must be `Ok(false)`; reproduces through the real
  outbound batch publication, announce, ack, close path)

## Tests
See reply. Draw component rebuild needed for the guest to pick this up.

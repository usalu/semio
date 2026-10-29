# Checkpoint Identity Progress

The selected Store law ran one test and failed at its first actual SetN(6) event publication, before the added checkpoint section. The cursor returned zero progress because both OperationIdentityCursor loops granted only the 64-byte output span. ArtifactPreparedOperationCursor first admits an inline codec frontier and publishes its exact copy demand through next_minimum_copy_bytes().

Both encode_into and commit now grant max(next_minimum_copy_bytes(), output.len()). NativeEncodeControl checkpoint, actual capacity charge, actual processed/copy work charge, identity writes and output append remain unchanged. No fixed budget or quota increased. The original operation-wire portable corpus now exercises the public advance entry for codec birth, verifies a demand-minus-one denial without progress, then exact admission with zero payload bytes and no heap allocation. Existing serde output comparisons remain unchanged. The actual failing Store law already exercises the complete identity producer and compares derived snapshots against the neutral JSON and Serde.

Native rerun is pending the single Rust-owned lane; no compiler was launched by this packet.

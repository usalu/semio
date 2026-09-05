# Verified SPR Writer Resume

## Purpose and Boundary

`ArtifactWal::open` currently deletes and recreates its active segment before rewriting and fsyncing recovered records. A crash during that process can destroy an earlier durable receipt. This protocol slice supplies the missing append-resume primitive; it does not yet change WAL recovery, storage durability, or Map group publication.

The existing retained SPR scanner already validates every frame CRC, canonical framing, commit sequence and predecessor offset, counts, covered lengths, and full hash chain. Its `VerifiedSprSpan` has a private constructor and owns the committed end, sequence, predecessor offset and chain tip.

`SprWriter::resume_verified` now consumes that span, checks the sink ends exactly at its committed boundary, checked-increments the sequence, restores the prior commit offset (or none for the header-only case), and seeds an empty pending accumulator with the verified tip. It writes no header and no committed bytes. The caller must retain the same verified prefix and exclusive storage authority: the span is protocol metadata, not a storage authorization capability.

## Tests

Extended the existing neutral retained-verification JSON Schema/fixture with six resume cuts: header only, a torn first commit, one clean commit, an uncommitted second record, a torn second commit, and two clean commits. Each resumed record adds exactly 89 bytes. Twelve mismatched sink lengths must be rejected. A further uint64 sequence-exhaustion case requires rejection before a commit frame is written; its source guard was observed RED before replacing the old unchecked increment with a pre-write checked successor.

TDD source check first failed with `protocol-owned verified writer resume is missing`. After adding the method, the registered `@semio-tech/framework-replication-rs:retained-verification-check -- --oracle-only` passed. Its independent AJV/CRC/LEB128/BLAKE3 oracle confirmed all six continued commit chains, in addition to 224 existing prefix cases, 26 hostile cases, and 10 compressed-frame cases.

The exact native target selects three laws, including `retained_spr_resume_preserves_exact_prefix_and_commit_chain`. The first native receipt `exact-cargo-laws-GsLkDT/00` built successfully and ran all three: the two existing verification laws passed, and the new resume law reached its exhaustion case and exposed the old unchecked increment as a panic. That executable was compiled before the checked-increment fix.

The cached corrected-source run `exact-cargo-laws-VL6qrv/00` is GREEN: all three exact native laws passed once. Native runtime output confirms six byte-exact prefixes, continued hash/sequence/offset, twelve wrong-length denials, and rejection of exhausted sequence without writing. Source-oracle rerun and scoped staged/unstaged whitespace checks also passed.

## Required Next WAL Work

- Verify and copy from the same retained segment bytes. Keep WAL-specific document/index/record-kind and cross-segment predecessor validation; generic SPR framing validation is not enough.
- Restore a buffered writer with `flushed_len` equal to the preserved committed prefix and zero pending records, so future flushes append only new bytes.
- Truncate only an uncommitted tail and require the backend's fsync to durably cover the new length.
- Handle empty and partial genesis, and a crash after sealing the highest segment but before creating its successor, without deleting previously committed bytes.
- Extend lifecycle fault injection beyond append/fsync to recovery boundaries, then prove previously durable receipts survive each restart point.

The full three-member Map journal still additionally requires a canonical prepared-outcome recovery codec, one durable anchor decision, and complete store roots behind one shared visibility decision. The read-only implementation map is in `📓️terra-map-durable-group-atomic-publication-replay-p0.md`.

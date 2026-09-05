# WAL Readable Capacity Fence

## Current Evidence

The new registered `wal-capacity-check` is source GREEN6 after an observed RED on the missing preflight. A JSON Schema/neutral fixture plus independent LEB128 frame sizing confirms Fsync/grouped transaction rotation, exact maximum and one-byte-over rejection arithmetic.

Exact native receipt `wal-capacity-exact/exact-cargo-laws-4iryqT/00` is GREEN. It built in 58.54s and ran `wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries` exactly once: real Memory and filesystem Fsync/grouped crossover, one-over no-effects rejection, preserved sealed bytes, close/reopen, and exact-maximum reopen all passed. Runtime output confirms those paths. Executable SHA-256: `a3f2bc3d24dc99467d9f1cfbcd4ec9f2289fddd563a2dacefd54b196fafaccdb`.

## Change

Storage now owns one public `DB_IO_MAX_READ_BYTES = 496 * 1024` ceiling. Core storage, PostgreSQL and Neo4j use it; duplicate backend constants are removed. WAL's default rotation limit and retained verifier use the same 507904-byte ceiling.

Before modifying transaction ids or the writer, `submit` calculates the exact SPR frame bytes for begin, every payload and logical commit, reserving a further physical commit. If the current segment cannot fit that reservation, it first confirms the transaction fits a fresh chained segment, then rotates before writing. An oversized transaction is rejected before any write/flush/seal/create. Grouped pending records always retain their eventual physical-commit reservation.

The fixture freezes three 250000-byte command submissions: segment indices [0,0,1]; Fsync segment lengths [500389,250291]; grouped lengths [500314,250291]. A 507645-byte command exactly fills document `d`'s genesis segment to 507904; 507646 is one byte too large and must leave transaction ids, buffered pending records and storage unchanged. Native tests exercise actual Memory and filesystem backends, close/reopen and preserved old bytes. Filesystem test artifacts are under the registered ticket-generated output root.

Related audit: `📓️terra-wal-tail-open-current-audit.md`. Failed/short append poisoning is being implemented separately by the execution agent; retained open-cursor and transaction-atomic replay work remain open.

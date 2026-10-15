# Namespace Baseline Implementation Independent Review

Read-only Source review, 2026-10-15. No gates, Originals, physical mutation or production edits. Storage supplied initial prototype70669/strict88172 and later constructor-owned image runtime82761; latest idempotent cleanup, directory accounting, receipt/fsync, cap verification and funding changes remain unqualified successors. Canonical enrollment/routing remains unwired.

Source owner: `🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️namespace/📒️baseline/🟦️.ts`; line numbers reflect reviewed Source.

## P1: Namespace aliases bypass authoritative baseline lookup

`namespace()` at 25 rejects traversal segments but accepts repeated/trailing separators such as `claims//nested` and `claims/nested/`. SQL classify at 39 uses the unnormalized string, while actual path join at 47 normalizes it. An actual captured namespace `claims/nested` therefore classifies an alias as `new` rather than baseline/unresolved, even though the eventual physical namespace is the same. That is unsafe as input to future new-only slot placement. Require one canonical portable relative namespace spelling: empty root or nonempty segments without repeated/trailing separators, drive-relative syntax or other platform aliases. Keep rejection rather than silently normalizing caller identity. Sent this exact finding to Storage; no production edit made here.

## Implemented baseline protections

- Constructor 57 owns the actual issued image around the entire build/token lifetime, instead of borrowing an image that could close while detached baseline operations survive. Image/control identity is checked at 60; real image/path observations precede metadata operations at 62.
- Actual queue-catalog EX is acquired at 63. Recursively sealed data/claim creation boundaries at 71–72 precede census. Census records all files/directories, not only readable SQLite records. Every hash-named SQLite creates a binding; zero, oversized, foreign and unreadable states remain unresolved (50–52,68). Queue-only directories reserve the matching namespace/hash at 69. A target row replaces the provisional queue-only row via explicit conflict handling, preserving actual target inode facts.
- Classification 39 requires full exact logical key equality for initialized rows; mismatches remain unresolved. Absent old hashes can be `new` only within new UTF-8 key limit. No arbitrary supplied physical path is exposed. Baseline acquisition 47 recomputes its old canonical hash path, verifies captured dev/inode before and after actual zero-timeout lease acquisition, and releases in finally. Old preopened/reopening participants therefore share the same original SQLite inode for known baseline bindings.
- Empty pending targets remain permanent unresolved hash reservations, avoiding false inference of a full logical name from a basename. This is honest refusal, not transparent completion of their prior initialization. The recorded tests explicitly cover old pending and queue-only classification plus actual old/new inode contention; no new reusable-slot integration is implemented here.
- Policy distinguishes old 4096 UTF-16-code-unit validation plus 12,288 UTF-8 bytes (24) from 4096-byte new keys (39). This conservatively preserves valid round-trippable old Unicode keys; lone-surrogate/invalid roundtrip strings refuse rather than alias silently. Exact new slot full-key identity still needs its separate implementation/proof.
- Private WeakMap token at 34–43 binds a held read-only generation DB to exact image/control, catalog identity and real generation guard. New calls refuse on accepting=false; pending operations drain at 77 before DB/guard/image closure. Recheck 38 verifies actual catalog dev/inode/size/times and issued image; no serialized receipt grants token authority.

## Metadata accounting and remaining concrete limits

Current policy explicitly caps 8192 bindings, 16384 inventory rows, 8192 directories, aggregate logical keys 4 MiB, paths 2 MiB, catalog 8 MiB, receipt 4096, initial journal 24 KiB, auxiliary 32 KiB, and per-uninitialized growth/journal allowances of 24 KiB each. Census enforces rows/path/directory/key limits (66–69); final SQL count includes queue-only bindings (73). SQLite max_page_count/page_size is checked, journal allocated bytes measured before commit, receipt schema/bytes/fsync and parent fsync occur at 74. New metadata charge includes metadata directory blocks at 76, and original directory blocks enter existing charge at 66. These address prior short-key/file-only census omissions in Source.

Funding checks at 65/73 are real available-byte observations under catalog EX and issued image lifetime, not reservation against every unrelated writer in the same image. This independent owner requires a dedicated namespace and does not claim global image-writer exclusion. Canonical publication must retain the eventual actual funding writer as well as catalog custody. The reported quote is also not a total finite-slot enrollment quote: populated PIN/GUARD/full-key slot storage and future live transaction peaks remain separate.

The construction persists baseline metadata but exposes no authenticated reopen API for a committed generation. Inode/hash receipt verification and private generation issuance on a later process are therefore not yet implemented; the current token exists only inside this constructor callback. Do not present this proof as a complete crash-recovery or cross-process canonical enrollment protocol.

Queue arrival directories intentionally remain writable for old published participants. Their ticket inventory can change while the async census runs; `inventoryRows` is consequently not an immutable snapshot of all future arrival-ticket content. The binding partition remains conservative for queue hashes, but canonical accounting must explicitly budget still-live old ticket mutations/growth rather than infer a frozen total from this census.

No actual slot exhaustion behavior is exercised by this owner: it classifies `new`, it does not allocate/reassign reusable guards. Global probing, full-key PIN identity, generation pinning and immediate safe-pool exhaustion refusal remain required in the separate placement integration.

## Result

Fix canonical namespace spelling before using `new` classification as placement authority. The implemented old-target/hash-reservation/inode mechanisms otherwise address the prior concrete baseline gap in Source. Latest funding/cleanup successors still need their real gates, and this report authorizes no canonical enrollment.

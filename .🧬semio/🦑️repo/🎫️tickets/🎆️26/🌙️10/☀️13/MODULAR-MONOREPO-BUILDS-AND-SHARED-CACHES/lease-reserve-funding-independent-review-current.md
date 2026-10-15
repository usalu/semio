# Lease Reserve Funding Independent Review

Read-only source inspection on 2026-10-15. No builds, tests, new Originals, image opens, reserve recovery, production writes, or physical measurements were performed. Root supplied the canonical observation: reserve logical/allocated 4,194,304 bytes, dev 16777260, inode 30776; available 356,352 bytes. This report does not independently revalidate that observation.

## Recovering the reserve is temporary liquidity, not permanent pool funding

`🧰️framework/🔨️modules/🏃️process/🔒️leases/📦️metadata/🟦️.ts` lines 7–10 requires live exclusive custody of the canonical `queue-claim-catalog`. Its initial `retain()` fills a zero-length reserve with actual 64 KiB writes and fsync, refusing sparse under-allocation. `recover()` truncates the same verified inode to zero and fsyncs. The returned `recheck` is actually `retain`, so it reextends and physically charges the full reserve again.

`🔒️leases/🟦️.ts` lines 222–230 holds catalog EX throughout retirement, recovery, and final reserve recheck. Recovery occurs only following ENOSPC (or the narrow EFAULT/free-block condition) during retirement at 226. The finally at 230 restores the full reserve before catalog release. It releases the catalog even if reserve restoration throws. Every controlled `acquireQueuedResourceLease` invokes that retirement owner before publishing its ticket (241), so the next ordinary admission attempts reserve retention again before claim admission. Therefore freeing 4 MiB does not fund a permanent 2,490,368-byte pool while preserving the current reserve contract.

Arithmetic using the supplied baseline: 356,352 + 4,194,304 - 2,490,368 = 2,060,288 bytes available after hypothetical permanent pool creation, before other overhead. Restoring the reserve would then be short by 2,134,016 bytes. The current permanent pool must fit in existing free space or obtain additional genuine reclamation; the same reserve bytes cannot count both as permanently allocated pool and retained emergency headroom.

## What the actual guard protects

`🔒️leases/🟦️.ts` 212 derives the canonical catalog in the same data namespace. EX custody at 222 serializes retirement and reserve mutations. Openers hold catalog SH while opening the per-arrival claim and writing its ticket (248–249), excluding retirement during that publication. Claims themselves remain held until lease release or failure (263–278). Exclusive identity custody is private-issued and checked at 74–75; underlying SQLite BEGIN EXCLUSIVE/BEGIN and release rollback/close appear at 154–164.

This prevents two conforming retirement owners from concurrently recovering the same reserve. It is not a physical capacity reservation for a separate pool: no production capacity-writer lock or durable pool enrollment exists in the inspected admission path. Catalog custody ends at 230, before subsequent arrival admission, and catalog SH permits concurrent openers. A statfs observation under catalog EX alone does not reserve future bytes after release. A permanent pool conversion would require actual writer custody through conversion, durable enrollment and all resulting allocations, plus an explicitly consistent reserve policy.

## Candidate accounting visible in current ticket source

Ticket `lease-namespace/📜️script.ts` 81–82 currently creates 16 entry catalogs, 64 resource guards, 64 pins, held queued arrivals/claims, then 16 catalog-reader guards, two capacity/enrollment guard files, and a padded 4096-byte namespace seed before its measurement. Thus those latter guards/header must not be described as omitted from the *current source candidate*. The supplied 2,490,368-byte historical figure cannot be assumed to include this newer source without its corresponding runtime result.

Its recursive `walk` at 82 sums regular-file allocated blocks and deliberately excludes directories. It subtracts 16 reserve capacities to derive `newMetadataWithoutDuplicatedReserves`. That figure omits directory allocated blocks and does not establish peak SQLite journal/creation/retirement overhead. Its separate before/held statfs difference includes filesystem effects at that observation but is not an exclusive global-writer measurement and does not bound transient peaks. The guard names are acquired in shared mode and are fixture-held files; they do not demonstrate a production capacity transaction. No exact missing-byte total is established by read-only inspection.

## Action

Keep recovery scoped to emergency retirement under canonical catalog EX. Do not count recovered reserve as permanent funding unless the reserve contract is deliberately redesigned and all admissions use that same policy. Requalify the candidate accounting after current guard/header additions, count directory/peak journal costs, and establish production capacity-writer custody before relying on available-byte arithmetic.

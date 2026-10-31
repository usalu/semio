# Nx Native Funding Pressure — 2026-10-29

Actual frontend physical fixture1171 terminated on Native storage capacity exhausted at the funded child reserved append for repository:nx-namespace. It crossed provider dispatch and the former repository-charge/Native-journal lease-order boundary. Full fixture acceptance did not pass. The independently audited atomic order Source remains frozen.

## Current Genuine Observation Limits

Frontend owned one supported inventory50524 through actual publisher control, received Original, repository nx store and observeNativeStorageStateV1 on the actual enforcing port/policy. It ended before the callback after92.16s with a resource deadline. At4s its process held nx image SH and Native mapping SH and was queued for the nx-root Native journal EX, with no repository-charge EX held. Therefore no current allocations, sequence, used bytes or remaining bytes were returned. Configured policy values are capacity1,610,612,736, filesystem1,609,564,160 and shared artifact owner536,870,912 bytes; configured values are not substituted for a measured journal snapshot.

A later supported repo MCP leases_status snapshot had6holders,0waiters,0cycles and no native-storage holder matches. That post-exit observation does not identify the former holder, prove it stale, or permit deleting claims. No duplicate Native inventory, cache reset, signal or claim mutation was performed.

## Proven Source Gap

Repo library caching/storage/enforcement/nx/graph capacity() passes only roster payload bytes, per-namespace limits=min(512MiB,payload+4MiB), and actual filesystem available bytes to nxGraphCacheCapacityV1. It checks logical committed namespace envelopes against896MiB and physical available minus unused namespace reservations and new demand against16MiB metadata reserve.

requireRoom adds64MiB+4MiB for a new logical namespace. Actual Native funding instead reserves64MiB+128KiB=67,239,936 bytes under the parent journal. The Native reducer independently checks the effective parent capacity against the sum of ALL active allocation bytes, including shared store allocations and reserved, committed and retiring charges. Nx room admission currently has no such journal-used/remaining input. These are distinct budgets; physical free bytes do not imply durable funding headroom.

A second gap: acquireNamespace passes additional=0 if its logical descriptor already exists, even when that namespace still has no funded child. Descriptor existence therefore is not an actual zero-cost Native birth quote. The final Native capacity error occurs in retainNxGraphChildV1 and is not NxGraphRoomNeededV1, so it propagates instead of entering the existing pre-admission idle-victim recovery loop.

## Existing Strict Recovery Boundary

poolWithRoom leaves the short exclusive pool pass before ordinary eviction. evictIdleNamespaces takes its genuine queued eviction gate and then one victim namespace EX without waiting, skipping current/busy namespaces and rereading the unchanged descriptor stamp. A later pass can ask an actual idle daemon to yield; an active user remains protected. It never infers eligibility from PID death or age.

retireNxGraphNativeChildV1 observes the actual matching charge under enforcing custody. Reserved or retiring rows are ineligible and remain charged with namespace and birth sidecar intact. A committed row still goes through recovered-reservation, exact parent/policy/backing/kind checks, accepted physical cleanup, child maintenance EX and complete kernel/reader/claim settlement before charge release. Only actual successful retirement permits namespace metadata removal. This API must remain authoritative; no lower guard can be replaced with a budget arithmetic result.

The previously observed three-record reserved birth cannot be reclaimed from sidecar presence or absent child alone. Neither current inventory nor authenticated terminal creator/cleanup authority has established that case eligible. No refund, forced commit, generic catch, cache purge or quota raise is proposed.

## Bounded Candidate To Validate After Facts

Keep the existing logical/physical policy and add an actual Native funding quote before child birth. Its used bytes must include every active durable row, its effective capacity must respect both policy and actual backing, and its new demand must come from the actual canonical funded-child charge. Whether another charge is needed depends on an actual matching funded child, not a logical descriptor. Insufficient funding should enter the existing RoomNeeded/strict eviction path before invoking the final Native producer, then reobserve after an eligible victim settles. The final Native reserve guard remains mandatory, and protected charges remain protected.

This is a causal plan, not implemented Source or a claimed current headroom value. No new pressure fix, heavy job or unchanged runtime retry has been launched. Actual facts and coordinated root ownership remain required for the next physical attempt.

The later new-reader global inventory11986 also expired before data after92.60s, now correctly waiting for journalSH (notEX). Proper1/23physicalreader proof is documented in native-existing-reader-2026-10-29.md. Current global used/remaining values are still unknown; no assumed-value pressure patch has been made.

## Genuine Current Funding Facts

After the profile writer released, frontend inventory76716 authoritatively exited0: effectivecapacity1,610,612,736; used1,545,469,952; remaining65,142,784; newcharge67,239,936, hence2,097,152shortfall. All16rows remain accounted (sharedstore536,870,912;14committed+1reserved namespace charges). Sequence71/journal21627B/SHA37b94fb1afc200e9f0d04800cee4343a78694fb6bbf9e0c220e3abebac6a8101. The frontend owns actual Native funding preflight implementation using these genuine facts and the new shared reader; no overlapping graph edits are made here. Exact lock metadata trace is native-journal-lock-owner-2026-10-29.md.

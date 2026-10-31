# Pooled Rust Cleanup and Waiting Custody

Bounded read-only defining Source and existing-law inspection. No live collector, preview, test, setup, Cargo, Source change or private lease operation was invoked. Native11's three actual-log commands remain conditional with zero attempts.

## Finding

The current automatic pooled collector can remove genuinely idle R/u/l lanes while unrelated requests retain the Rust image SH. The inspected collector path does not acquire image EX or compiler-writer EX. It operates under the already opened image lifetime and uses its own single-flight collector claim, then each lane's claim and live-marker recheck. Therefore the image SH held by a headroom waiter is not itself a demonstrated cleanup-blocking cycle. A live or claimed lane is intentionally protected; a successful scan returning zero can mean no eligible units, not a deadlock.

This is Source evidence of the ownership algorithm, not a fresh inventory of eligible current lanes or proof that any particular process is inactive.

## Actual Ordering and Claims

- Cargo launcher withAgentLaneV1 lines105–111 validates the genuine storage reservation, reads a pure warm-lane plan, then awaits admitImageHeadroomV1 BEFORE acquirePooledLaneV1. A headroom waiter has no acquired lane claim/live marker from this path.
- Headroom lines156–160 holds its ledger claim only for a decision. The awaited reclaim call at167–169 happens after that claim callback returns. It does not hold the ledger claim while awaiting the collector. Waiting state heartbeats and arrival/reservation accounting are ledger DATA, not a lane claim or collector lock.
- A space decision invokes collectGarbageSingleFlightV1 with the actual shortfall even below ordinary watermarks. A queue-only decision waits for earlier reservations. Once admitted, the reservation monitors actual free space and can trigger the same collector.
- Collector lines118–136 takes its own tryClaimWithBallast collector claim. If another collector owns it, it returns running:true instead of waiting for image EX. With root R/u it first performs lock-aware incremental/session and build-unit collection rooted at R, then measures and plans lanes/objects under R/u. It releases its collector claim in finally.
- planGcV1 lines49–76 excludes live lanes; demand can select otherwise idle lanes sooner than idle-age eviction. removeLaneV1 lines92–98 acquires the exact lane claim, rechecks laneLiveV1, then removes only if neither custody condition is busy. A lane that becomes live after planning survives at application.
- acquirePooledLaneV1 lines75–87 polls while the pool is at its live maximum without retaining a lane claim. On a selection it first claims lane-id, then enters the live marker; release removes marker and releases claim. Headroom reservation is released on the caller's success/failure paths; signal cancellation withdraws a waiting headroom record.
- Once a request already holds a lane, that lane remains protected while the command executes or waits at later compilation gates. That is intentional current custody, so other idle lanes remain collectible but this current lane cannot be reclaimed to fund itself.

A request admitted to headroom that then waits for a busy pool retains its headroom reservation, which can legitimately delay later admission. The bounded code reviewed shows no reciprocal dependency where that wait holds a lane/collector claim needed by the active lane to finish. This does not establish universal absence of concurrency defects; no concrete cycle was demonstrated here.

## Existing Regression Coverage Read, Not Run

Headroom tests line118 verify an alone command proceeds after a completed collection and refusal/cancellation removes its ledger row. Line159 verifies a running command triggers reclaim below the floor and stops monitoring on release. Line170 verifies demand reclaims oldest idle lanes without touching live lanes or protected newest objects. GC test line142 exercises second-collector running:true and release/reacquisition of the single-flight claim. Pool claim laws cover claimed/live lane retention, bounded busy growth, and later trimming. These files also contain neutral vectors and independent Python/SQLite comparisons. No current execution receipt is claimed by this review.

## Scope and Next Decision

Public inactive maintenance still covers R/b/p, not these pooled lanes; do not use its historical empty preview to infer pooled eligibility. The genuine automatic owner already attempts lane-aware reclamation. No new cleanup feature, all-peers-stopped rule, cap increase or manual cache action follows from this finding. Native11 stays the sole live request; only its defining owner and actual terminal evidence can determine the next action.

## Inspected Source Identities

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts: SHA256 2970758a1c5699c177d4de4b36b9bbd3ca80d0dffca84688db14839e101cbf23
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🏦️headroom/🟦️.ts: SHA256 34d73af59aff7300f9f543e9ab3e165b15ed24c4a6f3f6c883f3289a339cb31b
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts: SHA256 e484245c33aeface5bc4541f98893c4f778bb062d4cadcd2ec8a0961efff82d4
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🟦️.ts: SHA256 f17e07bf1019a264ed2d44dc95702156af0978022cce72aa778ac44bcadb2d12
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts: SHA256 cb9b3abae97742927259b099254b2fdcf2d7871f393a9cfe87b32fb2e0daea60
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🏦️headroom/🧪️tests/🟦️.ts: SHA256 b42207886bc286c7da1ce1b0edfd7294b7d98e8c33b448d13257b8c78c288b62
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧪️tests/🟦️.ts: SHA256 e72dc0e10541a02fe5a3923a524cf0354638a73b71eda94e91fa2897df13bcf1

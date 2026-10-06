# Live Cargo Preparation Lease Queue Diagnosis

Bounded read-only process and queue snapshots observed a live FIFO progression, not a proven stale lease/deadlock. No process was stopped, no lease/database/ticket was opened for writing or removed, and no owning gate ran.

Initial observation identified active LAS Source ancestry: Nx98007 → owner99712 → preparation99713, preparing ✏️s/Cargo.toml. Preparation99713 was running at33.2% CPU and later15.3%; it had no observed Cargo/rustc child, consistent with synchronous first-party preparation work rather than a blocked compiler. Subsequent snapshots showed99713,99712 and98007 gone; the next FIFO preparation99771 became running at66.6%, then78.6% CPU, with CPU time increasing from4.02 to9.18 seconds. That transition is concrete progress and rules out a permanently dead first holder at this observation.

The resource queue is /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/agents/resource-leases/d66997d14952d15930b33359e79fa513dc13f03f2c1ee6edeb22c3afedac951b.queue for cargo-preparation:/Users/ueli/Documents/semio. Fifteen live queue-ticket records remained in arrival order, with head99771 (2026-10-05T23:01:41.024Z), then99801,99840,99984,368,587,586,1436,2240,2253,2262,4445,4582,5197,5454. A head ticket remains while its holder executes, so its presence does not imply waiting or stale ownership. The exclusive SQLite rollback transaction carries the real lock; the immutable resource row contains resource/version, not a PID lease claim.

Relevant actual Root task ancestries:

| Root scope | Nx PID | Observed descendant / state |
|---|---:|---|
| BCF/Deflate Before | 95343 | Deflate owner99447; waiting behind shared preparation activity |
| Part21 After | 98004 | UI generator owner99800 → preparation99801 queued |
| OBJ/VCS/LAS/MD mixed | 98003 | UI generator owner99770 → preparation99771 became active head |
| LAS Source | 98007 | owner99712 → preparation99713 completed/disappeared during audit |
| TIFF selected test | 98215 | Nx still alive; no terminal inference from process presence |
| root sync Cargo | 66946 | owner77440 → sync78831 → ZIP oracle preparation99840 queued |
| Store owning integration | 97623 | owner98785 → script99983 → preparation99984 queued |

Long quiet logs therefore currently reflect one repository-wide preparation resource shared by many Root/other task descendants. Live queue and CPU evidence supports serialization and progress; it does not prove every queued task will finish or supply test success. A lack of per-file logs during prepareCargoOwners/publishCargoWorkspaceMembership is expected from the current synchronous protected body. No bypass, stale-ticket deletion or cancellation is recommended from this evidence.

Exact authorities: [preparation exclusive lease and protected body](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/📜️script.ts:15), [actual preparation functions](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts:195), [SQLite transaction lease](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:68), [live PID FIFO queue lifecycle](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:129).

# Result lq: queued (waiting) cache maintenance on ordinary commands

Status: code and tests done; real-CLI proof partly blocked (see "Blocked"). No package.json/bun.lock edit, no cargo run, no git-modifying command. `ticket_open` over repo MCP unavailable (repo MCP not connected), work recorded in coordinator ticket `C`.

## Inventory (every nonwaiting whole-image EX / writer takeover reachable from ordinary commands)

Converted to queued (progress via `control.publish`, cancellation via the issued control signal, bounded by `repositoryCacheOrdinaryMaintenanceV1()` = 30 min, taken BEFORE this invocation pins that partition):

| # | Caller | Command path | Before | After |
|---|---|---|---|---|
| 1 | `prepareNxGraphCacheStorageV1` first-time pool creation (`🔌️nx/📦️graph/🟦️.ts`) | run / run-many / affected / exec / watch via `prepareBuildToolStorageV1` (pre-pin) and the artifact-funding callback (post-pin release); `NxScript.runIssued` | `withRepositoryCacheStoreMaintenanceV1` ("Quiescent pre-pin cache storage admission required") | `withRepositoryCacheStoreQueuedMaintenanceV1(..."graph"...)` |
| 2 | same function, mandatory headroom recovery (`recoverBuildGraphImageHeadroomV1`) | same | `wait:false`, refused with "Active graph readers retain required Nx pool headroom" | request now carries `wait` (schema `📏️headroom/🧬️schema`), queued with the 30 min bound |
| 3 | products charge reconciliation (`📥️opening/📐️layout`) | first open of the products partition after a layout change | `wait:false`, "Products reconciliation requires quiescent whole image custody" | `wait:true` |
| 4 | bazel `reset` (`🏗️bazel/💾️storage/🗑️reset`) | `bun repo --tool=bazel reset` | `withRepositoryCacheStoreMaintenanceV1(products)` | queued |
| 5 | Nx publication writer (`🔌️nx/🟦️.ts` `withNxPublicationHeadroomV1`) | every Nx cache publication/restore of exclusive custody | `withCompilerInitialHeadroomV1({wait:false})`, concurrent Nx runs failed with "Resource lease timed out" | `{wait:true}` (queued writer) |
| 6 | tools headroom recovery (`🛠️tools/📜️script.ts`, `🧹️retention/📥️physical`) | tools setup, Nx tooling storage | queued but `maximumWaitMilliseconds:0` (unbounded) | the same bounded ordinary request |
| 7 | dependencies install (`📦️dependencies/💾️storage`) | dependencies sync | already queued (coordinator) | unchanged |

Kept nonwaiting on purpose (roster `🏪️store/🧪️tests/📥️queued/🔣️.json`, each with kind and reason, enforced by a test): inherited descendant of a pinning ancestor (`prepareNxGraphCacheStorageV1(...,"inherited")` used by `withRepositoryNxGraphNativeStorageV1` and by `NxScript.runIssued` when `SEMIO_BUILD_GRAPH_PATH` is set: waiting on the ancestor would deadlock), LRU eviction of other Nx namespaces, idle lease-image retirement, optional graph cache trimming, selected-tooling object GC, compact headroom recovery inside the caller's own compiler writer, the operator commands `cache-retire-inactive` (history/budget/execution), protected-parent retirement, and a type literal.

Also fixed (found while running the real scenario): first-time pool provisioning never created the parent directory `.cache/b/g/m/nx`, so the nested image open died with `ENOENT lstat .../nx` (`openNxGraphCacheStoreV1` now `mkdir`s the parent when provisioning).

Ordering rule (documented on `repositoryCacheOrdinaryMaintenanceV1`): queued maintenance must start before the invocation takes its own shared pin on that partition because later readers queue behind a waiting maintenance ticket (proved by the test scenario "queued-orders-late-readers-behind-it"). Consequence: a descendant process of a pinning invocation must never queue (hence "inherited"). Known residual: while an unrelated invocation B waits for EX on `graph` (only first pool creation or low disk), new shared opens by descendants of other pinning invocations block behind B until B finishes or its 30 min bound expires.

## Files
Changed (relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`):
- `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts` + `🧬️schema/🔣️.json`: `repositoryCacheOrdinaryMaintenanceV1()` (schema `$defs.ordinaryMaintenance`).
- `…/📦️budget/🕸️graph/📏️headroom/🟦️.ts`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🔣️.json`: `wait` in the request; tests: portable request corpus (Ajv + SQLite json functions), new scenarios `queued` and `queued-cancel` (darwin real image, see Blocked).
- `…/📦️budget/📥️opening/📐️layout/🟦️.ts`, `📥️opening/🟦️.ts` (docstring only).
- `…/🔌️nx/📦️graph/🟦️.ts` (queued/inherited admission, bounded headroom recovery, parent mkdir), `…/🔌️nx/🟦️.ts`.
- `🏗️build/🔧️tools/🏗️bazel/💾️storage/🗑️reset/🟦️.ts`.
- `⚡️caching/🚀️bootstrap/📜️script.ts`, `…/🛠️tools/📜️script.ts`, `…/🛠️tools/🧹️retention/📥️physical/🟦️.ts` (also dropped the unused nonwaiting import).
New: `…/📦️budget/📥️opening/🏪️store/🧪️tests/📥️queued/{🟦️.ts,🔣️.json,🧬️schema.json}`; temporary tools `C/lq-tools/{run-test.ts,probe.ts,tsconfig.json}` (inputs, kept).
Not touched on purpose: `🏪️store/🟦️.ts`, `🏗️build/📥️inputs/💾️storage/🟨️.mjs`, budget `🔣️policy.json`, `🏗️build/🕸️graph/🟦️.ts` (source-revision guarded: editing them makes every running `bun repo` die with "Stale loaded repository Store implementation").

## Tests (all run through `bun C/lq-tools/run-test.ts <file>`, which creates a real script envelope and runs `bun test` under it)
- `📥️queued/🟦️.ts`, 4/4 pass: (1) corpus closed + ordinary request equals the schema constant and validates against the store `QueuedMaintenanceRequest` (Ajv strict, SQLite); (2) every queued caller carries its queued token; (3) roster gate: AST count of nonwaiting sites (`withRepositoryCacheStoreMaintenanceV1(`, `wait:false`) per production file equals the roster and equals an independent Python scan; any new nonwaiting site fails the test until declared with a reason; (4) lock behaviour with a shared holder and a second maintenance requester (real lease directory, Python `sqlite3 BEGIN EXCLUSIVE` oracle): queued waits >1.2 s with progress callbacks and succeeds after release, queued cancelled while waiting leaves the holder intact, a late reader queues behind the waiting maintenance, opportunistic defers within 1 s.
- `📦️budget/🧪️tests/🟦️.ts` 4/4 pass, headroom portable-request test pass. `tsc` (lib config, changed files) 0 errors.
- Pre-existing failures, not caused by this work: every test that opens a private nested image fails with "Nested persistent image requires actual parent scope" / "Exact bounded Native receiver ancestry required" (headroom real-image scenarios incl. my two new ones, `retention/physical` queued-cache test). They are unverified, not green.

## Real-world checks (no CLI sync green yet, so not the requested two `show projects`)
- Two real processes on the real `graph` image (`probe.ts hold` holds `withRepositoryCacheStoreV1(graph)`, `probe.ts prepare` runs `prepareNxGraphCacheStorageV1`): the preparer waited 16-36 s with no refusal, then took EX and allocated the 0.9 GB pool; with a 6 s cancel it aborted exactly at 6 s ("lq cancellation").
- `bun repo run @semio-tech/repo:nonexistent-target` while the deps executor's `dependencies lock --all` held the dependencies queue: waited with one `[script]` progress line per second; SIGINT ended it in 1 s ("Original Script process interrupted").
- Two concurrent `show projects` does not exercise these paths (it never takes graph/products/nx EX); repeat the check with two concurrent `bun repo run …` once deps is GREEN.

## Blocked / next critical blocker (not locking)
Nested image attach fails on this host (macOS 27.0 / 26A428): after the queued EX is granted and `fixed.dmg` (0.9 GB) is created, `/usr/bin/hdiutil attach -nobrowse -noautoopen -mountpoint …/g/m/nx/cache/m …/fixed.dmg` prints `hdiutil: attach failed - Permission denied`; the pool is rolled back, so first-time `nx/cache` creation cannot finish and `bun repo run` still cannot work.
Reproduced by hand without repo code: attaching a sparse image inside a volume that was mounted by `hdiutil attach -nobrowse` (default = `noowners`, like every repo partition `.cache/b/{g,p,…}/m`) is denied; the same nested attach into a parent mounted with `-owners on` succeeds, top-level attach under `.cache/b` succeeds. Fix proposal (storage owner): add `"-owners","on"` to the three attach argument lists (`🧹️cleanup/🎛️command/🟦️.ts` activation, `📤️publication/🎛️owner/🟦️.ts` x2; their tests pin the exact arguments) and re-mount the parent partitions that host nested images (graph, products, rust) once.
Leftover: empty directory `.cache/b/g/m/nx` (created by the new provisioning mkdir, needed by the next attempt); no mounts or sparse images left from my probes.

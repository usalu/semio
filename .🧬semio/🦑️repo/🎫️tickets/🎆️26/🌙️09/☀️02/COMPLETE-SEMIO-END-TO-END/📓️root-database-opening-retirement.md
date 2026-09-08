# Database Opening Retirement

## Boundary and Evidence

The public administrator regression capture `directory-command-authority-native/exact-cargo-laws-9JAbu2/00` passed its first four laws, then timed out during its next native process's first `test_state()` Database open. The five-second deadline was not changed. That timeout has not been causally attributed to the opening ownership defect below.

Read-only inspection found that Filesystem and SQLite storage register an executor and acquire a WorkerPool use before awaiting the real BackendOpen task, but construct their concrete Storage only after that await. Cancelling the opening future or returning its registered rejection therefore has no constructed Storage whose existing Drop requests backend retirement. PostgreSQL, Neo4j, and Memory already construct their Storage before the await and are outside this repair's scope. The intended change is to move concrete Storage construction before BackendOpen, preserving its existing exact-control Drop and the explicit registered rejection type; no compatibility wrapper or public rejection redesign is needed.

## Test-First Work

Expanded the closed backend-pool-use language-neutral fixture/schema with four opening rows: FS and SQLite, each queued cancellation and a physical path-type conflict. The existing registered writer-authority source target validates these through AJV and an independent SQLite ownership-state oracle. Its native cohort now begins with two actual-storage laws, before the older writer/retirement regressions.

The cancellation law holds the real single I/O worker with a channel-controlled blocker, polls the real storage opening future until its BackendOpen is queued, verifies the exact registered pool use, drops the future, and inspects close-request ownership before releasing the worker. Ordinary mounted maintenance—not an explicit test-side close request—must return the exact ledger and backend-slot baseline. A fresh real storage then opens and retires on that same pool, after which pool shutdown must succeed.

The failure law uses a real existing file as an FS root and a real existing directory as a SQLite database path. Independent rusqlite also rejects that SQLite path. The actual returned registered rejection must already have deferred-close ownership before the test drops it, without retry_close. Maintenance, fresh opening, exact ledger/slot baseline, and pool shutdown are then checked as above.

These native laws have not compiled or run yet. Production constructors are intentionally unchanged while establishing RED. Root's active administrator native build is rebuilding the previously removed compiler cache; another heavy compiler has not been launched.

Source execution initially stopped on a duplicate renderer project path. Read-only filesystem inspection showed only the canonical path, while generated Nx project graphs contained a corrupted non-BMP package segment. A fresh cache alone reproduced it. The existing repository `devToolingEnv()` explicitly prevents this known plugin-worker IPC path corruption by setting `NX_ISOLATE_PLUGINS=false`. Root now invokes the repository-supported `bun ./📜️script.ts nx run ...` wrapper that applies that environment, rather than direct Nx. No namespace deletion, project renaming, or source workaround was performed.

## Files and Remaining Qualification

- DB storage Rust test module and backend-pool-use JSON fixture/schema.
- OS kernel package `📜️script.ts` extends the already registered writer-authority command; existing project and launch entries remain applicable.
- Planned production edits are limited to the FS and SQLite opening constructors.

No native success, first-open timeout diagnosis, live PostgreSQL/Neo4j result, or broad end-to-end goal completion is claimed. The ticket and goal remain active. Repository ticket MCP operations remain unavailable in the active tool catalog.

## Source Qualification

The repository-wrapped `@semio-tech/framework-os-kernel:wal-writer-authority-check` completed successfully: AJV=6, exact-u64=1, writer cases=3, mutations=6, remote=5, writer slots=32, retained result=1, directory barriers=4, WAL-open owner=1, backend-pool-use=9, and the four new physical-opening oracle rows. Scoped Rust formatting and diff whitespace checks passed. This source/independent-oracle result does not qualify the new native cancellation and physical-failure assertions.

The first native capture, `database-opening-retirement-native/exact-cargo-laws-i0Xaeu/00`, stopped at six compiler errors in other existing DB test modules, before discovery. The facade tests lacked imports of the already public root `Profile`, `DurabilityClass`, and `DbError`; the engine payload test lacked its existing `PayloadStorage` trait import. Only those test-module imports were corrected. No production API or test assertion was changed, and the native target is being retried from that concrete correction.

## Reproduced Defect and Production Repair

`W0CS3j/00` compiled and ran the real queued-opening law. It failed in 0.01 seconds at the causal assertion: cancelled Filesystem opening had `close_requested=false`, where deferred retirement requires true. The actual worker blocker was released before that assertion; no test-side close request masked the defect. This is native behavioral RED, not the unrelated historical five-second Hub opening timeout.

The minimal production repair is now applied to Filesystem and SQLite: concrete Storage is constructed immediately after backend registration, before awaiting BackendOpen. Its existing Drop therefore owns and requests retirement on cancellation or physical opening failure. Successful opening returns that same Storage; the registered rejection still carries the exact control for explicit observation/retry. PostgreSQL, Neo4j, Memory, the public error shape, and storage protocol deadlines were not changed. Native qualification is in flight.

## Native Qualification

The entire registered writer-authority cohort is GREEN40 in `sB9kHD/00`, executable `db-2eec516e8ad678d9`, SHA-256 `fa55dceb32c3d68be6e6313287cee6db8a4b5560030390647b181eb8acb963b8`. Both new exact laws passed in 0.03 seconds each. Their inspected `[DEBUG]` lines cover FS/SQLite queued cancellation and FS/SQLite physical opening fault: close requested before retry, exact ledger and backend-slot baseline restored, real storage reopened on the same pool, and terminal pool shutdown. The other 38 writer, result-owner, callback, directory-durability, replication, rejected-open, registration-pressure, and pool-use regressions passed on the same executable. Nx's generic flaky-task notice follows this target's intentional earlier RED and compile correction; no unchanged failing assertion was auto-retried by this workflow.

This qualifies the concrete registered-opening leak repair, not causal attribution or elimination of the historical first-Hub-open timeout. Current Hub administrator and directory socket regressions are being rebuilt against these constructor changes before the shared native target is released to GIS qualification.

Final slice includes storage FS/SQLite constructors and native tests, backend-pool-use fixture/schema, OS package script, test-only import corrections in DB facade/engine, and this report. No new project or launch command was required: the existing registered source/native writer targets run the expanded fixture and exact native cohort.

## Hub Regression Boundary

After the constructor repair, administrator capture `pf3kjS/00` passed its new exact fifteen-binding law and eight physical short-action cases, then failed the existing mapped-command law at its five-second initial `test_state()` deadline, before policy assertions. The separate global WebSocket cohort `de0FhB/00` passed all six exact laws on the same executable SHA-256 `1c2f87221a8aec928879bdfa5559d30d50dda118138704290aacd202777dea49`. This is direct evidence that the registered-opening retirement repair is not a demonstrated cure for the intermittent initial Hub state opening stall. No unchanged failing law was blindly retried, and its deadline remains intact. The opening stages and retained scheduling/wake path are under separate causal investigation while GIS owns the native build lane.

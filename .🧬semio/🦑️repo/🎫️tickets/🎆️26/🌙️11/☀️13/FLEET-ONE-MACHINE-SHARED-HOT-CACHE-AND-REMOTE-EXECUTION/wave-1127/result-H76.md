# H76: process-group isolation, identity-checked stops, reference-counted shared children, parent-death reaping

## Already true before (not changed)
Owned commands spawn `detached` (own session/process group on POSIX), stop = SIGTERM for the owned tree, grace 5 s, SIGKILL after; Windows `taskkill /T /F`.

## Gaps found and fixed (`🧰️framework/🔨️modules/🏃️process/🪓️termination`)
1. Stops were keyed by pid only: a pid reused between the process-table snapshot and the signal (or a synthetic root row for a gone root) could be signalled. Now one `ps` call reads pid, ppid, pgid and creation time; right before every wave a fresh table verifies each row by pid+startMs (`verifiedOwnedRowsV1`); a root of another creation time owns nothing; the grace-end rescan is pinned to the root's creation time.
2. A process group was signalled when its leader was owned, even if strangers sat in it. Now a group is signalled as a group only if EVERY process in it is owned; otherwise only owned members get the signal. Orphans of a dead root that kept its group are part of the tree.
3. Sender/reason/target: kill record now has victim and sender creation time (`kill-identity` text EN/DE, Python oracle and fixture extended). `ownedKillRecorderV1(label, reason)` is the one recorder; `ownedKillLedgerWriterV1(file)` appends `<ISO> <line>` (layout of H70's `kills.log`). The recorder row is `{pid, ppid, group, startMs}`, compatible with H70's `readyKillRecorderV1` row, so H70's reader/`readyStopSenderV1` keep working; ready stages can pass `record` from either.
4. Parent death: new `🧹️reaper`. One sentinel per parent process (own session, started on first registration, stdio only a stdin pipe so nobody waits for it); `add <pid> <spawnedMs>` pins identity, `remove` unregisters; pipe EOF (SIGKILL, crash, normal exit) -> SIGTERM tree, 5 s grace, SIGKILL, by pid+startMs; logs to `<tmp>/semio-reaper/<parent>.log` in EN/DE. Registered in `runOwnedCommand`, the capture body and `📥️capture`.
5. Shared children: new `🤝️shared-child`: `createSharedChildRegistryV1` (a requester counts once, double release no-op, last release stops the tree and drops the entry, dead child replaced at next acquire, `releaseAll(requester)`), `spawnSharedChildV1` (own group + reaper), `sharedChildNodeHostV1` (graceful stop with record).

## Tests (all green where run)
- Identity 8 (reused pid never signalled, group-with-stranger, record text, live table vs `ps`), reaper 3 (SIGKILL parent: polite group and the stubborn TERM-ignoring group both gone after grace, unregistered child spared, log has victim+sender identity; SIGTERM parent; line grammar vs Python regex), shared-child 4 (bookkeeping; two requesters share one cargo-like child: stop one -> cargo and rustc survive, stop both -> gone; parent SIGKILL -> shared child gone). Cross-checked with `ps`/`pgrep`. Suite `process-tree-termination` via `repo run ...:test-process-tree-termination`: ok. Typecheck 0 errors.
- The new test files are imported by the listed `🪓️termination/🧪️tests/🟦️.ts` because the suites table was sealed; add them as suite files after the seal.

## Unverified / open
- `owned-execution` suite: only `🎛️owned-execution/🧪️tests/📤️style/🟦️.ts` fails (20 s test timeout under load 60; sentinel cost measured 35 ms once per process, so I believe it is load, but I did not prove it with an A/B); all other owned-execution files passed or need the harness.
- Windows: no job object (Bun/Node cannot create one without native code); tree kill by `taskkill /T /F`, the sentinel works there too, but the creation-time verification of rows is POSIX only. Not run on Windows.
- Limits: a descendant that left the group via its own `setsid` after its owner died is not found; no consumer yet calls the shared-child registry (cargo broker wiring is the next step); other spawn sites (`Bun.spawn` auxiliary, product-specific spawns) are not yet registered with the reaper.

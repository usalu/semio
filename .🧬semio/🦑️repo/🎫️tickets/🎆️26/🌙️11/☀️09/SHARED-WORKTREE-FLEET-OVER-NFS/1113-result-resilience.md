# Result: resilience of the fleet replica daemon (2026-11-13) — LANDED

Status: everything is built and was tested in a temp copy, but **nothing was saved into the live module**. The landing protocol (`1113-landing-protocol.md`) says the warm-restart agent lands first; its lock `1113-landing.lock` was still held (owner `claude-warm-restart`) and `1113-landings.md` did not exist after more than 10 minutes. The live `🔁️replica/🟦️.ts` and `🩺️attest/🟦️.ts` were also at the incident version (05:12:20), so my patches (built against the 05:05 warm-restart version) no longer apply there. Re-landing needs: lock, `mk.sh`, `apply-all.sh` on a fresh mirror once the warm-restart code is back, then a three-way merge per file, all suites, `1113-landings.md`.

Landing kit (ticket folder): `1113-resilience-landing/` — anchored patch scripts (`reapply-*.py`, `patch-replica-live*.py`, `apply-all.sh`, assert on every anchor, so a changed live file fails loudly), the new member and tests under `files/` (`🛡️resilience/` complete with schema, fixture, Python oracle, tests, kill-point helper; `🛡️resilience.ts` and `💥️killpoints.ts` for `🔁️replica/🧪️tests/`; the new stable entry `📜️script.ts`). Fixture generator: `🧰️scripts/1113-resilience-fixture.py`.

Incident note I added after the coordinator message: the `self-check` command now also does a read-only dry open of the current local state (`dryOpenStateV1`, new export of `🔁️replica`: loads checkpoint, log, cursors, clock, machine into throw-away tables, no lock, no write) and fails with `local state cannot be read: <reason>`, so the daemon refuses a restart into code that cannot read the state of this machine (`restart-refused`). This is in the patch scripts (`patch-replica-live2.py`, `reapply-command2.py`) but was not yet run, because the live module was not in a state the patches apply to.

Also: while building my temp copy I ran a script whose name collided with another agent's file in the shared scratchpad; it created 49 stray symlinks (`<dir>/<dir> -> <dir>`) inside the live tree under `🧰️framework` and a `node_modules` folder in the module. I removed exactly those (checked every link target) within about two minutes; they may have replicated to other machines briefly.

## Findings of `1113-audit-robustness.md` (non-git)

| # | Verdict | Rule implemented (in the kit) | Test |
|---|---|---|---|
| 1 P0 self-restart | confirmed | Fresh-process self-check (`self-check`: all modules load, every schema definition compiles, texts complete in en/de, dry open of local state) with a 60 s deadline; restart only on exit 0 and a digest equal to the pending one; failure → `restart-refused` event, `pending:{code,ok:false,reason}` in the attestation, health file, re-check only when the digest changes; deterministic per-origin jitter 0–60 s (sha256 of origin+digest), minimum uptime 30 s, wait while queue non-empty or apply in flight (cap 5 min); code digest now covers imported libraries, schemas and texts; stable entry `📜️script.ts` (node built-ins only) loads the worktree code, self-checks it and falls back to the newest sealed snapshot in `.cache/f/code/<digest>/` (sealed, last 2, mirror of the import closure) that loads; `self-check` never falls back | `🛡️resilience` 16 tests + Python oracle; `🛡️resilience.ts`: refused then fixed restart, jitter/uptime/busy, real child self-check and seal, entry fallback incl. broken snapshot and none left |
| 2 clock jump | confirmed | `mint` refuses (`ClockAheadV1`) while the own clock is more than 60 s ahead; `clock-refused` event at open | test in `🛡️resilience.ts` |
| 7 torn local state | partly confirmed | state/checkpoint torn: handled by the warm-restart agent (`StateCorruptV1`). Added: torn `cursor.json`, `clock.json`, `machine.json` read as absent with a `state-rebuilt` event; `writeJson` fsyncs file and directory (sync calls, so the race suite's injected `open` faults are unaffected) | torn-files test |
| 8 hung store | confirmed | `createStoreGuardV1`: one outstanding probe, round abandoned and aborted after 10 s, no new store call while stalled, `store-unreachable` (reason `stalled`), local loop/hints/attestation to memory/persist keep running, recovery without restart; attestation write under `guard.call`; `storeReachableV1` shares in-flight probes and has a 15 s deadline; seams `storeProbe`, `storeGate`, `storeDeadlineMilliseconds`; self-check child has a deadline and is killed | guard unit tests (hang, abandon, recover, call, unreachable) and a daemon test with hung probe and gate |
| 9 lock | confirmed | steal serialised by `daemon.lock.steal`; lock carries boot time and a 5 s heartbeat; stale after 60 s without heartbeat or another boot; legacy locks keep the pid rule | lock test (boot, silent holder, live legacy holder, 8-way steal race) |
| 10 origin uniqueness | confirmed | per-machine key in `machine.json`, recorded in `home.json` by exclusive create; other key → `OriginTakenV1` at open, join and publish; legacy home upgraded; torn machine adopts the home key | test |
| 11 fleet verdict | confirmed | `code` voted; attestation more than 30 s in the reader's future is not fresh | test |
| 12 corrupt blob trusted | partly | `writeBlobV1`/`storeBlobV1` rehash an existing blob once per process and replace a bad one; relay does the same. (The store-io agent's copy path already verifies on apply.) Receivers still wait until a writer re-saves | test |
| 13 short segment, snapshot | confirmed | append seals a short segment at its last complete line and continues in the next segment (peers still reach it); `snapshotV1` refuses while a fold needs a rejoin | tests |
| 14 closed schema | confirmed, fix direction refuted | skipping unknown events would lose them silently; instead the origin stays stalled with a distinct `unsupported event version` error, counted under kind `unsupported` | test |
| 15 auto healing | not done | needs a periodic drift check; out of time |  |
| 16 noteOnce | confirmed | per-key counter, announce first and then once per minute with `count`, lifetime count and last time per kind in status (`health`), health file and attestation | tests |
| 17 retry spin | confirmed | exponential backoff of rounds (250 ms to 30 s) | test |
| 18 growth | partly | `lost/` pruned by age and 2000 entries, noted set capped, cooling map pruned, logs rotated in place (replica log, native bootstrap log), `unstable` keyed by path. Path table never reclaimed: not done | test |
| 19 watcher | confirmed | failure reported as an `error` event (stage `watch`); not tested (cannot force an fs.watch error portably) |  |
| 22 digest | confirmed | see 1 | |
| 23 bootstrap atomicity | not done (crash between renames is repaired by the immediate rebuild when the relay restarts) |  |  |
| 24 retire torn attestation | confirmed | refused unless `--force` | test |
| 25 restart storm | confirmed | jitter, minimum uptime | tests |
| torn last journal line | refuted | readers trust only `head.bytes` and append truncates the tail; kill-point sweep covers it | `💥️killpoints.ts` |
| idempotence | confirmed holds | kill-point helper aborts at every filesystem mutation (before and after) of publish, follow, close and journal append, reopens and checks convergence and `verify` | `💥️killpoints.ts`, 4 tests (about 100 s) |

Found by the sweep in the store-io agent's `copyHashedV1`: the last pending write's error was swallowed (`finally{await pending.catch(()=>{})}`), so a failed final write could leave a truncated blob under the right name, and a failed write while the next chunk was read was an unhandled rejection. Fixed in `reapply-journal.py`, test included.

## What Bun cannot guarantee

A filesystem call blocked by a hard NFS mount cannot be cancelled and holds a worker thread until the kernel returns. The guard therefore never waits for one, issues none while stalled, and aborts the round so nothing new starts once the blocked call returns; calls already in flight when the stall is detected (up to the pool concurrency plus the probe and one attestation write) stay blocked. The hot and sweep tiers run inside the guarded round and pause while the store is stalled; hints are still collected. A process stuck in an uninterruptible mount call may not exit on a signal.

## Tests (macOS, temp copy at 05:10 based on the warm-restart version, all green)

journal 127 + store 18, boundary 39, git 36, tiers 53, attest 60, replica `🟦️.ts` 77, race 14, `💾️store.ts` 8, chaos 9 (1 skipped), `🌿️git.ts` 19, bootstrap 12, topology 56, new `🛡️resilience` 16, `🛡️resilience.ts` 19 (+1 copy test added later, not run), `💥️killpoints.ts` 4 (passed on the 04:50 base, failed on the 05:10 base because of the swallowed-write bug above; fix not re-run). Not run: any test against the live tree, Linux, Windows, the dry-open addition.

## Not done or not verified

Landing, README, contract sections, schema-document-index registration of `🛡️resilience`, `repo/README.md` module list, the dry-open self-check, finding 15, path-table reclaim, bootstrap atomicity, a real NFS stall.

## Update 06:15: still not landed, live module regressed again

The patch scripts were rebuilt against the 05:16 relocation-era live files (they applied cleanly then; `1113-resilience-landing/` is current). At 06:14 the live `🔁️replica/🟦️.ts` (sha256 31cfe1675a0f4923) and `🩺️attest/🟦️.ts` (7d42790078a283fb) are byte-identical to the "overwriting landing" versions of the 05:12 incident: they contain no `state.ckp`, `checkpointReplicaV1` or `StateCorruptV1`. Another whole-file copy happened at 06:13:39 to 06:14:08 while `claude-cross-os` holds the lock. I did not queue for the lock or land onto that base; the daemons restarting now are in the incident state. Landing needs the warm-restart and relocation versions restored first (three-way, from the stored blobs listed in the protocol).


## Update 06:55: landed (see `1113-landings.md`)

- Landed at ~06:48 under the lock, as anchored edits onto the live files of 06:44 (relocation, warm-restart state, git guards, latency tiers). Every test file of the module (22) passes against the live tree. The kill-point sweep ran against the current `state.ckp` format (4 pass), and the `copyHashedV1` last-write fix with its test is in.
- Additions from the incident: `.cache/f/code/history.json` records the digests this machine ran (first run time) and refused; code on disk that was refused or ran before and was superseded is not checked and not adopted (`code-reverted` event, `pending` in the attestation, daemon keeps its code); the service entry prefers the newest sealed snapshot over worktree code that fails the self-check or is such a revert (last resort: the worktree code when no snapshot starts). `self-check` also dry-opens the current local state read-only (`dryOpenStateV1`); a state this code cannot read refuses the restart. Rollback on purpose: delete `history.json`.
- Changes outside my files: `📍️relocate` test now accepts the machine `key`; after an outage the full walk is no longer held back by the walk duty limit.
- Verified on this Mac: the daemon restarted into the new code at 06:48 (this first restart came from the old code, which has no self-check), `.cache/f/code/e69c3a5d…/` is sealed, `history.json` and `health.json` are written, the attestation (sync off) is under a second old with code `e69c3a5dca6f`. Not yet observed: a restart through the self-check (`restart-refused` or `restarting` after a self-check), which needs the next code change; and any other machine.
- Docs: README section "Resilience", contract section "Self-restart", schema-index entry and repo README module list. The attestation line of the contract and finding 15 (auto-heal), path-table reclaim, bootstrap atomicity remain open.

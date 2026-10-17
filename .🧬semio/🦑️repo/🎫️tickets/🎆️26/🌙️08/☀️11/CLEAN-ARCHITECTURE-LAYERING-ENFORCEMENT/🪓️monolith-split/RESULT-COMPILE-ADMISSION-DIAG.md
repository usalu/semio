# RESULT: COMPILE-ADMISSION-DIAG (`cad`)

Verdict (measured 2026-10-17 13:35 to 14:21): **the launcher does not currently wait on any lease.** The probe reaches the native-receiver inspection in about 6 s and stops there with a terminal refusal, three times in a row (13:35, 14:17, 14:19). The refusal is `Pending Native image publication must settle before ordinary admission`, caused by two interrupted `.publication/intent.json` files. Behind it sits the refusal the coordinator saw, `Exact canonical persistent Native publication required`, caused by `image.json` receipts that lack `mountDirectory`. The 15 and 58 minute stalls at 07:15 to 08:29 were **not reproduced and their wait point is not proven**; see section 5.

Pid 74190 is an orphaned MCP build launcher. It is not the blocker and not live work. See section 3.

## 1. Where the launcher actually spends its time

Entry: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts` `repo` → `runOwnedBuildInvocationV1` → `openRepositoryCachePartitionV1(root,"leases")`
(`…/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:~41`) → `openNativePlatformStorageV1` for each partition → `cleanup/🎛️command/🟦️.ts:125 inspectPersistentReceiver`.

I used `bun --preload` to trace `bun:sqlite` `exec` and `prepare` in the probe process (`cad-tools/trace.ts`; logs `🗑️generated/cad-trace.log`, `cad-trace2.log`). No launcher file was edited.

- In 6.4 s of wall time the probe ran about 300 `acquireUngatedResourceLease` iterations (`🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts:201` for-loop, `BEGIN` at :208, `delay(≤40 ms)` at :230).
- It also ran about 45 queue-claim retirement checks (:290) and 26 fresh claim-database initializations (:171 to :180).
- **Zero `SQLITE_BUSY` or any other sqlite error was raised.** Every `BEGIN` and `BEGIN EXCLUSIVE` succeeded on its first attempt, so this is churn, not contention.
- After the churn, `inspectPersistentReceiver` spawns `hdiutil info -plist` through `owned-execution/📥️capture` (1 s heartbeats). It then throws at `…/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:125`.
- The `Pending` throw comes from `requireNativeImagePublicationSettledV1` (`…/🌐️platform/📋️contract/🟦️.ts:63`), which fails when `<image dir>/.publication` exists.

Lock probes (read-only `fcntl(F_GETLK)` on SQLite's PENDING, RESERVED and SHARED byte ranges, `/tmp/cad_poll.py`, 117,227 rounds over 15 s on every file in `.cache/b/c/`):

- 0 PENDING hits and 0 RESERVED hits on any file. Nobody holds or polls for an exclusive lock.
- The only SHARED holder anywhere in `.cache/b/c` is pid 74190, on `3dfb398c…sqlite` (`native-fixed-image:…/.cache/b/l`).
- An earlier 124,562-iteration read loop on that file also saw 0 busy.

## 2. Who holds what

| Resource (lease dir `.cache/b/c`) | Holder now | Mode |
| --- | --- | --- |
| `native-fixed-image:…/.cache/b/l` (`3dfb398c…`) | pid 74190 (fds 6u, 7u, two connections) | SHARED (via F_GETLK) |
| `…/b/t` (`0bde0980…`), `native-storage-mapping` (`61f8e9d5…`), admission (`d92d5a25…`), claim-catalog (`fcf48215…`) | nobody | none |
| `l/m/data/*.sqlite` (53 files) | nobody | none |

At 08:33 pid 25234 (`repo run @semio-tech/draw-js:publication-authority-audit`, started 08:21:34) held the SHARED lease on `native-fixed-image:…/.cache/b/o` (`l/m/data/a36b85bc…`) and the only queue ticket for it. That ticket parses to pid 25234 (`0000jgy` base 36). That process no longer exists.

Every `bun repo …` process holds `l` SHARED for its whole lifetime. That is the design: `openRepositoryCachePartitionV1` re-opens `leases` for every other partition, and `withExclusiveMaintenance({wait:false})` runs with `timeoutMs:0`. A busy `l` therefore returns `{acquired:false}` and the sweep is skipped. It never blocks, so `l` SHARED cannot block a launcher.

## 3. Pid 74190: stale holder or live work?

Lineage: claude `74143` (this session, started 07:14:05) → `74182 bun script.ts dev mcp stdio client` (the `repo` MCP server) → `74183` (dead) → `74189 bun repo run @semio-tech/framework-os-mcp-rs:build --output=stream` (ppid 1) → `74190`.

- `74190` started 07:14:09, 4 s after the claude process. `SEMIO_BUILD_OWNER_PID=74183`, which is dead.
- It runs the `os-mcp-rs:build` step that the `semio` MCP server in `.mcp.json` (`dev mcp stdio os …`) waits on. That server reported `CONNECT_TIMEOUT` after 30 s. This is an inference, not proven: the timing and the dead owner pid fit, but I did not read the `dev mcp stdio os` source.
- It is **not** from the fleet or the coordinator's probes. It is this session's MCP startup child, orphaned when its client timed out.
- Its stdout and stderr are unix sockets with no peer. It has no children (100 samples at 0.1 s). It holds no queue ticket and no `l/m/data` file.
- RSS stays at 31 MB, about 166 CPU-minutes over 6 h 22 min, steady 10 to 18 % CPU. Two `sample` runs show a busy event loop, but the Bun inspector's `ScriptProfiler` returned 0 JS stack traces over 5 s, so its JS call site is **unknown**.

Per the lease code's own rules there is **no heartbeat, expiry or staleness notion**. A lease is an OS-level SQLite lock on an open file descriptor and lives exactly as long as the process. So 74190 counts as a live holder, but it holds only SHARED on `l`. That blocks nothing but `l` exclusive maintenance (`wait:false`, tolerated). Its work is moot: the MCP client is gone and nobody consumes its output.

## 4. Current terminal refusals (the coordinator's question)

Probes, each under `cargo-mutex.lock` with `perl alarm 1800`:

- 13:35 (`🗑️generated/cad-probe2.log`), 14:17 (`cad-probe3.log`, 6.4 s), 14:19 (`cad-probe4.log`): `error: Pending Native image publication must settle before ordinary admission`.
- 08:45 (`cad-probe.log`, old monolithic source, since refactored): `error: Exact canonical persistent Native publication required`.

State on disk:

| Partition | `image.json` | has `mountDirectory` | `.publication/intent.json` |
| --- | --- | --- | --- |
| `l` | 187 B | yes | no |
| `o` | 193 B | yes | no |
| `t` | 126 B | **no** | **yes**, 2026-10-17 08:43:17 (`current-publication`, `initialState:"mounted"`, `receiptSha256 528dcaf2…`) |
| `d` | 126 B | **no** | **yes**, 2026-10-17 11:27:10 (`receiptSha256 1a1ceecb…`) |
| `g` | 126 B | **no** | no |
| `p` | 129 B | **no** | no |
| Rust fixed (`.🧬semio/🦑️repo/⚡️cache/rust/fixed`) | 129 B | **no** | no |

The `ImageReceipt` schema (`…/💾️storage/🧬️schema/🔣️.json`) lists `mountDirectory` as `required`.

**Answer: the current first refusal is `Pending Native image publication must settle…` for `t` and `d`. Once those two `.publication` directories are settled, `Exact canonical persistent Native publication required` is the next terminal refusal.** It follows from the schema check and receipt sizes; I have not run past the first refusal. It is the refusal that stops `t`, `d`, `g`, `p` and the Rust fixed image alike, because their receipts have no `mountDirectory`.

The `.publication` intents were not written by the launcher. No non-test code outside `📤️publication/**` calls `publishNativeImagePublicationV1` or `publishNativeImageRetainedPublicationV1` (grep). They come from a publication run, probably the peer ticket's `native-publication` test scripts in `.vscode/launch.json` around lines 1121 to 1259. The 08:43:17 timestamp is 27 s after my first probe began (08:42:50); I cannot rule out coincidence with my own run, but the launcher has no call path into the publication owner.

## 5. What I could not establish

- The earlier 15 and 58 minute waits (coordinator probes 07:15 to 07:30 and 07:30 to 08:29). My first probe (08:42:50) did not fail fast either: it ran about 2.4 min and then died with the `Exact canonical…` error at 08:45:17. That is a long delay, but it ended in a refusal, not an indefinite wait.
- Candidate explanations, none proven: (a) FIFO queued-lease waits (`leases/🟦️.ts:347` to :369, 250 ms polling) behind long peer launchers, since removed; (b) CPU starvation. The machine is at load average about 60 with about 900 % total CPU, and the lease phase alone costs about 30 % of a core for 6 s on an idle fleet.
- JS-level location of 74190's spin. `Debugger.pause` produced no `paused` event and the profiler returned no samples, so I could not read its stack. I did not use a signal or `kill`.

## 6. Sanctioned way forward (decisions for the user)

1. **Pid 74189/74190** (orphan MCP build): not mine, not the blocker. It can be ended without affecting the lease protocol (the lock dies with the process). Ending it requires the user, for example `kill 74190 74189`. I did not signal it. Do this only if the `semio` MCP server is not wanted from this session; it will not recover on its own.
2. **Launcher unblock** is not a lease wait. It needs the peer `MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES` owner (publication owner, `…/💾️storage/📤️publication/🎛️owner/🟦️.ts`) to settle or recover the interrupted publications on `t` and `d`, then publish current receipts with `mountDirectory` for `t`, `d`, `g`, `p` and the Rust fixed image. No hand edit of `image.json` or `.publication` is allowed. I found no repo command that does this for the live images; the existing `native-publication` commands in `.vscode/launch.json` are the peer's test drivers on temporary images. The decision on which peer slice runs it is the user's or the coordinator's.
3. The earlier note that the fixed Rust partition lacks the 8 GiB initial headroom remains a separate, later gate (`Insufficient physical compiler initial headroom`) that this diagnosis did not reach.

Probe hygiene: all runs held `🗑️generated/cargo-mutex.lock` (owner `cad`) and released it; the lock is free now. No process of mine is running. Tools and logs are only `cad-tools/*` and `🗑️generated/cad-*`.

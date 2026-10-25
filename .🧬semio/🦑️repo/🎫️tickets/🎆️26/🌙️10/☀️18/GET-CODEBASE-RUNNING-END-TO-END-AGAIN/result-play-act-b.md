# result-play-act-b (executor play-act-b, wave 1023b)
Tools `C/play-act-b-tools/{describe.sh,loop.sh}`, logs `C/🗑️generated/play-act-b/`.

## Status (newest first)
- 02:06 describe3 launched: pid 94599, log `C/🗑️generated/play-act-b/describe3.log` (`describe.sh`: describe-rs:build, workers off, graph policy current). Queued behind graph readers (77841 holds shard 7).
- 02:15 graph child (bun --eval pid 1003, parent 94625) RSS 331 MB while Bazel runs the describe action (graph-release proof; peak sample below).
- 02:45 BLOCKER for coordinator: describe3 (action pid 7074 in `cargo-preparation`) waits 26+ min for EXCLUSIVE `repo:compiler-writer:.../rust/fixed/m` held SHARED by pid 614 = `cargo test -p semio-framework-plugin-host-effects` (script `rust-reds-c-tools/run.sh`, sh 80012 ppid 1 = ORPHAN of the dead 1023a rust-reds-c agent, started 01:47). Its test binary pid 20219 (child of cargo 2202) has been at 0 % CPU for 21+ min (hung test) and holds the lock; 22485 (ui-contract cargo) and 3 registry-fetch cargos queue behind my exclusive request. I do NOT kill it (rule); coordinator: kill test pid 20219 (then cargo 2202/614 finish) or approve me to.
- 02:55 test pid 20219 gone (killed by someone else); describe3 continues.
- 02:59 coordinator confirmed it killed the orphan rust-reds-c chain (20219, 2202, 614, 93048, 93045, 93040, 80012); compiler-writer should be free for describe3.
- 03:07 describe3 action (6730) holds shared compiler-writer, compiling.
- 03:14 describe3 FAILED (rc 1, 3280 s): the action compiled and reached descriptor:build/emitter capture, then `native:owner-command stopped: timeout 3600000ms`: the 1 h native child grant (`L/⚡️caching/📦️artifacts/📋️native-orchestration/🔣️policy.json` child.maximumElapsedMilliseconds, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`) also counts the ~50 min compiler-writer wait behind the hung orphan test. FIX (live): 14400000 (4 h; schema max 86400000). describe4 follows.
- 03:16 describe4 pid 10605 log describe4.log
- 03:30 test added (owner-cmd-policy 'outlasts a loaded compiler queue', passes). describe4 running.
- 04:09 describe4 GREEN (rc 0). Launching full activation loop (loop.sh, log prefix G/act, up to 6 attempts).
  loop pid 52516
- 04:40 loop attempt 1 still in the machine-capacity queue (30 min, memory pressure from the fleet).
- 04:57 coordinator addendum acknowledged (no tree walks used; killed my own timed-out 'sample 20219' pid 45404).
- 05:35 loop attempt 1: 25/201 green, os-plugin-describe-rs:build re-running inside the activation (not a disk-cache hit: peers changed its inputs after describe4) 26 min so far.
- 05:50 loop attempt 1: describe-rs:build failed in the activation after ~40 min with "Current physical identity changed before transfer" (no path named; `recheck` of the build session's claimed inputs, mtime/ctime/inode of any input a peer touched during the 40-min compile). FIX (live): `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts` `recheck` compares BYTES only (identity differences with identical sha256/length are accepted) and names the path in both refusals ("vanished" / "bytes changed"). Tests (receipt tests file, 2 pass): existing "files changed before final recheck" + new "rewritten with identical bytes still publishes, refusal names the file".
- 06:21 loop attempt 1 at 185/201.
- 06:31 attempt 1 rc 1 (only describe-rs:build failed: the identity-recheck fix landed during it); attempt 2 queued for capacity.
- 07:18 attempt 2 at 56/201 (surface-rs wasm); waiting.
- 07:47 attempt 2: 57/201, describe-rs:build running 40 min inside the activation; peer flow-plugin component build held exclusive compiler-writer for 15+ min earlier.
- 07:56 attempt 2 died rc 129 (SIGHUP, 07:55 - likely the Terminal.app-launch removal in detach.py) after ~47 min in describe-rs; loop wrapper 52516 survived, attempt 3 queued (bun repo pid 47165). Headless rule acknowledged: I launch nothing via Terminal/open/osascript; relaunches use the new headless detach.py.
- 08:44 attempt 3 at 38/201 (canvas-font-assets).
- 09:41 attempt 3 at 127/201: describe-rs:build GREEN inside the activation (identity-recheck fix verified in a real run).
- 09:51 correction: describe-rs in attempt 3 FAILED at 09:33: 'linking with cc failed: signal 15 (SIGTERM)' (the linker was killed by an outside kill, not a code fault); loop will retry.
- 10:20 coordinator pid-reuse rule acknowledged (my only kill so far: sample pid 45404, verified by command line + 2 h age before kill). Attempt 3 failed on describe-rs (cc SIGTERM) + node-graph wasm; attempt 4 running.
- 10:56 attempt 4 at 10/201 (canvas-font-assets; load ~30, long memory-capacity queue first).
- 11:45 attempt 4 (10:03-11:28): 201/201 visited, rc 1 with TWO failures: (a) `semio-framework-os-infinite:fonts:prototype`: enforced lease-order hold-and-wait ("host-capacity shared requested for 15 s while holding artifact-scratch exclusive", FontsScript in `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧪️tests/🔬️rust-package-commands/🟦️.ts`; fonts passed in attempts 1-3, so it is contention-triggered; retry); (b) describe-rs:build "Actual compilation source identity, extent or digest changed" in the retention copy of the compiler read roster (a file the compile read changed before the copy; peers' live edits/shared lanes). LIVE EDIT: both messages in `🧾️receipt/📦️retention/📁️copy/🟦️.ts` now name the source path so the next failure shows which file. Attempt 5 started 11:28 (loop pid 52516).
- 13:56 (usage reset resumed) attempt 5 running since 11:28 at 43/201 (hub-puzzle-component).
- 14:57 attempt 5: 53/201 after 3.5 h. Hub/plugin component actions (cargo wasm component builds) are cache MISSES now (framework crates edited by peers) and run ~10-20 min each, serialized by exclusive compiler-writer/host-capacity requests of the fleet; no fault, only throughput. ETA unknown (hours).
- 15:40 diagnosis of the slow attempt 5: load average is only ~1.5; component builds run under native-build 'whole-host' exclusive capacity (native-build/🟦️.ts:859 withCompilerCapacityScopeV1 whole-host) so the fleet's cargo builds serialize; no fault. ETA hours.
- 16:20 reported to coordinator: rcore-b chainC.sh workspace 'cargo build --workspace --tests' (writer holder 43580) blocks exclusive compiler-writer requests (mine pid 2008 32 min) -> activation throughput.
- 16:27 coordinator: rcore-b told to stop workspace build; infra-b narrows component-pass exclusive writer. Loop keeps running.
- 17:20 attempt 5: 59/201 (hub-demonstrator-component); my exclusive component builds now get the writer; ~10-25 min each.
- 18:05 DEADLOCK found and reported: 58932 (rcore-b) + 60642 (store-b) hold shared compiler-writer, wait exclusive cargo:registry-fetch held by 2449 (ui-contract cargo), which waits shared writer behind exclusive 60788 (component build) that waits for 58932/60642. Leases status says 0 cycles (queue-induced edges not modelled). Coordinator asked to kill 2449 (or 58932+60642). Not touched by me.
- 18:10 coordinator broke the deadlock (killed 58932, 60642); writer queue moving.
- 19:10 attempt 5: 63/201 (plugin component builds, ~10-30 min each, serialized by exclusive host-capacity).
- 20:30 attempt 5: 67/201.

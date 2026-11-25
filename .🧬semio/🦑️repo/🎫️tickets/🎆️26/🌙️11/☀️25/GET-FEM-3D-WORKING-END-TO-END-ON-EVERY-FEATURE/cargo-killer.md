# 🔪️ What Kills The Cargo Builds (2026-11-25)

Read-only investigation. Nothing was killed or modified.

## Verdict

No single culprit is proven. The killer is a user-space process that calls `taskkill /F` or `TerminateProcess` (Windows exit code 1, matching the observed rc=1 with no compiler error). The OS did not kill them: no crash, resource-exhaustion or WER events exist in the window.

Most likely candidate: the Windows branch of the slice-pool reaper, which kills new children of the service host when no slice load is running. It is the only code path found that targets processes it did not start by default. It is not confirmed on this machine, because the live cargo trees now have orphaned ticket `bash` roots, so their parent PIDs cannot be checked against the service PID.

## Ranked candidates (code)

1. Slice-pool reaper, Windows scope rule
   - `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🏡️resident\🏗️composition\🌐️host\👷️slice-pool\🟨️.mjs:35`: on `win32`, `match` selects every row with `ppid === process.pid` and not in `load.baseline` while `running.size === 0`. The comment at `reap\🟨️.mjs` line 9 documents this as "the new children of the service while no other load runs".
   - Same file lines 36-37: `reaped` (on load end) and `sweepAtExit` (on service exit) both call `reaper.reap` or `reaper.killNow`.
   - `...\slice-pool\🧹️reap\🟨️.mjs:37`: `execFileSync("taskkill",["/PID",pid,"/T", "/F"])` on SIGKILL. Victims come from `victims()` at line 50-56.
   - Fits: kills a whole tree, rc=1, several at once when a load ends. Gap: needs cargo's parent PID to equal the service PID at the time.

2. Job limiter tree kill
   - `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🧱️job-limits\🟦️.ts:71`: `spawn("taskkill",["/pid",pid,"/t","/f"])`.
   - Called at lines 83 (memory breach, `onBreach`) and 84 (abort). Any limited job whose tree a cargo run belongs to dies together. Fits the rc=1 signature. Not verified whether the ticket cargo runs go through this limiter.

3. Process-tree stage stop
   - `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🌐️remote-execution\🛰️sidecar\🚦️ready\🌲️process-tree\🟦️.ts:466`: `taskkill /PID <pid> /T /F` for each planned pid (`readyKillTreeV1`, line 461-466). Runs when a stage is stopped (for example daemon or replica stop).

4. Build executor terminate
   - `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🔧️tools\🐙️semio\🚀️exec\🟦️.ts:43`: `taskkill /pid <pid> /T /F` on abort of a task. Only relevant if a `repo` task is an ancestor of the cargo runs (none found).

## Ruled out

- Fleet daemon `C:\Users\Ueli\AppData\Local\semio\fleet\entry\boot.mjs`: kills only its own child with `proc.kill` (lines 116, 141-144, 153-163). On Windows that is not a tree kill and does not reach cargo.
- Host-capacity warden (`...\🌡️host-capacity\🛡️warden`): its policy has a `cargo` kind with `cancel`/SIGTERM (rank 65, tree scope), but `warden.log` says `platform win32 cannot suspend processes: observe mode only` on every start, the latest `state.json` shows `"mode":"observe"`, `warden\cancelled` is empty and unchanged since 2026-11-24 03:22, and there are no cancel lines in the log.
- OS: `Get-WinEvent` System 19:08-19:22 returns no events; Application shows only Software Protection events (19:16:33, 19:17:03). No event 2004, no Application Error, no WER.
- Scheduled tasks (Ready state): only `semio-fleet-compact` and `semio-fleet-store-mount` (via `quiet.vbs`). Neither kills processes.
- Ticket folder `...\🎫️tickets\🎆️26\🌙️11\☀️25`: no `taskkill`, `Stop-Process`, `pkill`, `killall`, `kill` or `timeout` in `*.sh`, `*.ps1`, `*.mjs`, `*.ts`.

## Timing correlation (not proven)

- Fleet daemon graceful reloads (`C:\Users\Ueli\AppData\Local\semio\fleet\entry\state.json`, `attempts`, local time): 17:35:54 and 19:06:14.
- Cargo failures: about 17:45 and 19:16. Each is roughly 10 minutes after a reload. The mechanism behind the 10-minute gap was not found.
- The replica service repeatedly logs `restarting` / `stopped` / `started` in `run.log` with `schtasks /Create failed for semio-fleet-replica`. A stop of that tree would reach candidates 3 and 1.

## Live state at the time of the check

- Many cargo/rustc runs are active (started from 19:17 onwards). Their roots are `bash.exe` ticket scripts (`gb-engine.sh`, `cp-law.sh`, `im-law.sh`, ...). Some roots are already dead (parent PID gone), so the chain cannot be walked further.
- No `service host run` process and no replica process are running at the check time.

## Next steps (no kill needed)

1. Enable process-creation auditing (Security 4688 with command line) and look for `taskkill` invocations at 17:44-17:46 and 19:15-19:17.
2. Add a `[DEBUG] ` log line in `slice-pool\🟨️.mjs:37` before the `reap` and in `job-limits\🟦️.ts:71` before the `taskkill`, then rerun one build.
3. Check whether the ticket cargo runs go through `job-limits` (look for a `LimitedJobV1` caller in the run path).

## Files read

- `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🏡️resident\🏗️composition\🌐️host\👷️slice-pool\🟨️.mjs`
- `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🏡️resident\🏗️composition\🌐️host\👷️slice-pool\🧹️reap\🟨️.mjs`
- `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🧱️job-limits\🟦️.ts`
- `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\🏗️build\🌐️remote-execution\🛰️sidecar\🚦️ready\🌲️process-tree\🟦️.ts`
- `C:\semio\🧰️framework\🔨️modules\🏃️process\🏗️build\🌡️host-capacity\🛡️warden\🔁️loop\🟦️.ts`
- `C:\semio\🧰️framework\🔨️modules\🏃️process\🏗️build\🌡️host-capacity\🛡️warden\🧬️schema\🔣️.json`
- `C:\Users\Ueli\AppData\Local\semio\host-capacity\warden\warden.log`, `state.json`
- `C:\Users\Ueli\AppData\Local\semio\fleet\entry\boot.mjs`, `state.json`, `run.log` (read-only, shared access)

## Resolution (coordinator, 19:43)
`kill-watch.ps1` logged every `taskkill` start with its parent chain (`🗑️generated/kill-watch.log`): all of them are `taskkill.exe /PID <n> /T /F` launched by the Claude desktop app (`Claude.exe --utility-sub-type=node.mojom.NodeService`), i.e. the app stopping background tasks or timed-out commands. `taskkill /T` walks `ParentProcessId`, which Windows never clears, so a reused PID links unrelated native cargo trees into a stopped task's tree and kills them silently (rc=1, no compiler error).
Mitigation: start long native cargo runs fully detached from agent shells (`powershell Start-Process -WindowStyle Hidden` writing their own log and `rc=` line), never as children of a Claude Bash task; `wasm-retry.sh` is started that way.

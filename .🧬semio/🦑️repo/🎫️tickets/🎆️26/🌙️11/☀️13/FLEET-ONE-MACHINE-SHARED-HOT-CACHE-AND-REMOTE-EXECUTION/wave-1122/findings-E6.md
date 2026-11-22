# Findings E6: g1-m05 (Windows) needs one daemon restart to reach the new code

## Why a person is needed once
m05 runs the old replica daemon code `e092f50c`. That code has the guard bug (one hung store call blocks every later store call), so its apply queue stood still at 10,560 incoming entries since 00:42Z and its worktree never receives the new module files. The old code has no way out; the new code (store guard fix, bounded wait, transient refusals, stuck-store restart, proven store offer first, module files first) has to arrive once. A restart of the replica daemon frees the hung call and lets the queue apply.

## Do NOT
- Do not stop or kill `bun.exe` processes in general: the Ready stage `dependencies` (the dependency install) runs on bun.exe and would be aborted.
- Do not reboot, do not delete `C:\semio\.cache`, `C:\semio\.cache\f` (replica state), the worktree, or anything in the store.
- Do not run git commands that change the tree (no checkout/reset/clean/stash).

## Steps (Windows PowerShell 5.1 or 7, as the user that runs the fleet task, no elevation needed)
1. Look first (read only):
   Get-ScheduledTask -TaskName semio-fleet-replica | Select-Object TaskName, State
   Get-Content C:\semio\.cache\f\health.json
   Get-Content C:\semio\.cache\f\boot\run.log -Tail 40
   Expected now: State Running; health.json has "store":"stalled" or an old wallMs; the log tail shows nothing new or repeated "Store call refused". (If the worktree is not C:\semio, use its path; the folders .cache\f\health.json and .cache\f\boot\run.log are the same below the worktree.)
2. Stop only the replica task:
   Stop-ScheduledTask -TaskName semio-fleet-replica
   Wait 15 seconds. The daemon may not stop gracefully while a store call hangs; that is fine, its state log (state.wal) is replayed at the next start.
3. Check that no replica process is left (read only):
   Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -match 'boot\.mjs|boot\.ps1|shared-worktree' } | Select-Object ProcessId, Name, CommandLine
   If a process with boot.ps1/boot.mjs/shared-worktree in its command line is still there after 30 s, stop exactly that process id with Stop-Process -Id <id>. Leave every other bun.exe (build and Ready processes) alone.
4. Start it again:
   Start-ScheduledTask -TaskName semio-fleet-replica
   (The task also restarts itself every minute after a clean exit, so a missed start is repaired.)

## What you should see afterwards
- Within 1 to 2 minutes: health.json shows a fresh wallMs and "store":"ok"; on another machine S/origins/g1-m05/attest.json has a fresh wallMs (every 30-60 s) and ready.json keeps updating.
- Within 5 to 30 minutes (the machine is busy with the dependency install, so slow): queue.incoming in attest.json falls from about 10,560 towards 0, the blocked-target-sharing-eperm counter stops growing, and the code digest changes from e092f50c to the digest the rest of the fleet runs (39529dcd at 00:50Z or newer). The daemon restarts itself once for that; the log shows "restarting".
- The Ready stage stays `dependencies` until the install finishes; it must not restart because of this.
- A new "[DATA] Store dependency installer stderr" line in ready-log-dependencies-failed.txt appears only if the install fails; that line holds the real error.

## If it does not work
Do not wipe anything. Copy C:\semio\.cache\f\health.json, the last 200 lines of C:\semio\.cache\f\boot\run.log and C:\semio\.cache\f\boot\beat.json to the ticket folder and report. If incoming has not fallen after 30 minutes and health.json says "store":"ok", the queue is still applying slowly under the install load; wait before repeating.

## Caveat
The task name semio-fleet-replica and the paths come from the provisioning script (`🗺️topology/🥾️provision/🔵️.ps1`) and the README; they were not run on m05 by this lane.

# Entry Prompts: the one privileged step per machine that already joined (2026-11-13)

Background: `1113-result-entry.md`. After the entry landing every healthy daemon installs the boot program into the account's own data directory and the `📜️script.ts` trampoline hands over to it, without any privilege. What no daemon can change is the service definition itself (root-owned LaunchDaemon plist, systemd unit): it still runs `repo ./📜️script.ts run` inside the worktree, so the trampoline file, the `repo` launcher and the working directory remain single points of failure until the definition runs the shim directly. This is the exact one-time step. It is optional for machines that are healthy, and recommended for every machine. Nothing here touches git; do not use `pkill` or `killall`; do not edit `AGENTS.md`.

Facts used below: `<account>` is the login account of the replica, `<home>` its home directory, `<worktree>` `/Users/Shared/semio` (macOS server and workers that never relocated), `/srv/semio` or `~/semio` (Linux), `C:\semio` (Windows). The entry directory is `<home>/Library/Application Support/semio/fleet/entry` (macOS), `<home>/.local/share/semio/fleet/entry` (Linux), `<home>\AppData\Local\semio\fleet\entry` (Windows).

## Precondition check (any OS, no privilege)

The new code must have run healthily once, otherwise the entry directory does not exist yet:

```
ls "<entry directory>"                 # boot.mjs boot.sh boot.ps1 worktree entry.json (state.json after the first start through the shim)
cat "<entry directory>/entry.json"     # bootVersion 1, worktree <worktree>
```
If `entry.json` is missing after 15 minutes of healthy uptime, read `<worktree>/.cache/f/health.json` for a failure of kind `entry-boot` and stop; do not continue with the steps below.

## (a) macOS with a system LaunchDaemon (the server, workers provisioned before the LaunchAgent change)

```
You are on a macOS machine of the semio fleet whose replica runs as the system LaunchDaemon tech.semio.fleet.replica. Switch its definition to the installed boot program. Use sudo for the commands marked sudo, nothing else. Do not use git, pkill or killall, do not edit AGENTS.md.

1. ENTRY="<home>/Library/Application Support/semio/fleet/entry"; confirm "$ENTRY/boot.sh" and "$ENTRY/entry.json" exist (precondition check above).
2. PLIST=/Library/LaunchDaemons/tech.semio.fleet.replica.plist; sudo cp "$PLIST" "$ENTRY/daemon.plist.previous"  (the way back).
3. sudo /usr/libexec/PlistBuddy \
     -c 'Delete :WorkingDirectory' \
     -c 'Delete :ProgramArguments' \
     -c 'Add :ProgramArguments array' \
     -c 'Add :ProgramArguments:0 string /bin/sh' \
     -c "Add :ProgramArguments:1 string $ENTRY/boot.sh" \
     -c 'Add :ProgramArguments:2 string run' "$PLIST"
   plutil -lint "$PLIST"   # must print OK
4. sudo launchctl bootout system/tech.semio.fleet.replica; sudo launchctl bootstrap system "$PLIST"; sudo launchctl kickstart -k system/tech.semio.fleet.replica
5. Wait 90 s, then: cat "$ENTRY/state.json" (phase running, source worktree), and from <worktree>/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica run: repo ./📜️script.ts status --lang en  (daemon running, synchronisation on) and repo ./📜️script.ts fleet --wait 120 --lang en.
6. If the daemon is not running after 3 minutes: sudo cp "$ENTRY/daemon.plist.previous" "$PLIST"; sudo launchctl bootout system/tech.semio.fleet.replica; sudo launchctl bootstrap system "$PLIST"; report the last 40 lines of /var/log/semio-fleet-replica.log.
7. Write what you did and the outputs to <worktree>/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/<originId>-entry.md.
```
macOS accounts that own a LaunchAgent (relocated machines, `~/Library/LaunchAgents/tech.semio.fleet.replica.plist`) need nothing: the daemon rewrites the plist itself (`migrateAgentV1`), launchd reads it at the next login or `launchctl bootout gui/<uid>/tech.semio.fleet.replica` plus `bootstrap`.

## (b) Linux with the systemd unit

```
You are on a Linux machine of the semio fleet whose replica runs as the systemd unit semio-fleet-replica@<account>. Switch its definition to the installed boot program. Use sudo only where marked. No git, no pkill or killall, do not edit AGENTS.md.

1. ENTRY="<home>/.local/share/semio/fleet/entry"; confirm "$ENTRY/boot.sh" and "$ENTRY/entry.json" exist (precondition check above).
2. UNIT=/etc/systemd/system/semio-fleet-replica@.service; sudo cp "$UNIT" "$ENTRY/unit.previous"
3. sudo sed -i -e '/^WorkingDirectory=/d' -e "s|^ExecStart=.*|ExecStart=/bin/sh $ENTRY/boot.sh run|" "$UNIT"; grep -n 'ExecStart\|WorkingDirectory' "$UNIT"   # one ExecStart with boot.sh, no WorkingDirectory
   (equivalent: sudo bash <copy of 🗺️topology>/🥾️provision/🐚️.sh <group> <origin> --user <account>, run from a temporary copy outside the worktree with the exports of the join; it installs the boot program and renders the same unit)
4. sudo systemctl daemon-reload; sudo systemctl restart semio-fleet-replica@<account>
5. Wait 90 s: systemctl is-active semio-fleet-replica@<account>; cat "$ENTRY/state.json"; from <worktree>/…/🔁️replica: repo ./📜️script.ts status --lang en; repo ./📜️script.ts fleet --wait 120 --lang en.
6. On failure: sudo cp "$ENTRY/unit.previous" "$UNIT"; sudo systemctl daemon-reload; sudo systemctl restart semio-fleet-replica@<account>; report journalctl -u semio-fleet-replica@<account> -n 80.
7. Write the record to <worktree>/.🧬semio/…/SHARED-WORKTREE-FLEET-OVER-NFS/<originId>-entry.md.
```
Check the restart settings in the unit as well: `Restart=always` with `RestartSec=2` must not trip `StartLimitBurst` (default 5 starts in 10 s); the shim itself never exits fast, but add `StartLimitIntervalSec=0` to `[Unit]` if `systemctl status` ever reports `start-limit-hit`.

## (c) Windows (no elevation, run as the worker account in PowerShell 7)

```
You are on a Windows machine of the semio fleet whose replica runs as the scheduled task semio-fleet-replica. Switch its definition to the installed boot program. Run as the worker account (the task is registered for it; no elevation is needed). No git, do not edit AGENTS.md.

1. $entry = Join-Path $env:LOCALAPPDATA 'semio\fleet\entry'; confirm boot.ps1, boot.mjs and entry.json exist there (precondition check above).
2. $xml = ((repo C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🤝️shared-worktree\🗺️topology\📜️script.ts worker-service --group <group> --platform windows) -join "`n").Replace('__WORKER_HOME__', $env:USERPROFILE)
   Export-ScheduledTask -TaskName semio-fleet-replica | Set-Content "$entry\task.previous.xml"
   Register-ScheduledTask -TaskName semio-fleet-replica -Xml $xml -User "$env:USERDOMAIN\$env:USERNAME" -Force
3. Stop-ScheduledTask semio-fleet-replica; Start-ScheduledTask semio-fleet-replica
4. Wait 90 s: Get-Content "$entry\state.json"; from C:\semio\…\🔁️replica: repo .\📜️script.ts status --lang en; repo .\📜️script.ts fleet --wait 120 --lang en.
5. On failure: Register-ScheduledTask -TaskName semio-fleet-replica -Xml (Get-Content -Raw "$entry\task.previous.xml") -User "$env:USERDOMAIN\$env:USERNAME" -Force; report $entry\shim.txt and $entry\state.json.
```

## After the step (all OS)

`cat <entry directory>/state.json` shows `phase: running` and the code source; `<entry directory>/shim.txt` appears only when the shim could not run a program. A broken `📜️script.ts`, a broken static import, a missing `repo` resolution inside the worktree and an unreachable store now end in `state.json` phase `exhausted` or `waiting` with the reason, and the daemon starts by itself as soon as any code runs.

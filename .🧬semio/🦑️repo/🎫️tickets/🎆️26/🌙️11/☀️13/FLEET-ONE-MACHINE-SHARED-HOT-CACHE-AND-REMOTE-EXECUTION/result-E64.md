# E64 result: service definitions that change without privilege

## Decision
Per platform, the service manager only ever holds a FIXED bootstrap; everything that varies (command, scheduling class, environment) lives in a file the service account owns and rewrites itself.
- macOS: already user LaunchAgents (E27 reload helper). Now also cache and relay agents of a server (`replica`, `compact`, `cache`, `relay` in one swap).
- Windows: already per-user tasks (`LeastPrivilege`); `schtasks /Create /F` needs no elevation. Now also cache and relay tasks (End + Run after the re-register).
- Linux (the only platform where a system unit blocked self-update): the system units become fixed bootstraps: `User`, `AmbientCapabilities=CAP_SYS_NICE`, restart policy, limits, mount ordering, and one `ExecStart=/bin/sh "<entry>/service/<name>.sh"`. The scripts (`replica`, `compact`, and on a server `cache`, `relay`) are rendered from the topology, owned by the account, carry the declared scheduling class (`renice`, `chrt --idle`, `ionice`; allowed by the ambient capability) and the environment/working directory, and end in `exec`. Rejected alternative: `systemd --user` + linger (still needs one privileged `enable-linger`, no negative nice, no `RequiresMountsFor` of root mounts).

## What changed (files below `🔨️modules/`)
- `📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🪜️priority/🟦️.ts`: `systemdPriorityLinesV1` replaced by `shellPriorityLinesV1` (schema text updated).
- `🤝️shared-worktree/🗺️topology/🟦️.ts`: Linux `definition`, `compact-definition`, `cache-unit`, `relay-unit` fixed; new parts `launch-target/-definition`, `compact-launch-target/-definition`, `cache-launch-target/cache-launch`, `relay-launch-target/relay-launch`; `linuxLaunchDirectoryV1`.
- `🛠️service/🟦️.ts` (+ schema texts EN/DE): plan = units + Linux launch scripts for ALL roles; a changed script is linted (`sh -n`), written atomically (0755) with `.previous`, the affected service is restarted by SIGTERM to its `MainPID` (cache, relay, replica; `Restart=always` brings it back; no privilege, no reload); a unit that differs and is not writable is reported once as `needs-privilege` with the exact command (scripts already in place); macOS server agents and Windows server tasks join the same debounce/restore flow; `serviceBootstrapScriptV1`.
- `⌨️command/🟦️.ts`: `service-bootstrap --group <g> --user <u> [--role server]` prints the one privileged script.
- `🥾️provision/🐚️.sh`: installs the launch scripts as the account (`install_launch`) next to the units.
- `fill` no longer XML-escapes values on Linux (`&` in a home path).

## Proof
- `repo test-file` priority 11/0, topology main 72/0, provision 22/0 + extraction 8/0, `🛠️service` 12 pass / 2 fail (see below).
- New tests: launch script change (debounce, lint, atomic 0755, previous kept, kill of MainPID, no reload, no unit write); rejected script; unmigrated unit -> `needs-privilege` with `service-bootstrap ... | sudo sh` and scripts already written; writable unit + failing reload restore; Linux server cache/relay/replica restart order and `--collect-blobs` kept; macOS server swap of 4 agents with env block kept; Windows server cache/relay re-register + End/Run; `service-bootstrap` executed against stubbed `id/getent/install/systemctl` (units + launch scripts written, mode 0755, enable/restart lines, idempotent, bad account and wrong server platform refused).
- Language-agnostic oracle: `🛠️service/🐍️.py` (python: configparser reads the unit, checks no class/env keys, `CAP_SYS_NICE`, ExecStart shape; reads the priority schema JSON itself and checks the renice/ionice/chrt lines; `sh -n`; exec last line). Negative case rejected.
- NOT green: the 2 real-launchd tests ("helper job swaps a running LaunchAgent", "plist that does not run is replaced") time out after 90 s at load 35-40 in all four runs, also before the Linux edits; the helper script they exercise is unchanged by this lane. Treat as load/environment until rerun on an idle host.

## Needs user
Only Linux machines whose system unit is still the old definition need ONE privileged step; after it no service change ever needs privilege again. From `repo fleet status` / `S/origins/*/attest.json` the only Linux machine is **g1-m04** (user `deck`, checkout `/home/deck/semio`, platform linux, code 5b85cc7f, last attest ~30 min old). macOS (g1-m01..m03, m06..m08, m10..m13, g1-server) and Windows (g1-m05) need nothing; g1-m09 has no origin (never ran a daemon, nothing to re-provision from here).
On g1-m04, after its replica has applied this code (the daemon then logs `service definition differs ... service-bootstrap` once), run:
```
cd /home/deck/semio && repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🗺️topology/📜️script.ts" service-bootstrap --group g1 --user deck | sudo sh
```
It installs the four fixed units + timer, writes the launch scripts as `deck`, reloads systemd and restarts replica (a few seconds gap). Idempotent. Read it first with `... service-bootstrap --group g1 --user deck | less`. A future Linux group server uses `--role server`.
Windows m05 still needs the one manual replica restart of `findings-E6.md` (old code, outside this lane).

## Not verified / notes
- Linux path only with injected runners, stubbed system tools and python oracle; never on a real systemd (`systemd-analyze verify` unavailable). `AmbientCapabilities` needs systemd >= 229; `renice -n -5` of the shell needs the capability, otherwise the line is a no-op and the process stays at 0 (class applied by role code as before).
- A restart by `kill -TERM MainPID` assumes the daemon and the service run as the same account (true for `User=%i`/`semio`).
- macOS g1-server will re-render its cache/relay agents when it runs this code (it still runs an older digest); the helper swap for four agents was proven with the fake, the real-launchd tests are the red ones above.
- Live fleet untouched; no sudo, nothing restarted elsewhere.

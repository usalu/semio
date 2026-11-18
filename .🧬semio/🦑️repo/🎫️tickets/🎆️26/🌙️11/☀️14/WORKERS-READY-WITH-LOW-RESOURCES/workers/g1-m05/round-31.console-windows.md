# g1-m05 (Windows): The Build Agent Flooded the Desktop With Terminal Windows; Source Changed

Written 2026-11-17T20:50Z by the agent session on the machine, on the owner's order ("this machine is constantly starting new terminal windows, make sure that it doesn't flood"). Unlike my other notes this one reports changes: 13 edits in shared source, two test files, and one restart of the build agent's processes on this machine. No setup command, nothing under `.cache/f/ready` touched, the running `tools` try (wrapper 32372) not touched.

## What Was Seen (UTC, After the Reboot of 20:13)

- Windows Terminal is the default terminal here, so every new console becomes a tab with its own `OpenConsole.exe`. Count of `OpenConsole.exe`: 13 at 20:23:55, 29 at 20:26, 45 at 20:28; bursts of 8 in the same second (20:23:37, 20:24:26, 20:24:47, 20:26:07, ...).
- The bursts were the workers of the owner pool: `owner-pool/🧒️child/🟨️.mjs` (started by the pool with `detached: true`) starts 8 `project-farm` or `walk-farm` children, each got a console of its own. Single windows: the agent's own child (`sidecar agent`), `repo show projects` of the graph read, `semio-repo-watch.exe serve`, the replica task's `powershell.exe`.

## Cause

A process started with `detached: true` has no console on Windows (`DETACHED_PROCESS`; `windowsHide` beside it changes nothing). Every console program it starts without `windowsHide` then gets a new, visible console. Probe with the workspace Bun 1.4.2 (`hide-probe.mjs` in my ticket; a detached middle process starts one leaf):

| Leaf started with | New terminal host |
|---|---|
| `node:child_process` `spawn`, pipes | 1 |
| the same with `windowsHide: true` | 0 |
| the same with an `ipc` channel and `windowsHide: true` | 0 |
| `Bun.spawn` | 1 |
| `Bun.spawn` with `windowsHide: true` | 0 |

## Changed in Source (`windowsHide: true` Added, Nothing Else)

Under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`:

- `🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs` (worker spawn)
- `🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🏃️runner/🟨️.mjs` (child spawn)
- `🕸️graph/🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🟨️.mjs` (worker spawn)
- `🌐️remote-execution/🛰️sidecar/🟦️.ts` and `🌐️remote-execution/🛰️sidecar/👷️agent/🟦️.ts` (role spawn)
- `🌐️remote-execution/🛰️sidecar/🕸️graph/🏃️runner/🟦️.ts` and `…/🕸️graph/🔬️stall/🌳️tree/🟦️.ts`
- `🏡️resident/🔭️watch/🟨️.mjs`, `🏡️resident/🔭️watch/🌐️hub/🟨️.mjs`, `🏡️resident/🏊️delegate/🟨️.mjs` (two spawns), `🏡️resident/🏊️delegate/🏭️executor/🟨️.mjs`

The option does nothing on macOS and Linux. Not run: the tests of these modules (I ran no `repo` command; a grep found no test that pins the old option text).

## Result on This Machine

After the edits I ended the build agent, its role processes and the pool's worker tree (20:40Z); the daemon started them again at 20:40:13Z. `OpenConsole.exe` count 4, no new one in 270 s (20:44:43Z to 20:49:13Z) while a new pool child (30116) was working. The three old consoles that remain belong to processes started before the edits (`repo show projects` 12180, `semio-repo-watch` 29220, the replica task).

## Second Source: The Scheduled Tasks (Changed in Source, Not Applied Here)

- `semio-fleet-store-mount` starts `cmd.exe` once a minute and `semio-fleet-replica` starts `powershell.exe`; both are interactive tasks, so each start opens a terminal host. Probe with two temporary tasks of the user (`task-probe.ps1`, both removed again): `cmd.exe /d /c ping` gives 1 new terminal host, `conhost.exe --headless cmd.exe /d /c ping` gives 0, both with result 0.
- Changed: `taskOf` in `🤝️shared-worktree/🗺️topology/🟦️.ts` now writes `<Command>conhost.exe</Command>` and `<Arguments>--headless <program> <arguments>`, for all five Windows tasks. Expectations changed in `🧪️tests/🟦️.ts` (lines 236, 237, 325, 456, 458, 518, 533) and `🧪️tests/🔵️.ts` (156, 206).
- Test run with the workspace Bun directly (not through `repo`): `🧪️tests/🟦️.ts` 51 pass, 20 fail; none of the failures names the task text, the ones I read are of this platform (`Absolute account home required: C:\Users\Ueli`, a Python `UnicodeEncodeError` on stdin). `🧪️tests/🔵️.ts` 2 pass, 18 fail with `ENOENT … mkdir` of a doubled `C:\…\C:\…` path. I have no run of these files from before my change, so "same as before" is not shown. Please run them on a Mac.
- Not applied on this machine: `Register-ScheduledTask` answers `Access is denied` without elevation, and I do not elevate. The registered tasks are still the old ones.

## Also Seen

- `semio-fleet-compact` ends with result 2147942402 (0x80070002, file not found) every night; its command is `repo` with a working directory.
- The replica daemon ended with exit 58 at 20:22Z and was started again by its boot stage.

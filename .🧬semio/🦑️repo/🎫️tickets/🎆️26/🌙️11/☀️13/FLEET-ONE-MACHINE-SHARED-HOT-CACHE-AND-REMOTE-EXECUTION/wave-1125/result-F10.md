# F10: no console popups from background sync on Windows (g1-m05)

## Cause (from ticket 11/22 PREVENT-BACKGROUND-SYNC-TERMINAL-POPUPS, issue usalu/semio#2795, and the code)
1. Scheduled tasks: the renderer (`🗺️topology/🟦️.ts` `taskOf`) already runs every task under `conhost.exe --headless`, but the installed definitions are only kept current by `maintainServiceDefinitionV1` for replica, compaction, cache and relay. The once-a-minute `semio-fleet-store-mount` task was never maintained, so an installed `cmd.exe` definition stayed and opens a window every minute.
2. Child processes: 59 child-process calls in fleet, replica, readiness, build, caching and process code had no `windowsHide`. A parent without a console (a detached sidecar, a windowless task) gives every console child its own visible window on Windows.
3. No evidence of popups exists in `S/origins/g1-m05/` (logs carry no window facts); m05 shows `loop-blocked-restart` x82 and dependencies attempt 46, unrelated.

## Changed
- `windowsHide:true` on all 59 sites (46 files; list = output of the law test before the fix). Node only applies CREATE_NO_WINDOW when no stdio is inherited, so `stdio:"inherit"` sites keep the user's console.
- Service maintenance includes the mount task (`🛠️service/🟦️.ts`); on m05 the next maintenance pass (quiet period, then `schtasks /Create /F`) replaces it. Tests updated plus one new.
- Law test `🗺️topology/🧪️tests/🪟️hidden-spawn/` (schema-less data in `🔣️.json`: roots, exempt POSIX-only programs, 17 cases; TS scanner and independent Python oracle must agree on cases and on the real tree). Roots: shared-worktree, library/build, library/caching, framework/process. A new spawn without `windowsHide` fails it.

## Not done / not verified
- Nothing ran on Windows. `conhost --headless` loses the child's exit code (ticket: returned 0 for 17), so Task Scheduler's RestartOnFailure never fires; the 1-minute time trigger (replica) and 1-minute repetition (mount) cover restarts. The wscript launcher of the ticket hung under Task Scheduler and was not adopted.
- Out of the law roots: 26 more child-process calls elsewhere under `🧰️framework` (other modules, products); Go launcher/`workspace` git (`exec.Command` without `SysProcAttr.HideWindow`, needs per-OS files); PowerShell provisioning scripts.
- Manually installed leftovers of the 11/22 attempt on m05 (wscript `quiet-tasks` launcher, `%LOCALAPPDATA%\semio\fleet\quiet-tasks`) will be overwritten by maintenance (it re-registers the tasks), but the stuck orphan test processes named in the ticket are untouched.

## USER (confirm on the physical Windows box g1-m05)
1. Within about 10 minutes of the code arriving, run `schtasks /Query /TN semio-fleet-store-mount /XML` and `semio-fleet-replica`: `<Command>` must be `conhost.exe` and `<Arguments>` must start with `--headless`.
2. Watch the screen for 3 minutes: no console flashes at each minute (mount task, replica revive) and none while workfleet/readiness runs (dependencies stage).
3. In Task Manager (Details) check no `cmd.exe` or `powershell.exe` with a visible window; `conhost.exe` instances are expected.
4. Say whether the mount drive still maps (`net use`) and `repo fleet status` shows m05 syncing.

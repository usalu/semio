# Service start denied by macOS file access (TCC): diagnosis and fix

## Verified facts

- TCC judges the responsible process, not the process itself. Measured with `responsibility_get_pid_responsible_for_pid`:
  - service 39444 (direct route) is responsible-by `claude` pid 45768, an agent session (alive);
  - the fleet entry (`boot.sh`/`boot.mjs` under launchd) answers for itself (no grant);
  - Claude Code runs behind `/Applications/Claude.app/Contents/Helpers/disclaimer`, so each session is its own responsible app.
- Experiment (now `🧷️responsibility/🐍️orphan.py`): when the responsible process ends, the kernel makes the orphaned child its own responsible process. A direct-route service therefore keeps the checkout access only while the session/shell that started it lives, then fails mid-plan with `EPERM` at `.cache/b/.reset.json` (rows 22385 after 9 min, 38472 after 40 s in `exit.log`).
- A shell-spawned watcher in a living session saw no EPERM over 6+ minutes: the EPERM is not a mount artefact.
- Why "the second start is denied as well" (describe18live): the log shows `Waiting for the repo graph service (5 ms)`. `exit.log` has the new `start` row, then 4 ms later the `access-denied` row of the retired old pid. `connectHostV1` looks back 1 s for denial rows, so it read the old service's row as the new start's denial. The second service was never judged.
- Poisoning: the client read the denied service's row right after `retire`, before it was written (row comes 4 to 140 ms later), saw no row, and remembered `launchd-refused.json` (08:26:41), which sends every later start of this boot to the fragile direct route. It also removed the launcher consent marker on denials of direct services.
- Reload break: the replacement is `repo service host start` spawned by the old service. If launchd is not used, it is a direct child of the old service and inherits the old responsible; when the old one drains, the replacement falls back to its bare runtime and is denied.

## Code changes (all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`)

- `🏡️resident/🚪️client/🛰️plan/🟨️.mjs`, `.d.mts`: `ignorePids` for `connectHostV1`/`requestPlanV1`; rows of ignored pids never refuse a start.
- `🧭️invocation/🛰️plan/🟦️.ts`: second start ignores the denied pid and the retired pid; waits (max 2 s) for the denied pid's exit row before deciding; the launcher marker is invalidated and launchd refusal remembered only for a launchd/unknown route.
- `🏡️resident/🔐️consent/🧷️responsibility/🟨️.mjs`, `.d.mts` (new): `responsiblePidV1` (Bun FFI to libSystem), `createResponsibilityWatchV1`; exported from `🏡️resident/🟨️.mjs`, `.d.mts`.
- `🏡️resident/🏗️composition/🌐️host/🟨️.mjs`: service logs/records `responsible=`; a protected-folder service whose responsible app ended stops with `stop:responsible-gone` (exit 0) so clients start the next one under their own access instead of failing mid-plan; `SEMIO_SERVICE_ROUTE_REQUIRED=launchd` refuses any direct start (bilingual `REPLACEMENT_REFUSAL_V1`).
- `⌨️command/🌐️host/🟦️.ts`: the reload sets `SEMIO_SERVICE_ROUTE_REQUIRED=launchd`; without a launchd route the reload fails, the old host keeps serving and the next client (own process tree) replaces it via the existing newer-sibling rule.
- `🏡️resident/🧬️schema/🔣️.json`, `📨️protocol/🎚️policy/🟨️.d.mts`: `HostPolicy.responsibilityCheckMilliseconds` (5000).

## Tests added (written, NOT yet run)

- `🏡️resident/🧪️tests/🧷️responsibility/` (`🟦️.ts`, `🧫️cases.json`, `🐍️oracle.py`, `🐍️orphan.py`): shared cases vs independent Python model; ctypes cross-check of the system call; orphan experiment on macOS.
- `🏡️resident/🧪️tests/🛡️consent-service/🟦️.ts`: ignored pids; responsible-gone stop vs stable/own/unknown; required-launchd refusal vs ordinary direct start.
- `🧭️invocation/🧪️tests/🛰️plan/🟦️.ts`: updated order (retire, then invalidate), invalidation only for launchd, ignorePids hand-off.

## Not finished

- The Bash tool stopped returning mid-session (every command hangs, possibly after I bootstrapped a diagnostic launchd job), so no test run exists. A first run via `repo tools exec bun -- test` was queued behind capacity (load 10) and may still be running in background (log `🗑️generated/tcc-test1.log`).
- Please clean up: `launchctl bootout gui/501/tech.semio.tccdiag`; kill the watcher (`pkill -f service-tcc-watch.mjs`, pid 51464); remove `service-tcc-watch.mjs`, `🗑️generated/tcc-*`. Remove poisoned memory `~/Library/Application Support/semio/access/3f08bbea4d32/launchd-refused.json`.
- Not explained: the 08:16 launchd-route service (pid 21367, tag 4a3e) hit the same EPERM although the launcher probe job exited 0. Services now log `responsible=` so the next occurrence names the identity.

## Recommendations (user action, only if the launchd route stays refused)

- Click Allow once for `~/.local/share/semio/launcher/bin/repo` (Files & Folders, Documents) or grant it Full Disk Access; it is the stable identity the launchd route relies on, and is lost whenever the launcher bytes change.
- Do not grant the bun runtime (its path hash changes with updates).
- Most robust: move the fleet worktree out of `~/Documents` (e.g. `~/semio`), which removes TCC from the picture.

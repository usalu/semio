# E27 result: declared OS scheduling class per supervised role (service-priority)

## Finding (measured, not assumed)
- Provisioning rendered NO scheduling keys anywhere: macOS plist without `ProcessType`, systemd units without `Nice`/`CPUSchedulingPolicy`/`IOSchedulingClass`, Windows tasks without `<Priority>` (Task Scheduler then runs a task at 7 = below normal). The supervisor (`sidecar/🟦️.ts` launcher) spawns role children without any priority; only ad-hoc `setPriority(10)` in `pool`/`ready` and 19 in idle compaction existed.
- launchd probe (throwaway jobs from the scratchpad, booted out again): no `ProcessType` -> pri 20, `Standard` -> 20, `Interactive` -> 31, `Background`/`Adaptive` -> 4. So the replica agent at 20 is exactly "no ProcessType".
- macOS: a process can lower itself (`taskpolicy -b -p self`: 31 -> 4, `-B` restores) but can never get back above what launchd gave the job (clamp inherited, `taskpolicy -c utility` -> 20, `-t/-l` tiers do nothing). Hence the host service must itself be the highest class of its roles; lower classes are applied downwards. On Linux/Windows a nice raise needs the unit/task to be at that class too.

## What changed
Declared once, schema-first: `B/🌐️remote-execution/🛰️sidecar/🪜️priority/🧬️schema/🔣️.json` (classes interactive nice -5 / standard 0 / background 10 with per-platform facts; roles: cache, scheduler, graph = interactive, agent = standard, pool, ready = background; services: replica = interactive, cache = interactive, relay = standard, compact = background; EN/DE texts).
- `🪜️priority/🟦️.ts`: renderers `launchdPriorityKeysV1` (ProcessType, LowPriorityIO, Nice), `systemdPriorityLinesV1` (Nice, CPUSchedulingPolicy, IOSchedulingClass/Priority), `taskPriorityXmlV1`; runtime `applyServicePriorityV1(role, ports)` (nice via setPriority, which libuv maps to the Windows priority class; Linux `ionice`; macOS raises are left to ProcessType and reported `inherited`; never throws).
- `🛰️sidecar/🟦️.ts`: the role child applies its class at start and logs one EN/DE line (`<role> scheduling: ...`). Takes effect on the NEXT ROLE RESTART with no reinstall (child code reloads on every restart). The launcher itself is unchanged logic-wise, so no launcher restart is needed.
- `🏊️pool`, `🚦️ready`: hard-coded `setPriority(10)` removed (now declared background, same value). Idle compaction child (`cache-gc` nice 19) left as is: already lowest.
- Topology (`🤝️shared-worktree/🗺️topology/🟦️.ts`): macOS agentOf renders ProcessType/LowPriorityIO/Nice per service (replica, compact, cache, relay); Linux replica/compact/cache/relay units render the systemd lines; Windows replica/compact/cache/relay tasks render `<Priority>` (3 / 8 / 3 / 5). Mount task untouched.

## Proof
- `sidecar/🧪️tests/🪜️priority` 11/0: declaration complete and ordered (interactive roles never below the host service), cross-checked with python `jsonschema` (🐍️.py), renderer output, `applyServicePriorityV1` with injected facts for linux/darwin/win32 (set calls, ionice args, refusal, missing ionice), EN/DE text, and a real child process whose nice `ps -o nice=` reports 10.
- topology tests 72/0 incl. new test (plutil -lint + `plutil -extract ProcessType` read-back; xmllint Task XSD test still passes with `<Priority>`); sidecar restart 23/0, sidecar 7/0, pool 7/0, ready 56/0. Landing logged in `T/landings.md`.
- LIVE (natural restart at 05:58 after the landing): `.cache/f/sidecars/build-cache.log`: `cache scheduling: class interactive, nice 0, inherited from the service that hosts the role`; scheduler same; pool `class background, nice 10, applied`. `ps -M` of the new cache pid 54894: pri 20, nice 0. Expected and honest: PRI stays 20 until the replica LaunchAgent itself carries `ProcessType Interactive`.

## How and when it takes effect on the fleet
- Role code (apply + pool/ready): already live after each machine's sidecar restart (roles restart themselves after the quiet period). Effect on Linux/Windows hosts: batch roles nice 10 (as before) and `ionice` idle; interactive raises only where the unit/task is at that class.
- The service definitions only change when provisioning rewrites them: macOS `tech.semio.fleet.replica.plist` must get `<key>ProcessType</key><string>Interactive</string>` (`repo` provision re-render, then the agent re-bootstrapped: logout/login or `launchctl bootout/bootstrap gui/<uid> ...`), Linux `systemctl daemon-reload` + restart, Windows task re-registered. I did NOT touch ~/Library or restart anything. The current definition renders from `fleet worker-service --group g1 --platform macos`. There is no auto-rewrite of an installed plist (the existing `migrateAgentV1` only rewrites the program array); adding one would be a migration, so it was not built.
- To verify after the plist is re-bootstrapped and the cache restarts: `ps -M -o pri,nice,command -p <cache pid>` should show 31 (was 20), and the log line should read `applied`/`inherited` with nice 0.

## Not verified / notes for others
- Not run on real Linux (systemd unit is checked textually; `systemd-analyze verify` unavailable) or Windows (task XSD checked by the existing xmllint test; ABOVE_NORMAL raise by setPriority untested live).
- Under interactive at 31 the agent/ready/pool children inherit 31 on macOS except pool/ready (nice 10, which only weights within the same pri band); builds run by the agent are therefore NOT throttled on macOS once the plist is raised. If that is unwanted on workers, the launcher can wrap agent spawn with `taskpolicy -c utility` (launcher restart needed); left undecided.
- E21 hypothesis (utility priority causes the starvation) is only supported by the throughput handicap E21 measured; the stall rate at equal load before/after the plist change is still to be compared.
- I started a background wait loop by mistake once (stopped via TaskStop); no processes left, locks released.

# Round 2: zero-touch rollout of the service definition

## What changed
- New `🤝️shared-worktree/🗺️topology/🛠️service/` (`🟦️.ts`, `🧬️schema/🔣️.json` with State, Result, Limits, EN/DE texts). `maintainServiceDefinitionV1(topology, groupId, ports)` renders the replica and compact definitions from the topology (same fill as provisioning; macOS keeps the machine's own `EnvironmentVariables` extras, which provisioning owns), compares with the installed file and, when they differ, installs the rendered one.
- Safe point: the difference must persist for the quiet period (300 s, same value as the sidecar code-restart debounce, state in `<entry>/service-definition/state.json`) and `busy()` (a sidecar slot `waiting`/`backoff`) must be false; otherwise `pending` and the daemon re-checks after `dueMs`.
- macOS: lint with `plutil`, keep `<label>.previous`, write the plist atomically, then start a detached one-shot helper job (`<label>.reload`, a LaunchAgent of its own so it survives the `bootout` of the replica): `bootout`, `bootstrap`, wait up to 20 s for `state = running` (compact: loaded), otherwise copy the previous plist back and bootstrap that, record `result.json`, boot itself out. The next daemon start reports a `restored`/`failed` result once. If the helper cannot even be started the written plists are put back.
- Linux: write the unit where the account may (a root-owned `/etc/systemd/system` unit is NOT writable by the daemon: reported once as `needs-privilege` with the path, provisioning must run once with privileges), `systemctl daemon-reload`, `systemctl --no-block restart semio-fleet-replica@<user>`; failing reload restores the previous unit and reloads again.
- Windows: `schtasks /Query /XML` saved as previous, `schtasks /Create /XML <utf-16> /F`; a failed create re-registers the exported XML (also for tasks already replaced). Applied digest kept in state.json because Windows does not return the XML byte for byte.
- Hook: `🔁️replica/🟦️.ts` `keepService`, called from `keepEntry` (at daemon start and hourly) and by a timer after `dueMs`; `stage:"service"` events say what happened. Only when no `entryDirectory` override is given (same guard as `migrateAgentV1`).

## Proof
- `🗺️topology/🧪️tests/🛠️service` 7/0: injected platform runners for all three platforms (debounce, busy, lint failure, helper start failure, unrecognised/absent definition untouched, restore on failed reload, privilege report, once-only helper report, `sh -n` of the script) plus 2 REAL throwaway LaunchAgents (scratch labels, booted out afterwards): a running job is swapped to the new plist by the helper job (pid changes, pri 20 -> 31, helper gone) and a new plist that cannot run is rolled back (outcome `restored`, job running again at pri 20).
- Dry run against the real installed plists (in-memory ports): the only difference found for the replica plist was the `ProcessType` line; nothing else of the file would change.
- topology 72/0, replica 68/0, entry 24/0, sidecars 3/0.

## LIVE on this Mac (not done by me; the daemon did it)
- 06:04:56 daemon saw the difference (state.json pendingDigest), 06:09:56 after the quiet period it wrote the plist and started the helper; 06:10:10 `result.json` = `applied`; new replica pid 66484 at pri 31 (was 20); `launchctl print gui/501/tech.semio.fleet.replica.reload` gone. Replica log: `service definition installed / Dienstdefinition installiert`. The compact agent got `ProcessType Background`.
- After the reload the sidecars were started again by the new daemon (06:10:41): cache pid 66991/66992 pri 31 nice 0, scheduler 31, graph 31, pool 31 nice 10. `ps -o pri` of the cache = 31 as required.

## Not verified / caveats
- Linux and Windows paths only with injected runners; a Linux fleet worker with root-owned units will report `needs-privilege` until provisioned once with privileges (no unprivileged way exists to change a system unit).
- The reload restarted the replica and its sidecars (a few seconds of service gap, ~30 s from the SIGTERM drain), as does any launchd-level restart; it is deferred by the quiet period and sidecar restarts only, not by agent work in flight (the agent drains/adopts through its own protocol).
- macOS server-only agents (`cache`/`relay` LaunchAgents) are not re-rendered by this path (they need the token environment of provisioning); only replica and compact are.
- Machines that rejoin with an older definition keep reloading only when they run this code; a machine without a graphical session cannot bootstrap an agent (the helper then restores and reports `failed`).

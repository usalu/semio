# Round note R23: what the Windows worker (g1-m05, `C:\semio`) must run and copy after the R23 landing

For the coordinator to post to the agent of g1-m05 once the R23 landing is on the group server (`build-agent.log` shows `agent code changed, restarting`). Prompt basis for the machine: `…/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/1113-windows-join-prompt.md` (join, `S:` store, provision script). The agent changes no file outside `W/workers/g1-m05/`, runs no git command that changes anything, no `taskkill` by name, prints no token; one graph command at a time.

## 0. Before releasing the worker (coordinator, read-only on g1-server)

- `python3 -I T/🧰️scripts/R23/land.py --dry-run` prints `same` for all 22 files.
- `S:\v1\origins\g1-m05\attest.json` (or `/Users/Shared/semio-store/v1/origins/g1-m05/attest.json`) carries the code digest of the server: the replica of the Windows machine has the new code. Without it the Windows machine still writes the old record form and nothing below can be read.
- The first Windows request in `build-agent.log` of the server must produce, within 5 to 10 s: `graph-server: serving <N> stale projects for g1-m05`, followed by `graph-server: g1-m05 (win32-x64, class windows-shared) is served across platform classes by a machine of the class posix-shared: <N-1> projects served, 1 bound to the host and left to a machine of its class: @semio-tech/os-plugin-describe-rs`. If it shows instead `all <N> stale projects of g1-m05 … are bound to the host`, `boundOwners` in `GS/🌍️platform-class/🧬️schema/🔣️.json` matches the whole request: report it. If it shows nothing for g1-m05, the server does not see the request: the heartbeat `requesters` count (machines seen) must include g1-m05.
- The heartbeat (every 5 min) reads `graph-server: alive, <machines> machines seen, <held> requests held, <cross> projects served across platform classes, <left> projects left to a machine of their class, read …`; `cross` is the number of projects of the Windows request while the server holds it, `left` is 1 (the descriptor emitter) plus the projects of requests that hold bound owners only.

## 1. What the agent on g1-m05 runs (PowerShell 7, elevated, the login account stays logged on)

```
$env:SEMIO_LANGUAGE='en'; $env:SEMIO_WORKSPACE_ROOT='C:\semio'; $env:SEMIO_FLEET_GROUP='g1'; $env:SEMIO_FLEET_NFS_HOST='192.168.178.70'; $env:SEMIO_FLEET_NETWORK='192.168.178.0/24'
Remove-Item Env:SEMIO_FLEET_CACHE_ENDPOINT,Env:SEMIO_REMOTE_CACHE_ENDPOINT,Env:SEMIO_REMOTE_CACHE_ACCESS,Env:SEMIO_REMOTE_CACHE_INSTANCE,Env:SEMIO_REMOTE_CACHE_TOKEN -ErrorAction SilentlyContinue
Set-Location C:\semio
$log="$HOME\native-bootstrap-r23-try-1.log"
(Get-Date).ToUniversalTime().ToString("'start 'yyyy-MM-ddTHH:mm:ssZ") | Out-File $log
powershell -NoProfile -ExecutionPolicy Bypass -File "C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🔩️native\🥾️bootstrap\🔵️.ps1" setup *>&1 | Tee-Object -Append $log
"native_rc=$LASTEXITCODE" | Out-File -Append $log
```

While it runs, every 30 s: `Get-Process | Sort-Object WorkingSet64 -Descending | Select-Object -First 3 Id,ProcessName,@{n='MB';e={[int]($_.WorkingSet64/1MB)}}` and `(Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory` (hard stop: a process above 8 GB resident or free physical memory under 1 GB in two samples in a row; no new `graph:` line for 15 minutes: one Ctrl-C). Then, from `C:\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\🤝️shared-worktree\🔁️replica`: `repo ./📜️script.ts status --lang en`, `repo ./📜️script.ts fleet --wait 600 --lang en` (this machine's line must read ok), and `repo preflight --lang en` once the bootstrap ends with `native_rc=0`.

At most 6 tries, 3 minutes apart, after `native_rc=75`, `closure-missing`, retry wording; stop after two tries in a row with the same other error.

## 2. What the service log of g1-m05 (lines starting `graph:`; find it with `Get-ChildItem C:\semio\.cache -Recurse -Filter host-*.log | Sort-Object LastWriteTime -Descending | Select-Object -First 3`) must show, in this order

```
graph: <N> stale projects, 0 taken from the fleet, <K> too heavy for this machine (…; it can give about <G> MB per project), it can infer <L> of the others (<W> per wave); asking the fleet for <R>
graph: waiting for the fleet: <x> of <R> projects arrived; the server last progressed <S> s ago, estimated time left: about <M> min
graph: 1 projects are bound to the host (@semio-tech/os-plugin-describe-rs); the servers of other platform classes leave them, this machine infers them itself
graph: <50|100|…> projects taken from the fleet with <C> closure entries, <D> of them did not hold here and were inferred again, <F> not shared, <I> inferred here
```

Expected: the bound line appears once, after the wait; `taken from the fleet` grows to several hundred; `not shared` stays 0 on a taker (it counts records the machine itself refused to publish or could not map); `did not hold` stays a small share. On the stored records 92 % of the projects have identical neutral forms across Windows and macOS; the 8 % that do not (directory listings with entries only one machine has, other inferrer stamps, generated files) are `did not hold` plus `inferred here`. Projects that are not in the heavy list and outgrow the machine end as before (`native_rc=75`, `grew by … marked too heavy here`): copy the line.

## 3. What to copy into `W/workers/g1-m05/round-r23-<try>.md` (and the tail of the log into `…tail.txt`)

- the first plan line, the last `waiting for the fleet` line, the bound line, every `taken from the fleet … did not hold here … not shared … inferred here` line (they carry the numbers of this package) and the `restored … from a state A min old with S projects stale` line;
- if a read ends in `native_rc=75`: the refusal text with the line `graph: this machine is win32-x64 (class windows-shared); machines of every class serve its request except the projects bound to its host, …`, and from the coordinator's `build-agent.log` the `serving`, `served across platform classes`, `answering` and `served` lines of g1-m05 for the same minutes;
- **three names of projects that did not hold** (names only; the coordinator replays them against the stored records);
- the `[capacity]` lines, the sampler's peak, `native_rc`, wall time, the code digest of the replica `status`, and whether `C:\semio` is a Developer Mode / symlink-capable volume (`fsutil behavior query SymlinkEvaluation`).

## 4. A Windows machine that serves

A Windows machine with at least 15 000 MB of physical memory runs the group graph server for its own class and then serves the whole request of another Windows machine including the bound owner (`graph-server: serving …` without a `served across platform classes` line). g1-m05 is a worker; do not start a server. If its service log nevertheless contains `graph-server:` lines, copy them.

## 5. What a good try looks like, and what it does not prove

`taken from the fleet` in the hundreds, `inferred here` far below the local waves, no `graph-plan-too-heavy`, `workspace:setup` reached. It does not prove that every project record holds on Windows: only the `did not hold` count does, and the three names it asks for are the sample the coordinator checks against the real stored records.

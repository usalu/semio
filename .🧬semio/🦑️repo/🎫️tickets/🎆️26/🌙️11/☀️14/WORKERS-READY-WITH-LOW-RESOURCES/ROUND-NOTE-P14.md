# Round note P14: what the Linux worker (g1-m04) must show after the P14 landing

For the coordinator to post to g1-m04 once `LAND-OK` was given, P14 landed, and the group server runs the new code (its `build-agent.log` shows `agent code changed, restarting`). Nothing here starts a graph command of its own, kills anything or writes into the worktree.

## 0. Before releasing the worker (coordinator, read-only on g1-server)

- `T/🧰️scripts/P14/land.py --dry-run` prints `same` for all 29 files (landed), `build-agent.log` shows the restart.
- After the worker's first try: `build-agent.log` of g1-server must show, within 5 to 10 s of the worker's request, `graph-server: serving <N> stale projects for g1-m04`. If it shows instead `graph-server: <N> stale projects of g1-m04 (linux-x64) are left to a machine of the class posix-shared; this machine serves the class …` the class table (`🌍️platform-class/🧬️schema`) does not list the server's platform: report it, nothing else is needed.
- The heartbeat every 5 min now reads `graph-server: alive, <machines> machines seen, <held> requests held, <foreign> of other platform classes left to others, read <none|request|refresh>`. `foreign` is 0 while only macOS and Linux machines ask; a Windows machine's request is counted there (nobody serves Windows yet).

## 1. What the worker's service log (`host-*.log`, lines starting `graph:`) must show, in this order

```
graph: <N> stale projects, 0 taken from the fleet, <K> too heavy for this machine (<names, each with about X MB>; it can give about <G> MB per project), it can infer 192 of the others (48 per wave); asking the fleet for <R>
graph: waiting for the fleet: <x> of <R> projects arrived          (x grows within about a minute)
graph: <50|100|…> projects taken from the fleet, <M> inferred here  (every 50 decisions of the worker's own inference)
```

Expected numbers on a 16 GB Linux worker with about 10 GB `MemAvailable` (swap is not counted, neither added nor subtracted):

| Item | Before (46 tries) | Expected now |
|---|---|---|
| projects called too heavy (K) | 517 of 517 | at most 14, only names with a measured cost above the grant, among them (physical MB) `@semio-tech/framework-os-dev`, `@semio-tech/repo-dashboard-rs`, `@semio-tech/repo-lib`, `os-hub`, `repo`, `workspace`, `@semio-tech/framework-plugin`, `@semio-tech/framework-os-kernel`, `@semio-tech/framework-renderer-wgpu`, `@semio-tech/plugin-publication-mounted-rs`, `semio-framework-job-native-turn`, `repo-schema-commands`, `repo-mcp`, `@semio-tech/print` (table of the server, `.cache/f/graph-costs/owner-costs.json`, read by the worker from the fleet) |
| "it can give about G MB per project" | 0 | about 3 300 (physical) at `MemAvailable` >= 10 GB; less when less is free: G = (min(half of 15 848, MemAvailable - 2 048) - 3 800) / 1.25 |
| "it can infer" | 0 (0 per wave) | 192 (48 per wave) |
| projects asked of the fleet (R) | 517 | the stale ones of the worker (R = N), the wait ends when at most 192 of them remain and none of the heavy ones |
| `0 machines seen` / `0 requests held` (worker's own sidecar) | every 5 min | unchanged on the worker (it is below 24 000 MB and never serves); the group server's line counts the worker |
| taken from the fleet : inferred here | 0 : 0 | most taken; about 1 in 10 records of the Mac cannot hold on Linux (directory listings that contain machine local entries, measured 23 of 209 projects of equal sources), those are inferred here (a few dozen), plus the one host bound project `@semio-tech/os-plugin-describe-rs` (its target declares `hostPlatformOutputsV1`), which is keyed by the exact platform |

If a project that is not in the heavy list outgrows the worker while it infers, the memory governor stops it and the read ends with exit 75 and the line `graph: project <name> (<kind>) grew by <X> MB, more than the <Y> MB this machine gives one project; … marked too heavy here`: copy it; the next try asks the fleet for that name first (bounded: it is one name per refusal).

## 2. What to copy per try

- the `graph:` lines above (first plan line, the last `waiting` line, every `taken from the fleet` line, the `restored … from a state A min old with S projects stale` line of P13),
- if the read is refused (`native_rc=75`): the refusal text with the new line `graph: this machine is linux-x64 (class posix-shared); only machines of that class serve its request, …` before it, and the group server's `serving` / `answering` / `served` lines for g1-m04 from `build-agent.log` for the same minutes,
- the `[capacity]` lines (they say what the machine really booked), the sampler of `ROUND-NOTE-P12.md`, and the peak resident size.

## 3. What a good try looks like (and what it does not prove)

`taken from the fleet` grows to several hundred, `inferred here` stays below the 192 of the four local waves, no `graph-plan-too-heavy`, and `workspace:setup` is reached. It does not prove that the Mac records hold on Linux for every project: only the log lines `taken … / inferred here` do, and they are the numbers asked for in this note.

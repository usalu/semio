# Package R30: Linux zero grant and the `repo tools setup all` exit 1 (PREPARED, NOT LANDED)

T = `…/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`. Nothing landed; no graph command, no `repo tools setup`, nothing killed. Scripts: `T/🧰️scripts/R30/` (`land.py`, `build_payload.py`, `overlay.py`, `src/`, `payload/`).

## A. g1-m04: `it can give about 0 MB per project ... infer 0 (0 per wave)`

### Real readings (g1-m04 files)
- `round-14.pre.txt` / `round-3.md`: `free -m` Mem total 15848, used 5718-6307, buff/cache 8228, available 9541-10130; Swap 25331 total, about 10 GB used. `round-14-try-*.samples.txt`: `MemAvailable` 10 473 644 kB at try 6 start, 10 244 952 kB at try 12 start, lowest 7 828 948 kB, SwapFree about 15.37 GB, PSI full avg10 0.00 to 1.09. `[capacity]` lines of the same minutes: 7606, 8347 and 9675 MB available, floor 1981 MB.
- So memory was NOT the cause: kB parsing is right, MemAvailable exists, swap/zram/tmpfs are not in it, the machine had 8-10 GB. Both the P14 formula (`min(7924, available-2048)`) and the live host-only law give 3298 MB for these readings.

### Cause (code, with the number that reproduces it)
The plan line is printed by the resident graph service (`host-*.log`, `🕸️graph/🕰️host` `fleetRead` -> `fleetReadStaleGenerationV1`). The size of the machine came from `hostMemoryMegabytesV1()` = `process.constrainedMemory()` when below `os.totalmem()`. On Linux the service is placed by us into its own cgroup `semio-service-graph-<hash>` with a hard `memory.max = GRAPH_MEMORY_POLICY_V1.serviceLimitMegabytes = 4096` (`🏡️resident/…/🌐️host/🟨️.mjs:98-100`, `prepareServiceGroupV1`). Inside it the "host" is 4096 MB: admitted = min(2048, 4096-2048) = 2048, wave = floor((2048-2200)/104) = 0, grant = admitted = 2048, footprint 2048-3800 -> 0 MB, every owner with a measured cost is "too heavy" (68-77), inference 0. macOS has no cgroups, so the Macs got 3512 after R4; P14/R4 were verified on a Mac and with synthetic readings only. Matching evidence: the `waiting for memory headroom (873 of 1278 MB …)` lines (service headroom of a few GB on a 16 GB box) and the `1024 MB this machine gives one project` stops of tries 4 and 5 (half of 2048).
The census wording (swap, page cache, GPU reservation, the service's own memory) was checked and excluded above; the cgroup guess in the task was the right layer, with the twist that the limit is the service's own budget, not a user-slice limit.

### Fix (law: the machine is the machine; a limit counts only when it bounds the machine)
- `🧱️job-limits/🐧️cgroup`: new `cgroupMachineLimitBytesV1`: tightest numeric `memory.max` / `memory.high` over the cgroup chain, skipping every group this system creates (`semio-*`, `job-*`).
- `🧠️memory`: new `fleetHostReadingV1` = physical memory (declared smaller machine honoured) bounded by that limit, plus the kernel available memory (Linux `MemAvailable`, macOS `vm_stat`) bounded by it. Used by `prefetch` (fleet read) and `host` (the `heavy` probe). `hostMemoryMegabytesV1` is unchanged for its other users.
- A structural limit lowers the projects per wave before the grant (existing `fleetAdmitted`/`fleetGrant` law; 8 GB user slice -> wave 18, 6 GB `memory.high` -> wave 8). The free memory of the moment decides nothing: this is the live law written after R4 (`🍏️idle-grant`, small-booking: 464 MB spare once caused exit 75), so the task's "busy moment lowers waves" is deliberately NOT reintroduced; the capacity gate queues the inference. Report if you want it back.
- The plan is preceded by `graph: memory readings: machine {host} MB (physical {physical} MB, cgroup limit {limit|none}), available {n} MB` (de: `Graph: Speicherwerte: Maschine … verfügbar …`), schema texts en+de.

### Tests (overlay, BUN_RUNTIME_TRANSPILER_CACHE_PATH=0)
NEW `🪪️graph-state/🙋️wanted/🧪️tests/🐧️linux-host` (15): table of real g1-m04 readings (MemAvailable 10473644 / 10244952 / 7828948 kB, service group 4096 MB, slice-load leaf), 8 GB slice, 6 GB `memory.high`, unlimited scope, idle/busy/starved darwin and linux, each against an independent Python computation; the fault (host 4096 -> 0 MB, wave 0) vs fixed (3298 MB, 48); idle = starved; both-language readings text; cgroup walk; a real `fleetReadStaleGenerationV1` run proving the readings line precedes the plan line. Before: red (module missing). After: 15 pass. Neighbours unchanged and green: idle-grant, two-platforms, parent-budget. Pre-existing reds in the scratch overlay, identical before and after (not touched): fleet-read small-booking and macos-reading (heavy list has 2 names, expectation 1), job-limits watchdog, retirement driver test.

## B. `repo tools setup all` exit 1
Landing times (headings of `T/landings.md` are CET; UTC = minus 1 h): R6 04:01Z, R6b 04:28Z (not 04:45), R6c 05:05Z (not 05:10), R18 06:08Z (not 06:15). A running process keeps the code it loaded at start.

| Run | Window (Z) | Last error line (verbatim, 20 lines before are only `[repo] still working after N s; last phase started: source.served`) | Class | Fixed by | Run vs fix |
|---|---|---|---|---|---|
| g1-m10 attempt 2 (round 16) | 04:06:42-04:18:06 | `Internal error: Original physical body, drain and cleanup settlement failed: … Current complete Source binding or whole receipt changed; Original process admission closed` (`round-16.tools-setup.last15.txt`; before it `[DATA] Dependency sync incomplete {…}`) | source binding moved | R6b | before (R6b 04:28) |
| g1-m02 step 0 r19 | 04:18:16-04:32:38 | same text, ending `- first failure: Current complete Source binding or whole receipt changed` x3 (`round-20.md` 27-55) | source binding moved | R6b | started before, ended 4 min after landing; old code in the process; before |
| g1-m02 step 0 r20 | 04:32:55-05:02:41 | `The dependency store returned no complete retirement plan (malformed, 227696 bytes, first violation: /value/roles/names must contain at most 16 item(s)); nothing was retired. Run the setup again; if this repeats, the dependency store image is damaged and must be reset` | NEW: retirement plan wire bound 16 vs producer bound 24 | R30 (R6/R6c did not touch it) | after R6b, before R6c |
| g1-m02 step 0 r24 | 06:23:27-06:43:29 | identical text (`round-24.tools-setup.md` 34) | same | R30 | AFTER R6, R6b, R6c, R18: proves R6c did not fix it |
| g1-m07 round-17 setup | 04:33:44-06:17:58 (6254 s) | not in the files (`round-17.tools-setup.txt` has start/end only; `round-18` attempt 1, 03:56-04:09Z, same) | undetermined | - | started after R6b; R6c and R18 landed mid-run; same error as r20/r24 is plausible after a 104 min funding, unproven |

None of the five shows EPERM, bazel role, bun workspace ownership exhaustion (R26, prepared) or the retirement-plan-required text; those classes are not evidenced by these runs.

### Cause of the open class (code)
`publication/🟦️.ts` `requireRepositoryDependencyImageRolesV1` / `repositoryDependencySelectedNamesV1(root, protocol.maximumRoles = 24)` accepts a store root with up to 24 names; the declared closed image has 18 (8 directory roles, 2 file roles, 7 partition roles, `.fseventsd`), and the wire shape `setup/♻️retirement/🧬️schema/🔣️.json` limited `roles.names` and `candidates[].roles` to 16. A fully funded image therefore always ended malformed (the 227 696 bytes are the whole plan). The advice "image damaged, reset" is wrong: nothing to reset.
Fix: both bounds 16 -> 24 (the protocol bound, one number). NEW test `…/♻️retirement/🧪️tests/📏️roster-bound` (11): bound equals `maximumRoles`; rosters of 0, 1, 16, 17, 18, 24, 25 names against an independent bound; all declared roles pass. Before: red on 17, 18, 24 and full image; after: green.

## Landing (exact)
`python3 "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R30/land.py" --land` (takes `T/landing.lock`; `--lock-held` if the coordinator holds it; `--rollback <dir>` from `T/🗑️generated/R30-landing-*`). Prepared with: dry run clean (8 edits `edit`, 2 `new`, 0 refused, parse proof ok), rehearsal + rollback byte-identical in a scratch root (10 files). 8 live files, 2 new test folders; none of the R24/R26/R28/R29 files.

## What to see next
- g1-m04 (after the service restarts on the new revision, new `host-*.log`): `graph: memory readings: machine 15848 MB (physical 15848 MB, cgroup limit none), available 7800-10500 MB`, then `… N stale projects … too heavy (≤ about 14 names; it can give about 3298 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for …`. If a user slice limit exists the line says `cgroup limit <X> MB` and the wave is smaller, grant first unchanged.
- Macs: `machine 16384 MB (physical 16384 MB, cgroup limit none), available <n> MB`, `3512 MB per project`, `192 (48 per wave)`.
- Setup: g1-m02 and g1-m07 step 0 must end without `retirement plan (malformed …roles/names…)`; if it still exits 1, copy the last error line (new cause).

## Not verified
No Linux run (cgroup walk proven with fake ports only); the 4096 MB service group on g1-m04 is derived from code, not read from the machine. `tsc`/taxonomy gates not run. Other users of `hostMemoryMegabytesV1` inside the service (parent budget share, plan-stage limit, farm workers) still see the 4096 MB group; possibly intended, not changed. g1-m07's error line is missing.

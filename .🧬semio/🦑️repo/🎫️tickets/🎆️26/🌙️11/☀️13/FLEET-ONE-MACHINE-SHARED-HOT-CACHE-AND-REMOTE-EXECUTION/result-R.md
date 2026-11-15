# Package R: the native bootstrap on a 16 GB worker (memory admission)

Owner report 03:00: "All the workers fail on the bootstrap because they are lacking RAM." Measured and changed on this machine (g1-server, 32 GB, `/Users/Shared/semio`), 2026-11-14 03:00 to 03:35. Scripts: `🧰️scripts/R/` (the five patch scripts that were applied under the lock). Landing: `landings.md` ("Package R landing 1"). Worker prompt: `prompt-worker-native-bootstrap.md`.

## 1. Cause (read in code, reproduced in a test)

The message `Reading the project graph slice: reservation 0 of 8192; running: (none); waiting for memory headroom (8192 of 6624 – 8875 MB reserved)` comes from the slice pool of the host service (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`, `reserving`). Four defects, all found:

| # | Finding | Where |
|---|---|---|
| a | The booking is `max(min(8192, learned peak), 2048)`: 8192 is `sliceHostElasticity.maximumMegabytes` (`🏡️resident/🧬️schema/🔣️.json`), the learned peak is a file `semio-slice-peak-<uid>.json` in the temp directory that grows only (`max(read, peak)`, times 1.25 after a kill) and counts the whole process tree. 6627 was such a learned number. On a Mac (no cgroup) nothing else limits it, nothing relates it to the physical memory. | slice pool, `🧱️limit/🟨️.mjs` |
| b | The gate (`evaluateHostCapacityV1`) grants a request only if `free memory − floor − unrealized` covers it. The floor is 12.5 percent of the memory, 2048 MB on 16 GB. | `🌡️host-capacity/📏️decision` |
| c | A request that does not fit waits. The only override (`overrideAfterMilliseconds`, 120 s) needs `reason !== "memory"`, so a memory refusal with `running: (none)` waits for memory that no run of ours holds, for ever. A deadlock by policy. | decision, last branch of the loop |
| d | "Granted although 7830 MB reported above the 6627 MB asked": no bug in the comparison. The progress line printed the raw available memory (7830) while the decision compares `7830 − 2048 floor = 5782` against 6627. The margin is the floor, the line did not show it. Same for "(8192 of 6624 – 8875 MB reserved)": 8875 − 2048 = 6827 < 8192. | `reserving`'s progress (`budget` was the raw available bytes) |

Reproduction (before the fix, test `🌡️host-capacity/🧪️tests/📉️shrink` "the refusal the workers met"): a simulated 16 GiB host (`total 16384`, `available 7830`, nothing running) with a request of 6627 MB and no minimum is still queued after 600 ms and prints `waiting ... need 6627 ...`; the same request with a minimum of 2048 MB is admitted at once with 5782 MB.

## 2. The fix (rules and where they are proven)

New request field `minimumMegabytes` (schema first: `Request`, `Operation`, reasons `shrunk`/`never`/`starved`, messages EN and DE in the host-capacity schema). Rules, all in `evaluateHostCapacityV1` and the gate:

1. **Sized from the machine.** `sliceBookingV1` (`🧱️limit/🟨️.mjs`): the booking is the declared or learned size capped at `hostCapBasisPoints` (4000, schema `sliceHostElasticity`) of the physical memory, never below `minimumMegabytes` (2048). 16 GB: 6553 MB (not 8192 or 6627); 8 GB: 3276; 4 GB: 2048; 32 GB: unchanged (6627). Test `📐️booking` against a Python oracle on 4/8/16/32 GB.
2. **Never queued when it can never fit.** `minimumMegabytes > (total − floor)` is refused at once with the typed error `memoryNeverFits`: names the minimum, the total, the floor, what can ever be given out, how much is needed, and what to do (a machine with more memory, or take the result from the fleet cache), in English and German (resolved by the shared language rule). Test: 4096 MB host, 3072 asked, refused in under 2 s, both languages contain 3072, 4096, 1536, 2560, 4608.
3. **Nothing running means waiting is pointless.** When the full request does not fit but `min(free observed, free budget) ≥ minimum`, it is admitted with what is free (`shrunk`, line `[capacity] admitted graph-plan with 5782 of 6627 MB: only 5782 MB are free above the 2048 MB floor right now, and 2048 MB are enough to start`). When even the minimum is not free and nothing of ours runs, it waits at most `load.overrideAfterMilliseconds` (120 s) and then is refused (`memoryStarved`: numbers, what to close, `repo service host stop` for the resident service that can hold more than 1 GB). With something of ours running the request waits as before (the run frees memory).
4. **Exact comparison.** Granted as soon as `available − floor` equals the request: test cases 8675 → 6627 MB in full ("fits"), 8674 → 6626 MB ("shrunk"), against the Python oracle. The progress line now prints `available − floor` and the floor.
5. **Progress says what it waits for and for how long.** `host-slice-owners-reserving` (EN/DE): `waiting for memory: 6553 MB wanted, 2048 MB are enough to start, 5782 MB are free above the 2048 MB this machine keeps in reserve ... if nothing else is running and they are not free after 120 s, it stops and says what to close`. Test renders it in both languages with all numbers.
6. **The graph doors too.** The operations `graph`, `show` and `dependencies` carry `minimumMegabytes: 3072` in `🔣️policy.json` (fixed blocks 5120/5120/6144 MB). A 16 GB host with 7000 MB available is admitted with 4952 MB instead of waiting (test). The 3072 is a guess from the fixed blocks, NOT a measured minimum: see section 8.
7. A worker started on a smaller grant gets `SEMIO_GRAPH_MEMORY_MB = max(1024, grant × grantShare)` (the explicit parent budget the graph code already reads), so the parent's collection threshold follows the grant (test: grant 2891 → 1445; full grant leaves the variable alone). `reserveGraphPlanCapacityV1` answers `megabytes` (granted).

Tests (all green): `🌡️host-capacity/🧪️tests/📉️shrink` 6 (process-side semantics: Python oracle on 13 cases incl. 4/8/16/32 GB, the exact-margin cases, gate end to end with a stubbed sample for refusal, shrink, never, starved, EN and DE, graph-plan reservation, the three doors); `🏡️resident/🧪️tests/📐️booking` 3. Neighbours: every test file of the process host-capacity module pass (24, 31, 5, 10, 4, 11, 4, 10, 14, 11, 3, 7, 5, 9, 2, 6, 3); build host-capacity: `🛰️planned-grant 3`, `🌤️followup 18`, `📈️memory-fit 4`, `🤫️quiet 3`, `🧮️graph-plan 4`, `🔼️upgrade 4`, `🛠️setup 1`, `🔁️restart 2`, `🪟️partial 2`, `🪜️phases 6`; resident `📨️protocol 23`, `🛑️cancel 37`, `🌡️pressure 7`, `🌐️host 32 of 33`. Real command through the changed code: `repo capacity status`.

Red before my change and not mine (seen, not touched): `🏊️farm` 3 (the farm's default size became the `script` kind, 768 MB, the tests still expect `build` 2048), `👥️agents` 1, `🧾️label` 1 (farm label), `🧮️arguments` (needs `SEMIO_TEST_PYTHON_BINARY`), resident `🌐️host` idle-retirement test, resident `🏗️provision` watcher custody, `🧬️policy-literals` two "inference" tests (literals in `🧭️script-inputs`, package M's area). The host-capacity entry of the policy-literals test passes.

## 3. Peak memory of the graph step: before and after

Measured here (not on a 16 GB machine, not by me running a graph: forbidden tonight): before, the slice booking was 6627 to 8192 MB regardless of the machine and was never granted on a 16 GB worker (55 minutes and 5507 s without a grant, worker records). After, simulated by the test seam (total 16384, available 6624 to 8875): the grant is 4576 to 6553 MB, at once.

What the step then holds, from package M's live observations (32 GB machine, a peer's graph at 03:21: `repo capacity status` shows 17 814 MB observed for a grant of 5120 MB): the graph parent has no heap bound (JavaScriptCore grows to about half the RAM); with M's collection every 16 projects the parent held 8.8 to 10.7 GB at 330 to 400 projects, 8 farm workers about 0.75 GB each. That is more than a 16 GB machine has. I did NOT make a cold graph fit 16 GB: the heap hint `BUN_JSC_forceRAMSize` is soft (M measured 5.5 GB resident for a 2 GiB hint), it can only be set in the launcher registry (`🚀️launcher/🗂️registry/🔣️.json`, `runtimes.bun.launch.environment`, a fixed map: no machine-sized value possible) or by re-executing the door, and I found no way to measure the effect tonight (no graph run allowed, 95 percent disk). What bounds the worker count by memory already: the farm asks the gate for its workers as elastic 1024 MB requests (M), so a 16 GB host never gets more workers than `free − floor` allows; the parent's 9 to 10 GB then leaves one or two workers, i.e. hours. The 16 GB answer is not to compute (section 4).

## 4. What a worker's native bootstrap still computes locally (after packages H, N, B, M, R)

`🔩️native/🥾️bootstrap/🐚️.sh setup` (steps in order, "what" = local work):
- Pinned tool downloads (rustup-init, Rust toolchain archives, uv, cmake, ninja, bazel, bun runtime, Go): fleet fetch tier of H (store, then cache CAS, then upstream; first machine downloads); local work = unpack and verify (CPU, small memory).
- Go launcher build (`ensure_launcher`): compiled locally (about a minute, under 1 GB).
- `repo run repo-mcp:install`: a small Rust or script build; Rust units come from the fleet unit cache (B).
- `repo dependencies sync --all`: install pack restore from the fleet (N: 12 s measured, 1088 hashed blobs, no `bun install`), receipt verification 7.7 s (M).
- `repo run workspace:setup` (the stuck step): the project graph. Local work: the slice load (reservation 2048 to 6553 MB), per-project records are taken from the fleet cache or claimed and computed (`createProjectFleetPortV1` in `project-nodes`, landed by M at 02:31: `[fleet] seeded/published/offered/waited/computed` counters printed at the end of the inference phase), the generation exchange (`graph: generation N restored from the remote cache`) when the group server has published a whole generation. Then the 30 Nx tasks of `workspace:setup` (script kind, 768 MB each, gate-bounded).
- Pending, not mine: the fleet has NO complete graph published yet. A peer's cold graph is running on this machine since 03:00 (pid 86954, 13.5 GB resident, 9 farm workers) and will publish per-project records and the generation when it finishes. Until then the first worker that arrives claims projects and computes them (single-flight claims split them between machines). The prompt tells the worker's agent to stop and report if it sees thousands of projects computed locally.
- Peak memory of the worker's graph step: not measured on a worker. By construction: slice reservation 2048 to 6553 MB, doors 3072 to 6144 MB, one gate grant at a time for the graph partition. With a fleet hit it is lookups: the host service (125 MB, 1.3 GB after long runs) plus one slice worker.

## 5. Fleet footprint on a worker (measured here, `ps`, 03:15 to 03:21, 60 s of samples)

| Process | RSS |
|---|---|
| replica daemon (`--boot-child`) | 387 to 692 MB within one minute (sawtooth of the garbage collector), 546 MB at 03:05; earlier measure: median 364 MiB, peaks 1.3 GB; restarts at every code landing |
| boot supervisor (`script.ts run` and `boot.mjs run`) | 19 + 24 MB |
| build agent (sidecar launcher + child) | 16 + 61 MB |
| (group server only) cache server 17 + 78 MB, scheduler 16 + 57 MB, remote-cache serve 59 MB | |

A worker pays for the fleet: about 0.55 to 0.85 GB steady (daemon 0.4 to 0.7 GB, supervisor and agent about 0.12 GB), up to 1.4 GB during a daemon peak. On a 16 GB machine that is 3.5 to 9 percent, and it counts as "unmanaged" in the gate's floor accounting. NOT reduced: the cheap lever (`BUN_JSC_forceRAMSize` and the malloc purge delay in the environment of the `--boot-child` spawn, `🔁️replica/🛟️supervisor/🟦️.ts:241` and `🥾️entry/🟨️.mjs:135`, as the slice workers and the resident service already have) belongs to the shared-worktree module: its landing protocol (`1113-landing-protocol.md`) demands its own lock, every test file of the module and a restart of all eight daemons; package P was running exactly that test suite while I measured, so I did not touch it. Recommended, unmeasured: 512 MB hint (expected to cut the sawtooth top from 690 to about 450 MB).

## 6. What the other packages must know

- I took over package M's `landing.lock` at 03:09:46 (owner file dated 02:29:26, older than the 40 minutes of the binding rule; M's last record was 02:26) and released it at 03:27. Edits of mine in the process module (`🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity`) and the build module (`🏗️build/🌡️host-capacity`, `🏡️resident`, `🔣️policy.json`, `⌨️command/🌐️host`).
- `🔣️policy.json` is part of the replica daemon's code digest (O): the daemons of all machines restarted once because of it (the daemon pid changed at 03:18).
- `HostReasonV1` has three new members (`shrunk`, `never`, `starved`); code that switches over it needs them (the gate and `describeHostReasonV1` are done; `hostBlockersV1` returns nothing for them, correct: they end or admit).
- A graph door (`graph`, `show`, `dependencies`) can now be admitted with 3072 to 6144 MB; the grant row carries the smaller number and `graphMemoryAllocationV1` reads it, so the parent's budget follows. Nothing asserts a graph run completes within 3072 MB.

## 7. Prompt for the worker

`prompt-worker-native-bootstrap.md` (this folder): check that the code arrived by `grep -c` of four strings plus the replica `status`, the exact command (`native-bootstrap.sh`), what each exit status and message means, what to report.

## 8. NOT verified

- Nothing ran on a 16 GB machine. Simulated: the gate with a stubbed host sample (`totalMegabytes: 16384`, `availableMegabytes: 6624 to 8875`, nothing running), the pool with stubbed `reserve`/`createWorker`, the policy and the schema. The real vm_stat/`totalmem` path, the real `reserveGraphPlanCapacityV1` against a live ledger beside other grants, and a slice worker started with the reduced `SEMIO_GRAPH_MEMORY_MB` are not exercised.
- The minimums 2048 (slice) and 3072 (doors) are declared, not measured: nobody has run a graph step under a measured peak on a small machine. If a step needs more than its minimum it runs under a reservation smaller than it uses; the gate's pressure checks and the slice pool's pressure watch are the guard, as before.
- Peak RSS "before/after" of a real graph step: not measured (a graph command was forbidden tonight, disk 95 percent, load up to 35). The parent heap hint at launch was not changed (section 3).
- The fleet graph (generation or enough per-project records) does not exist yet, so the "lookups instead of compute" path of a worker is not exercised end to end.
- Daemon footprint reduction not done (section 5). Windows and Linux (the cgroup branch of the booking is only simulated through the same function).

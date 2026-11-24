# Results: Fleet Memory Watchdog, Admission and Unit Carving (2026-11-24)

Detail per part: `measurement.md`, `design.md`, `impl-S1-warden.md`, `impl-S3-wiring.md`, `impl-S4-fleet-status.md`, `impl-C2-unit-ceiling.md`, `carve-C1-final-memory.md`, audits `map-A` … `map-E`.

## 1. What was wrong

Every memory mechanism of the repo only refused or delayed starts. Nothing acted on a process that already ran, the admitted ceilings on a 16 GB worker add up to far more than 16 GB (graph read up to 9.8 GB, pool children 4.9 GB, stage up to 8 GB, one lent final up to 6.5 GB), `repo test-file` and several tool paths bypassed admission altogether, and no worker published its available memory, swap or compressor state. In 5 answered fleet readings within 21 minutes, six of eleven Macs were at `warn` or `critical` at least once.

## 2. What exists now

### Host warden (`🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/`)

- One detached process per machine. Subjects are classified from the process table by authored patterns (remote unit, local unit, pool child, farm worker, warm deriver, heavy question, test, slice worker, stage), so no launch path has to register and none can escape.
- Declared ladder on available memory: admission holds below 15 % (existing gates); the warden suspends (`SIGSTOP`) below 10 % or when swap is worked, cancels (`SIGKILL`, stage `SIGTERM`) below 5 %, at OS level critical, or after 120 s of suspension; it resumes above 15 % after three calm ticks. Order: cheapest to redo first (rank, then least elapsed, then largest); at least one subject always keeps running.
- Every action writes its record first, then signals, then alarms in English and German, naming the process, its kind, its owner, its megabytes, the action and the host numbers (`alarms.jsonl`, `state.json`, warden log).
- A starting warden resumes whatever a dead predecessor left suspended; every exit resumes its own suspensions; a changed source digest ends the warden and the next caller starts the new code.
- Started (cheaply, memoised) by the capacity gate, the remote-execution worker, the Ready supervisor tick and `repo test-file`.
- Requeue: a worker task whose compiler the warden cancelled is reported `refused` (`warden-cancelled`) and placed on another machine; a cancelled heavy-question child is retried.
- Mode: one schema value (`Policy.defaultMode`) for the fleet, a machine-local `mode` file to override, a `protect.json` list of workspaces never touched.

### Visibility

- Each worker mirrors its warden state to `<store>/v1/origins/<origin>/memory.json`; `repo fleet status` shows available, swap, compressor, level, mode and interventions per worker and raises `worker-memory-tight`, `worker-warden-intervened`, `worker-warden-absent`, `worker-warden-observing`, `worker-swap-high`, `worker-warden-overhead` (EN/DE). The path does not use the scheduler endpoint.

### Unit ceiling

- `Policy.unitCeiling` in the native-build admission schema: 4096 MB per compile unit for the 16 GB class, with a closed carving list (cad 6,550 MB, puzzle 6,148 MB) that may only shrink; a law (`unitCeilingVerdictV1`) and a runtime line in the build when an unlisted unit exceeds it.

## 3. Proof with numbers

### Tests (all through `repo test-file`)

| suite | result |
|---|---|
| warden (decision vs. independent Python oracle: 38 decision cases, 15 classification cases; mounted tests with real children: stopped state `T`, resumed, killed, cancel record, alarms EN+DE, lock, predecessor recovery) | 19 pass, 0 fail |
| host-capacity gate (after wiring) | 28 pass |
| host sampling (after `compressedMegabytes`) | 12 pass |
| remote-execution worker / Ready supervisor | 10 pass / 58 pass |
| owner-isolation / heavy-questions / test-file | 23 / 16 / 16 pass |
| fleet status (5 new tests, extended oracle) | 23 pass |
| unit ceiling (14 cases vs. Python oracle) | 6 pass |
| native-build admission main suite | 14 pass, 3 fail: the three real-cargo tests, because the coordinator's `.cache/b/r/fixed.dmg` does not mount (unrelated, existed before) |

### Live fleet (workers picked the code up through the replica by themselves)

- 12 machines publish `memory.json` (11 Macs/Linux and the Windows worker); 11 run the warden in `enforce` since 03:5x, Windows is observe-only by platform, the coordinator is `observe` with the play clone protected.
- Classification on real workers matches: `stage`, `slice-worker` (0.5–4.1 GB), `pool-child` (up to 2 × 2.0 GB), `local-unit` seen live.
- Overhead on the 16 GB Macs over 12 minutes: 0.4–0.9 % of one core (`cpuBasisPoints` 40–90), tick p95 35–81 ms, 40–104 MB resident. Coordinator (load 50+): tick p50 64 ms, p95 211 ms.
- Per-worker memory in the first 12 minutes (observe): minimum available 3.1–6.3 GB (guard line 1.6 GB), swap used up to 3.1 GB on the Macs and 8.9 GB on the Linux worker (swap-out peaks 110 MB/s there), compressor up to 6.4 GB; no level left `calm`, so no intervention was due.

### First live intervention: g1-m10 (16 GB), 03:43:22–03:43:29

From the worker's own `memory.json` (`lastAlarms`), warden in `enforce`:

| time | host | level | action |
|---|---|---|---|
| 03:43:22 | 2,393 MB available, swap traffic 243 MB/s, OS pressure warn | guard | suspended `local-unit` pid 51355 (254 MB) |
| 03:43:23 | 2,280 MB available, 57 MB/s, OS pressure **critical** | danger | cancelled pid 51355; suspended `local-unit` pid 51311 (`rustc semio_s_artifact_stdio_jpg_inferences`, 454 MB) |
| 03:43:25 | 2,463 MB available, 150 MB/s, critical | danger | cancelled pid 51311; suspended `local-unit` pid 51419 (34 MB) |
| 03:43:29 | 3,097 MB available, 58 MB/s, warn | calm | resumed pid 51419 |

OS pressure level critical is the state in which macOS starts its own measures; it lasted about three seconds here and ended after two compile units were cancelled, cheapest first. The warden kept ticking and publishing throughout (record age 5 s afterwards), the stage, the graph slice worker and six other compile units kept running, counters: 3 suspended, 1 resumed, 2 cancelled. Swap on that worker stood at 4.5 GB with a 8.0 GB compressor afterwards: the machine was at its edge, which is what the earlier sampling showed for it (`warn`, refusing).

Seen in this event and not yet changed: the alarm text says "available memory is below the guard/danger line" although the trigger was swap traffic and the OS pressure level (available was above both lines); the decision should carry its trigger into the text. The resume came four seconds after the last cancel (three calm ticks at the 0.5 s tight interval): the hysteresis is short for a host whose swap is still worked.

### Not proven

- One live intervention was observed (below); a second machine or a heavier case (a final, a read tree) has not occurred yet.
- "Zero OS pauses" can only be claimed for the observed window; macOS writes no record the repo can read from a worker.
- A complete Ready `workspace` run on a worker was rare tonight for reasons outside memory (stalls), so a before/after of one full run could not be taken.

## 4. Carving: state

- The two units above the ceiling are the hub finals `cad` and `puzzle`. Their settings: profile `release-wasm`, opt-level `s`, 16 codegen units, **thin LTO over the whole dependency closure** (cad 1,041 crates, puzzle 1,192), one front-end thread, serial back end at a one-job grant. CPU/wall 0.9–0.97: they run single-threaded, more threads cannot lower the peak. The thin-LTO stage is the likely owner of the peak; this is a hypothesis.
- It could not be measured: the coordinator's Rust cache image for the main checkout does not mount, no warm dependency set exists, and a cold cad build is 60–90 minutes while another session's scratch build runs. The guarded harness (`carve-C1-run.sh`) and the variant matrix are ready.
- Until then both finals are on the carving list, admitted at their measured peak, and are subjects of the warden (`remote-unit` / `local-unit`) like every other unit.
- Graph read: its own guard already settles and refuses dynamically against the host's reserve; its 60 % ceiling was left as tuned. The read tree, its heavy children, the farm workers and the pool children are warden subjects now.

## 5. Open

1. Measure the finals (matrix in `carve-C1-final-memory.md` §5) and apply the per-unit LTO setting or the split; remove the entries from the carving list.
2. Native-build broker start does not call `ensureHostWardenV1` itself (its builds run under the gate, which does).
3. Windows: observe-only (no suspend), its process walk costs 34 % of one core per 2 s tick, so its tick is 15 s; a native sampler would lift both limits. The same native sampler would cut the macOS tick (three short-lived commands per tick today).
4. Linux and Windows behaviour is seen live (records arrive) but no enforce action was exercised there.
5. The Ready stage peak is still summed RSS (the stage claim learns from it); the footprint measure of the gate would be more exact.
6. German texts in the warden schema use `ae/oe/ue` transliterations in two descriptions.
7. Alarm text names the available-memory line even when swap traffic or the OS pressure level was the trigger; calm hysteresis of three ticks is 1.5 s at the tight interval.

## 6. After the switch to enforce (03:43–04:07)

| worker | trigger | action | outcome |
|---|---|---|---|
| g1-m10 | OS critical, swap traffic 243 MB/s | 2 compile units cancelled, 1 suspended | calm after 7 s, resumed |
| g1-m02 | OS critical, 1,889 MB available, 242 MB/s | `rustc host_laws` (631 MB) cancelled, pool child (223 MB) suspended | 7,434 MB available, resumed |
| g1-m10 | OS critical, 1,937 MB available, 367 MB/s | final `semio_hub_draw_component` (2,000 MB) cancelled, slice worker (294 MB) suspended | 7,158 MB available, resumed |
| g1-m01 | OS critical, 2,316 MB available, 486 MB/s | `rustc semio_s_artifact_stdio_semio` (2,531 MB) cancelled, 1 unit (406 MB) suspended | 6,500 MB available, resumed |

Every event was triggered by the OS pressure level with heavy swap traffic while 1.9–2.4 GB were still available; each returned to calm after one or two cancels and each worker kept publishing. Not verified: whether a cancelled local final is cancelled again when its pass restarts on a worker that stays at its edge (g1-m10).

The Windows worker's record flapped between absent and calm: its 15 s observe-only tick plus the slow process walk passed the 30 s freshness bound; the tick is 8 s now.

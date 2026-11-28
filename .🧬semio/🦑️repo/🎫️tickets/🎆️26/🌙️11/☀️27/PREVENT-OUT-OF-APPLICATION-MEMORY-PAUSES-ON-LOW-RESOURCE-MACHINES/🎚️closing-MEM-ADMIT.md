FINAL (2026-11-27 23:58: elastic reservations, sidecar measurements, memory-bound cargo jobs and stale-trust estimates implemented; 120 of 124 host-capacity test files green, the 4 others fail for reasons outside this change (section 4); typecheck 0 errors)

# 🎚️ Closing Report, Lane MEM-ADMIT

Two problems, one machine class (8 GB, 4 cores, `📊️unit-peaks.md`): the queue starved because holders were charged what they feared, not what they used, and cargo ran as many rustc as there were cores. Scripts kept beside this report: `🎚️replay-waits.py`, `🎚️replay-simulate.ts`, `🎚️estimate-audit.ts`, `🎚️estimate-decay.ts`. Only the host-capacity code, its policy, schema and tests changed (list in section 7).

## 1. Elastic reservations

Where the starvation came from (live status of 23:19, `repo capacity status`): grants of 1900/3600/4599 MB reserved held 852/1557/2746 MB, so 4152 MB were "reserved but not yet resident" and the room read -896 MB (status: -2558 MB spare) with 5304 MB available; seven requests of 1.2 to 3.6 GB waited 7 to 22 minutes. The old rules shrank a retained grant only after 15 minutes (`settledAfterMilliseconds`) and kept at least 3072 MB of an idle ramping grant (`rampIdleKeepMegabytes`), which on an 8 GB machine is the whole session cap.

New rule (`elasticHostMegabytesV1` in `📏️decision/🟦️.ts`, a pure function with a Python oracle):

- A grant that retains its reservation (explicit `--memory`, cargo orchestration, graph plan) is charged its whole reservation while it warms up: 60 s since admission/resume and 3 heartbeats.
- Then its charge converges linearly within 120 s to `peak + margin`; margin = the largest of 256 MB, 3 standard deviations of its own measurements, 15 % of the slack its class estimate (the reservation) still holds above the peak.
- It is never charged under its observed peak or what it holds now, never over its reservation. The charge of a retained grant is the smaller of this and the old settle rule, so nothing got more expensive. Not elastic: quiet grants, holders and farm children (unchanged).
- Freed headroom is admittable at once (`reservationOf` feeds `unrealized`, `committed`, `headroom`).
- Grow-back: the peak only ratchets up and the charge follows `max(peak, observed)` at the next heartbeat (15 s); at its reservation the grant is charged in full again, so nothing more is admitted into its room. Above its reservation the existing `committedOf = max(charge, observed)` applies. The warden still measures every grant against its whole `reservedMegabytes` (excess >= max(512 MB, 25 %) and no spare memory; MEM-GUARD), so the over-reservation that elastic admits into is exactly the room the warden protects: it suspends or cancels only on real danger.
- Measurements travel in a sidecar `grants/<id>.stats` (schema `GrantStats`: peak, observations, deviation; Welford at each heartbeat), not in the grant row. Reason: a process of the previous version reads a grant row with an unknown field as `unrecognizedGrant` and refuses admission. With the sidecar every running process of the old code keeps working, reads the row, charges the whole reservation (safe) and ignores the file; processes of the new code merge it (`withGrantStatsV1` in `🎫️gate/📖️status`). The sidecar is removed with the grant and by the stale reclaim.
- Policy `hostCapacity.memory.elastic` in `🏗️build/🔣️policy.json` (schema `Policy.memory.elastic`, closed object).

### Proof with numbers

Today's grants are deleted on release and the metric rings keep no per-grant history, so the replay uses the two real records that exist: the `[capacity] waiting` lines of today's measuring runs (2672 lines, 8 wait episodes in `🗑️generated/*.log`, needs of 1800, 2200 and 2500 MB, median spare 471 MB, median "reserved but not yet resident" 3079 MB) and the live gate snapshot of 23:19, decided by the real `evaluateHostCapacityV1`.

1. Contract test (`🧪️tests/🎚️elastic-reservation`), the 23:19 snapshot with the oracle's numbers: charges 1108/3260/1960 MB instead of 1900/4599/3600; unrealized 1173 instead of 4152 MB; headroom +2083 instead of -896 MB. The 1200 MB and 1900 MB requests that waited 7 minutes are admitted, 3600 MB still waits (the room is real), the same snapshot without the rule reproduces the starvation. A grown-back grant (holds 3300 MB, peak 3400) is charged 3600 MB again and nothing is admitted into its room.
2. Replay of the recorded waits (`🎚️replay-waits.py`, each running grant assumed to hold 43 to 60 % of its reservation as the live ones did): the memory rule alone predicts 4867 s of waiting without elastic charges (the recorded waits, 14903 s, also contain queue-order waits the model does not know) and 234 s with all grants converged (-95 %; 6 of 8 episodes fit at once instead of 0), 2152 s with only half of them converged (-56 %). The longest recorded wait is 3767 s.
3. Full replay through the real admission function (`🎚️replay-simulate.ts`: the four live grants and seven waiting requests of 23:19 plus a request every 10 minutes for two hours, grants ramp to 45 % of their reservation, 8 minutes each; foreign memory 5857 MB as measured), same workload in both worlds:

| world | policy | admitted in 2 h | still waiting | mean wait | median wait | max wait | least available |
|---|---|---|---|---|---|---|---|
| 16 GB, 10 cores | rigid | 19 | 0 | 1547 s | 2201 s | 3079 s | 5298 MB |
| 16 GB, 10 cores | elastic | 19 | 0 | 1225 s (-21 %) | 1325 s (-40 %) | 2604 s (-15 %) | 5177 MB |
| 8 GB, 4 cores | rigid | 13 | 6 | 3570 s | 3600 s | 4519 s | 4025 MB (floor 1536) |
| 8 GB, 4 cores | elastic | 15 (+15 %) | 4 | 3111 s (-13 %) | 3090 s (-14 %) | 4164 s | 3755 MB (floor 1536) |

The gain is smaller than the model of 2. because the head of the line (3600 MB) and the 5.9 GB of foreign memory bind, not the reservations alone. The host stays 2.2 GB above its floor on the 8 GB machine.
4. Worst case, every grant grows to its whole reservation at +30 min (`GROW=1`): rigid and elastic admit the same number (6 and 6 on 16 GB; 2 and 2 on 8 GB) and reach the same least available memory (515 MB, 1380 MB): the grown grants are charged in full again before anything else is admitted. Both worlds go under the floor in this synthetic worst case, which is the room the warden protects (it acts on real danger, not on the charge).
5. The 17:30 incident cannot be admitted into: the counters of 17:30:20 (2980 MB available, swap 3060 of 3220 MB, 705 MB/s out, pressure warn, compressed 8.4 GB) refuse every request of 512, 1200 and 1900 MB with the hard stops `swap`, `swap-activity` or `pressure-warn`, and `decideHostCapacityV1` evaluates those before it reads any charge (test `the 17:30 incident counters block every request`). The hard stops of `hostSwapStopV1` and the guard lines of MEM-GUARD are untouched.

### Learning in `estimates.json`

Audit of today's store (`🎚️estimate-audit.ts`, `🎚️estimate-decay.ts`):

- Classes that still carry weight (`service:graph`, `slice:graph`): estimate = 1.45 x median peak, and the maximum peak is under the estimate in 3 of 3 classes: the 90 % quantile plus 25 % margin is a margin, not a bias.
- The systematic error is the decay: 32 of 35 classes have fewer than 3 effective samples (newest sample 7 to 15 days old, half-life 3 days), so every class was blended back to the 100 % default: `preflight` learned 22 % and answered 78 %, `run:script` 22 % and 60 %, `dependencies:ensure` 20 % and 87 %, while the heavy classes were pulled down as well (`cargo:test:...shooting` learned 218 % and answered 144 %). 13 classes answered more than they learned, mostly the light ones.
- Fix: `estimates.staleTrustBasisPoints` (7000), one-directional: evidence that a class needs less than the default (or than its parent class) keeps at least 70 % of the share of `minimumSamples` it stores as trust, so an idle weekend does not wipe what it learned; evidence that it needs more earns only the trust its decayed weight gives (an old heavy run never raises a reservation by itself; a first version that raised heavy classes as well pushed quiet-growth requests of two replay tests of the product above the fallback ceiling and was narrowed). Recency order and quantiles are unchanged. Effect on the same store: `preflight` 78 to 45 %, `run:script` 60 to 45 %, `run:script:repo-mcp:install` 60 to 30 %, `dependencies:ensure` 87 to 71 %, `phase:assembly` 81 to 56 %, `tools` 107 % unchanged; the heavy classes (`cargo` 134 %, `run:build` 151 %) are unchanged. Test `Law: a class nobody ran for days ...` compares 32 states with the Python oracle (changed in step) and checks that an old heavy class is not raised.
- Not learned at all: the class `command` (an explicit `repo capacity run --memory=N`, the requests that waited 40 to 100 minutes). Their size is the caller's number; elastic reservations are what corrects them, from the grant's own peak. Learning a per-label class for `command` is the follow-up if callers keep asking for 3600 MB for 450 MB of work.

## 2. Cargo parallelism from memory

The 8 to 12 GB cargo classes are sums of up to ten rustc at once; the jobs came from the cores, and the learned ratio priced a job below what one rustc takes.

- `cargoUnitsFittingV1(megabytes)` (`🦀️cargo-grant/🟦️.ts`): the most rustc units whose worst case fits, with the unit classes of the measured peaks (`unit-peaks.md` 2.3): the largest class 2150 MB (`kernel_event_receiving`, 2006 MB RSS), then 1450 MB (`semio_framework_kernel`), then the typical unit 1088 MB (129 of 131 units measured 33 to 1064 MB) for every further slot. Examples: 1764 MB (8 GB host, session cap 3300 MB minus the 1536 MB fixed block) gives 1 unit; 5632 MB (16 GB host) gives 3; 11216 MB gives 9. Constants live in the cargo grant schema with their source.
- Where the grant sets the parallelism:
  - Request sizing, both entries: `createToolAdmissionV1` (`🧰️tools/🏃️run`, ad-hoc cargo of the tool registry) and `admitBuildCapacityV1` (`repo cargo` and every build of the product) apply `boundCargoRequestV1`: every job priced at least 1088 MB and the preferred jobs at most the units that fit the most a request may ask for (50 % of the memory above the floor). Cargo `-j`/`--parallelism` and `CARGO_BUILD_JOBS` follow the admitted jobs, which are now memory-bound (an explicit `-j 16` on an 8 GB host is granted 1).
  - Compile units of a running build: `planCompileUnitsV1` gets `reservedJobMegabytes` (the grant's reservation above its fixed block, read by `readLiveHostCapacityJobMegabytesV1`) and limits the units by `cargoUnitsFittingV1` (new reason `reservation`, bilingual en/de text); this is what sets `CARGO_BUILD_JOBS` and `SEMIO_COMPILER_PARALLELISM` in the native build. 8 GB / 4 cores: 1 unit; 16 GB / 10 cores: 3; 64 cores with a large reservation: half the cores as before.
  - Nx task parallelism and Bazel local resources keep following `handle.jobs` / the plan (`SEMIO_BUILD_PARALLEL`, `--parallel`), which are the admitted jobs; the envelope per job stays the default 2048 MB.
- With a higher honest reservation per cargo job the elastic charge is what keeps the queue moving: the reservation converges to the real peak after warm-up.
- Tests: `🧪️tests/🦀️cargo-grant` (units, request bound, 240-point grid against the Python model, 8/16/64 GB admissions: 1 job, at most 3, never more than -j), `🎟️capacity/🧮️units/🧪️tests` (6 x 4 x 15 x 2 grid of states against the oracle, 8 GB and 16 GB cases).

## 3. Takeover without a restart

Nothing was restarted. The gate runs in every `repo` process, so new processes use the new code and old ones keep working: the old code reads the unchanged grant rows (measurements are in a sidecar it ignores) and charges in full; a policy or estimate file is read per process (`estimates.json` format unchanged). The warden loads its sources and ends itself on a source change with the `--handover` successor of MEM-GUARD; I did not touch it. Grants of processes started before this change have no sidecar and stay charged in full until they end.

## 4. Verification

- `repo test-file` over 124 test files (all of `host-capacity`, `🎟️capacity` incl. compile units, the product's `host-capacity` and `🧰️tools/🏃️run`), final run: 120 passed, 4 failed. A first run over the 66 files of `host-capacity` before the cargo and sidecar changes: 66 passed.
- The 4 failures are not caused by this change:
  - `🎟️capacity/🧪️tests/🪆️scope` (2 tests) and `🔐️inheritance` (1): a source census of the callers of the compiler capacity (expects 100 literal calls, finds 99) and of the cargo launcher; both read sources I did not touch (other lanes edit them).
  - `🌤️followup` (g13/g29, 2 tests): "reservation grown to 9126 MB" above the 8192 MB fallback; fails identically with `staleTrustBasisPoints` set to 0, my request bound is the identity for every operation but `cargo`, and the fixture `🌤️followup/🧫️fixtures/🔣️.json` is modified in the worktree by someone else.
  - `🧹️disk-recovery/🖼️image-slack` (1): the product's partition table, unrelated.
  Two of my own earlier failures were real and fixed: the gate test counted the files of `grants/` (the sidecar is now filtered out) and the g31 grow replay enshrined "2048 MB carry two compile units of 900 MB", which is exactly what the new rule replaces (now 1 unit, reason `reservation`).
- New and extended: `🧪️tests/🎚️elastic-reservation` (7 tests, Python oracle), cargo grant (units, request bound, 8/16/64 GB admissions, sidecar), compile units (reservation grid against the oracle), estimates law (oracle in step). Fixtures hand-authored from the live gate; expectations from the Python oracles.
- `repo typecheck` of the 12 changed sources and their 237 importers: 0 errors.

## 5. Findings for the other lanes

1. The recorded waits are not only memory: "an earlier request is still waiting and holds the next reservation" is the barrier of the head of the line; with seven requests of 1.2 to 3.6 GB one 3600 MB head delays six smaller ones that would fit.
2. `command` requests are sized by the caller: 3600 MB for a job that held 450 MB. A per-label learned class would shrink the first minutes too.
3. `readGrants` throws `unrecognizedGrant` for a row with an unknown field and so blocks admission of every process of the old code; any future field on the grant row has the same transition problem, a sidecar does not.

## 6. Open points and honest limits

- Elastic charges act only on grants whose process runs the new code (they write the sidecar) and only once the reader runs the new code: the live queue of the other agents drains with the old rule until their processes end.
- The replay of the real wait lines is a model (it assumes how much of a reservation the holders used); the full replay through `evaluateHostCapacityV1` is the conservative number (-13 % mean wait on 8 GB, +15 % admissions). Both agree in direction.
- A graph plan is elastic too (its reservation was the "no-kill guarantee"). It is charged its peak plus margin; if it grows back, the next heartbeat charges it in full and the warden covers the host. If plans turn out to grow faster than a heartbeat in practice, exclude `graph-plan` in `elasticHostMegabytesV1` (one condition).
- Margins (256 MB, 3 sigma, 15 % of slack, warm-up 60 s, 3 beats, 120 s) are authored in the policy and were not tuned on a long history: today's grants give 26 to 470 observations each.
- The per-unit classes of rustc (2150 / 1450 / 1088 MB) are the peaks of one day of builds; they are conservative on purpose (the largest unit is assumed to run beside the second largest). A host with more memory per session gets more units: raise the classes only with new measurements.
- Not done: Bazel `--local_ram_resources` has no separate source of memory beside the admitted jobs; `command` class learning (section 1).

## 7. Files

Code and schema (all under `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/` unless noted): `📏️decision/🟦️.ts` (`elasticHostMegabytesV1`, charge in `decideHostCapacityV1`, types), `🧬️schema/🔣️.json` (`Policy.memory.elastic`, `GrantStats`, `Estimates.staleTrustBasisPoints`), `🎫️gate/🟦️.ts` (heartbeat sidecar, `readLiveHostCapacityJobMegabytesV1`), `🎫️gate/📖️status/🟦️.ts` (merge and reclaim), `📐️estimates/🟦️.ts` (stale trust), `🦀️cargo-grant/🟦️.ts` and its schema (units, bound), `🎟️capacity/🧮️units/🟦️.ts` and schema (reservation limit, reason texts en/de), `🧰️tools/🏃️run/🟦️.ts`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`.
Tests: `🧪️tests/🎚️elastic-reservation/{🟦️.ts,🐍️.py,🔣️.json}` (new), `🧪️tests/🦀️cargo-grant/*`, `🧪️tests/📐️estimates/🟦️.ts` and oracle, `🎟️capacity/🧮️units/🧪️tests/{🟦️.ts,🐍️oracle/🐍️.py}`.

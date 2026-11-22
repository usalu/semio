# 🧭 Census of Fixed Resource Thresholds for Low-Resource Machines

Read-only audit (static reading, no runs). Reference machine: 16 GiB RAM (16384 MB), 8-10 cores, 60 GB free disk.

Abbreviations for paths:
- `L` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`
- `P` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build`
- `C` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching`

## 1. Memory: host capacity gate

Authored values are in `L/🔣️policy.json` under `hostCapacity`. Decision logic is in `P/🌡️host-capacity/📏️decision/🟦️.ts`.

| Threshold | Value | Where | Scales with RAM? | Unmet behaviour | 16 GiB result |
|---|---|---|---|---|---|
| Free-memory floor | `clamp(12.5% of total, 1536, 4096)`, never above total | policy `memory.floorBasisPoints/Minimum/Maximum`; `decision.ts:107-111` | Yes, up to an absolute cap of 4096 (reached at 32 GiB) | Reduces cap, no wait by itself | 2048 MB |
| Cap and budget | `cap = total - floor - unmanaged`; `budget = cap x 100%` | `decision.ts:191,196` | Yes | Wait (`memory`) | 14336 MB |
| Observed room | `available - floor - unrealized` | `decision.ts:196` | Yes | Wait (`memory`) | depends on free RAM |
| Soft stops | `pressure-warn`, open files above 80% of max, 32+ attached images | `decision.ts:120-125`, policy `files`, `attaches` | Partly (files and pressure are shares) | Overridable after `load.overrideAfterMilliseconds` (120 s) when nothing of ours runs (`decision.ts:245`) | bounded |
| Hard stops | `pressure-critical`; `swap` when swap used > 25% of total AND swap-out >= 1 MB/s; `swap-activity` when swap-out > 64 MB/s | `decision.ts:201-204`, checked at `:219` before any override branch | Swap ceiling scales; the 64 MB/s rate is absolute | Wait; **no override path**, so it waits for as long as the stress lasts | 4096 MB swap ceiling |
| Load ceiling | `perCoreCeiling 2 x cores` | `decision.ts:198-199` | Yes | Wait (`load`), overridable after 120 s | 16 to 20 units |
| Queue size | 256 waiters | `P/🌡️host-capacity/🎫️gate/🟦️.ts:282-283` | No | Typed refusal `queueFull` (actionable) | n/a |
| Grant liveness | `wait.staleMilliseconds` 6 h | policy `wait` | No | Not a wait bound | n/a |

### Operation catalogue (per-job and fixed megabytes)

Source: `policy.operations`, `kinds`, `envelopes`, `classCeilings` in `L/🔣️policy.json`. Requests are sized in `decision.ts:281-287`.

| Operation or kind | Fixed MB | Per job MB | Minimum MB (shrinkable) | Free RAM needed for one job at 16 GiB |
|---|---|---|---|---|
| `run`, `run-many`, `affected`, `exec`, `watch`, `cargo` | 1536 | kind max (test 1536, check 4096, build 2048, script 768, generate 1024, service 2048) | none | 1536 + kind + 2048 floor |
| `check` (kind) | | 4096 | none | 5632 + 2048 = 7680 MB available |
| `test-e2e` envelope (`^test-e2e`) | 1536 | 6144 (cpu 4) | none | 7680 + 2048 = **9728 MB available** (about 59% of RAM) |
| `show`, `graph` | 3072 | | 2304 | shrinks to free room, or `starved` after 120 s when nothing runs |
| `dependencies` | 3584 | | 2304 | same as above |
| `command` | 2048 | | none | 2048 + 2048 |
| `tool-format`, `tool-query` | 512 | | none | 512 + 2048 |
| `course-build` class ceiling | | 12288 cap | | exceeds 16 GiB host; ceiling only, never reachable on this host |

Learned estimates (`policy.estimates`) can shrink the per-job reservation, but the cold values above apply until samples exist (minimum 3 samples).

### Unbounded or unclear waits (memory)

1. **Non-shrinkable request that does not fit, with nothing of ours running.** `decision.ts:244-245`: override is only for `reason !== "memory"`, and the replace-path override needs a replaced grant. The gate prints a progress line every 5 s (`gate.ts:323-324`), but there is no deadline (`gate.ts:342`, `delay(... budget ?? Infinity)`). The `test-e2e`, `check` and plain `run` requests therefore wait without limit on a busy 16 GiB machine. The line names need, spare and holder, but never says "this can never fit here".
2. **`unsatisfiableHostRequestV1`** (`decision.ts:259-261`) is the designed escape (degrade after `wait.bypassMilliseconds` when drained room could not fit). It has **no callers**. It is dead code, so the escape is not wired.
3. **Hard stops never reach an override** (`decision.ts:219`). Swap or pressure-critical with nothing of ours running waits without limit. Only the reason changes in the progress line.
4. **Shrink path only for `minimumMegabytes` requests.** `starved` (`decision.ts:241`, thrown at `gate.ts:308-310`) is typed and actionable, but only reachable for `show`, `graph`, `dependencies`.
5. **Compile lease poll** `P/🎟️capacity/🎫️lease/🟦️.ts:82`: `while(!await pick())` has no deadline; only the control signal stops it.

### Refuse or degrade (positive)

- `never` is raised immediately when `minimumMegabytes > cap x budget` (`decision.ts:234`) or when a rebuild request exceeds the budget (`decision.ts:218`). Typed refusal, good.
- Compile units (`P/🎟️capacity/🧮️units/🟦️.ts:57-67`) degrade to one unit when memory is short. Never zero, so never a wait on memory.

### Inconsistency: two admission budgets disagree

- Host capacity budget (16 GiB): 14336 MB (`decision.ts:196`).
- Scheduler budget (16 GiB): `min(12288, total - systemReserve 2048 - daemonReserve 3072)` = **11264 MB** (`L/🗓️scheduler/💾️store/🟦️.ts:27-29`, policy `L/🗓️scheduler/🧬️schema/🔣️.json` `Policy`).

Both are authorities over the same RAM with different ceilings. The scheduler is stricter by 3072 MB. Scheduler admission always admits when nothing runs (`L/🗓️scheduler/🟦️.ts:156`), so there is no deadlock. Its lock wait is bounded at 2 x `lockStaleMs` = 30 s (`L/🗓️scheduler/💾️store/🟦️.ts:58`, typed error). Whether both gates apply to the same job was not verified.

### Cargo admission and compile parallelism

- Compile share: `preferred = max(1, floor(cores x 50%))`, `minimum = max(1, ceil(preferred x 50%))` (`P/🎟️capacity/🧮️units/🟦️.ts:27-31`). Core-relative, good.
- Memory limit: `floor((available - 2048) / unitMB)` with `megabytesPerUnit` 512, floor 256 after 8 samples (`P/🎟️capacity/🧮️units/🟦️.ts:63`, schema `Policy`). Reserve 2048 MB is absolute (12.5% of 16 GiB). Degrades, does not wait.
- Native cargo broker: `share = min(eff, reserveBytes + (eff - reserveBytes) x tokens/parallelism)` with `reserveBytes` 6 GiB, absolute (`/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts:33-36`, schema `reserveBytes` 6442450944). On 16 GiB this is 37.5% of RAM held as fixed reserve. Should scale.
- Workers: memory share 30% of RAM (`hostShareBasisPoints` 3000) with 512 MB per worker, `localJobs` maximum 8 (absolute). Scales in the share, absolute in the job count.

### Sampling (what "available" means)

On darwin, `P/🌡️host-capacity/📡️sampling/🟦️.ts:29-30`: available = free + max(inactive + speculative, file-backed + purgeable). This is generous, so the floor is measured against cache-inclusive memory. A 16 GiB Mac with browsers will often read "available" well below the true reclaimable amount; it still reports cache as available.

## 2. Disk: floors, budgets and readiness

| Threshold | Value | Where | Scales? | Unmet behaviour | 60 GB free result |
|---|---|---|---|---|---|
| Readiness stage needs (declared) | native 1024 MB; repo-mcp 1024; **dependencies 6144 MB** (+300000 inodes); tools 4096; workspace 4096 | `L/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json` `diskRequirements` | No (absolute) | Typed refusal before the stage starts (`ready/🟦️.ts:161-172`) | fits |
| Readiness measured need | `max(1024 MB floor, ceil(max growth over last 5 ok runs x 2))` | `ready/🟦️.ts:145-151` | Grows with observed growth, no ceiling | Refusal before stage | fits unless growth exceeds about 30 GB |
| Preflight minimum | 8 GiB free per platform | `L/🩺️preflight/🔣️policy.json` | No | Preflight refusal | fits |
| CAS store cap | 32 GiB (`capBytes` 34359738368), high 0.9, low 0.75 | `L/🗄️cas/🧬️schema/🔣️.json` `Policy` | No | Background collection, then wait on cap-exceeded (collect-now) | fits |
| CAS partition shares (declared) | high 0.9, low 0.8, critical 0.97 | same | Yes (fractions) | Collect; refuse at critical after one collection (`L/🗄️cas/🌡️partition/🟦️.ts:37-46`) | used when partition not derivable |
| CAS partition shares (derived) | `largestNeed` 6 GiB, cushion = need/4, reserve = need x 2 (family `safetyFactor`), derivable when partition >= (reserve + hysteresis) x 2 = 36 GiB | `L/🗄️cas/🌡️partition/🟦️.ts:49-65`, `L/🗄️cas/🧬️schema/🟦️.ts:43-45`, `L/📏️workspace-scale/🧬️schema/🔣️.json` | Yes | as above | on 60 GB partition: high 78.5%, critical 97.3% |
| Elastic image minimum (rust) | `minimumBytes` 32 GiB, `needBytes` 32 GiB, headroom 25%, max 96 GiB, host reserve 50% | `C/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` | Absolute minimum overrides the 50% host reserve | `hdiutil` refuses above free + current (ticket result-I) | 32 GiB taken, leaving about 24 GiB |
| Elastic minimums graph and products | 6 GiB each (products policy minimum 6 GiB; ticket notes say 24 GiB) | same | Absolute | same | 12 GiB |
| Fixed partition declared capacities | dependencies 36 GiB, tools 8 GiB (logical 64 GiB), diagnostics 2 GiB, nx 1.5 GiB | same, `partitions` | No | Sparse; create or resize refused when above free | 47.5 GiB declared |
| Elastic sizing rule | `target = min(wanted, max(50% x free, minimum))`, aligned down to 1 GiB | `C/💾️storage/🔒️enforcement/📦️budget/📐️elastic/🟦️.ts:11-14` | Ceiling scales, minimum does not | Minimum wins over the reserve | see above |
| Unit-cache GC shares | unit 0.5; lane pool 0.234375 of min(capacity, machine quota) (clamped to capacity, `C/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts:22`); incremental 0.1; admission 0.015; high water 0.875; low water 0.75 | `C/🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json` `Policy.gc` | Yes (fractions) | Collector; over high water waits | fits |
| Headroom floor | `max(1 GiB, 3% of volume)`; max reservation 15% of volume; minimum 128 MiB | `C/🦀️cargo/🗃️unit-cache/🧹️gc/🏦️headroom/🧬️schema/🔣️.json`, `🟦️.ts:35-38` | Yes, except 1 GiB floor | `space` wait until free >= floor + reserved; `alone` after cleanup | 1.8 GB floor, 9 GB max; fits |
| Headroom prior | cold 2 GiB, warm 768 MiB; component class cold 5.5 GiB, warm 3 GiB | same | No | estimate only | fits (5.5 GiB <= 9 GB) |
| Process disk refusal | `refuseLowSpaceV1` and `ENOSPC` mapping | `P/💽️disk/🟦️.ts:29-39` | n/a | Typed refusal naming partition, free and command | actionable |
| Disk growth constant | `EXPECTED_GROWTH_BYTES` cargo 4 GiB | `P/🌡️host-capacity/💽️disk/🟦️.ts:48` | No | estimate only | fits |

### Values the brief named that were not found

- `hostFreeFloorBytes` 15 GiB: not in the repository (grep for the name and the byte value 16106127360 finds nothing in source).
- "Laboratory floor" 60,129,542,144 bytes (= 56 GiB): not in source, policy or ticket notes.
- "rust 96 GiB budget on 40 GiB capacity": current policy has rust `capacityBytes` 96 GiB and elastic `minimumBytes` 32 GiB. The 40 GiB case is not in policy; the ticket notes discuss a 91 GiB host.
- "graph 6 GiB budget on 18 GiB": current graph `capacityBytes` 6 GiB, elastic minimum 6 GiB, maximum 24 GiB. No 18 GiB value.
- "6,144 MB dependency threshold": confirmed as `dependencies` in `diskRequirements` (6144 MB).

### Inconsistencies (budget > capacity or drift)

1. `tools` has `maximumLogicalBytes` 68719476736 (64 GiB) against `capacityBytes` 8589934592 (8 GiB), in `C/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`. Logical budget exceeds capacity by 8x.
2. Elastic minimums (rust 32 GiB, graph 6 GiB, products 6 GiB) plus fixed capacities (47.5 GiB) total about 91 GiB against a 60 GB disk. Sparse images make this legal on paper, but creation can be refused when the host volume cannot give the size (ticket result-I: resize refuses above free + current). Not verified on hardware.
3. Products minimum: policy says 6 GiB, ticket landing notes say 24 GiB. Drift to confirm with owner.
4. Scheduler budget 11264 MB vs host-capacity budget 14336 MB on 16 GiB (see memory section).

## 3. Caching library

- Headroom reserve: `floor + max share` = 1.8 GB + 9 GB = 10.8 GB on a 60 GB volume. Admission can always eventually fit once content stays below volume minus reserve (`headroomReserveBytesV1`, `🟦️.ts:38`). Good.
- Unit-cache shares: unit 0.5 plus incremental 0.1 plus admission 0.015 = 0.615, below high water 0.875. Whether the lane pool (0.234) is inside or added to the unit share was not verified in the code.
- Wait path `space`: `headroomDecision` waits for the commands ahead; with `reclaimed` false the `alone` admission needs cleanup first. Bounded by collector interval 5 min.

## 4. Scaling summary

Scales with machine size (fraction): host floor (12.5%, cap 4096), budget (100% of cap), swap ceiling (25%), CAS shares (derived from partition size), unit-cache shares, headroom 3%/15%, elastic ceiling (50% of free), load ceiling (2 x cores), compile share (50% of cores), workers memory share (30%).

Absolute: per-kind and per-operation MB, test-e2e 6144, check 4096, floor minimum 1536/1 GiB, scheduler ceiling 12288, scheduler reserves 2048/3072, cargo admission reserve 6 GiB, CAS cap 32 GiB, readiness disk needs (6144 MB for dependencies), elastic minimums (rust 32 GiB, graph/products 6 GiB), fixed declared capacities, preflight 8 GiB, queue 256, swap rate 64 MB/s.

## 5. 16 GiB / 60 GB verdict

| Requirement | Satisfiable locally? | Notes |
|---|---|---|
| `check`, `build`, `test` jobs | Yes when about 5.6-7.7 GB is free | Waits on free RAM; no escape if RAM stays busy |
| `test-e2e` job | Only when about 9.7 GB is free | Likely waits indefinitely on 16 GiB |
| `show`, `graph`, `dependencies` | Yes, shrinks to 2.3 GB | Starved after 120 s if nothing runs |
| Compile units | Yes, degrades to 1 | Never waits on memory |
| Readiness stages | Yes | Dependencies 6 GiB free required |
| CAS store | Yes | Derived shares on 60 GB partition |
| Elastic images | Marginal | 32 GiB rust minimum plus fixed 47.5 GiB declared; verify on hardware |

## 6. Proposed clean rule

1. Budgets are fractions of the machine, computed from total RAM and total disk, never from free. Absolute values are sanity bounds only.
2. Requests are clamped to the achievable local ceiling at request time (`min(request, budget_fraction x total)`). A request that can never fit locally is offloaded to the fleet scheduler (`:8981`) or refused at once with a typed `never` naming the numbers. No indefinite wait.
3. Every wait has a state that ends it: after `overrideAfterMilliseconds` with nothing of ours running, a non-shrinkable request either degrades (use `unsatisfiableHostRequestV1`, or delete it) or fails with `starved`. Extend this to hard stops, or emit an actionable line after a bound naming the foreign holder.
4. Single memory authority: derive the scheduler budget from the host-capacity budget (cap) so the two gates agree, or remove one.
5. Disk: derive the elastic minimums and fixed declared capacities from the host volume (fraction), and check their sum against free disk at plan time with a refusal naming the consumer.
6. Reconcile `tools` logical budget (64 GiB) against capacity (8 GiB), and the products minimum (6 GiB policy vs 24 GiB notes).
7. Make the cargo admission reserve a fraction of RAM instead of an absolute 6 GiB.

## 7. Caveats

- Static reading only. No run, no measurement on 16 GiB hardware. Hardware checks needed for: elastic image creation at 60 GB free, `available` on a loaded 16 GiB Mac, and whether the scheduler and host gates both apply to the same job.
- Not audited in depth: `P/🌡️host-capacity/🧺️unmanaged/🟦️.ts` (unmanaged memory reduces the budget), `P/🌡️host-capacity/🎫️gate/🤝️handover/🟦️.ts`, and the memory footprint `.mjs` modules (their numbers are trace thresholds, not admission).

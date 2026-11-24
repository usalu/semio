# Map E: prior memory findings, measured peak table, crate sizing

Read-only research. No build, no test, no git write, no source edit. Only this file was written.
Prior-ticket root: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`
(abbreviated below as `T13/`). Files named without a folder are in `T13/wave-1122-dist/`; `coordination.md` and `result-E*.md` are in `T13/wave-1122/`.

Note on `coordination.md`: two copies exist and differ. `T13/wave-1122/coordination.md` is 214 lines (the whole file, so the "last 250" is all of it) and runs to 17:27. `T13/wave-1122-dist/coordination.md` is 151 lines. The wave-1122 copy was used.

---

## Part 1: digest of prior findings

### H1-headroom (`T13/wave-1122-dist/H1-headroom.md`)
- Measured: at 15:26 fleet status, 11 of 13 entries refused (m01 m02 m03 m05 m06 m07 m08 m10 m11 m12 m13). CPU was not the cause (m01 load 3.0/12, m07 3.1/10, m10 5.1/10).
- Eight of the 11 refusals happen while a Ready stage claim is live. Seven come from the claim's memory hold (`entitlement - observed`). Fleet-wide the claims hold about 24 GB (11.9 GB across six 16 GB Macs, about 12 GB on m05, estimated).
- Per-unit admission uses a fixed 2048 MB, and the stage claim is subtracted before admission. Pressure-warn of a running stage refuses every offload (line 95 of `🌡️load/🟦️.ts`).
- Changed: nothing (audit only). Left open: `ready-stage-claim.json` not in the snapshot, so the hold is partly derived; scheduler's own per-machine table was auth-blocked.

### H2-headroom (`T13/wave-1122-dist/H2-headroom.md`)
- Changed (schema first, `remote-execution/`): stage claim = measured use x 1.25 (floor 256, `reservePercent` 125). Hold was 2457 MB on a 16 GB worker with 2.6 GB used, now about 650 MB; 0 while the stage waits on a named holder.
- Changed: per-unit admission by the unit's own `megabytes` (2048 only as default). `freeMegabytes` is now spare above floor. New `unit-admission` test 7/0 (Python oracle, 700 admissions).
- Changed: learned peaks per crate (`<crate>.peak`, larger at once, smaller halfway), written from local compile provenance; killed unit (exit 137) recorded as 2 x placed. Pressure gate narrowed (critical refuses all; warn refuses only units above 1024 MB).
- Live: 10 of 13 headroom true after landing; not proof, workers had not restarted.
- Open: learned peaks come only from local compiles; remote-only units keep the 2048 default (wasm 6144).

### O11-final-peak (`T13/wave-1122-dist/O11-final-peak.md`)
- Measured wasm component finals (from driver telemetry, 24 rows, 6 crates): editor 325/382/385 MB, surface 507/548, node_graph 1156/1229/1232, flow 1704/1857, puzzle 3279/5022/6148 (3 builds), cad 6550 (1 build, 2277 s, 214 MB output). Peak grows with output size.
- Changed: family peak (p90 of one sample per crate, minimum 3 crates) = 6350 MB; unseen final placed at 7938 MB; kill raises the family. Patch set for the play snapshot (6 files, not in main).
- Found: scheduler double-charged running units (spare already reflects them). Fixed in O12.
- Open: 45 `semio_hub_*` finals never measured; remote-only finals teach nothing (O12 fixes that).

### O12-single-charge (`T13/wave-1122-dist/O12-single-charge.md`)
- Changed: scheduler charges a running task only its declared peak minus measured memory (`max(0, declared - measured)`), ramp 120 s for old workers. Heartbeat `usage` added (schema); worker samples process-group RSS once per second (`ps`) and returns peak in `execution_metadata`.
- Live before fix: `/v1/fleet/idle` slots 68 @0, 4 @1024, 1 @2048, 0 from 3072. After: worker spare 1.5 to 5.3 GB on 16 GB Macs.
- Open: needs scheduler restart (automatic) and worker restart (self); Windows worker has no measurement.

### O14-local-jobs (`T13/wave-1122-dist/O14-local-jobs.md`)
- Cause: `componentJobs` only asked when `fleetRemote > 0`, so with offload never/fleet down the component lane was strictly serial.
- Changed: local component jobs admitted from memory: `peak <= available + owned - floor - reserved`, cap 3, normal pressure. Peak per pass from the unit history (p95 x headroom) or 6144 MB bound for unseen finals (p90 of recorded finals below 3 records: 6144).
- Floor is the broker reserve, min(25 % RAM, 6 GB) = 6 GB on 32 GB host. Unmeasured `semio_hub_*` finals reserved at about 7 GB.
- Tests: local 10/0 (oracle), job-slots 5/0, driver stub PASS isolated.
- Open: needs driver restart; `ownedMegabytes` counts only rustc RSS; FIFO no backfill of queue head.

### O15-final-tokens (`T13/wave-1122-dist/O15-final-tokens.md`)
- Cause: `semio-build-scheduler` added `-Z no-parallel-backend` to every cdylib final whenever a grant existed. Energy final (pid 66999) ran 61 min wall / 52 min CPU at RSS 0.7 GB, LTO on one thread.
- Fixed in main: flag only when the grant is exactly one job. Output identical; test real-rustc 4/0 (grant 4 CPU/wall > 1.25; grant 1 < 1.25).
- Play snapshot patched (4 files) and binary swap option (rm + cp + codesign). Driver restart needed for source patch alone.
- Open: remote offload mirror of the flag (fixed in O16).

### O16-remote-backend (`T13/wave-1122-dist/O16-remote-backend.md`)
- Changed: the remote cdylib path no longer adds the flag. Worker grants codegen threads from its own cores, load and spare memory (`codegen {maximumThreads 8, reserveCores 1, extraMegabytesPerThread 512}`), with a jobserver pool.
- Output bytes identical at 1/2/4/9 threads (wasm32-wasip2, 48- and 120-module crates).
- Tests: codegen 3/0, worker 8/0, cluster 2/0.
- Open: old workers run serial-uncapped until restart; Windows pool untested.

### W4-convergence (`T13/wave-1122-dist/W4-convergence.md`)
- Measured: 0 of 13 workers Ready at every look 22:40 to 23:05. Graph child reads 1 to 3 h on 16 GB workers holding 2.3 to 4.6 GB RSS.
- Changed: taken-owner memo now remembers before release (was empty under memory pressure); inventory rounds 12 to 4 with lenient last attempt; a worker whose stage runs lends at most 1 remote unit (`stageConcurrency`).
- Open: head has no durable retention class; stall rule kills reads; mapping-lease waits.

### GR8-read-followups (`T13/wave-1122-dist/GR8-read-followups.md`)
- Measured: whole-read OOM at 16:48. Heavy child grew 3.1 to 5.3 to 11.3 GB in about 110 s and died. Cause: journal `wave` built all rows as objects plus one buffer (about 4.5x bytes written).
- Changed: wave journal written in 8 MiB part files; regression test (new code 171 MiB added for 281 MiB journal vs old 313 MiB for 70 MiB).
- Measured effect of GR7: owners per minute 2.4 to 5.7 fleet-wide.
- Open: the read that OOMed ran old journal code; limiter is closure-row mismatch between machines.

### SUMMARY (`T13/wave-1122-dist/SUMMARY.md`)
- Wave goal: one fleet cache for all compile units. Proof P1: build 17 rustc + 4 build-script RUN, rebuild on another worker 0 compiled, 17/17 + 4/4 fleet hits, binary byte-identical.
- Day 2 and night addenda: 0 Ready workers (0 of 11 at 01:10); component final on a worker unproven; host CAS 27.75 GiB cap full; coordinator disk 96 to 97 % (2202 entries evicted); Play release not finished.
- Open: memory of finals on workers not proven; retention dry-run; closure narrowing deferred.

### coordination.md (`T13/wave-1122/coordination.md`, whole file, 214 lines, 00:40 to 17:27 on 2026-11-23/24)
- Memory-relevant timeline: 01:13 policy.json swap key reverted by a stale whole-file write (x3, repaired). 01:49 disk 97 %, cache CAS 41 to 14 GiB via gc. 03:20 prod break from another stale overwrite (restored). 06:16 E73 (16 GB stage admission) launched. 15:49 E61: slice "loop lag" is starvation (CPU 6 to 12 %, 30 s timer fired after 1114 s on m08), not JS blocking; E65 fixed double-counted remote actions in admission. 16:06 E73 landed (stage claim subtracted). 16:30 E54: seal failures were in-process OOM of the heavy child (10.8 GB, stream-patch retaining rows), fixed 15:52. 16:43 E77 generations content-addressed, collector paging. 17:27 E83 load-scaled test deadlines.
- Open at end of file: no worker Ready (1 of 12 at 15:00, 1 at 16:17 then lost); seal of the 605-owner selection not yet reached by any read; `tools` and `dependencies` stage admission failures on 16 GB machines.

### E73 worker-memory (`T13/wave-1122/result-E73.md`)
- Measured (15:50, all 13 workers are 16 GB Macs): slice process RSS 1.0 to 1.8 GB (m11 3.3 GB); stage tree peak 6.2 to 10.2 GB (RSS sum, includes shared pages); owner puller 3 to 6 children of 0.8 to 3.1 GB; agent took offloads (m10 running 3) while the stage was still small.
- Changed: `ready-stage-claim.json` (entitlement = 0.9 quantile + 25 % of completed runs, else 30 % of machine, floor 1024, cap 50 %). Pool and agent subtract entitlement - observed from available memory. Agent refuses offload under pressure while a stage runs.
- Oracle (16 GB): children 2 instead of 7 at available 8000 MB; 5 instead of 9 at 10000 MB.
- Open: no live proof on m08, m10, m12; 30 % default is a model value; real slice holds about 1.2 GB.

### E61 slice-loop-lag + watchdog (`T13/wave-1122/result-E61.md`)
- Watchdog 8 of 9 red fixed: adoption re-counted the already-logged progress as an advance, so every supervisor restart reset a stalled read's quiet clock once.
- Measured slice timer gaps: m08 1114 s, m10 178 s, m12 1 sample in 7 min; slice CPU only 6 to 12 % of wall. Conclusion: starvation/paging, not JS compute. No loop-yield fix landed.
- Lever: memory admission on 16 GB Macs (stage 10 GB peak + puller children together).
- Open: live check not possible until slice processes restart with E55 code.

### E18 learned estimates (`T13/wave-1122/result-E18.md`)
- Cause: failed/cancelled runs taught the estimate whenever the partial peak was higher (tools stage peak 6163 MB = 300 % of 2048 pushed the class to the 200 % cap). `command` had no minimum (4096 MB unshrinkable, waited 50+ min).
- Changed: only completed runs on unpressured hosts teach; learned peak clamped to 50 % of room above floor and class ceiling; `command` minimum 1024 MB; `starved` refusal after 30 min (`memoryWaitedOut`).
- Round 3: dry-run `phase:plan` learned 11,176 MB floor (thrashing samples up to 14 GB); fixed by `maximumPhaseMegabytes` 4096 and class ceiling 2304 MB for dry runs. Live: 2304 MB reserved.
- Open: stored poisoned samples decay only; non-dry single-target runs still learn up to 4096 MB.

### E7 memory thresholds and fseventsd (`T13/wave-1122/result-E7.md`)
- Changed: swap stop now needs occupied swap above 25 % RAM AND traffic above 0.6 bp/s of RAM AND available below 25 % RAM (was absolute 25 % + 1 MB/s). Absolute MB thresholds converted to shares (cold request sizes capped at 50 % of room above floor; cargo reserve 2 to 6 GiB by host size).
- Tests: scaling 4/0 with oracle over 8/16/32/64 GB.
- Measured fseventsd: 9.6 GB RSS, delivery 0 of 6 probes after all moves (cache index, old cache dir removed 02:33, worker roots move). Not recovered after 1 h.
- Open: fseventsd restart needs sudo (done by user at 12:45 per coordination.md); `/Users/ueli/Library/Caches/semio/host-capacity/estimates.json` written by the same lane.

---

### Consolidated list: known open memory gaps
1. No per-machine free-memory export in `repo fleet status --json` (observability gap, H1, O12).
2. `ready-stage-claim.json` not in any snapshot, so the stage hold could not be verified directly (H1, H2).
3. Remote-only compiles never report peak in the ledger before O12; the family estimate still depends on local compiles (O11, O12).
4. 45 of 47 `semio_hub_*` component finals have no measured peak (O11).
5. Final on a worker never proven (SUMMARY, P3); no live proof of codegen threads on a loaded fleet (O16).
6. Graph heavy child: healthy 3.0 to 3.6 GB, OOM 9.7/11.8/10.8 GB before the journal fix; live seal of the 605-owner selection not yet proven by a read after the fix (GR8, E54, GR13).
7. Stage claim 30 % default entitlement is a model value; real slice about 1.2 GB (E73).
8. Learned `phase:plan` and `slice:graph` classes still hold poisoned high samples (E18 round 3, host-capacity estimates.json: `slice:graph` max 8192).
9. Stage tree RSS sums count shared pages; `peakMegabytes` of a thrashing stage overstates (E73, E18).
10. fseventsd delivery not recovered after cache-index moves (E7 round 3).
11. Scheduler and worker restarts needed before O12/O14/O16 take effect; live effect not observed.
12. Windows worker: no memory measurement (O12), pool untested (O16).

---

## Part 2: measured peak table

### Where the learned-peak ledgers are on this machine
- `.peak` files (per-crate unit peak ledger from O11/H2): **none exist** in `.cache`, `~/Library/Caches/semio` or `/Users/Shared/semio-store/v1` (search for `*peak*` found only `owner-peaks.json` and the unrelated `speaker-view.html`). The per-crate ledger is in the play snapshot, which was not read.
- `~/Library/Caches/semio/host-capacity/estimates.json`: 22 operation classes, each with samples (`peakMegabytes`). Source for the learned rows.
- `.cache/b/g/m/inference/selections/*/*/owner-peaks.json`: 50 files, 8-element arrays per owner kind (format not documented in code found; units taken as bytes, reported in MiB as written).
- `/Users/Shared/semio-store/v1/origins/g1-m*/ready-timings.json` (12 workers; workspace 326 runs, dependencies 80, tools 55) and `ready.json` (one per worker).
- Numbers quoted in reports (O11, GR1, GR13, E73, H1, E54, GR8).
- Not read: anything under `/Users/ueli/.sinc/play` (as instructed). The play-snapshot telemetry quoted by O11 is cited only through the report.

### Table (MB, decimal; sorted descending; MiB converted where the source gave MiB)

| # | Unit | Kind | Peak MB | Source |
|---|---|---|---|---|
| 1 | graph read heavy child, OOM peak (whole read) | graph read | 11,800 | `T13/wave-1122-dist/GR1-graph-on-workers.md:35` (also 9.7 GB and 10.8 GB OOMs; GR8 11.3 GB) |
| 2 | workspace stage tree, worst (m07) | stage | 10,662 | `/Users/Shared/semio-store/v1/origins/g1-m07/ready.json` |
| 3 | workspace stage tree, stage-failed max (m06) | stage | 8,648 | `g1-m06/ready-timings.json` |
| 4 | slice:graph (learned class max) | graph read | 8,192 | `~/Library/Caches/semio/host-capacity/estimates.json` |
| 5 | dependencies stage tree (m05, Windows) | stage | 7,398 | `g1-m05/ready.json` |
| 6 | semio_hub_cad_component (wasip2 final) | component final | 6,550 | `T13/wave-1122-dist/O11-final-peak.md` table |
| 7 | semio_hub_puzzle_component (max of 3) | component final | 6,148 | `O11-final-peak.md` (min 3,279, median 5,022) |
| 8 | workspace stage, best ok run (m11) | stage | 5,873 | `g1-m11/ready-timings.json` |
| 9 | tools stage, stage-timeout max (m02) | stage | 4,194 | `g1-m02/ready-timings.json` |
| 10 | graph slice RSS, long inventory (m02) | graph read | 4,600 | `T13/wave-1122-dist/H1-headroom.md` section 1 table |
| 11 | graph heavy child, healthy read | graph read | 3,598 | `T13/wave-1122-dist/GR13-verify.md:35` |
| 12 | dependencies (learned class) | stage | 3,478 | `estimates.json` |
| 13 | dependencies, best ok run (m11) | stage | 3,451 | `g1-m11/ready-timings.json` |
| 14 | tools, best ok run (m03) | stage | 3,400 | `g1-m03/ready-timings.json` |
| 15 | tools (learned class) | stage | 3,322 | `estimates.json` |
| 16 | service:graph (learned class) | graph service | 2,560 | `estimates.json` |
| 17 | run-many:dry (learned) | command | 2,076 | `estimates.json` |
| 18 | semio_os_flow_component | component final | 1,857 | `O11-final-peak.md` |
| 19 | cargo:check:semio-framework-pack-json-source | cargo op (learned) | 1,984 | `estimates.json` |
| 20 | cargo:check (learned class) | cargo op | 1,984 | `estimates.json` |
| 21 | semio_os_node_graph_component | component final | 1,232 | `O11-final-peak.md` |
| 22 | cargo:check:semio-framework-async-timer | cargo op (learned) | 1,169 | `estimates.json` |
| 23 | cargo (learned class) | cargo op | 1,169 | `estimates.json` |
| 24 | semio_framework_surface_component | component final | 548 | `O11-final-peak.md` |
| 25 | semio_framework_editor_component | component final | 385 | `O11-final-peak.md` |
| 26 | preflight (learned) | command | 807 | `estimates.json` |
| 27 | run:dry, run:repo:test-cache-reset:dry (learned) | command | 656 | `estimates.json` |
| 28 | cargo:metadata (learned) | cargo op | 528 | `estimates.json` |
| 29 | cargo:build (learned) | cargo op | 490 | `estimates.json` |
| 30 | cargo:build:p-leaf | rustc crate (learned) | 313 | `estimates.json` |
| 31 | run:script (cache reset) | command | 110 | `estimates.json` |

Graph owner-peak arrays (`owner-peaks.json`, max per owner kind, MiB as written; kind label is the owner kind, not a single unit): language:typescript 1,752 MiB (about 1,837 MB, 16 files); language:cargo 1,746 MiB (about 1,831 MB, 23 files); project 1,496 MiB (about 1,569 MB, 38 files); root:language:cargo 633 MiB; language:rust 609 MiB; language:dotnet 477 MiB; language:go 88 MiB. Source: `.cache/b/g/m/inference/selections/*/*/owner-peaks.json`. These are not counted in the bands.

### Units above 2,000 MB (explicit)
Rows 1 to 17 and 20:
- 11,800 graph read heavy child (OOM)
- 10,662 workspace stage (m07)
- 8,648 workspace stage (m06 stage-failed)
- 8,192 slice:graph (learned)
- 7,398 dependencies stage (m05)
- 6,550 semio_hub_cad_component
- 6,148 semio_hub_puzzle_component
- 5,873 workspace stage (m11 ok)
- 4,600 graph slice RSS (m02)
- 4,194 tools stage timeout (m02)
- 3,598 graph heavy child healthy
- 3,478 dependencies learned
- 3,451 dependencies ok (m11)
- 3,400 tools ok (m03)
- 3,322 tools learned
- 2,560 service:graph learned
- 2,076 run-many:dry learned

Note: "unit" here includes stage trees and graph processes, which are not crates. Rustc-crate peaks above 2,000 MB are only the two hub finals (6,550, 6,148); the other crate-level finals are below 2,000 MB.

### Count of units in bands (31 table rows; 1 GB = 1,000 MB)
- under 1 GB: 8 (rows 24 to 31)
- 1 to 2 GB: 6 (rows 18, 19, 20 at 1,984 is in this band; 21, 22, 23)
- 2 to 3 GB: 2 (rows 16, 17)
- 3 to 4 GB: 5 (rows 11 to 15)
- over 4 GB: 10 (rows 1 to 10)
Total 31. The owner-peak arrays are excluded.

Note on row 19 vs 20: both 1,984 MB; row 19 is the pack-json-source cargo-check class and row 20 the cargo:check class. Both are in the 1-2 GB band, which makes the 1-2 GB count 6 (18, 19, 20, 21, 22, 23).

---

## Part 3: top 10 units, crate directories, cargo profiles

### Crate lookup result
Only two crate-level component units could be located as manifests in this checkout. The other measured hub finals could not be found in the checkout; their sources appear to live only in the play snapshot (not read).

| Unit (rustc crate) | Package | Peak MB | Manifest found | .rs files | .rs lines (`wc -l`) |
|---|---|---|---|---|---|
| semio_os_flow_component | semio-os-flow-component | 1,857 | `./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️component/📦️packages/🦀️rust/Cargo.toml` | 0 | 0 (manifest only, 720 bytes, no `src/`) |
| semio_framework_surface_component | semio-framework-surface-component | 548 | `./🧰️framework/🔨️modules/🗺️surface/🧩️component/📦️packages/🦀️rust/Cargo.toml` | 0 | 0 (manifest only, 1,566 bytes, no `src/`) |
| semio_hub_cad_component | not found in checkout | 6,550 | - | - | - |
| semio_hub_puzzle_component | not found in checkout | 6,148 | - | - | - |
| semio_os_node_graph_component | not found in checkout | 1,232 | - | - | - |
| semio_framework_editor_component | not found in checkout | 385 | - | - | - |
| semio-framework-pack-json-source | not found (package `semio-framework-pack-source` exists under `🎒️pack/🚰️source`) | 1,984 (cargo:check class) | - | - | - |
| semio-framework-async-timer | not found in checkout | 1,169 (cargo:check class) | - | - | - |

Searches run: grep for exact `name = "..."` in `🧰️framework`, `🌎️hub`, `🧰️framework/🛍️products`, `🧰️framework/🔨️modules/🗺️surface`, and `🏢️semio-tech/🎡️play`; find for `*cad*` and `*puzzle*` directories (the full `find` over Cargo.toml trees timed out at 120 s, so the search is not exhaustive).

Only 2 of the top-10 crate-level units have a manifest here. For the other rows the crate directory is not in this checkout (likely the play snapshot, not read). The remaining top-10 slots are non-crate units (stage trees, graph read), which have no crate directory.

### Cargo profile settings in effect
- No `[profile.*]` section exists in the three workspace roots: `./Cargo.toml` (`[workspace]` at line 3), `./🧰️framework/Cargo.toml` (line 3), `./🌎️hub/Cargo.toml` (line 3).
- No `codegen-units`, `lto`, `opt-level` or `debug` setting exists in those manifests or in the two crate manifests above.
- The two crate manifests contain only `[package.metadata.wasm-pack.profile.*]` tables (surface: dev, release, custom at lines 15 to 26; flow: release, custom at lines 15 to 18). These are wasm-pack metadata, not cargo profiles.
- `./.cargo/config.toml`: sections `[build]` (line 7), `[unstable]` (line 14), `[target.wasm32-unknown-unknown]` (line 27), `[target.wasm32-wasip2]` (line 45), `[target.'cfg(all(target_os = "linux", target_env = "gnu"))']` (line 54), `[env]` (line 67). No profile setting. Line 41 is a comment that mentions `opt-level = 0` for a wasm debug build (`--profile wasm-prototype`), which is a profile name used by the cargo command line and defined elsewhere (not located).
- Not exhaustively searched: manifests outside the three workspace roots (the full search timed out). A `[profile.wasm-prototype]` referenced by a comment was not found in the checked files.

Overall: the default cargo profile applies to these units unless a manifest outside the checked set overrides it. This means `codegen-units` defaults (16 for non-incremental release, 256 for dev) and `lto = false`, unless set elsewhere.

---

## Caveats
- The O11 and GR1 numbers come from the play snapshot telemetry as quoted in the reports. They were not re-measured.
- `estimates.json` peaks are sums of RSS across an operation's process tree and include shared pages; they overstate real footprint under thrash (E18, E73).
- Stage `peakMegabytes` in `ready.json` and `ready-timings.json` is the same RSS-sum metric.
- The `owner-peaks.json` value format is not confirmed; units are reported as written (MiB).
- Crate file counts for the two located crates are 0 because the manifests have no `src/` directory in this checkout.

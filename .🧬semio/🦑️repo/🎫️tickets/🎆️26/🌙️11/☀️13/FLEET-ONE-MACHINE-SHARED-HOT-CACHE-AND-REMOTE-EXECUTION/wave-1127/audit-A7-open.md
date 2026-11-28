# Audit A7: Open Items of Wave 1127

Read-only audit by lane A7 (haiku, foreground). Sources: every `result-H*.md`, every `findings-*.md`, and `coordination.md` (POST-SEAL and owed lines), plus `HANDOVER-1127.md` for the USER list. File names are relative to this wave folder. Lanes H77, H78 and H73/H74 were launched after the last result file was written (12:40 and 11:55) and have no result file yet, so they are listed as in-flight.

Types: `staged-for-seal` = patch or design waiting for the seal window / POST-SEAL queue; `needs-live-proof` = landed or measured in code, not yet observed on the fleet; `not-done` = known work not implemented; `USER` = needs a person.

## Staged graph patches and seal-window work (first)

| id | area | one-line description | type | source |
|---|---|---|---|---|
| H9-1 | graph seal | Whole-graph seal and `repo-mcp:build` / `repo-mcp:install`, then coordinator MCP restart; still pending after round 4 | staged-for-seal | result-H9.md (Not done); coordination.md:15, :123 |
| H60-1 | graph, resolve rows | Journal vouching of `resolve` rows (8117/8118 vouched) staged as `T/🧰️scripts/H60/land.sh`, not landed | staged-for-seal | result-H60.md (Landed-ready staged); coordination.md:132 |
| H60-2 | graph, post-land | After land.sh: landings entry `Wave 1127 H60`, test-file on three tests, typecheck, dangling-imports; 300 s owner-scan effect not measured | staged-for-seal | result-H60.md (Not done, line 140); coordination.md:132 |
| H56-1 | graph, per-owner memo | Per-owner / per-file analysis memo keyed by parser identity + file hash, and consulted dirs recorded in `resolve` rows (design, not coded) | staged-for-seal | result-H56.md (Next, staged ideas; single-owner design) |
| H56-2 | graph, service | Warm slice worker loop in `worker.ts` (saves about 10-15 s only), not done | staged-for-seal | result-H56.md:9 |
| H67-1 | graph custody | Graph hand-over limit `maximumStalledHandovers` 3 in sealed `B/graph` custody policy; proposal to count hand-over as progress or raise limit in landing slots | staged-for-seal | result-H67.md:30 |
| H48-1 | units, POST-SEAL queue | Units test 22 pass / 4 fail: resource control issuance (`Live Original resource control issuance required`) and Rust unit output inventory mismatch; listed on the POST-SEAL queue | staged-for-seal | result-H48.md (Status 09:15); coordination.md:137 |
| H75-1 | units, POST-SEAL | `cacheLookup` counter owed post-seal; latency budgets provisional | staged-for-seal | coordination.md:153 |
| H6-1 | process3d_host carving | Retained-laws cut staged in scratch, not applied (coordinator decision: cfg(test) absent from lib build) | staged-for-seal | result-H6.md:33, :38 |
| H39-1 | cargo retirement | Pruning custody-race fix: result header says STAGED, coordination says landed 05:00; treated as landed, residual Windows conditional filter unverified | needs-live-proof | result-H39.md:8, :19, :26; coordination.md:100 |

## Live proofs owed (landed or measured in code, not yet seen on fleet)

| id | area | one-line description | type | source |
|---|---|---|---|---|
| H1-1 | workspace stage | m03 own retry builds a fresh bundle (last retry ~00:10 not confirmed); no worker ready.json flipped in window | needs-live-proof | result-H1.md:11, :16, :27 |
| H1-2 | bundle guard | Bundle guard has no live end-to-end test (placement test only) | needs-live-proof | result-H1.md:28 |
| H2-1 | unit-cache registrations | Agent re-sync not exercised live; a real denial needs a second reading | needs-live-proof | result-H2.md (Unverified) |
| H3-1 | cache server / uploader | Live counters owed after cache-server outage fix | needs-live-proof | coordination.md:32 |
| H4-1 | offload gate | Live delta of offloadGateV1 unverified (counters cumulative since 11-14) | needs-live-proof | coordination.md:31; result-H4.md (Not verified) |
| H5-1 | fleet status | Windows m05 still on old ring ids; Linux m04 only via rings; `repo fleet status --json` still ~61 KB | needs-live-proof | result-H5.md:34-50 |
| H6-2 | build peaks | Fleet peak comparison for per-unit `-Z threads=1` owed | needs-live-proof | coordination.md:80 |
| H7-1 | Windows m05 tools/source | No Windows run; `source.served` hang and 64 MiB partition overflow on Windows compiler consumer unconfirmed; new census record needed | needs-live-proof | result-H7.md:16-17, :35, :57 |
| H8-1 | dependency scopes | Real install through merged per-scope receipt on macOS; bun-cache purge on real store; m04 115 GB growth census | needs-live-proof | result-H8.md:44 |
| H10-1 | ready suite stall | Rest of ready suite not rerun; m04 watch for `door.capacity` restart | needs-live-proof | result-H10.md:32, :41 |
| H13-1 | host capacity gate | m08 still held by foreign memory (fseventsd, swap); gate correct, live effect of minimum-2368 MB request unproven | needs-live-proof | result-H13.md:28 |
| H14-1 | cache server restart | Restart policy (quiet 15 min, max deferral 3 h) not verified live | needs-live-proof | result-H14.md (Unverified / open) |
| H15-1 | divergences | Live divergence counts after restart; 40 stream-only pairs unclassified until blobs kept | needs-live-proof | result-H15.md:31, :37, :48 |
| H16-1 | remote cache probe | Cold/warm/remote-down probe not completed; `Not verified` items open | needs-live-proof | result-H16.md:27, :35 |
| H17-1 | cache retirement | R11 Rust `cache_retirement` tests still UNVERIFIED (cargo run cancelled, then blocked in source admission) | needs-live-proof | result-H17.md:29, :49 |
| H18-1 | Nx runtime inputs | Plain `node`/`bun` runtime inputs resolve from per-machine PATH, not compared across workers | needs-live-proof | result-H18.md:22 |
| H19-1 | broker wedge | Running-silence watchdog and bounded probe estimate: not verified live | needs-live-proof | result-H19.md:31 (section) |
| H20-1 | remote-execution | Duplicate-registration check after scheduler restart (must list 15 workers, no duplicates) | needs-live-proof | result-H20.md:26 |
| H22-1 | fseventsd | fseventsd sampling on m08 not verified (needs sudo) | needs-live-proof | result-H22.md:26 |
| H23-1 | disk guard | Ready-suite timing test "detached grandchild" hangs alone at 360 s, not diagnosed | needs-live-proof | result-H23.md:28, :32 |
| H25-1 | recheck path | Re-check after 10 s admission wait not exercised against live fleet; Windows untouched | needs-live-proof | result-H25.md:30-31 |
| H26-1 | fleet stage | Workers re-running the stage with the overflow-message fix (m01 backoff, m06/m12 running) | needs-live-proof | result-H26.md:12 |
| H27-1 | native image parent scope | Compiler-state producer child image refusal and 5 red units: product-side, partially addressed by H35 | needs-live-proof | result-H27.md:14, :22, :27 |
| H28-1 | ticket generated sweep | m04 has not run the new sweep; folder removed only if idle 2 h | needs-live-proof | result-H28.md:15-16 |
| H29-1 | remote auto offload | Big file never ran remotely under pinned code (all peers "No other machine has headroom"); `auto` never seen offloading live; Windows/Linux not exercised | needs-live-proof | result-H29.md:38-44 |
| H30-1 | repo inputs | `--tool=semio inputs` unverified and unmeasured on workers | needs-live-proof | result-H30.md:22-23 |
| H31-1 | cargo preparation | native-dependencies heavy suite and compiler-slot policy not verified | needs-live-proof | result-H31.md:37 |
| H32-1 | cache retirement | Full-file run may still red; fresh unindexed pack path in `freshClosures` is the suspect | needs-live-proof | result-H32.md:23-24 |
| H33-1 | Nx notice / wait | No live worker ran the new code; Nx child vs host notice inferred from log order; dangling-imports not re-run; m12 custody race | needs-live-proof | result-H33.md:14-19; findings-H33.md |
| H35-1 | owned tool port | Windows owned-port path unverified; nested-image allowance effect live not measured | needs-live-proof | result-H35.md:15 |
| H36-1 | go-plan lease test | Lease test fails on events list (possibly pre-existing); m12/m14/m15 ready.json after re-run | needs-live-proof | result-H36.md:6 |
| H37-1 | Nx cross-machine hit | Cross-machine Nx hit proof from logs after 06:05 (skip lines) | needs-live-proof | result-H37.md:18; HANDOVER-1127.md (Open 4) |
| H38-1 | narrow graph read | Live re-measure 293 s (64 s capacity wait + 170 s graph-state check); structural-pending owners still wait for whole read | needs-live-proof | result-H38.md:19-26 |
| H40-1 | remote offload | Offload deadline and blob bound live only after sidecar hand-over; no offload to an idle machine shown | needs-live-proof | result-H40.md:15, :28 |
| H43-1 | graph state lease | Remaining ~6 s worker boot, ~25 s image open, ~40 s inventory not profiled live | needs-live-proof | result-H43.md:17, :20, :22 |
| H44-1 | Windows/Linux offload | Windows and Linux workers not exercised for test file offload | needs-live-proof | result-H44.md:23 |
| H45-1 | lease order | Isolated open lifetime child unexplained; nested exclusive open self-deadlock reported (H48 reports fix) | needs-live-proof | result-H45.md:12, :14-15 |
| H46-1 | build retry | Build-layer retry against real moving tree and landing burst replay not run | needs-live-proof | result-H46.md:16, :30 |
| H47-1 | r image ENOSPC | One-hour proof of r free / no ENOSPC not shown (only 06:08-06:17 window); lane births need host free ~24 GB | needs-live-proof | result-H47.md:20, :33 |
| H49-1 | divergences | Live effect of every item unverified | needs-live-proof | result-H49.md:21 |
| H50-1 | suite runner offload | Units file on a worker not verified (430 s run not repeated) | needs-live-proof | result-H50.md:23, :62 |
| H51-1 | m05 replica | Next steps and m05 replica daemon fix unverified | needs-live-proof | result-H51.md:26, :32 |
| H53-1 | m04 tools stage | Whether m04 reaches Ready once puzzle-job reservation falls below 2048 MB spare | needs-live-proof | result-H53.md:21 |
| H54-1 | remote typecheck | Two suites `capabilities-differ` on g1-m01/m13 unverified locally (peer-caused) | needs-live-proof | result-H54.md:18 |
| H55-1 | unit-cache guards | Live effect, which guard fired on workers, hit-rate after wave | needs-live-proof | result-H55.md:10-12 |
| H57-1 | interactive priority | m04 old-code tools process keeps waiting until stage restart; priority not run live | needs-live-proof | result-H57.md:21-24, :32 |
| H58-1 | replica daemon (Windows) | Windows behaviour of async chunks (threadpool 4) and m05 watch | needs-live-proof | result-H58.md:25-28 |
| H59-1 | always-run | Repeat of first always-run after revert on g1-m01 (replica lag) | needs-live-proof | result-H59.md:14 |
| H61-1 | bundle rebuild retry | Landed 10:00; after-number not measured; needs a run once workers restart | needs-live-proof | result-H61.md (Update 10:01) |
| H63-1 | .cache/b/d deletion | Deleter of 09:47 image still unknown; workers not checked beyond m14 | needs-live-proof | result-H63.md:5, :13-18 |
| H64-1 | storage root capability | Raw `rm -rf` in test code not covered; `enforce` mode not the default | needs-live-proof | result-H64.md:21-23 |
| H65-1 | Windows m05 / m04 | Watchdog fired in production (11:05); m04 tools failed twice after restart; m05 boot ~12 min | needs-live-proof | result-H65.md:13, :17, :25-31 |
| H66-1 | r image growth | Nothing live on m11/m12; Linux and Windows online paths untested | needs-live-proof | result-H66.md:17 |
| H67-2 | SIGTERM sender | Sender of stage SIGTERM still unknown; failure log overwritten per attempt | needs-live-proof | result-H67.md:29, :36 |
| H70-1 | stage stop sender | No worker runs new code; historic 11 SIGTERMs unprovable | needs-live-proof | result-H70.md:29-30 |
| H71-1 | staged input reader | m04 re-run after landing unverified | needs-live-proof | result-H71.md:12 |
| H72-1 | test-file vs suite envelope | 8 red in parent (stretched budget vs test assertions), owned by H77 (in flight) | needs-live-proof | result-H72.md:12-13; coordination.md:156 |
| H69-1 | rust remap | Unverified section on worker remap effect | needs-live-proof | result-H69.md:14 (section) |
| H62-1 | stage failure tail | Mirror of 200 lines unverified after H62 fix | needs-live-proof | result-H62.md:17 |
| H68-1 | provenance streams | Stream rendering checks TS/Python and server only; live cache divergence unchanged | needs-live-proof | result-H68.md:16 |
| H42-1 | descriptor tests | Unverified and left section (test extractor, Windows fake) | needs-live-proof | result-H42.md:13 |
| H24-1 | dashboard fleet window | Dashboard fleet window verified headless only; not interactive | needs-live-proof | result-H24.md (section) |
| H21-2 | graph observations | Heap effect not proven; registered collection estimate not done | needs-live-proof | result-H21.md:39-40 |
| H21-3 | graph hold | Live effect of observations-hold change unmeasured | needs-live-proof | result-H21.md:38-39 |
| H12-1 | test reds | T1 `optional-new-reference` red: retention deletes orphan leaf | not-done | coordination.md:77 |
| H11-1 | units test | Units file 21/5: three separate causes still OPEN (lease, timing, unit inventory) | not-done | result-H11.md:9 |
| H11-2 | Nx graph journal | Nx graph journal file red: sparse image precondition (35 red cascade); decide whether the law demands preallocation | not-done | result-H11.md:17 |
| H11-3 | native dependencies | Native probe timer 90 s red; 7 red "owned child terminal events lost" seen once | not-done | result-H11.md:23 |
| H11-4 | bun files | Neutral owned Bun files test times out at 120 s; predates change unverified | not-done | result-H11.md:24 |
| H15-2 | divergence classes | rmeta and stderr-only causes unverified (blobs garbage-collected) | not-done | result-H15.md:19, :26 |
| H17-2 | usage walk | Incremental put/delete deltas not done (in-process base only drifts up) | not-done | result-H17.md:43 |
| H20-2 | topology tests | `repo test-file` for topology, command and tests file not run (typecheck only) | not-done | result-H20.md:32 |
| H21-1 | graph catalog walk | Catalog walk and membership hash still lstat/readdir whole list on each attempt; L16 pass result 195 MB unchanged | not-done | result-H21.md:22-23 |
| H23-2 | replica daemon | No free-space check on apply in replica daemon | not-done | result-H23.md:6 |
| H27-2 | cache producer | Compiler-state producer child image refusal (see H27-1) | not-done | result-H27.md:66 |
| H29-2 | suite runner | Suite runner and parallel dispatch for `repo test-file` not done; retry window 20 s is a guess | not-done | result-H29.md:45 |
| H34-1 | image sizing | Cap of sum of envelopes at 50 % host total, per-purpose sizing from measured peak, not done; held profile lease still protects every unit | not-done | result-H34.md:17, :19 |
| H41-1 | cargo shards | Lane-birth sharding, shard retirement, remount repair, relayout not wired; `.cargo/config.toml` not edited by design | not-done | result-H41.md:13-16 |
| H41-2 | cargo cache | Residual risk: unit idle >6 h could be reused fresh (E0463); flake in `explicit reclaim` test | not-done | result-H41.md:17-18 |
| H47-2 | shard growth | No in-place growth of a CARGO-HOME shard; relief of a shard runs the unit collector only | not-done | result-H47.md:16-20 |
| H48-2 | goal 26/0 | Goal 26/0 not reached; Windows path of `ownedNativeToolPortV1`; first compiler-state image on real worker | not-done | result-H48.md:25 |
| H52-1 | actor_input decoder | Memory fix not implemented (type-erased core candidate A staged idea; 50-min cold lane per A/B) | not-done | result-H52.md:7-13 |
| H6-3 | units ceiling | Units over ceiling still listed; first heavy unit compiles at threads=0 until learned peak | not-done | result-H6.md:41-51 |
| H63-2 | tsc self-heal | Typed install instead of failing for missing tsc not implemented | not-done | result-H63.md:13-18 |
| H7-2 | compiler consumer | Compiler recovery does not retry on `space:true` refusals; lease pre-relief keys on volume bytes | not-done | findings-H7.md:6; result-H7.md:50 |
| H8-2 | dependency inode budget | Two-generation peak not counted; staged swap not done; backend `alias` refused by manager protocol | not-done | result-H8.md:24-25 |
| H9-2 | edge publication | Publication refuses with stale owner no heal path touches (root cause not fixed) | not-done | result-H9.md:14, :57 |
| H9-3 | graph read 8 | Bounded closure reference DATA required on m01 (read 8 running; H78 in flight) | not-done | coordination.md:151, :156 |
| H21-4 | suspect | Prime suspect for facts-regression is H8 round 3 per-scope sync (unverified) | not-done | findings-H21.md:9 |
| H49-2 | rmeta divergence | `.rmeta`/`.rlib` rows from g1-m13: compiled under other Cargo unit identity; cause not found | not-done | findings-H49.md:6 |
| H33-2 | m12 pruning | Pruning custody race (findings-H33); live run of custody fix needed | not-done | findings-H33.md; result-H33.md:19 |
| H56-3 | single-owner discovery | Narrow single-owner read still ~300 s (50k rows rediscovered); clean narrow run target < 60 s not reached | not-done | result-H56.md (Round 2); result-H61.md:43-47 |
| H40-2 | worker started signal | "Worker started" signal to cut tasks held but never started; not done | not-done | result-H40.md:9 |
| H43-2 | graph open | Skip maintain(false) open/close and 5 s inline eviction for readers; not done | not-done | result-H43.md:17 |
| H73-1 | agent prefetch | Staged prefetch (agent lanes prefetch from fleet), in flight, no result file | not-done | coordination.md:149-150 |
| H74-1 | repo land gate | Repo land compile-checked changesets, in flight; no-[DEBUG] law to be included | not-done | coordination.md:149, :153 |
| H77-1 | test-file vs suite envelope | Parity fix for 8 reds in `📥️parent`, launched 12:40, no result file | not-done | coordination.md:156 |
| H78-1 | m01 closure bound | Closure bound diagnosis on m01 (stage-only), launched 12:40, no result file | not-done | coordination.md:156; coordination.md:151 |
| H5-2 | fleet status payload | `repo fleet status --json` still ~61 KB (workers 31 KB, findings 16 KB) | not-done | result-H5.md:37 |
| H13-2 | host capacity | Gate test and fseventsd: "Not verified / not done" section | not-done | result-H13.md:20 |
| H2-2 | counters | Historic denial cause (m11/m12) unproven; only counters remain | not-done | result-H2.md:21 |

## USER items (need a person)

| id | area | one-line description | type | source |
|---|---|---|---|---|
| U-1 | fleet machine m09 | Provisioning of m09 (never provisioned) | USER | HANDOVER-1127.md (USER) |
| U-2 | hostnames | `repo fleet hostname apply` on coordinator and m08, m10, m13, m14 (duplicate LocalHostName) | USER | HANDOVER-1127.md (USER) |
| U-3 | m08 fseventsd | Restart of fseventsd or host cleanup needs the user (sudo) | USER | result-H22.md:26; result-H13.md:28 |
| U-4 | m05 Windows | Defender exclusion `C:\semio` (ready child boot ~12 min) | USER | result-H65.md:13 |
| U-5 | m04 dev workstation | Puzzle agent jobs (~6 GB) queued ahead of m04 tools stage; agent is not allowed to end them | USER | result-H53.md:16-18 |
| U-6 | 09:47 image deletion | Owner of the 09:38-09:45 storage edits to confirm test tmp checkout use | USER | result-H63.md:5, :13 |
| U-7 | optional data | Docker and Zotero data (optional) | USER | HANDOVER-1127.md (USER) |
| U-8 | stale lock | Stale `landing.lock.remote-execution` (H62, 10:00) left for coordinator | USER | result-H67.md:36 |

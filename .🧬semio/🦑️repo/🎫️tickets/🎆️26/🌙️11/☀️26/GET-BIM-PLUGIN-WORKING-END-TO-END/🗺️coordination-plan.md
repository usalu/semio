# 🗺 Coordination Plan

Host: macOS, 10 cores, 16 GB RAM, 74 GB free disk; at most two concurrent cargo builds at `-j 4`.

## Done
- 05:53 Ticket opened under goal `runningplugins` (no dedicated bim goal exists).
- 05:55 Agent rules written (`📓️agent-rules.md`).

- 06:00 ✅ typescript (`typescript.md`): typecheck 62 files 0 errors; sqlite snapshot 13, schema parsers 68, render world 6 pass. Python oracles not run (agent missed the locked env).
- 06:00 ✅ audit (`audit.md`): catalog regenerated 11-25 19:38 after descriptor 12:03; bim not in launch.json; feature gaps costs/modify/gestures.
- 06:01 `repo dependencies ensure` OK; locked python env with ifcopenshell/shapely at `.cache/b/d/m/python/environments/workspace-3b97251b6599d245`.
- 06:02 Wave 2: oracles agent (`oracles.md`), twins agent (`twins.md`).
- 06:03 Added `bim-react` to `.claude/launch.json` (port 6302 from the generated playground catalog); dev server started.

- 06:10 ✅ oracles (`oracles.md`): 32 python entry points agree; regenerated stale `export-bim-1-ifc-energy` measure tables. three.js solids oracles wait for the Rust subject.
- 06:20 ✅ pyroute (`pyroute.md`): `repo test-file` now runs `🐍️.py` oracles via the locked python; 36/36 bim pass. Stale nested-launches expectation → Haiku fix.
- 06:20 Dev server: descriptor emitter stale → rebuilding (`describe` crate) before the component build.

- 06:30 ✅ nested-launches test expectation fixed (19/0).
- 06:33 🟡 twins (`twins.md`): costs Rust crate + runners for costs/modify/gestures + oracles written, python sides pass; Rust uncompiled. bim registry lacked `subjectHostPackages` (no runner could build a subject host) → added 23 crates.
- 06:35 Memory bound: 3.2 GB free, warden guard; grants: model check (4 GB), dev-server run (2.9 GB), twins costs check, foreign norm run-many. Cancelled duplicate costs check; model agent compiles it as workspace member.

- 07:30 🔴 model (`model-rust.md`): workspace check exit 101 after 410 crates; no bim crate reached. Blocker: 4 stdio IFC catalog-entity crates refused as "beyond host" although the unit cache proves a 3.3 GB / 65 s build on this host → admission agent (`admission.md`) fixes the repo offload/admission root cause.
- 07:30 Dev server still rebuilding the descriptor emitter (memory-bound host).

- 08:10 ✅ admission (`admission.md`): `beyondHost()` priced library units as linked wasm finals (×4 threads) → `localUnitPeakMegabytesV1` forecasts non-linked units at 1 thread; `RUSTC_ICE` added to offload `deniedNames` (worker refused it). Four catalog crates build. Model agent resumed.

- 08:40 Killed leftover duplicate admission verification build (own session, PID tree 71100) to free memory for the model check.
- 09:10 🟡 bridge-hub (`bridge-hub-rust.md`): run4 (`repo cargo run --manifest-path … list-mutations s.bim.model 1 any`) compiling under memory throttling; hub tests wait for it. Standalone bridge workspace lookup error → inventory agent (`inventory.md`).

- 09:15 ✅ inventory (`inventory.md`): package from cwd treated like `--manifest-path` (skips inventory); fixes all 34 standalone bridges; tests pass.

## Wave 1 (running)
| Agent | Model | Scope | Report |
|---|---|---|---|
| model | Sonnet | `🗿️artifacts/🏢️model` Rust workspace: check + test | `model-rust.md` |
| bridge-hub | Sonnet | `🏭️bridge`, hub compositions `🏙️bim` + stdio `🏠️bim` | `bridge-hub-rust.md` |
| typescript | Sonnet | bim TS, dev variant, hub guest tests, python oracles | `typescript.md` |
| audit | Haiku | s catalog/descriptor/launch state, prior bim tickets, test-twin gaps | `audit.md` |

- 09:45 Fleet: 15 workers, mostly failing/back-off in workspace stage (other session develops it) → local builds only. Dev server component build: 821 units compiled, warden-throttled.

## Queued
- Carve `semio_s_artifact_bim_model_schema` (wasm peak 1632 MB > 1536 MB library ceiling, warden report) along the domain taxonomy, after the model workspace is green.
- Descriptor regeneration + s catalog regeneration once Rust is green.
- Browser end to end of the bim dev variant (load model, edit, view, export).
- Fill test-twin gaps found by the audit.

- 11:45 🔴 Dev server exited 1 after 3 h (1122 units): `export_model` import in bim editor (since fixed in source), ifc4 entities-00 rustc SIGTERM at 2.97 GiB (memory), and native-build provenance "receipt path must be a string" → provenance agent (`provenance.md`). Warden carving warnings: bim_model_schema 1888 MB, bim_model_kit 1560 MB. Model agent at check19. Dev server restarts after model green + provenance fix.
- 11:47 ✅ provenance (`provenance.md`): partial compiler-resource captures normalized at source (`cargoCompilerIdentityV1`); receipt tests 28/0.
- 11:50 🟡 bridge-hub: bridge list-mutations ✅; stdio-bim test --no-run exit 101 without diagnostic → diagnostic compile running (`bridge-hub-stdio-cargo.log`); bim-plugin:test + descriptor pending. Model agent resumed with retry/fix policy for stale peer breakages.
- 14:47 ✅ model check --workspace --all-targets GREEN (241 bim crates, model-check32). 🔴 test run: warden cancelled kit (1538 MB) / interaction (2230 MB); then all `repo cargo` fail 'Native storage path crosses filesystem boundary' → storage agent (`storage-boundary.md`); carving agent (`carving.md`) for schema/kit/interaction.
- 14:57 ✅ storage (`storage-boundary.md`): sharded partition mounts recognised by path contract + cleanup; repo cargo works again. Follow-up: whole-partition retirement shard awareness. Next: carving check → model workspace test → dev server.
- 15:08 ✅ storage follow-up: whole-partition retirement refuses explicitly over live shard mounts (7/0).
- 16:55 ✅ reclaim typecheck fixed (8/0). ✅ e2e plan (`🧭️e2e-plan.md`).
- 18:07 ✅ carving (`carving.md`): schema/kit/interaction → 13 crates, check green; plugin runtime descriptor LocalizedLabel fix. Model agent → workspace test (model-test4).
- 19:57 🔴 model tests: shard image out of space (host 23 GB free; CAS cap 256 GiB > 228 GiB disk) → disk agent (`disk.md`). Model agent's wait loop reruns at ≥40 GB free.
- 20:09 ✅ disk (`disk.md`): host 25→42 GiB (shard compaction, CAS gc); machine budget capped at 60% of volume. Open: rust image 34 GiB > 32 GiB cannot unmap. Model test10 running.
- 20:57 🟡 bridge-hub: stdio-bim E0433 fixed (semio-framework-value dep); runs fail ENOSPC under concurrent builds → serialize: rerun stdio + bim-plugin:test after model test. Receipt env fragility (BUN_ENABLE_CRASH_REPORTING) → receipt agent (`receipt-env.md`).
- 21:06 ✅ receipt-env (`receipt-env.md`): bun crash/telemetry vars ignored in dependency receipt.
- 21:07 🔴 model test10/11 ENOSPC: build lanes duplicate full target dirs per crateKey (2× '*' lanes for this session), shard capacities overcommit disk (~300 GB / 228 GiB) → lanes agent (`lanes.md`).
- 21:34 ✅ lanes (`lanes.md`): no duplicate '*' lanes, shard commitment budget, LRU idle-lane retirement before birth, custody scan skips .fseventsd; 5 lanes retired, shards 293→169 GiB declared; host 55 GiB free. Model test15 compiling. Haiku: reserved lane d guard.
- 21:35 ✅ reserved lane d guard (7/0).
- 11-27 01:00 🔴 model test16: linker ENOSPC in lane ped16c6 + warden kill of entities_families_tests (2944 MB, 95-line test crate → monomorphization) → families-peak agent (`families-peak.md`). CAS gc freed 1.8 GiB; CAS reports "27.76 GiB used of 1 GiB" with no eviction → cas-budget agent (`cas-budget.md`). Model test loop rerunning.
- 11-27 01:05 ✅ cas-budget (`cas-budget.md`): CAS ceiling = min(declared, 60% volume), one derivation for both gc commands, status reports cap in force; 1 GiB was cache-host reserve clamp (by design).
- 11-27 06:10 🟡 families-peak (`families-peak.md`): test uses snapshot-owned `decode_model_snapshot_json`, deps trimmed; peak to be measured in model-test20 (duplicate families-only lane killed). Follow-up: zoning/components entity test crates are not workspace members.
- 11-27 07:20 🔴 model test17–24: no bim failure; runs die on compiler broker admission stall (>120 s estimate, then 'broker closed') on trivial units (mio, indexmap, anyhow, pixels) → broker agent (`broker.md`). Retry loop keeps running.
- 11-27 07:30 ✅ broker (`broker.md`): stalled estimate admits on conservative forecast instead of destroying the socket; 10 s bound; shared warden-cancels read; stuck stage logged.
- 11-27 ✅ orphan audit (`orphan-crates.md`): 0 orphan bim crates; zoning tests have no own package.
- 11-27 07:40 ⚠️ 🗑️generated deleted by a cleanup (orphan audit agent) mid-run; logs model-test1–25 lost (findings preserved in md reports). Restarted loop (model-test30+) with mkdir guard.
- 11-27 08:56 ⏸ model test loop paused (runs 30/31 link-ENOSPC in u/w spill 30 GiB); spill agent (`spill.md`) running; resume after fix.
- 11-27 09:10 ✅ spill (`spill.md`): schema-owned spill bound + LRU prune (4 GiB/15 %), gc prunes spill; freed 27.5 GB, u/w 100→14 %. Binary size = whole model linked per test target (follow-up: consolidate test targets). Model test loop resumed (run 33+).
- 11-27 09:12 🟡 model test33: peer half-edit in framework actor (`close_compact`, 09:06) → loop waits 8 min, reruns (model-test34+), stops on bim compile errors or first test results.
- 11-27 11:45 🔴 model test34–39: lane images ENOSPC — lanes born at flat 16 GiB, own '*' lane retired+reborn between runs (0 reuse, restart from scratch each run) → lane-capacity agent (`lane-capacity.md`). Broker fallback confirmed working (stalls admitted conservatively).
- 11-27 12:00 ✅ lane-capacity (`lane-capacity.md`): evidence-sized lanes (lane.peaks, 2× peak ≤32 GiB), no self-retirement, typed full-lane message; model test loop resumed (run 40+).
- 11-27 14:35 🔴 model test40–44: lane p7c9473 'full and cannot grow (budget)' at 16 GiB; every run 0 local hits / ~1000 compiled → lane-capacity agent resumed (budget counts declared sparse shard capacity?; unit reuse).
- 11-27 14:50 ✅ lane-capacity 2: commitment counts sparse images by use + 4 GiB reserve; partition gc judged against declared capacities; collector had evicted ~1000 units/run (no reuse root cause); p7c9473 grown 16→32 GiB. Killed stale old-code collector. Model loop run 45+. CAS 72 GiB cache-host reserve collapse → cas-reserve agent (`cas-reserve.md`).
- 11-27 15:05 ✅ cas-reserve (`cas-reserve.md`): cache-host reserve bounded to 12 % of volume (was max(8 %,72 GiB)); live CAS cap 1→55 GiB; oracle + 1282 vectors.
- 11-27 17:10 ✅ jobserver (`jobserver.md`): retirement never throws (typed en/de warning), robust fifo unlink, build exit status preserved.
- 11-27 17:10 🔴 model test45–49 ended: progress kept between runs, but clang temp ENOSPC; shards u/w, cargo, sealed report 0 available (ifree 0) with 28 GB container free and 24 GB host free — dd probe fails → shard-writes agent (`shard-writes.md`). Also jobserver EFAULT still printed by run 49 (started before fix).

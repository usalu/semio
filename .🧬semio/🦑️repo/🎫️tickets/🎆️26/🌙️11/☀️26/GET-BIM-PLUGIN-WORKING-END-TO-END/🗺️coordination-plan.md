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

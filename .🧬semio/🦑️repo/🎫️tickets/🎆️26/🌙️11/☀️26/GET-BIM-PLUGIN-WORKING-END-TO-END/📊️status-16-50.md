# 📊 Status 2026-11-26 16:50

## Progress
| Area | State |
|---|---|
| TypeScript (typecheck, sqlite snapshot, schema parsers, render world) | ✅ green |
| Python oracles (36 bim entry points via `repo test-file`) | ✅ green |
| Test twins: costs / modify / gestures (`🥒️.feature` + `🦀️.rs` + `🐍️.py`) | ✅ written, Rust checks clean |
| Model workspace `cargo check --workspace --all-targets` (241 bim crates) | ✅ green (`model-check32.log`) |
| Bridge `list-mutations s.bim.model 1 any` | ✅ 216 mutation rows |
| Carving schema / kit / interaction (13 new crates, facades) | 🟡 source done, `check` retrying (warden kill in `carve-kit8`) |
| `@semio-tech/stdio-bim-plugin:test` | 🟡 rerun (`bridge-hub-stdio7`, reading graph) |
| Model workspace `cargo test` | ⏳ after carving check |
| `@semio-tech/bim-plugin:test` + committed descriptor | ⏳ after stdio |
| Dev server http://localhost:6302 + browser e2e (demo/house/office/checks) | ⏳ after model tests; e2e plan being mapped |
| Descriptor + catalog regeneration, ticket close | ⏳ last |

## Expected finish
About 3–5 h if no new peer breakages: carving check about 30 min, then model tests about 60–90 min (cached units), stdio/bim hub tests about 60 min (they overlap with the model tests), dev server component build about 60 min (mostly cached now), then e2e and close about 30 min. The estimate depends on the memory warden. Each warden kill or peer half-edit costs one rerun, and cached units shorten each retry.

## Problems faced → fixes (all at root cause, with tests)
| Problem | Fix | Report |
|---|---|---|
| `repo test-file` refused `🐍️.py` oracles | python route via locked env | `pyroute.md` |
| stale nested-launches expectation | test updated | `coordination-plan` |
| stale `ifc-energy` measure tables | regenerated | `oracles.md` |
| bim registry lacked `subjectHostPackages`; costs/modify/gestures twins missing | 23 crates registered, twins + crate added | `twins.md` |
| catalog crates refused "beyond host" (library units priced as linked wasm ×4 threads) | `localUnitPeakMegabytesV1` (non-linked = 1 thread); `RUSTC_ICE` added to offload `deniedNames` | `admission.md` |
| standalone bridge workspaces: "Unknown current repository Cargo package" | cwd package treated like `--manifest-path` | `inventory.md` |
| 56 bim compile errors (tests, configs, imports, `export_model`) | fixed in bim crates | `model-rust.md` |
| native-build provenance "receipt path must be a string" after killed units | `cargoCompilerIdentityV1` normalises partial captures at source | `provenance.md` |
| every `repo cargo` failed "Native storage path crosses filesystem boundary" (sharded cache partitions) | path contract + cleanup recognise live shard mount points; whole retirement refuses explicitly | `storage-boundary.md` |
| crates above 1536 MB rustc budget (schema 1888, kit 1560, interaction 2230) | carved along taxonomy into 13 crates | `carving.md` |
| peer half-edits (replay, job-context, store-engine `[DEBUG]`, `.mjs` rewrite) | waited out / minimal fixes when stale | `model-rust.md`, `bridge-hub-rust.md` |
| duplicate own builds wasting RAM | cancelled (twins costs check, admission verify tree) | `coordination-plan` |

## Anticipated risks
- Memory warden kills on transient spikes: retries are cheap because units are cached.
- More peer churn in framework crates: the stale-edit policy is to wait at most 10 min, then fix minimally.
- First test run of the new twins and the carved crates may show API mismatches.
- Dev server guest component: ifc4 `entities-00` reached 2.97 GiB and may need carving as well.
- Fleet is unusable (15 workers in back-off), so everything builds locally.

## Resource use
- Host: 10 cores, 16 GB. At 16:50 there is 76 % memory free and load 4.2, so the CPU is **not** saturated. The bottleneck is the memory warden. It kills compilers above its per-unit and host ceilings, so blindly adding parallel cargo builds causes kill/rerun loops, as earlier runs showed. The policy stays at two concurrent cargo builds, `-j 4`.
- Builds now: carving check + stdio hub test.
- Non-build agents added to use idle capacity: Haiku `reclaim-typecheck` (stray TS error) and Haiku e2e browser plan mapping.
- Fleet offload: unavailable (back-off), so builds stay local.
- Disk: 42 GB free, which is fine.

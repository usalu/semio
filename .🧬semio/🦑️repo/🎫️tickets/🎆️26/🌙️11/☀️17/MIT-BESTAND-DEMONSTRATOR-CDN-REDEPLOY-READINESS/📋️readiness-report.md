# Readiness Report: Mit-Bestand Demonstrator CDN Redeploy

Draft, 2026-11-18. Ticket `MIT-BESTAND-DEMONSTRATOR-CDN-REDEPLOY-READINESS`, goal `runningdemonstrator`. Sources: the ticket folder markdown files (main log `🚀️redeploy-log.md`) and the driver scripts in the ticket folder. Fixes dated 2026-11-17 to the early hours of 2026-11-18.

## Status

| Field | Value |
|---|---|
| Verdict | pending: site build and per-app verification |
| Release components | Not passing describe yet. Demonstrator, procedural, fem and energy each have a fix in source; the staged describe binary has not been rebuilt for all of them. |
| Site build | Not passing. The last recorded site log (`🗑️generated/site-native-1.log`, 2026-11-17) ends with `Original parent Script process invocation required`, a direct invocation without the build-caller envelope. The envelope path is `native-wasm.sh site`; no run since. |
| Plugin catalog | Empty until component descriptors are regenerated (`catalog-generate` depends on the component stage). |
| Prepare lanes (nine) | Not run to completion. |
| Static verification | Not run. |
| In flight | `describe-write` and the energy component were writing logs at 03:54 on 2026-11-18. Their results are not in this report. |

Completed done markers in `🗑️generated/done/`: `wasm-opt`, `plugin-web-support`, `font-assets`, `infinite-fonts` and `describe-build` (rebuilt 00:45). No marker exists for the four components, the catalog, the sessions, the prepare lanes or the site.

## Deployment

Build natively on macOS arm64. The release bundle is the static site folder produced by `native-wasm.sh site`. All commands run from the ticket folder `T`. Output location: ``.cache/b/p/m/vite/release/5dc1b077e86de2aa/out`` (placeholder, to be filled after the first successful site build).

| Step | Command | Does | Marker or result |
|---|---|---|---|
| 1. Prereqs | `./native-lanes.sh prereqs` | wasm-opt, plugin-web support, canvas font assets, infinite fonts, describe tool build | `done/wasm-opt`, `done/plugin-web-support`, `done/font-assets`, `done/infinite-fonts`, `done/describe-build`. Use `FORCE=1` when describe-tool sources changed. |
| 2. Wasm and registries (not in the requested sequence, required) | `./native-wasm-all.sh`, then `./native-wasm.sh surface`, `./native-gen-all.sh` | editor, node-graph, puzzle, flow, canvas packages; generated registries for trinity, reasoning, writer, draw, puzzle-5d and flow-plugin | Receipts `.🧾️store/<target>.json` under `.cache/b/p/m/browser/release/wasm-packages/` |
| 3. Describe and write | `./native-describe-write.sh` | builds and describes the four release components, marks them, regenerates the catalog, then calls `native-finish.sh` | `done/component-*`, `done/catalog-generate` |
| 4. Finish | `./native-finish.sh` | nine sessions and prepare lanes, demonstrator prepare check, then `native-wasm.sh site` (`MEM=4096`) | `done/session-*`, `done/prepare-*`, `done/demonstrator-prepare`; site output ``.cache/b/p/m/vite/release/5dc1b077e86de2aa/out`` |
| 5. Verify | `./verify-static.sh `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` [port]` | serves the site at a local root and runs the 16 pane boot checks (8 panes, en and de) with Playwright | `🗑️generated/static-verify.log`, `🗑️generated/pw/` |

Notes:
- Step 3 already calls step 4 at its end. Running step 4 by hand afterwards is redundant except for the site build, which has no done marker and always reruns.
- `native-lanes.sh components` (or `native-rebuild.sh`) is the component-only stage; step 3 is its describe-writing variant.
- Every script hardcodes `ROOT=/Users/ueli/Documents/semio` and assumes `darwin` and `diskutil`. See Open items.

## Hosting requirements

From `🌐️static-hosting-audit.md`, plus the static verification notes.

| Requirement | Detail |
|---|---|
| Root deploy | The bundle must be served at the root of the CNAME host. A sub-path deploy breaks plugin routes, extension routes and brand logos. |
| Root-absolute routes | `/🔌️plugin-modules` and `/🧩️extension-modules` are root-absolute. `publishedPageUrl` rewrites them only when `SEMIO_PLAY_PAGE_ORIGINS` is set. Brand logos use `/${DEMONSTRATOR_ASSETS_DIR}/`. |
| No COOP/COEP | SharedArrayBuffer is optional, so no cross-origin isolation headers are needed. |
| No SPA fallback | One page with hash routes. A missing module JSON must return 404, not `index.html`. |
| Hash routes | `/#generator`, `/#koordinator`, `/#aggregator`, `/#energie`, `/#aussuchen`, `/#bearbeiten`, `/#verfolgen`, `/#statik`. |
| Map tiles | Only Switzerland z0 to z10 is bundled, under `/osm/{z}/{x}/{y}.png` and `/vt/{z}/{x}/{y}.pbf`. Tiles outside that set return 404. `verfolgen boot` needs painted tiles. |
| Known dead UI actions | Extension install and remove call `/🧩️extension-modules/install`, answered only by the serve-mode dev plugin (404 on static). |
| Known dead backbone | `/semio-backbone?uri=` exists only as dev middleware (404 on static). Remote backbone reads retry for 15 s, then fail. |
| Inert dev routes | `/_semio/dev/local-session` and the hub service worker are gated on `VITE_S_HUB_URL`, unset in the release. |
| Not covered by the audit (verify on the CDN) | `.wasm` served as `application/wasm`; `.js` modules served with JavaScript MIME types; 404 for missing `/osm`, `/vt` and module JSON. |

## Fundamental fixes made

| Issue | Root cause | Fix | File |
|---|---|---|---|
| Build failed in planning: `undefined is not an object (evaluating 's.busy')` | A concurrent edit read `pool.farm.snapshot`, which the farm did not have. | Added `get snapshot()` to the farm, typed as `GraphFarmSnapshotV1` (applied). | `🏊️project-farm/🏊️pool/🟨️.mjs`, `🏊️pool/🟨️.d.mts`, hub at `🏊️project-farm/🟨️.mjs:254` |
| Graph inference starved, `unserved build.graph.no-progress` | Memory governor starved the graph slice. | Native tools with a build-caller envelope via `native-launch.ts` and `native-wasm.sh` (fallback, in use). | `native-launch.ts`, `native-wasm.sh` |
| Component builds compiled one unit at a time (`-j1`) | The planner used the authored 2048 MB per unit, not the measured history (q75 416 MB). | Unit history read once, used by planner and memory broker (source; the expected six-unit plan is not yet observed). | `🧩️component-pass/🟦️.ts` |
| `semio-plugin-describe` rejected fem and demonstrator: no owned Semio actor ABI | `OwnedSemioExport::core_type()` typed 19 exports as `i64`. | Correct signatures (`()->i64`, `(i32,i32)->i64`, and 19 `i64` where right). Descriptor tool rebuilt (applied). | `🧠️interpreter/🔌️owned-semio-boundary/🦀️.rs` (about line 213) |
| Energy component 612 MB, demonstrator 519 MB (describe cap exceeded) | `ControlledRetirement<T>` was monomorphized per owner type (641 types in energy). | T-independent frontier moved to the non-generic `ControlledCore`, with dyn callbacks (applied). Energy: 612 MB to 442 MB, then 379 MB after the driver-chain cuts. | `🌱️value/♻️retirement/🎮️controlled/🦀️.rs` |
| Generic driver chains and per-field derive code kept bloating components | Driver code generic over producer and closure, though only the error and host types mattered. | Object-safe drivers, `native_close_settle_dyn`, `admit_frame_slot`, `plan_field_stage` (applied; about 14 % of bytes). | `🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs`, `…/🧳️frame/🎛️driver/🦀️.rs`, `🌱️value/✨️derive/🛬️receive/🦀️.rs` |
| Disk full at 11:36; every native build failed; cache images unmounted | Raw fixed images under `hdiutil attach` never returned freed blocks; images were sized 96 GiB. | `diskutil image attach` (trims on mount); reclaim module; rust image sized 32 GiB with 40 GiB birth (applied; the 2026-11-17 cycle freed 80 GB). | `native-build/💾️storage/🍎️image/👤️owners/🟦️.ts`, `🍎️image/♻️reclaim`, `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` |
| Two drivers stalled 20+ minutes in `loading` on the lease image | The exclusive waiter was never re-evaluated after a peer activated the image. | The waiter re-reads state and retries as a shared reader on `FixedPeerActivationV1` (applied, test passes). | `storage/🧱️fixed/🟦️.ts` |
| Lane custody unresolved for prototype after release | `discoverLaneEvidenceV1` bounded directories by the 4096 ledger-file limit; release lanes hold 4.8k to 6.6k. | New policy `retention.scanMaximumDirectories` (65536) (applied; end-to-end unconfirmed). | `unit-cache` schema, `withLaneRetentionV1` (`unit-cache/lane/retention`) |
| Describe refused: `Original mutable compiler input changed` | The emitter re-digested the live, continuously edited worktree at the end of each build. | Recheck verifies only sealed digests; refusals name each path and both digests (applied). | `🖨️describe/🛠️emitter/🔏️sealed/🟦️.ts`, `🖨️describe/🛠️emitter/🟦️.ts` |
| Describe refused: `native io: original interpreter deadline requires its caller clock` | The owned codec path never bound the caller clock (`original_job_clock: None`). | Both `assemble_codec_origin` and `origin_instance` bind the clock (source; staged binary not replaced). | `🔌️plugin/🧠️interpreter/🏃️runtime/🔄️owned-runtime/🦀️.rs` |
| Describe refused: `artifact codec schema has no structural record specification` (fem, flow, process3d, cad and others) | The pack-schema hash gated on the borrowed producer, while the hash came from `record_spec()`. | Hash taken from `record_spec()` only; producer and its overrides removed (applied; check passes for fem, demonstrator, energy, procedural). | `🧰️framework/🔨️modules/🚪️io/💾️binary/📸️snapshot/🦀️.rs` |
| Describe refused: `no declared composer capability owns the runtime claims` (demonstrator, procedural, flow, gis and others) | Foreign composer rows declared plain `dialect` claims; assembly requires `composition-route` claims. | Framed `composition-route` claims (applied for procedural, demonstrator and the listed hubs by `composition-route-fix.py`; energy model deferred). | `🌀️procedural/🗿️artifacts/{🌀️generation2d,🧊️generation3d}/🦀️.rs`, `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs` (deferred) |
| Energy derive failed on mutation source authority | Two taxonomy keys were renamed to names with no directory; the registry is generated from the taxonomy. | Keys renamed to directory names; registry regenerated (applied; `check --target wasm32-wasip2` passes). | `🔣️taxonomy.json` (lines 34140, 34231), `…/✨️derive/🔣️mutation-authority.json` |
| jco transpile traps on the 379 MB energy component (`out of bounds memory access`) | jco limit, not a memory cap (also fails at 1.78 GB). | Native transpile (`js-component-bindgen` 2.16.0) replaces jco; byte-identical on a 137 MB component, energy 379 MB in 5.4 s (applied). | `🌐️browser-bundle/🏗️materialization/🦀️native-transpile/`, `…/🏗️materialization/🟦️.ts` |
| Demonstrator and energy final crates jetsam-killed (exit 137) | Front end ran with many threads; the cdylib back end ran parallel LTO and codegen jobs. | cdylib finals get one front-end thread and `-Z no-parallel-backend` (applied). Not confirmed: energy still exited 137 in the last run. | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🦀️.rs` |
| Retirement queue refused close (`UnsupportedOwner`) | `RetirementQueue` had no `close_into`. | Inherent `close_step` and `native_retirement_reply!()` added (applied; 6 of 7 lib tests pass). | `📋️queue/🦀️.rs` |
| Retirement tests failed after the `ControlledCore` refactor | `reserve_cursor_slot` lost its closure parameter; test drift. | Public `reserve_cursor_slot` forwards to `ControlledCore`; tests updated (applied; 30 retirement tests pass). | `🎮️controlled/🦀️.rs`, `🎮️controlled/🧪️tests/🦀️.rs` |
| Image on the data volume reported 4.5 GiB free with 82 GB unallocated | Volume free space was sampled at mount time, while the host was full. | Re-attach through `remount-image.sh r` (log `remount.log` at 00:37 shows 71 GB available; confirm). | `remount-image.sh` |

## Open items and follow-ups

### Blockers for the redeploy

| Item | Detail | Next step |
|---|---|---|
| Describe binary is stale | The staged describe binary predates the caller-clock, sealed-recheck and pack-hash fixes. | `FORCE=1 ./native-lanes.sh prereqs`, then the component stage. |
| Energy model composer rows deferred | `apply-deferred-describe-fixes.py` holds the energy model rows (3) and the diagnostics change. Energy describe will refuse until applied. | Apply once the component chain is idle, then rebuild the describe tool. |
| Component builds unverified | Fem, procedural and demonstrator were not rebuilt after the latest fixes. Energy final crate was killed in the last run. | Run `./native-describe-write.sh` after the describe tool rebuild. |
| Site build not rerun | Last recorded failure was a missing envelope on a direct call. | Run the site build via `native-wasm.sh site` inside step 4. |
| Plugin catalog | Empty until components are regenerated. | Follows from the component stage. |
| Size cap ambiguous | Shipped profile cap is 256 MiB (`🖨️describe/🏷️component-receipt/🧪️tests/🎚️profile/🔣️.json`). Energy and demonstrator notes cite 512 MiB. Current sizes: energy 379 MB, demonstrator about 359 MB. | Decide which cap governs; if 256 MiB, apply the drafted cuts (below). |
| `wasm-opt` status | Done marker exists (06:26), yet a round-2 note says `wasm-opt` is not provisioned. | Verify `repo tools list` before relying on size reduction. |
| Wasm and registry gaps | `native-gen-all.sh` omits `puzzle-2d-rs` and `puzzle-3d-rs`, which the log says were generated. `native-wasm-all.sh` does not include `surface`. | Add both to the scripts and rerun them. |
| `nx-contract` fixture | Named in the readiness request but not recorded in any ticket markdown. | Confirm the item with the user or locate it. |

### Unverified fixes

| Item | Detail |
|---|---|
| Serial-unit planner | Expected six units per run; not observed. |
| Lane custody | End-to-end `repo cargo --store=shared --configuration=prototype -- test` unconfirmed; `dead-holder` test not run (bun not on PATH). |
| Image reclaim | Suites not run through `repo test`; end-to-end `repo cache gc` on a live image not exercised. |
| Describe recheck | A post-stage recheck failure leaves the binary in the store; the next `describe-build` overwrites it. |
| Serial back end | Demonstrator and energy final crates not rebuilt since `-Z no-parallel-backend`. |

### Deferred test drift and known bugs

| Item | Detail |
|---|---|
| Retirement-queue stall | `original_turn_tests::retirement_turn_original_queue_preserves_full_grants_all_physical_currencies_and_paused_custody` fails with `original queue stalled`. Hypothesis: factory `demands`/`close_step` disagree with the queue quote by one depth level. Not verified. |
| Retirement-queue targets | Older drift in other targets, untouched. |
| Stale ABI fixtures | `🧪️tests/🔬️unit/🦀️.rs` and `runtime/🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs` encode the old export signatures. |
| Image livelock test | One pre-existing failure needs `SEMIO_TEST_ARTIFACT_DIR`. Restart stalled `repo mcp` processes (pids 61563, 61693 if still alive). |
| Known hosting bugs | Extension install and remove, remote backbone reads, and tiles outside Switzerland z0 to z10 return 404. Accepted for this redeploy. |
| Route naming | `🔨️modules/🧩️runtime/🔣️.json` still says `energy` and `fem3d`; brand names are `energie` and `statik`. |

### Drafted bloat cuts (not applied)

| Cut | Estimated effect |
|---|---|
| Round 6: `apply`, `close_plan`, `close_steps`, `advance*`, `quote` through object-safe field slot traits | 3.5 to 4 M of 15 M IR lines in the initialization crate. |
| `NativeReceiverBackingClose::drive` and `quote` as a `BackingStorage` trait | About 2.2 M IR lines. |
| `native_relay`, `admit_retirement_cursor`, `retirement_cursor_birth_demand` | Smaller; same pattern. |
| `-Zshare-generics=yes` and `wasm-opt` | Not yet measured; `wasm-opt` provisioning uncertain. |

Combined estimate: about 40 % of original bytes, roughly 300 MB for energy before `wasm-opt`. Apply after the redeploy.

### Platform and hygiene

| Item | Detail |
|---|---|
| Cross-platform | `native-lanes.sh`, `native-wasm.sh`, `verify-static.sh` hardcode the repo root, use `darwin` host platform and `diskutil`. This violates the zero-touch cross-platform rule. Move behind a tool-independent interface after the redeploy. |
| Playwright dependencies | `verify-static.sh` needs `node_modules/@playwright` at the repo root; it was empty earlier. Run `repo dependencies sync --all` first. |
| Log hygiene | Section numbers in `🚀️redeploy-log.md` repeat (5, 6, 13, 15 are duplicated or out of order). Cosmetic. |
| Generated folder | `🗑️generated/` holds 76 entries. Keep scripts and markdown; delete generated logs when the ticket closes, per CLAUDE.md. |
| Disk | The data volume reached 97.7 % once; the guard refuses builds above 97 %. Watch the `.cache/b/r/m` image. |

## CDN MIME types

| Extension | Content type |
|---|---|
| `.wasm` | `application/wasm` |
| `.js`, `.mjs` | `text/javascript` |
| `.json` | `application/json` |
| `.pbf` | `application/x-protobuf` |
| `.css` | `text/css` |
| `.svg` | `image/svg+xml` |
| `.woff2` | `font/woff2` |

The host is `demonstrator.entwerfen.mit-bestand.de` (`CNAME`); dotfiles (`.nojekyll`) must be served, and missing files must answer 404, not `index.html`.

# WP-W3 — All-Plugin Build/Publish Owner (successor of [W2](📓️wp-w2.md))

Session 13 slice W3. Canonical hub 7800, hubs 8000–8009, serves 6500–6509. Scripts `wp-w3/`, captures `wp-w3/generated/`, durable
data `.🧬semio/🌐hub/s13-w3-*`. Handover: [📓️wp-w2.md](📓️wp-w2.md) (Hub Handoff, Session 12, §2/§3).

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | Publish 4 root cause (`browser actor artifact: unsupported import interface`) | **FIX LANDED** 19:4x (parley `no_std`+`libm` in ui-render + ui; boundary law); native + wasm32 checks green, ui-render 136/136; the real-bytes proof (layout component without `wasi:filesystem`) is the chain's own `preflight-catalog` step after rebuild-all |
| 2 | Fail-fast preflight (browser-actor import admission + late admissions) over all packages | **LANDED** 20:3x: `browserActorImportAdmissionV1` (jco oracle 34/34 release components agree), actor build refuses early + asserts codegen == admission, `trusted-catalog-preflight` verb (60 components in 27 s); findings: layout (fs), demonstrator/draw/puzzle dev deliverables stale (rebuild-all fixes) |
| 3 | Land W2's prepared sets (item4 + supervised 7800 restart; `VersionReq` narrowing moved to S17 by the coordinator) | **LANDED** item4 20:48 (auto-committed 22:00); laws green; native check **green 05:23** (kernel, os-mcp, os-run, plugin-host `--lib --tests`); supervised restart = `w3-restart-7800.sh` + `w3-hub-resume.sh` |
| 4 | Kernel derive macro reads a narrow generated input | **LANDED** 20:4x: committed projection `✨️derive/🔣️mutation-authority.json` (schema + generator contract + generate/preview/check verbs), expansions include only it; derive tests 16/16; kernel lib + lib-test check green 05:13; a peer regenerated the projection at 00:44 after a taxonomy edit (`check-generated` fresh) |
| 5 | Requests triage (`wp-w1`, `wp-w2`, `wp-w3` requests) | **DONE** 21:2x (table below): all served by the consolidated rebuild + catalogs, except WG9's browser wgpu renderer `wasm-release` and P8's un-landed patch sets → routed to main |
| 6 | ONE consolidated chain (rebuild-all → B3 → 7800 → `--packages all` → 7800) | b3 **DONE** (run 6: B3 published 13:54, 7800 READY 13:57); phase `final` PREPARED + dry-run 14:15, waits for the end of landing window 2 05:2x (early steps dry-run, lane orchestration simulated); prepared 21:2x: `wp-w3/w3-chain.sh b3|all` (+ `w3-restart-7800.sh`, `w3-hub-hold.ts`, `w3-hub-resume.sh`, `w3-open-plan-probe.ts`); product chain gained `provenance`, `mutation-authority`, `flow-core-bindings`, `preflight-catalog` steps; waits for REBUILD START |
| 7 | Hub Handoff rewrite when 7800 serves the all-package catalog | B3 handoff WRITTEN (7800 ready on B3 13:57); ALL handoff after phase `final` |

### Hub Handoff (7800)

**CATALOG B3, READY since 2026-09-27 13:57** (chain b3 run 6; boot ~30 s). Next: phase `final` (`--packages all`) moves 7800 to a fresh root;
this B3 root and binary stay for rollback.

| what | value |
|---|---|
| URL | `http://127.0.0.1:7800` (loopback, development), `/readyz` `status: ready` (runId `fd90596d4296708cc463c42c0170a410`) |
| catalog | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-catalog-b3`: profile `local-stdio-gis-note-animate-block-writer-draw-puzzle-wfc-open-v1`, generation `e3c0c98ef201c30195f1cdbb5cadd1c490e4ca5edf3e67a23e3adf1b428dfc0d`, bundle sha256 `fdf20877049a4a14e60a3084391d5da9c4bd765e5c7a8d03b8fc25fe7377f055` (published 13:54, 330 s, current-tree envelope wire) |
| binary | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-bin/s13-w3-hub-7800-b3/os-hub` (sha256 `962ba3726a70d1945bb1ae935c024636b506a48a0e627c8843f62aa10c3afd4c`, `os-hub:build-dev` 13:54) + `os-hub.sources.json` beside it; `bun nx run os-hub-ts:hub-freshness --hub http://127.0.0.1:7800` works (14:1x: `stale`, 8004 sources, landing window 2 edits after the 13:54 build) |
| data root | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-hub-7800-b3` (fresh, new envelope wire) |
| hold | supervised `wp-w3/w3-hub-hold.ts` pid `4540`, hub pid `4548`; state `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-state-7800/{pids.txt,status.txt,hold.txt,capture.txt,ready.json,admin-capability.json}` |
| users | `user1@semio.dev` / `gm1-local-dev-pass-1` (`01a0e2b9-7c61-7c91-90c1-9bf87c0e570b`), `user2@semio.dev` / `gm1-local-dev-pass-2` (`01a0e2b9-7c83-7dfa-ad7a-f970740183b5`) |
| admin | `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-state-7800/admin-capability.json` (`0600`, one `admin-relay` session, 15 min); fresh one: `touch …/s13-w3-state-7800/admin-request` |
| stop / resume | stop: `touch /Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-w3-state-7800/stop`; after a whole-process loss: `zsh .tmp-ticket/wp-w3/w3-hub-resume.sh s13-w3-hub-7800-b3` |
| MCP | `semio-framework-os-mcp` built by the chain (`@semio-tech/framework-os-mcp-rs:build`, 13:56); agent credential recipe as in W2's handoff (`POST /auth/agent-delegations`) |

### Session 13 Log

- 19:05 state: disk 115 GiB free, load ~20, 11 rustc, hub 7800 = hold 28673 → hub 54029 (B2), no wasm lock held.
- 19:1x **Item 1 root cause (measured).** The failing actor build was NOT imperative: the component under `actor-codegen: 0/16161078`
  is 16 161 078 B = `📏️layout/…/dist/component-release/semio_s_plugin_layout.wasm` (imperative built just before, layout was warm).
  `jco wit` over all 60 component-dev + every present component-release (`wp-w3/w3-import-scan.sh`, `generated/import-scan-1.txt`):
  **only layout** imports outside the admitted browser-actor set: `wasi:filesystem/types@0.2.9` + `wasi:filesystem/preopens@0.2.9`
  (dev and release). Call-graph triage over the dev component's core module (`wp-w3/w3-import-callers.ts`, direct calls, name
  section; `generated/layout-fs-callers.txt`): `LayoutEngine::layout_story` → `ui_render::TextSystem::shape_paragraph` → parley
  `shape_text` → swash `Selector::select_font` → fontique `query::load_font` → `FontInfo::load` → `SourceCache::get` → `load_blob` →
  `std::fs::File::open` → wasi-libc `open` → `wasi:filesystem`. Cause: `semio-framework-ui-render` (and `semio-framework-ui`) take
  `parley = "0.5.0"` with default features = `system` → `std` → fontique path/mmap font sources, although both text systems register
  embedded font bytes only (`CollectionOptions { system_fonts: false }`). Other ui_render users (dag, flow, reasoning, sequence,
  trinity) carry the same latent path but never reach shaping.
- 19:1x **Item 1 decision:** the guest must not import `wasi:filesystem` (no browser actor, hub or MCP host grants filesystem authority, and
  both text systems use embedded font bytes only), so the fix is at the dependency, not the admission list. `semio-framework-ui-render`
  and `semio-framework-ui` now take `parley = { version = "0.5.0", default-features = false, features = ["libm"] }` (`no_std`: fontique
  has no `SourceKind::Path`/`load_blob`/`memmap2`, and the native build drops the CoreText/fontconfig system-font backends it never used).
  `cargo tree -p semio-s-plugin-layout --target wasm32-wasip2 -e features -i fontique` now shows only `libm`. `Cargo.lock` lost
  fontique's `memmap2`/`objc2*` edges. Law: ui-render `boundaries` forbids `memmap2` (4 forbidden crates absent, 17 s).
- 19:23–19:46 checks (captures `wp-w3/generated/`): `cargo check -p semio-framework-ui-render --lib --tests` rc=0 (0 warnings),
  `-p semio-framework-ui --lib --tests --features wgpu-engine` rc=0 (106 pre-existing warnings), under the wasm mutex
  `-p semio-s-plugin-layout --lib --target wasm32-wasip2` rc=0 (11m47s), `-p semio-framework-ui-render --target wasm32-unknown-unknown`
  rc=0, `-p semio-framework-ui --features wgpu-engine --target wasm32-unknown-unknown` rc=0. 20:02 `cargo test -p semio-framework-ui-render`
  **136 passed** (incl. the parley text oracle).
- 20:03 layout `component-dev` rebuild (to prove the import is gone on real bytes) failed in `semio-framework-os-kernel`:
  `no field observed on MutationEnvelope` (LD's in-flight wire change, 4 errors). Retry after LD lands.
- 20:0x–20:3x **Item 2 (landed).** `🌐️browser-bundle/📜️script.ts`: `browserActorImportAdmissionV1(component)` parses the component's
  own type/import/alias/export sections (component-model binary format) and returns the root imports the codegen must bind (every
  imported instance whose type exports a func or resource, every root func), each mapped to its admitted spelling (exact, else the one
  semver-compatible admitted version, `0.x` pins the minor) or refused. No codegen, 0.4–56 ms per component.
  `buildClosedBrowserActorArtifactOwned` refuses before codegen with the named interfaces and asserts after codegen that jco's own
  import manifest equals the admission (a live oracle on every actor build). Third-party oracle (`wp-w3/w3-admission-oracle.ts`, jco
  `generate` with the actor's exact map/async options): **34/34 release components agree**, incl. layout refused
  `wasi:filesystem/{preopens,types}` (`generated/oracle-release.txt`). Law: the actor-factory suite asserts admission == jco
  `importInterfaces` on the fixture component, the renamed hostile import refused, a truncated component malformed
  (`artifactLaws` +`import-admission-matches-codegen`); `closed-browser-component-factory-check` **green** (artifact-laws 21,
  `generated/factory-law-3.txt`). Hub `📜️script.ts`: `trustedBootstrapPreflightComponentsV1` (every selected package + every extension
  of one: `dist/component-dev` == committed descriptor `hashes.wasmSha256`, import admission; ALL findings in one refusal) runs with the
  descriptor preflight BEFORE the bootstrap's 15-min hub build; new verb `trusted-catalog-preflight [--packages …|all]`. tsc: only the
  pre-existing leb128 error + LD's in-flight `MutationEnvelope` TS error at line 882.
- 20:3x **Preflight over all packages (measured, 27 s, `generated/preflight-all-1.txt`):** 34 descriptors PASS; components 56/60 PASS;
  refused: **layout** (`wasi:filesystem/preopens@0.2.9`, `wasi:filesystem/types@0.2.9` → fixed at the source, pending its rebuild),
  **demonstrator, draw, puzzle** (`dist/component-dev` ≠ committed descriptor: peers rebuilt those deliverables after the last describe;
  the consolidated `rebuild-all` re-describes them). No other package imports anything outside the admission.
- 20:3x–20:4x **Item 4 (landed).** The mutation derive macros (`🗣️dsl/✨️derive/🦀️.rs`) read and `include_str!`-track ONE file:
  the committed projection `🗣️dsl/✨️derive/🔣️mutation-authority.json` (`MUTATION_AUTHORITY_LOCATOR`, workspace-relative), schema
  `MutationSourceAuthorityProjectionV1` (+ `MutationAuthoritySegment`) in `✨️derive/🧬️schema/🔣️.json`: `sourceFilename`,
  `descriptorFilename`, `mutationCollection`, `mutationPayloadFacet`, `mutationDomainOwners`, `mutationAggregateSources` (taxonomy
  order, so expansions are unchanged). Gone from every expansion: `include_str!` of root `nx.json`, root `📋️project.json` and
  `🔣️taxonomy.json` (the workspace root is still found by the `nx.json` + `📋️project.json` marker pair, existence only).
  Generator: `bun nx run @semio-tech/dsl-derive-rs:generate` (writes only when bytes change), `preview-generated`, `check-generated`;
  registered as generator contract `mutation-source-authority` in the taxonomy (`loadTaxonomy` validates, 23 contracts) and as two
  launch rows (`📦️generate|📦️check✨️dsl-derive🧬️mutation-authority`, 206.1693/206.1694). Laws: derive crate
  `cargo test -p semio-framework-os-kernel-dsl-derive` **16/16** (source-authority fixture cases renamed to
  `missing-authority`/`malformed-authority`/`symlink-authority` + new `unsafe-authority-segment`; aggregate `authority-filename-change`;
  expansion law: exactly one `include_str!`, the projection, none of nx/project/taxonomy; new law that the locator names this
  workspace's committed projection and it parses), TS oracle `test-source-authority-source` green. Effect: after this one-time
  recompile, taxonomy/nx/project.json edits no longer invalidate any guest closure (only a change of these six facts does).
- 20:48 **Item 3 (W2 item4 set) applied**: `w2-item4-apply.py --apply` (23 files) + `w2-item4-describe-codemod.py --apply` (120 files,
  0 problems), both dry runs clean on the current tree first; copy kept in `wp-w3/item4-w2-copy/`. `bun nx show project
  @semio-tech/note-plugin`: `describe` is the inferred component target (`dependsOn component-dev`, command `🖨️describe/…/📜️script.ts
  component --manifest …`), `materialize-dev` depends on `component-dev`. Compile-atomic checks follow.

### Session 13 Request Triage (21:2x)

| request | verdict |
|---|---|
| `wp-w3/requests/db1.txt` (semio-framework-async pool atomics) | **rebuild**: every guest re-described/re-materialized/released from the tree |
| `wp-w3/requests/ld.txt` (kernel store announce + replication range encoder) | **rebuild** (+ fresh 7800 roots: pre-rebuild WALs cannot replay, rule 23) |
| `wp-w3/requests/s17.txt` (VersionPin, 26 extensions, descriptors must re-describe) | **rebuild**: the chain's `components` step re-describes all 60; the preflight re-checks every committed descriptor's pins |
| `wp-w3/requests/wg9.txt` item 1 (link-shortage retry) | **rebuild** |
| `wp-w3/requests/wg9.txt` item 2 (echo suppression, "browser wgpu renderer wasm-release must be rebuilt") | guests: **rebuild**; the browser renderer `framework-renderer-wgpu:wasm-release` (wasm32-unknown-unknown over every plugin crate) is NOT in rebuild-all → **routed to main** (one extra wasm span after the B3 publish, or WG9's own under the mutex) |
| `wp-w1/requests/p8.txt` (lowpoly/fem/puzzle/reasoning/architect already in tree) | **rebuild** |
| `wp-w1/requests/p8.txt` (flow `p8-flow.py`, `p8-land.py` SDK/cad/space sets "NOT in the tree yet") | **needs landing** (not a W3 set) → **routed to main**; nothing in the Session 13 landing window shows them |
| `wp-w2/requests/wg7.txt` (declaration-tree `io.artifact_schema`) | landed 09-26 15:35 → **rebuild** (re-describe) |
| `wp-w1/requests/*` older rows | W2's session-12 triage stands (all served by describe-all/restage/catalog); r8 flow_core bindings are now the chain step `flow-core-bindings` |

### Consolidated Chain (prepared, measured parts only)

Coordinator decisions folded in: step 0 `cargo-provenance check`, flow-core bindings, B3 first then all, fresh 7800 roots, `os-hub.sources.json`
beside every 7800 binary + `hub-freshness` after each move, os-mcp for G11. `w3-chain.sh b3`: [hub `build-dev` prewarm (native) ‖
fleet `wasm` mutex: `rebuild-all --to flow-core-bindings` = provenance → mutation-authority → components (describe + materialize-dev,
`--parallel=2`) → generate → check → activate-s → verify-s → flow-core-bindings] → `trusted-catalog-preflight --packages all` (27 s) →
fleet `wasm` mutex: B3 bootstrap `stdio,gis,note,animate,block,writer,draw,puzzle,wfc` into `.🧬semio/🌐hub/s13-w3-catalog-b3` with ONE
warm-behind `component-release` lane walking the list from the end (wfc, puzzle, draw, writer, block, animate, note; stopped by a stop
file + SIGTERM to its own nx pid when the bootstrap ends; `W3_LANE=0` disables it) → `os-hub:build-dev` (sources record) +
`framework-os-mcp-rs:build` → `w3-restart-7800.sh` (fresh root `s13-w3-hub-7800-b3`, binary dir `s13-w3-bin/<root>/{os-hub,
os-hub.sources.json}`, users, old hold 28673/hub 54029 stopped) → readiness ≤ 2 h → `/readyz`, `hub-freshness`, footprint, open-plan
probe over every creatable kind, footprint. `w3-chain.sh all`: preflight → `--packages all` into `s13-w3-catalog-all` (warm-behind lane
over the remaining 25 from the end) → build-dev → 7800 onto `s13-w3-hub-7800-all` → the same verification. Logs `.🧬semio/🌐hub/s13-w3-logs/`.
- 21:30 usage cut; overnight every process died (7800 down: hold 28673/hub 54029 gone). 22:00 auto-commit took all my edits. 04:57 resume:
  tree consistent — taxonomy has the `mutation-source-authority` contract (`loadTaxonomy` ok, 23), `check-generated` fresh (a peer
  regenerated the projection at 00:44 after adding the gltf `📸️snapshot` domain owner).
- 05:0x peer compile fix (rule 14, one obvious line): WG9's `🏪️store/🔄️sync/🧪️tests/🔬️document-echo-suppression/🦀️.rs` initializer
  lacked LD's new `MutationEnvelope.observed`/`.target` (`observed: None, target: Vec::new()`); it blocked every kernel `--tests` check.
- 05:13 native check (build-landing): kernel lib (4 warnings) + lib test (17 warnings) **green** with the new derive input; plugin-host
  `--lib` fails on `missing field prepared_child_ops in AppCommand::TransactionPrepare` (`🔌️plugin/🖥️host/🦀️.rs:7343`): a peer is
  editing `📡️spr/🧵️channel/🦀️.rs` right now (mtime 05:05, not in `📓️landing.md`), not my code; retry after it lands.
- 05:0x chain dry runs of the early steps: `repo:cargo-provenance-check` rc=0 (0 foreign units, 16 s); `dsl-derive-rs:check-generated`
  rc=0 (fresh); `trusted-catalog-preflight --packages all` 42 s, rc=1 as expected before the rebuild: layout (fs, fixed at source),
  demonstrator, puzzle, raster dev deliverables ≠ committed descriptors (the rebuild re-describes them). Chain b3 now builds the browser
  renderer `framework-renderer-wgpu:wasm-release` first in the publish lane (never cut; output
  `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/dist/wasm-release`), then the warm-behind releases.
  `w3-restart-7800.sh` kills an old hold/hub only when its command still matches (pid reuse after the overnight restart).
- 05:15–05:20 **chain restructured for throughput** (`w3-chain.sh` + new `w3-wasm-hold.sh`, `w3-lane.sh`): all wasm32 work of a phase
  runs in ONE fleet `wasm` hold. b3: a forward warm lane releases the 9 B packages in the bootstrap's order WHILE rebuild-all runs (it
  starts once the native hub prewarm is done, so at most two cargos compile at a time; release and dev units differ, so the two do not
  wait on each other's unit locks except shared host units) → preflight → B3 bootstrap ‖ reverse lane (renderer `wasm-release` first,
  never cut or skipped; then B releases from the end, cut by TERM to their own nx pid when the bootstrap ends) → `b3-publish.rc` →
  the chain builds os-hub + os-mcp and moves 7800 while the hold keeps warming the other 25 releases in the `--packages all` order
  (`rest-warm`), which the `all` phase stops (stop file + TERM of the current component-release) before queuing its own hold.
  Simulated end to end with a fake `bun` and a pass-through mutex (`W3_HUB_ROOT`, `W3_MUTEX` overrides): ordering, stop/cut (rc=143 on
  the cut item), renderer immune to stop, `wasm_hold` publication rc hand-off — all as designed.
- 05:23 native landing check **green** (build-landing): kernel lib + lib test, os-mcp lib + lib test, os-run, plugin-host lib + lib test
  (G11 finished its `prepared_child_ops` landing at ~05:15).
- 05:3x `w3-restart-7800.sh`: re-signs the copied binary (`codesign -s - -f`, W2's rm+cp+codesign rule; the sources record carries no
  binary hash, so `hub-freshness` is unaffected); pid-reuse guards written as `if` statements (a `! cmd` inside an `&&` list is fragile
  under `set -e`). `nx show projects --with-target describe`: 61 = 60 components + the excluded describe tool. Waiting for REBUILD START.

### Session 13 Files

Landed (repo): `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/{Cargo.toml,📜️script.ts}`, `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`,
`Cargo.lock` (fontique edges), `💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/{📜️script.ts,🧫️fixtures/🧊️actor-factory/🔣️.json}`, `💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts`,
`🌎️hub/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`, `💻️os/🔨️modules/🗣️dsl/✨️derive/{🦀️.rs,🔣️mutation-authority.json,🧬️schema/🔣️.json,📦️packages/🦀️rust/{📜️script.ts,📋️project.json},
🧪️tests/🔬️{mutation-source-authority,mutation-aggregate-source-authority,mandatory-mutations}/🦀️.rs,🧫️fixtures/{🛂️mutation-source-authority,🏛️mutation-aggregate-source-authority}/🔣️.json}`,
`📚️library/🔣️taxonomy.json` (contract `mutation-source-authority`), `💻️os/🔨️modules/🔌️plugin/📇️registry/{🔁️rebuild/{🔣️.json,🟦️.ts},🧪️tests/{🔁️rebuild,🚀️launch}/🟦️.ts}`,
`.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc` (3 rows), W2's item4 set (143 files via its scripts), WG9's `🏪️store/🔄️sync/🧪️tests/🔬️document-echo-suppression/🦀️.rs` (1 line).
Ticket scripts (`wp-w3/`): `w3-chain.sh`, `w3-wasm-hold.sh`, `w3-lane.sh`, `w3-restart-7800.sh`, `w3-hub-resume.sh`, `w3-hub-hold.ts`, `w3-open-plan-probe.ts`,
`w3-detach.py`, `w3-import-scan.sh`, `w3-import-callers.ts`, `w3-admission-oracle.ts`, `w3-check-parley-wasm.sh`, `w3-check-landing.sh`, `tsconfig-hub-script.json`.
- 05:4x–05:52 **fast wasm32 gate (coordinator rule 29)** as a PRODUCT step of the chain: `🔁️rebuild/🔣️.json` step `guest-framework`
  (stage `inputs`, after `provenance` + `mutation-authority`, before `components`) = `nx run @semio-tech/plugin-registry:guest-framework-check`,
  whose checks are declared as data (`guestFrameworkChecks`): (1) `cargo check --lib --target wasm32-wasip2 -p semio-framework -p
  semio-framework-replication -p semio-framework-os-kernel -p semio-framework-os -p semio-framework-plugin --features
  semio-framework-plugin/component-guest,semio-framework-os-kernel/deflate,semio-framework-replication/deflate` (the feature set note's
  guest closure resolves, `cargo tree -e features`), (2) `--target wasm32-unknown-unknown -p semio-framework-os-kernel --features
  semio-framework-os-kernel/sync`, (3) `--target wasm32-unknown-unknown -p semio-framework-os-renderer-wgpu`; cargo stops at the first failing
  crate. `readGuestFrameworkChecks` validates targets/crates/crate-qualified features; `GuestFrameworkCheckScript` + registry target + launch
  row `🧊️check-guest-framework🔌️plugin-registry` (206.160155). Law `🧪️tests/🔁️rebuild` 4/4 (step order, both targets, arg vector, 3 hostile
  declarations). `w3-wasm-hold.sh` prints on failure: the failing chain step, the first `could not compile` crate and the first error block
  (simulated: `FAILED in rebuild-all 3/11 guest-framework … first crate: semio-framework-plugin`). Not run for real (wasm mutex queue ~12).
- 06:0x gate extended (coordinator, R9's uncommitted stdio rollout): 4th and last `guestFrameworkChecks` entry `cargo check --lib --target
  wasm32-wasip2 -p semio-s-plugin-stdio` on its own (default features = `plugin-root` = the feature set `component-dev`/`-release` build,
  `pluginComponentRustcArgs` passes no `--features`), so a stdio compile error stops the chain before the 60-component step. Law 4/4
  (asserts it is the last check).
- 09:53 **REBUILD START** (coordinator): chain b3 launched detached (pid 70074). provenance rc=0, mutation-authority rc=0 (40 s),
  **guest-framework gate green in 6.5 min**: wasip2 framework set 61 s, kernel[sync] 24 s, renderer-wgpu 130 s, stdio plugin 173 s.
  Hub prewarm (build-dev, sources record) 263 s rc=0 → forward warm lane started 09:57 (stdio release). 10:13 components 20/60
  described + materialized (extensions first), 0 compile errors. Monitor helper `wp-w3/w3-status.sh`.
- 10:1x–10:5x **run 1 failures (components step)**: `semio-s-artifact-raster-raster` 16 errors, root cause the Codex raster team's
  half-landed mutation `🎛️change-layer-adjustment-parameter` (01:55–01:57, untracked): leaf without its descriptor `🔣️.json` (first error
  `MutationLeaf source authority failed: No such file or directory`, the derive finds no descriptor), variant already in the `RasterMutation`
  aggregate + `pub mod`, codec matches not extended (E0004 at `💾️binary/🦀️.rs` :350/:2467/:2560/:2955/:3146, `📝️text/🦀️.rs:141`); NOT the
  mutation-authority projection (raster's ✳️any root is a flat owner). T13 parked the variant (native green 10:53). `semio-s-artifact-forms-forms`
  1 error (`NumberStepperProps` min/max), fixed by its owner. nx run-many keeps building the other components after a failure, so run 1 had
  46/60 described + materialized when stopped.
- 10:55 **clean stop** (coordinator): TERM to chain 70074, lane 70105, hold 70102, then nx run-many 78303 (its cargo 9077 + stdio dev rustc
  exited with it); the wasm mutex zsh finished and its trap released the lock; queue empty, hub mutex free. Deliberate survivor (agreed): the
  forward lane's stdio wasm-release build (nx 72813 → cargo 73798 → rustc 82552, 48 min into its single LTO crate). Run-1 logs renamed
  `s13-w3-logs/run1-*`. `w3-wasm-hold.sh` failure summary fixed for nx-prefixed lines (every failing crate + first real error), replaced by
  rename so a running hold keeps its open inode. 10:55:53 run 2 launched by the coordinator (chain 16364).
- 11:0x–12:08 **run 2**: gate green again (wasip2 94 s, kernel[sync] 82 s, renderer 190 s, stdio check), components 59/60 green; forward
  lane: stdio release rc=1 (waited on the survivor, which the coordinator killed at 11:5x in a memory emergency: swap 45/47 GB, the
  components' stdio wasm-dev rustc reached an 85 GB footprint), gis cut at the failure. **stdio failed to link**:
  `wasm-component-ld … failed to encode component … functions count exceeds limit of 1000000`. Root cause (measured by diff):
  the Codex stdio rollout (`🗄️stdio/🔌️plugin/🦀️.rs` mtime 01:15, `📦️packages/🦀️rust/Cargo.toml` 01:19; auto-committed 11:30 in
  `6b8089dcb21`) deleted the `full-app-catalog` / document-catalog split, so the default `plugin-root` component now closes `StdioApps`
  over all 88 subsets (176 apps, was 18). The deleted doc comment recorded the same measurement (≈600 000 functions for that fleet →
  over wasmparser's 1 000 000 ceiling, "library fleet only"). The 176-app monomorphisation in one CGU is also what drove rustc to 85 GB.
  My fast gate cannot see it (`cargo check` does no codegen or link). Routed to main (owner: Codex stdio team): restore the split, or
  split stdio into several components (design change). Chain ended 12:08:55 cleanly (no cargo left, mutex released).
- 12:1x–13:57 (coordinator-run relaunches while I was cut; from `chain-b3.txt`): runs 3–5 failed on 29 norm example slugs (coordinator
  renamed), Nx replaying cached registry steps (coordinator added `--skip-nx-cache` to generate/check/activate-s/verify-s in
  `🔁️rebuild/🔣️.json`), stdio html/tsv/txt open targets without linked codecs (LB: publisher treats them as unowned); stdio's 18-app
  component split restored by LB (lb-p3). **Run 6**: rebuild-all 1007 s, preflight 24 s, **B3 published 13:54 (330 s)**, renderer
  wasm-release rc=0 13:49 (beside the bootstrap), hub build 23 s, os-mcp build 122 s, restart 7 s → **7800 READY on B3 ~13:57**. At 14:0x
  the coordinator stopped the chain (rest-warm lane + hold; G11's kernel `DirectoryClient` fix forces a full recompile) and opened
  landing window 2, so the chain's own post-ready verification did not run; measured since: `/readyz` ready, `hub-freshness` works
  (`stale` after landing-window edits, as expected). Hub Handoff (B3) written above.
- 14:1x **phase `final`** (replaces the `all` phase): `w3-chain.sh final` = hub build-dev prewarm ‖ ONE wasm hold (`w3-wasm-hold.sh final`:
  forward warm lane over every release in the `--packages all` order, starts after the prewarm ‖ rebuild-all --to flow-core-bindings →
  preflight all → `--packages all` bootstrap into `s13-w3-catalog-all` ‖ reverse lane (renderer wasm-release first, never cut; then
  releases from the end) → `final-publish.rc`) → os-hub build-dev + os-mcp build → `w3-restart-7800.sh` onto a fresh root
  `s13-w3-hub-7800-all` (stops B3 hold 4540/hub 4548 via its stop file; B3 root + binary kept for rollback) → readiness, `/readyz`,
  `hub-freshness`, footprint, open-plan probe over every creatable kind, footprint. Refuses when the all root or a published all catalog
  already exists. `W3_DIR` added to the overrides. **Dry run** (fake `bun`, pass-through mutex, stub restart, scratch hub root): whole
  sequence rc=0 in 41 s — prewarm → rebuild → preflight → publish → rc hand-off → hub/mcp build → restart → READY → freshness → probe →
  footprints → DONE; lanes stop/cut as designed.
- 15:0x **release plugin-module root (WG9 finding)**: `…/🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules` is written by
  `@semio-tech/framework-plugin-web:support-release` (`🧵️shard/🟨️shard-worker.js` = `shardWorkerSource()`, `🪞️vendor/…/🪟️preview2-shim` copied
  and patched by `patchPreview2ShimGuestLogClassification`/`…LineRelease`, the guestslim font seed) and by every component's inferred
  `materialize-release` (dependsOn `component-release` + `support-release`; per module `🟨️.js` = `hostShimSource()`, `🌉️bridge.js` =
  `pluginComponentBridgeSource()`, the jco output post-processed by `rewritePreview2ShimImports`/`rewriteJcoAsyncResultLifting`/
  `rewriteJcoComponentAssetUrls` + wasm-opt'd cores, descriptor copies). Nothing had run them since 09-25 (the rebuild chain only runs
  `materialize-dev`). New check `wp-w3/w3-release-root-check.ts <dev|release>` (shard worker == `shardWorkerSource()` and has `case
  "codec"`, 60/60 registry modules present with all listed files, host shim == `hostShimSource()`, vendor cli.js present); oracle: dev
  root **passes** (60/60, 0 findings), release root **failed** (shard stale, no codec case). 15:0x ran `support release` directly (1 s, no
  cargo): release check **passes** now (shard current with the codec case) — WG9's release serves get the current worker immediately.
  Phase `final` gained, in the hold after the publication rc (so 7800's move is not delayed): `support-release --skip-nx-cache` →
  `nx run-many -t materialize-release --skip-nx-cache --parallel=2` (all 60 components, release builds warm from the lanes) →
  `w3-release-root-check.ts release`; the chain prints the result after its final wait. Dry run rc=0 (the check ran for real against the
  then-stale root and reported its 2 findings, as designed).

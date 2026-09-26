# WP-W3 — All-Plugin Build/Publish Owner (successor of [W2](📓️wp-w2.md))

Session 13 slice W3. Canonical hub 7800, hubs 8000–8009, serves 6500–6509. Scripts `wp-w3/`, captures `wp-w3/generated/`, durable
data `.🧬semio/🌐hub/s13-w3-*`. Handover: [📓️wp-w2.md](📓️wp-w2.md) (Hub Handoff, Session 12, §2/§3).

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | Publish 4 root cause (`browser actor artifact: unsupported import interface`) | **FIX LANDED** 19:4x (parley `no_std`+`libm` in ui-render + ui; boundary law); native + wasm32 checks green, ui-render 136/136; layout component-dev re-check BLOCKED by LD's in-flight kernel `MutationEnvelope` edit |
| 2 | Fail-fast preflight (browser-actor import admission + late admissions) over all packages | **LANDED** 20:3x: `browserActorImportAdmissionV1` (jco oracle 34/34 release components agree), actor build refuses early + asserts codegen == admission, `trusted-catalog-preflight` verb (60 components in 27 s); findings: layout (fs), demonstrator/draw/puzzle dev deliverables stale (rebuild-all fixes) |
| 3 | Land W2's prepared sets (item4 + supervised 7800 restart; `VersionReq` narrowing moved to S17 by the coordinator) | item4 **APPLIED** 20:48 (dry runs 23 + 120 files clean, inferred `describe` verified); checks RUNNING |
| 4 | Kernel derive macro reads a narrow generated input | **LANDED** 20:4x: committed projection `✨️derive/🔣️mutation-authority.json` (schema + generator contract + generate/preview/check verbs), expansions include only it; derive tests 16/16; kernel check RUNNING (lock wait behind 4 peer kernel checks) |
| 5 | Requests triage (`wp-w1`, `wp-w2`, `wp-w3` requests) | **DONE** 21:2x (table below): all served by the consolidated rebuild + catalogs, except WG9's browser wgpu renderer `wasm-release` and P8's un-landed patch sets → routed to main |
| 6 | ONE consolidated chain (rebuild-all → B3 → 7800 → `--packages all` → 7800) | **PREPARED** 21:2x: `wp-w3/w3-chain.sh b3|all` (+ `w3-restart-7800.sh`, `w3-hub-hold.ts`, `w3-hub-resume.sh`, `w3-open-plan-probe.ts`); product chain gained `provenance`, `mutation-authority`, `flow-core-bindings`, `preflight-catalog` steps; waits for REBUILD START |
| 7 | Hub Handoff rewrite when 7800 serves the all-package catalog | PENDING |

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

# Dependency Direction Audit

Read-only audit of the layering rules from the parent ticket `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`, run on 2026-10-15. No source file was edited and no modifying git or build command was run. Raw gate outputs are in `🗑️generated/` next to this file (the `gate-*`, `depcruise-*`, `cargo-*`, `rust-include-*`, `ts-*` and `plugin-dir-*` files; other files in that folder belong to other agents and were not touched).

Rules audited:

- R1: `🧰️framework` (including the `💻️os` product) must not know `✏️s` or any concrete plugin.
- R2: `✏️s` must not know concrete plugins.
- R3: plugins must not know their extensions.
- R4: General (framework or neutral) code must not import Specific (repo product or `✏️s`) code.

## 1. How the gates are run

Root `📜️script.ts`, `verify` route (`runGate` at about line 8240, `runLayering` at about line 8215):

| Command | What it does | Status in this audit |
|---|---|---|
| `bun ./📜️script.ts verify gate` | Pre-`ticket_close` gate. Runs canonical-architecture, a direct dependency-cruiser boundary run, generated-catalog check, renderer lint, owned-schema freshness, storybook, `verify layering`, and more. | Not run end to end (it is long and writes). Its dependency-cruiser step was run directly, see below. |
| `bun ./📜️script.ts verify layering` | Calls `bun repo run-many -p @semio-tech/repo-lib -t lint-dependency-direction lint-rust-source-direction lint-cargo-dependency-direction --skip-cache`. | Not run. The `bun repo` runner blocked on `cache:leases-store` (contention with other devs' `repo:test-*` jobs running on this machine) and was stopped by the 280 s alarm. I did not kill other processes. |
| `bun ./📜️script.ts verify dependencies literal-external` | Red-until-zero third-party literal dependency truth gate (source only). | Run. Result below. |
| `bun ./📜️script.ts verify dependencies summary/list/self-test` | Source-only dependency truth views. | Not run. |
| `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` | The exact TS boundary command used by `verify gate`. | Run. Result below. |
| `bun ./📜️script.ts lint dependency-direction` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript`) | Nx target `lint-dependency-direction`. Requires the parent `repo` invocation envelope. | Not runnable standalone (`Original parent Script process invocation required`). |
| `bun ./📜️script.ts lint cargo-dependency-direction` and `lint rust-source-direction` (same cwd) | Nx targets `lint-cargo-dependency-direction` and `lint-rust-source-direction`. | Not runnable standalone for the same reason. Cargo result replaced by static scan (section 4). No cargo compilation was attempted. |
| `bun ./📜️script.ts lint framework-neutral-product-direction` (same cwd) | Nx target `lint-framework-neutral-product-direction`. | Not run. |

Rule source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` (`buildDependencyDirectionPolicy`), loaded through `🧭️direction/🚀️bootstrap/🟨️.cjs` and the cruiser config `🧹️lint/🕸️dependency-boundaries/🟨️.cjs`. Dump of all 1603 generated rules: `🗑️generated/depcruise-rules.txt`.

## 2. Current counts

### 2.1 Gates

- dependency-cruiser boundary run: **14441 error lines**, 15725 modules and 38437 dependencies cruised. Per-rule counts: `🗑️generated/depcruise-rule-counts.txt` (185 distinct rule names). Raw: `🗑️generated/gate-tsforbidden-depcruise.txt`.
- `verify dependencies literal-external`: **target 0, current 2543** (rust 183, js 2318, python 42, go 25 first-party). oracle-conflicts 3 (`rust:image`, `rust:quick-xml`, `rust:serde_json`). Raw: `🗑️generated/gate-verify-dependencies-literal-external.txt`. This is a third-party literal-dependency gate, not a direction gate.
- Cargo layering lint: **not run**. Static scan result: 0 path dependencies from any framework `Cargo.toml` into `✏️s/` (section 4a).

### 2.2 Discrepancy with the parent ticket

The parent summary says "TS108 forbidden edges" and "Cargo301 declarations plus 9 role mismatches". The cruiser in this audit reports 14441 error lines, which is a different metric (one line per cruised edge per rule). The TS108 and Cargo301 numbers come from the repo-lib lint targets, which I could not run. I did not reconcile them. Treat the numbers below as edge counts from the cruiser, not as the ticket's gate counts.

## 3. Categorized violations (cruiser, by direction rule)

Counts are cruiser error lines. "Production" means not under a `🧪️tests` folder.

| Category | Rule (cruiser name) | Edges | Production? | Evidence |
|---|---|---|---|---|
| R1: framework/os → s (TS) | `framework-no-implementation`, `framework-no-s`, `framework-no-plugin-packages`, `no-cross-technology-🧰️framework-to-✏️s` | 17 (16 for plugin-packages) | **All test-only** | `🗑️generated/ts-framework-to-s-edges.txt`. Sources: `🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🟦️.ts`; `…/📺️renderer/🧑‍🎨engine/🧪️tests/♿️editable-controls/🟦️.tsx`; `…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`; `…/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧪️staged-arg-controls/🟦️.tsx`; `…/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts`; `…/🧑‍💻dev/🧪️tests/🧪️multi-shell-harness/🟦️.tsx` (→ `✏️s/🧑‍💻dev/🧩️catalog`). |
| R1: framework/os → s (Rust include or #[path], Cargo) | `#[path]`, `include_str!`, `include_bytes!`, `include!`, Cargo `path =` | **0** | n/a | `🗑️generated/rust-include-path-to-s.txt` has only comment lines. `🗑️generated/cargo-framework-to-s-edges.txt` has one comment line. No Cargo `path =` into `✏️s/` (`cargo-framework-pathdeps-sample.txt`). |
| R1: framework → s (literal ids and dir paths in production code) | hardcoded `s.` schema ids and `✏️s/🔌️plugins` paths | ~10 id sites, 21 path lines in 9 files | **Yes** | Section 3a. |
| R4: framework/os → repo product | `framework-no-products` | 77 | Yes (Rust and TS) | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units`; `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts → …/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`; many `…/📦️packages/🦀️rust/📜️script.ts → …/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts`. Mostly Cargo build-infra, parent-owned. |
| R4: general UI → framework root barrel and repo build infra | `ui-no-framework-packages` | 231 | Yes | e.g. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx → 🧰️framework/📦️packages/🟦️typescript`; `🖱️ui/🪟️viewport/📦️packages/🦀️rust/📜️script.ts → …/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo` (parent-owned). |
| Framework renderer may only host UI | `renderer-hosts-only-ui` | 466 | Yes | e.g. `…/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/* → …/📇️registry/🌎️hub-source/🔍️resolution/🟦️.ts`; `…/🧱️elements/🖌️Paint2dHost/✍️editing/* → 🧰️framework/🔨️modules/🔲️pixels/🎯️selection/🟦️.ts`; `…/🧱️elements/👕️canvas-presence/* → 🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🟦️.ts`; `…/🧱️elements/🎬️MediaTransportHost/…` (parent-owned Media). |
| R2: s → concrete s plugin | `no-cross-plugin-*` (32 edges), `plugins-framework-sdk-only` overlap | 32 | Yes | Section 3b. |
| R3: plugin → its extension or artifact | `plugin-no-extension-or-artifact-*` | 2 | Test-only | `✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🟦️.ts → ✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/🧪️tests/🔬️unit/🟦️.ts`; `✏️s/🔌️plugins/🏛️architect/🧪️tests/🧪️graph-tool/🟦️.ts → …/🏛️architect/…`. |
| s → framework internals (allowed direction, SDK boundary broken) | `no-cross-technology-✏️s-to-🧰️framework`, `plugins-framework-sdk-only` | 1758 / 1688 (overlapping) | Yes | Targets: `🧰️framework/📦️packages/🟦️typescript/🌱️value/🔢️ieee754` (270); `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts` (245) and `…/🧩️artifact` (167) and `…/🔢️ieee754` (71); `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` (87); `🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator` (52); `…/🧾️record` (37); `…/🗿️artifact-reference` (37); `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store` (78, parent-owned). |
| s → os store internals (direction allowed, boundary broken) | `no-state-outside-os` | 80 | Yes | `✏️s/🔌️plugins/✒️writer/…/🧬️schema/🔺️diff/🟦️.ts → 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts`. Parent-owned (Store). |
| Hub and mit-bestand → framework (specific → general, allowed; deep relative) | `no-cross-technology-🌎️hub-to-🧰️framework`, `no-cross-technology-♻️mit-bestand-to-🧰️framework` | 164 / 62 | Yes | Hygiene only, not a direction violation. |
| Cycles | `no-circular` | 518 | Yes | Top clusters: `os/🔨️modules` 134; `🖱️ui/🧱️elements` 61; `🖱️ui/🧬️contract` 42; `🖱️ui/🎯️targets` 35; `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` 31; `os/🎚️config` 14. |
| Naming (banned `base`/`core`/`shared`/`util` stems) | `no-core-path` | 867 | Yes | 717 come from `✏️s/🔌️plugins/🗄️stdio` (`🧱️base` 631, `✉️base` 61); then `🌀️procedural` 25, `🧱️block` 22, repo 33, os 20. Hygiene. |
| Undeclared dependencies | `not-to-unlisted` | 823 | Yes | Resolved through the root hoisted `.cache/.../node_modules`. Top sources: repo library 78, renderer 60, `🖍️draw/…/🖍️drawing` 55, `🌀️procedural/…/🧊️generation3d` 31, `🔌️plugin` 23. Hygiene. |

### 3a. Hardcoded s ids and s paths in framework production code (R1)

- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:17,20,23`: `CARRIER_BINARY` `s.stdio.binary`, `CARRIER_TEXT` `s.stdio.txt`, `SQLITE_SNAPSHOT` `s.framework.sqlite-snapshot`.
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts:5`: `SQLITE_SNAPSHOT` `s.framework.sqlite-snapshot`.
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:676-677`: `CARRIER_BINARY_DIALECT` `s.stdio.binary`, `CARRIER_TEXT_DIALECT` `s.stdio.txt`.
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:2952,3072`: `artifact_kind: "s.stdio.json"`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1495-1497` and `…/🎯️targets/🧊️wgpu/🦀️.rs:748`: `"s.space"` index schema.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts:343,356,357`: `CLIENT_E2E_PINNED_*` constants `s.note.note`, `s.wfc.bitmap`, `s.wfc.bitmap.solve`.
- Repo product inside the framework tree, reading plugin directories, 21 non-comment lines in 9 files (`plugin-dir-hardcode-noncomment.txt`):
  - `🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🏛️abstraction/⚖️law/🟦️.ts:95-96`
  - `…/🧬️schema/🗿️artifact/🔍️owner-discovery/🟦️.ts:13`
  - `…/🧬️schema/🗺️surface/🔍️owner-discovery/🟦️.ts:9`
  - `…/🧬️schema/🗺️surface/⚖️laws/🚚️config-relocation/🟦️.ts:36,43`
  - `…/🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts:12`
  - `…/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs:92,99,203,334,338,341,354,362,416` (policy rule generator)
  - `…/🕸️dependencies/🧭️direction/🚀️bootstrap/🟨️.cjs:97`
- Parent-owned skip: `🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts:173,888` (Cargo host build script reading stdio plugin files).
- Not re-verified: the ticket's claims about `studio` keyword, `DEFAULT_HOST_VARIANT` and `"s"` defaults in plugin-registry and build scripts.

### 3b. Plugin-to-plugin (R2, R3)

Cross-plugin edges from the cruiser (about 32, plus 2 R3 test edges): `layout→forms` 6; `sourcing→stdio` 4; `layout→stdio` 4; `procedural→stdio` 4; `dag→stdio` 3; `raster→stdio` 2; `trinity→stdio` 2; `forms→stdio` 2; `energy→cad` 2; `block→stdio` 1; `tests→stdio` 1; `tests→energy` 1. Stdio is the hub target of most of them.

## 4. Static scans (Cargo, Rust, TS)

a. Framework Cargo path dependencies into `✏️s/`: **0** (`cargo-framework-to-s-edges.txt`). The ticket's six edges (surface→puzzle, →gis-terrain, os-host/os-flow/infinite/wgpu-renderer→semio-s-3d/2d, ui-wgpu `#[path]` into `✏️s/🔨️modules/🧊️3d`) are no longer present. The 3d/2d edges now point to framework `🔨️modules/🧊️3d` and `◻️2d`.
b. Framework Cargo path dependencies outside `🛍️products` that point into `🛍️products`: **0** outside tests (`cargo-framework-to-products.txt`). `semio-framework-os-kernel` is consumed only by os products (`🏃️run`, `🌊️flow/🗿️artifacts`), not by framework crates. The ticket's "framework depends on os-kernel" inversion appears resolved.
c. Rust `#[path]`, `include_str!`, `include_bytes!`, `include!` into `✏️s/`: **0** outside comments and fixtures (`rust-include-path-to-s.txt`). The ticket's "390 include_str! paths in `🧬️schema`" is not present now.
d. TS/TSX relative imports under framework resolving into `✏️s/` (non-test): **0** (`ts-import-framework-to-s.txt` is empty). All framework→s TS edges are package-specifier or test-only (section 3).
e. `mesh-engine/🚪️io/🦀️.rs` has 172 lines with 1 `dwg` mention and 2 `stdio` mentions. The ticket's ~3500-line stdio codec copy in `🔺️mesh/🦀️component.rs` is not present (`🏗️mesh-engine` has no matching component file). Stale claim.
f. `🚪️io/📇️registry/📇️catalog.json`: the directory does not exist. Stale claim.
g. `🛂️manifest` closed `Contribution` enum: replaced by `TopicContribution` (string topic plus payload, `🛂️manifest/🦀️.rs:6172`). Stale claim.
h. Remaining `"s.` literals in os and framework modules (non-test, non-comment): 25 lines. Listed in section 3a. Most others are doc comments or dist output.

## 5. Ranked work packages

Ranking: independence first, then impact on R1/R2/R4. "Parent-owned" means the package touches Cargo build-infra, Actor, Store, Member or Media files of the parent ticket and must be skipped here.

1. **Move framework s-ids to framework-owned ids** (R1, S, about 8 files). Files: `🚪️io/🧬️schema/🦀️.rs`, `🚪️io/🧬️schema/🟦️.ts`, `🎠️kernel/🟦️.ts`, `os/🖥️host/🦀️.rs`, `os/…/🏛️ShellHost/🟦️.tsx`, `os/…/🧊️wgpu/🦀️.rs`, `os/🌉️mcp/🟦️.ts`. Wrong edge: framework carrier and dialect constants name stdio and sqlite plugin kinds. Correct: framework defines neutral ids (`io.binary`, `io.text`, `io.sqlite-snapshot`, `os.space-index`) and each s plugin registers its concrete kinds through the existing `register_composer_entries` / dialect registration. Delete the old ids outright (greenfield).
2. **Relocate the 17 test-only framework→s imports** (R1, S, 6 files). Files listed in section 3, first row. Wrong edge: os renderer and flow tests import procedural, draw, stdio and dev plugins. Correct: move these tests into the owning plugin's test tree, or into a repo-level integration test root that is allowed to depend on s. This clears the TS `framework-no-s` gate with no production change.
3. **Repo product stops hardcoding plugin roots** (R1 inside the framework tree, S-M, about 9 files). Files in section 3a (repo lib ownership/discovery/config-relocation/direction policy and bootstrap). Wrong edge: repo-lib and the direction policy scan `✏️s/🔌️plugins` directly. Correct: one injected plugin-root constant from taxonomy `pluginAreas`, and the direction policy's plugin rules generated from s-side manifests. This also removes the "studio/`DEFAULT_HOST_VARIANT`" hardcoding the ticket describes (not re-verified here).
4. **Stdio becomes a neutral carrier, not a shared plugin** (R2, M, about 9 consumer files plus stdio). Edges: `sourcing→stdio` 4, `layout→stdio` 4, `procedural→stdio` 4, `dag→stdio` 3, `raster→stdio` 2, `trinity→stdio` 2, `forms→stdio` 2, `block→stdio` 1, `tests→stdio` 1. Wrong edge: plugins consume symbols from another plugin. Correct: the symbols each consumer needs (codecs, carriers) move down into `🚪️io` (framework) or a neutral package; plugins depend on the framework only. Parent-owned: none of these files are Cargo, Actor, Store, Member or Media.
5. **Fix the layout↔forms and energy↔cad plugin edges** (R2, S, 4 files). Edges `layout→forms` 6, `energy→cad` 2. Correct: move the shared type or helper down into framework or into the consuming plugin; or invert with a framework registration that the other plugin implements.
6. **Make test-only plugin cross-imports go through fixtures** (R3, S, 2 files). `✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🟦️.ts` and `🏛️architect/🧪️tests/🧪️graph-tool/🟦️.ts` reach into extension and artifact folders. Correct: copy the fixture into the test's own folder, or reference it through the owning plugin's public test export.
7. **Ship an SDK barrel for `sqlite-snapshot` and make s import it** (R1/R2 SDK boundary, M). Edges 483 (`🪶️sqlite-snapshot/🟦️.ts` 245, `/🧩️artifact` 167, `/🔢️ieee754` 71, `/🧪️tests` 47). Wrong edge: s deep-imports framework relative internals. Correct: framework module exposes a package `exports` entry; s imports `@semio-tech/framework-…` specifiers only; mechanical codemod with a lint rule. Start here because it is the largest mechanical reduction.
8. **Same for `ieee754`/value and the root framework barrel** (SDK boundary, M-L). Edges: `🌱️value/🔢️ieee754` 270; `📦️packages/🟦️typescript/🟦️.ts` 87; `🔮️oracles/✅️validator` 52; `🧾️record` 37; `🗿️artifact-reference` 37. Same correct direction as item 7.
9. **Break UI internal cycles** (cycles, M, about 140 edges). `🖱️ui/🧱️elements` 61; `🧬️contract` 42; `🎯️targets` 35. Correct: move shared types into a contract leaf file; no new dependency in either direction. Fully independent of parent-owned files.
10. **UI stops importing the framework root barrel and repo build infra** (R4, M, about 230 edges). `ui-no-framework-packages`: replace `🧰️framework/📦️packages/🟦️typescript` barrel imports with specific subpath imports, and move `🧬️dsl` or `ownership` types down if needed. `🪟️viewport/📦️packages/🦀️rust/📜️script.ts` imports repo cargo admission and is parent-owned (skip that one file).
11. **Renderer elements consume only UI contract and injected ports** (R4, L, about 460 edges). `renderer-hosts-only-ui`. Biggest specific items: `WasmSessionLoader` → `🌎️hub-source/🔍️resolution` and flow `🫀️core/🕸️bindings`; `Paint2dHost` → `🔲️pixels/🎯️selection`; `canvas-presence` → `📡️replication`; `TaskManager` → `🎭️actor/📮️shard-client` (Actor, parent-owned, skip that edge). `MediaTransportHost` edges are parent-owned (Media). Correct: os host injects the hub/session port into the renderer; renderer imports only `ui` contract.
12. **Break the `os/🔨️modules` cycles** (cycles, L, 134 edges in os modules plus 14 in `🎚️config`). Run `depcruise --output-type dot` on `os/🔨️modules` to get the strongly connected components, then break the smallest cut. Skip any component touching `🏪️store`, `🎭️actor`, `🪟️member` or `🎬️media` (parent-owned).
13. **Declare undeclared dependencies in nearest `package.json`** (hygiene, S-M, 823 edges, mechanical). `three`, `sharp`, `jsonc-parser`, `d3-scale`, `web-tree-sitter`, `fast-json-stable-stringify` and others are resolved from the root hoisted `node_modules`. Correct: declare them where used, then re-run the cruiser.
14. **Rename `base`/`core` stems inside stdio, procedural, block** (naming, M, 717 edges in stdio, about 50 elsewhere). `🗄️stdio/🗿️artifacts/…/🧱️base` (631) and `✉️base` (61). Pure rename plus path updates. Do after item 4, which moves files out of stdio.
15. **Hub and mit-bestand use package specifiers for framework** (hygiene, M, 226 edges). `no-cross-technology-🌎️hub-to-🧰️framework` 164, `♻️mit-bestand-to-🧰️framework` 62. Specific → general is the allowed direction; this only removes deep relative imports. Lowest priority.

Parent-owned, skip (listed for completeness, not ranked):

- `framework-no-products` (77 edges): framework build modules importing repo cargo admission, caching and storage (`🏃️process/🦀️cargo/🎟️admission`, `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler`). Cargo build-infra.
- `no-state-outside-os` (80 edges): s and framework importing `os/🔨️modules/🏪️store`. Store.
- Actor, Member and Media edges inside items 11 and 12.
- `os/🖥️host/📦️packages/🦀️rust/📜️script.ts:173,888` (Cargo host build script reading stdio files).
- `🖱️ui/🪟️viewport/📦️packages/🦀️rust/📜️script.ts` (cargo admission import).

## 6. Not done and why

- Cargo layering lint (`lint-cargo-dependency-direction`) and `verify layering` could not run here: `bun repo` blocked on `cache:leases-store` under contention from other devs' jobs. The static scan in section 4 is the substitute. The cargo count (Cargo301 declarations and 9 role mismatches) is therefore unverified in this pass.
- The TS108 figure from the parent ticket could not be reproduced. The cruiser's 14441 lines are a different metric. Reconcile by running `verify layering` once the cache lease is free.
- `lint dependency-direction` and `lint rust-source-direction` need the `repo` invocation envelope, so they were not run standalone.
- No cargo compilation was attempted.
- Cleanup: the files I created in `🗑️generated/` (listed at the top) are tool output and should be removed when the ticket closes. The other files there (`seam-*`, `infinite-*`, `renderer-*`, `shell-*`, `decl-*`, `region-*`, `inner2-*`) belong to other agents and were not touched.

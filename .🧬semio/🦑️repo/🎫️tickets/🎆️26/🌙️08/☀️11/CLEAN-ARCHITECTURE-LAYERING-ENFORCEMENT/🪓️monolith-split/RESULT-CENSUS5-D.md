# Result CENSUS5-D (prefix `c5d`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool, no cargo/nx/bazel. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/c5d-tools/` (the aborted predecessor copy of `c5b-tools` was overwritten; nothing of it had edited the repo). Logs: `F/🗑️generated/c5d-*` (nothing deleted). Sandboxes, stages and stubs: `/Users/ueli/.cache/c5d/`.

## 0. Verdict

**DONE for packages 10, 11 and 12 (static evidence; the real `bun repo cargo` check is the coordinator's).** Last update 2026-10-18 02:3x.

| Pkg | File | Before | Root after | Children (lines) | Mechanism |
| ---: | --- | ---: | ---: | --- | --- |
| 10 | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🦀️.rs` | 1,139 | 68 | 8 (115, 106, 146, 125, 208, 134, 153, 142) | `#[path]` modules, `pub use child::*` |
| 10 | `🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️.rs` | 1,672 | 87 | 14 (62 to 189; the inline `geom_sel` module became the file `🎯️world-selection`) | `#[path]` modules, `pub use child::*` |
| 12 | `🌎️hub/🧪️tests/🔬️bin-unit/🚀️boot/🔬️unit/🦀️.rs` | 1,894 | 71 | 8 fragments (182 to 299) | `include!` fragments (test names unchanged, 43 = 43) |
| 11 | `PL/🌐️browser-bundle/🏗️materialization/🟦️.ts` | 1,562 | 78 | 8 leaves (23 to 710) | sibling folders with `🟦️.ts` leaves, explicit named imports |
| 11 | `PL/📇️registry/✅️catalog-verification/🟦️.ts` | 1,052 | 159 | 9 leaves (44 to 190) | sibling folders with `🟦️.ts` leaves, explicit named imports |

Largest new file: 710 lines (`🏗️materialization/🧵️shard-worker-source`, one 700-line generator function that stays whole). Longest new path: 168 UTF-8 bytes relative, 141 characters absolute. No `export *` barrel and no compatibility re-export was added; the two `export { … } from` re-exports of `🏗️materialization` (`PREVIEW2_VENDOR_RELATIVE`, `rewritePreview2ShimImportSource`) were dropped and their importers point at `🕸️imports` directly.

## 1. Found state

- `F/c5d-tools/` existed with 39 copies of `c5b` tools, empty `orig/`, `specs/` (only `lib.py`), no stage and no `🗑️generated/c5d-*`: nothing had been edited. Rebuilt from `c5c-tools` (sed `c5c`→`c5d`).
- Snapshots: `F/c5d-tools/orig/{rcc,ge,hb}.rs`, `{bm,cv}.ts`, hashes in `orig/SHA256`; tag table `F/c5d-tools/files.tsv`. Live line counts: 1,139 / 1,672 / 1,894 (census said 1,896) / 1,562 / 1,052.
- A peer (CENSUS5-E, package 13) edited a comment inside `shardWorkerSource` of the `🏗️materialization` file between the census and my install (`📜️.wit` to `📜️reactor.wit`). I refreshed the snapshot and re-staged; the edit lives in `🧵️shard-worker-source/🟦️.ts` now. A peer script that anchors on the old file will fail on that anchor.
- Baseline census (gate): `F/🗑️generated/c5d-census-own-before.jsonl`.

## 2. Packages 10 and 12 (Rust)

New folders (all RGI, resolved by the live taxonomy, unique among siblings, used as folder prefix elsewhere): contract `📜️clone-and-cursor-traits 🔢️scalar-cursor 🧮️array-cursor 🔤️string-cursor 📚️vec-cursor 🎁️option-cursor 🗃️box-cursor 🧱️tuple-cursors`; geometry `🧩️shape-ref 📍️point-and-vector 🧮️affine 🔲️rect-shapes ⭕️curve-shapes 🛤️bez-path ✂️path-seg 🌓️arc-flattening-and-path-append 📏️generic-geometry 📈️curve-extensions 🔺️polygon-extensions 🧭️vec3 🧱️mat4 🎯️world-selection`; boot `🔑️startup-auth-and-readiness-laws 🚪️catalog-stall-gates-and-artifact-creation-laws 🛰️startup-progress-check-in-and-payload-laws 🛡️hostile-input-and-socket-batch-laws 🧱️batch-limit-space-read-and-refusal-layer-laws 💡️gis-map-approval-and-inference-fixtures 🧭️gis-map-proposal-and-inference-laws 🗂️admin-extension-and-canonical-pair-laws`. Specs `c5d-tools/specs/{rcc,ge}.py`, `FRAG["hb"]` in `c5d-tools/specs5.py`.

| Check | Result |
| --- | --- |
| lossless (`modsplit.py lossless`, `chk.py`) | rcc and ge: missing 0, extras are wiring only (`#[path]`, `mod`, `pub use`, docstring, `use super::*;`); ge declares the removed `pub mod geom_sel {` and its brace (unwrap). hb: spliced sequence equals the snapshot, gone 0, extra 0, test fn multiset 43 to 43, two balanced region markers added (the `💡️Inference` region spans a cut) |
| structure (`vmod.py live`, `chk.py live`) | every child mounted once, docstring emoji first/equal to the folder/unique/different from the root, regions balanced, brace depth 0, rustfmt parse, include targets, path lengths: `VMOD OK`, `CHK OK` |
| `defscan.py`, `ambig.py`, `modclash.py` | 0 cross-file private leftovers, 0 ambiguous glob names, no module-name clash |
| `fmt.py` | rustfmt hunk counts: rcc 60 to 61, ge 6 to 6; changed-line multisets identical |
| scoped patched differential (`oracle5.py`, original restored in one sandbox against the live tree in the other, `-o` moved to `--out-dir` because `/dev/rmeta*` is refused in this sandbox) | **ge (geometry primitives has no external dependency): `rustc` compiles the lib crate with 0 errors and 0 warnings on both sides; test config 45 errors on both sides (missing `kurbo`) with identical multisets**; rcc: 676 / 859 errors on both sides (extern crates removed), identical multisets and warnings. After the install an edition-2024 `rustc --crate-type=rlib` metadata compile of the geometry root (the form of the native specimen test) is clean on stable and nightly-2026-07-20 |
| typeck oracle (`posrun.py`, family `c5d10`, labels `pre` before the install, `post`/`post2` after) | rcc: canary E0308/E0616/E0624 reached, privacy 0/0, trait-scope lib 0 and test 14 (equal), `POSDIFF IDENTICAL`: lib 1205 = 1205 rows, test 1983 = 1983 rows, 0 only-pre, 0 only-post |
| gate census (lib / test closure, files, bytes, active lines) | rcc 9 to 17 / 22 to 30 files (ratio 0.938, bytes 237,293, lines 3,524); ge 1 to 15 / 5 to 19 files (0.619); hb test closure 13 files (0.596, 1,413 lines). Logs `c5d-census-own-after-p10.jsonl`, `c5d-census-own-after-p12.jsonl` |

Source-text readers re-pointed (package 10 and 12):
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🟦️.ts`: three `readFileSync` of the contract root now read the root plus the cursor modules it mounts from its own folder (`inspectRustPathMounts`, sibling owners and tests excluded). The three anchor assertions were replayed in Python over the new files and hold (bun test cannot run: no `node_modules`).
- `🧰️framework/🔨️modules/🖼️canvas/♻️retirement/➰️path/🧪️tests/🟦️.ts`: geometry read became `readModuleTreeSource`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧪️tests/🧫️fixtures/📦️specimens.json`: the `semio_framework_geometry_primitives` specimen lists its input closure, now 15 files (root first, then the 14 children in mount order); the native specimen law compiles the root with plain `rustc`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🧫️fixtures/🔣️.json` `rustReceivers`: the boot unit root (no `artifact_hash` use left in it) was replaced by the two fragments that use the receiving API.
- Checked and unchanged: `crate-boundaries/{🧬️cuts.json,🔣️.json}` (the root keeps the canonical `pub use semio_framework_value_clone_source::{…}` line and the entry path), `hub/🧪️testing/📖️source-readers` (`readHubBinTestSource` already reads the module tree incl. `include!`), the hub project inputs (`**/🦀️.rs`). A scan of every `include_str!`/`include!`/`#[path]` and of every relative string literal in TS found no other reader.

## 3. Package 11 (TypeScript)

Method: `c5d-tools/tssplit.ts` (TypeScript compiler API: top-level statements to leaves, imports re-synthesised per file from the identifiers each file references, `export` added only where a non-exported declaration crosses a file, region markers re-opened and closed per file), `tsrewrite.ts` (every importer re-pointed name by name), `tsinstall.py`, oracles `tscheck.ts` and `tspubowners.ts`. Specs `c5d-tools/specs-ts/{bm,cv}.ts`.

### 3.1 `🏗️materialization` (1,562 lines to root 78 + 8 leaves)

| Leaf | Lines | Content |
| --- | ---: | --- |
| root `🟦️.ts` | 78 | `rewritePreview2ShimImports`, `transpilePluginComponent`, `spawnNodeBinAsync`, `transpilePluginComponentModulesAsync`, `transpilePluginComponentAsync` (shebang and file docstring kept) |
| `🧾️contract` | 64 | file names, shard byte/heartbeat/diagnostics/drive declarations, `PluginWebMaterializeContext` |
| `🔤️guestslim-fonts` | 23 | font seed and placement |
| `🩹️preview2-shim-patches` | 89 | vendored shim copy, log classification and whole-line release patches |
| `🧵️shard-worker-source` | 710 | `shardWorkerSource` |
| `🌉️component-bridge-source` | 234 | `pluginComponentBridgeSource` |
| `🪶️wasm-opt` | 68 | arguments, tool selection, blocking and async core optimization |
| `🧬️jco-module-rewrites` | 123 | async result lifting, versioned host shim and core asset urls (the only file that imports `typescript`) |
| `🪢️host-shim-source` | 195 | `hostShimSource` |

- Lossless: 1,450 non-import/non-blank lines before, 1,450 after, missing 0, extra 0 (8 docstring lines are the only additions); regions 4/4 balanced. Declared changes: three declarations became `export` because the root uses them (`rewriteJcoAsyncResultLiftingAt`, `rewriteJcoComponentAssetUrlsAt`, `optimizePluginCoreModules`); the unused `import { spawn }` was not carried; the two `export { … } from "../🕸️imports"` re-exports were dropped.
- **Behavioural proof (runtime)**: with the unsplit snapshot loaded next to the leaves under `bun` (a stub for the one unresolvable workspace package), `shardWorkerSource()` (41,245 chars), `hostShimSource()` (8,947 chars) and `pluginComponentBridgeSource("comp","comp.wasm")` (27,253 chars) are byte-identical to the originals, the nine constants are equal, and `guestSlimTypstFontSeed` is equal.
- Name-resolution oracle (`tscheck.ts`, TypeScript program with the staged tree as overlay, only 2304/2305/2307/6133 and related codes): the 9 new files report 0 diagnostics; a mutation (one import deleted) is detected. Live tree: 33 files (every importer, the root, the leaves and the edited tests) report no 2304/2305 that the baseline lacked; the remaining 6133/6192 are the pre-existing unused names of the wide import lists, now re-pointed (`c5d-tscheck-bm-before.txt` / `-after.txt`). `bun build --no-bundle` parses all 33 files.
- Importers re-pointed (explicit named imports): 16 files (staging catalog of `✏️s/🧑‍💻dev`, hub `bootstrap-bundle`, play `playpages`, `wasi-activation`, build `installation` and `descriptor`, store `installation`, `retryable-lifecycle-deadline`, `native-optimization`, `cancel-job-reply`, `actorreturnresponseframing…`, `shardclient-reserved-response-settlement` (6 imports), `component-codec-reply` (2), the three `🧑‍💻dev/♻️activation` files, the materialization `🚀️commands`, `🧪️tests/🚪️lifetime`). The test kit (`ticket-owned-browser-host-staging/🧰️test-kit`) spreads one namespace bag, so nine new `import * as External17..25` and spreads were added (25 is `🕸️imports`, which supplied the dropped re-exports to the jco-rewrite test).
- Text readers re-pointed: `package-assets` (worker splice to the worker leaf), `more-work-drive` renderer test (constants from the contract leaf, worker text from the worker leaf), `runtime-admission` (helper takes the leaf), `hub-session-capability` (the "no session port in shard source" law now reads the root plus all eight leaves), `actorreturnresponseframing…` (both AST readers read the leaf per generator), `shardclient-reserved-response-settlement` (AST of the worker leaf), `🧪️tests/🏛️bootstrap` (worker and bridge leaves, parsed one by one), `🧪️tests/🚪️lifetime` (worker leaf and the bridge import), `🚀️commands/🟦️.ts` (the transpile cache identity `materialization` now digests root + `🧬️jco-module-rewrites` + `🧾️contract` through `materializationIdentityV1`, because the cache key must cover the sources that decide the transpile output). Doc references updated in `segmented-download`, `wire-turn`, `cancel-job-reply`, `component-codec-reply`, `⚛️reactor/🧪️tests/🚚️more-work-drive/🦀️.rs` (comment only).
- Configuration and fixtures: `🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` `namedInputs.default` and the matching `os-dev-composition-ownership/🔣️.json` `projectInputs` list root + 8 leaves; the `shard-worker` generated boundary `authorityPath` and the library project target input moved to `🧾️contract` (it declares `SHARD_WORKER_FILE`); `manifestless-source-closure/🔣️.json` owner split into three owners with their anchors; `🗣️languages/🔮️oracles/🔣️.json` `productionDebt.reachableFrom` now names `🧬️jco-module-rewrites` (the only production `typescript` importer).
- One tool defect found and fixed during the slice: the first rewrite pass truncated a multi-declarator statement in `🧪️tests/🚪️lifetime/🟦️.ts` (`const {a}=await import(..),{b}=await import(..),…`); the name-resolution oracle caught it, the line was restored from `HEAD` with only the specifier changed, and the rewriter now refuses multi-declarator statements. A first importer census missed short relative imports (`"../🟦️.ts"` from inside the folder); it was redone with a lexical pre-filter plus a scan of every relative string literal (`tsliterals.ts`): 3 further readers found and handled.

### 3.2 `✅️catalog-verification` (1,052 lines to root 159 + 9 leaves)

| Leaf | Lines | Content |
| --- | ---: | --- |
| root `🟦️.ts` | 159 | limits, constants and all shared types (contract) |
| `📚️navbar-example-audits` | 44 | picker coverage and example payload audits |
| `🎛️commands` | 76 | the four package scripts (lazy-loaded by the registry router) |
| `🛡️guards` | 112 | bounded diagnostic, path containment, strict json, enum normalisation, duplicate-name refusal |
| `🗺️verification-plan` | 88 | node ordering and plan execution |
| `🧬️descriptor-pair` | 107 | execution protocol and descriptor shape, placeholder refusal, pair decode and verification |
| `📥️artifact-io` | 81 | contained read, chunked sha256, verified read, receipt |
| `🔎️source-audit` | 85 | `auditPluginCatalogSources` |
| `🏗️fresh-build-verifier` | 122 | commit marker, exact row, build verifier |
| `🎯️native-selection` | 190 | native selection plan, preview, schema validator, oracle |

- Lossless: 911 lines before and after, missing 0, extra 0; region `CatalogCompleteness` (spans the cut) is re-opened and closed in each file (orig 1/2 incl. one orphan end marker that stays in `🎛️commands`; every other file balanced). Declared change: `native-selection` holds two `import.meta.dir` paths, so they gained one `".."` (`join(import.meta.dir, "..", "..", …)`) because the file sits one folder deeper.
- Import graph is acyclic (the root imports no leaf); name-resolution oracle: staged 10 files 0 diagnostics; live 28 files no new 2304/2305, only the unused-name re-labelling (`c5d-tscheck-cv-before.txt` / `-after.txt`); all 28 files parse.
- 16 importers re-pointed, including three `(await import("./✅️catalog-verification/🟦️.ts")).Class` lazy loads in the registry router and the hub stdio script's destructuring split into three imports.
- Ownership law replay (`tspubowners.ts`, a runner-free replay of `plugin-publication-source-ownership`): owners 26, consumers 13, isolated consumers 59, required edges 8, forbidden edges 15 (nine added, one per new leaf), acyclic runtime graph over 59 files: **0 problems**. Fixture `📣️plugin-publication-source-ownership/🔣️.json` now lists the root and nine leaves as owners (one export each), consumers 2, 8, 11 re-pointed; the law's hard-coded owner count went from 17 to 26 in `🧪️tests/📣️plugin-publication-source-ownership/🟦️.ts`. The `plugin-registry` generator `inputPatterns` in the library `🔣️taxonomy.json` gained the nine leaves in byte order (JSON validated).

## 4. Open risks

1. `bun test`, `tsc` and the vitest suites cannot run here: `node_modules` is absent repo-wide (external sweep). Evidence for TypeScript is therefore the compiler-API name-resolution oracle, parse checks, the byte-identical generator outputs, and the replays of the ownership laws; the coordinator should run the plugin, registry, hub and renderer TS suites once dependencies are installed (`🏗️materialization/🧪️tests`, `🛡️runtime-admission`, `🚚️more-work-drive`, `📦️package-assets`, `✅️catalog-complete`, `📣️plugin-publication-source-ownership`, `🧑‍💻os-dev-composition-ownership`, `🧱️manifestless-source-closure`, `🧪️oracle-governance`).
2. The `os-dev-composition-ownership` target-input equality replay already reported four unrelated extra expected inputs and brace-glob differences before my edit; my entries match.
3. CENSUS5-E may still anchor an edit on the old `🏗️materialization/🟦️.ts` (section 1).
4. rcc test closure is now 30 of 32 files (ratio 0.938); any further child under `🧬️contract` needs a merge first.
5. The typeck oracle for rcc is the only typeck-level evidence for that crate (extern crates are removed in the differential); the real `cargo check` is the confirmation.

## 5. Coordinator commands

`cargo check -p semio-framework-value-retained-clone --lib --tests`
`cargo check -p semio-framework-geometry-primitives --lib --tests`
`cargo check -p semio-hub-boot-laws --lib --tests` (feature set as in its `Cargo.toml`; `--tests` compiles the `include!` fragments)
`rustc --edition=2024 --crate-name semio_framework_geometry_primitives --crate-type=rlib --emit=metadata <geometry root>` is the native specimen form (already run clean).

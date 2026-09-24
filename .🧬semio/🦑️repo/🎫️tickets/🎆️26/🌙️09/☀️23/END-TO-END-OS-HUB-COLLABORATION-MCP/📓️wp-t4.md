# WP-T4: Parity Infrastructure, Schema-Catalog Scopes, gltf Catalogs, Contract Delta

Slice: T4 (session 10). Captures: `.tmp-ticket/wp-t4/generated/`. Ticket inputs (kept): `.tmp-ticket/wp-t4/*.py|*.sh|*.ts`, `.tmp-ticket/wp-t4/gltf/*` (authoring script, templates, plan), `.tmp-ticket/wp-t4/sdl/` (one-off SDL render).
Inherits: T2 §2.6 and §3, R5, T1 leftovers (added mid-slice by the coordinator).
Sibling T5 owns runtime inventories, binary drift, unmet oracle requirements and editor vocabularies; none of those were touched.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1a. Go test host module resolution | **Done.** The host joins a generated `go.work` built from the repository workspace. statutes went from 11/13 to **20/20, parity 13/13** | `parity-statutes-1.txt` |
| 1b. Oracle-only TS adapters dispatched as subjects | **Done, schema-first.** New taxonomy key `testSubjectBoundaryOwnerKinds`. 36 repo-module TS adapters are no longer subjects. A new live test covers it | `subject-selection-2.txt`, `subject-survey-*.txt` |
| 1c. print `compilePrintTexOnce` | **Done.** Exported from the compilation module; tectonic and the tracked fonts are prepared on first use. **104/105 cases execute** (was 3) | `parity-print-1.txt`, `parity-📓️print-2.txt` |
| 1d. Parity runs per owner | **12 of 13 owners green with 0 errored:** graphql 34/34, hooks 57/57, tickets 59/59, model 14/14, providers 41/41, languages 26/26, mcp 22/22, statutes 20/20, contributors 11/11, goals 13/13, todos 11/11, cli 25/25 (Go subject only, §1.4). **print** 486/623 passed, 137 errored, from two print-owner gaps (§1.3). **dashboard** is not exercised because its crate does not compile (§5) | `parity-*-2.txt`, `parity-owners-2.log` |
| 2. `asset://🧬️schema/…` rows | **Done: 9 → 0.** New catalog scopes: `repo.tickets`, `repo.contributors`, `repo.goals`, `repo.todos`, `repo.model`, `repo.graphql`, `repo.mcp`, `repo.test-runner`, `print` (3373 → 3385 scopes) | `schema-catalog-delta-1.txt`, `schema-generate-1.txt` |
| 3. gltf scene/buffer/mesh | **Done: 3 catalogs claimed, 76 kinds.** 304 executed, **304 passed, parity 152/152** (buffer 16/16, scene 66/66, mesh 70/70) | `parity-*-gltf2.txt`, `parity-owners-gltf2.log` |
| 4. Contract delta | **573 → 318** (T1's `contract-2` → mine; includes peers' work) | `contract-2.txt`, `contract-delta-2.txt` |
| 5. T1 leftovers | Test depth **18 → 6**, self-test **3 → 2**, rust-wiring **5 → 0** (stale; T2's renames had already fixed it). Vitest 4 and registration 4 still open (§4) | `contract-delta-2.txt` |
| 6. graphql Go fixture; wgpu `project.json` law | **Done.** graphql `go test ./...` is ok. The layout law now reads the wgpu TypeScript project and awaits `libraryBootstrap`; 1/1 passes; suite 78/0 | `test-layout-1.txt` |
| 7. Peer compile issues | mcp, graphql, procedural 2d and procedural 3d: `cargo check --tests` exit 0. The procedural 2d `🔬️unit` test compiles. **Open:** dashboard, an unfinished extraction (§5) | `check-crates-1.txt` |

Test-platform suite: **114 pass / 2 fail** out of 116 tests (T1 left it at 111/2 of 113).
- The contract-zero test fails, as expected.
- `discovery finds the committed cases` hit bun's 5 s timeout under peer load. T1 saw the same.

Evidence: `test-platform-1.txt`.

## 1. Parity infrastructure

### 1.1 Go host

- **Root cause.** `materializeGoHost` wrote a `replace` for `semio.tech/repo/test` only, and ran with `GOWORK=off GOFLAGS=-mod=mod`. Every adapter import of `github.com/usalu/semio/repo/*` therefore went to the Go proxy, which fetched the whole repository and failed with "downloaded zip file too large".
- **Fix** (`🧪️test/🖥️host/🏗️materialization/🟦️.ts`):
  - New `repositoryGoWorkspace` reads the root `go.work` through the Go tool itself (`go work edit -json`).
  - The host writes its own `go.work`: `use .` plus every repository module, with absolute paths. `go.work.sum` is copied, and `GOWORK` points at the generated workspace.
  - The host `go.mod` now declares only its own module.
- The result is zero-touch and cross-platform: the paths come from `path.join`, and nothing is hard-coded per OS.
- The command-composition fixture now lists the new declaration.

### 1.2 Oracle-only adapters as subjects

- **Root cause.** `ownerShipsImplementation` walked every ancestor. `🧰️framework/📦️packages/🟦️typescript` therefore made every repo module a "TypeScript subject", including modules whose TypeScript adapter only hosts a reference (node:zlib, ajv, graphql-js, the MCP SDK).
- **Schema-first fix.**
  - New taxonomy key `testSubjectBoundaryOwnerKinds: ["🛍️products", "🔌️plugins"]`, with a rationale comment.
  - The key is declared in both taxonomy types and validated in discovery: each entry must name an owner kind.
  - The subject search now stops at the nearest product or plugin root.
- **Survey** (`ts-subject-survey.py`, `subject-survey-*.txt`):
  - Exactly the 36 repo-module TS adapters change. Framework modules, the hub and plugins are unchanged.
  - Python and Go adapters are unchanged.
- **Tests.**
  - New live test: "a package of an enclosing framework never makes a nested product's reference adapter a subject".
  - It asserts that TypeScript is a subject only where the product ships a TS package. `🖥️server/🎛️coordinator` still has both roles.
  - Result: 2/2.

### 1.3 print

- **`compilePrintTexOnce`** was imported from `🖨️tectonic-template-compilation/🟦️.ts` but had never been defined there. `ensureTectonicBinary` did not exist anywhere.
  - The owner API is now `compilePrintTexOnce(texPath, outDirectory, workDirectory, signal?)`. It compiles one pass against the print library staged beside the source.
  - Tectonic is acquired through the toolchain's own `prepareTectonic`. The tracked fonts are staged once per process through `stagePrintFonts`. Before this, `dist/fonts` was missing and every compile failed on `Anta-Regular`.
  - The probe now imports `prepareTectonic`.
- **Remaining print errors (137)** come from two print-owner gaps. A TeX log classifier over every subject work dir found only these two:
  1. **The pinned TeX bundle lacks the tikz `patterns` library**, which `🖋️latex/semio-viz-mark.sty` loads. This affects 40 documents. `arrows.meta` is loaded on the same line and is missing too.
     - The local tectonic index gives the archive coordinates:
       - `tikzlibrarypatterns.code.tex`: offset 27310080, 770 B
       - `pgflibrarypatterns.code.tex`: offset 27978240, 7936 B
       - the `.meta` variants: 27311616/3036 and 27986944/15474
     - Adding them to `📚️bundle/🔒️dependencies.json` needs each file's sha256, which means downloading those ranges (~27 KB) from `data1b.fullyjustified.net/tlextras-2022.0r0.tar`. **I did not download without approval.** The bundle manifest then needs regenerating (print owner or the user).
  2. **`semio-viz-charts-distribution.sty` is required** by 7 `semio-viz-*` packages but has never been committed (not in git history). It holds the `demo-scores`/`demo-parts` tables. 5 documents. Owner: print.
- **`🖼️gallery-render`** imports `measurePrintGalleryVariant`/`printGalleryMatrix` from a module that never existed. Its committed evidence is `{"variants": {}}`. This is unfinished print-owner work.

### 1.4 Other owners

- **cli.** Its Rust adapters call `semio_framework_repo_cli::repo_cli::*`, which has never existed in any revision. The Rust port of the repo CLI is unwritten, so only the Go subject runs.
  - The Rust host build exits 101 and the platform does not count it as errored. That is a visibility gap worth a follow-up.
- **dashboard:** see §5.

## 2. Schema-catalog scopes (the 9 fixture rows)

- **Root cause.** Each module's schema `$id` was non-canonical, so `schemaScopeIdFromDocumentId` returned no scope and the catalog silently skipped the module. The bad `$id`s:
  - `semio-tech.com/schema/repo/tickets/1`
  - `…/repo/goals/🔣️.json`
  - `…coms/repo/graphql/ast-projection.json`
  - `semio-tech.com/schema/repo/mcp/v1`
  - `…/repo/test-runner.json`, which even produced a colliding `repo` scope
- **Fix.**
  - Canonical `$id`s of the form `https://json.schemas.assets.semio-tech.com/<scope path>/schema.json` for tickets, contributors, goals, todos, model, graphql, mcp, test-runner and print. Nothing else referenced the old ids (`git grep`).
  - graphql's lowercase `$defs` became PascalCase exports: `Value`, `Selection`, `Document`, `DocumentCorpus`, `CoercionCorpus`, `ErrorCorpus`, `Diagnostic`.
  - The print root needed an owner level. New taxonomy level `product-root` (`🧰️framework/🛍️products/*`, `♻️mit-bestand/*`, `🏢️semio-tech/*`).
  - Catalog regenerated through its generator, `bun ./📜️script.ts schema generate`. New scopes: `print`, `repo.contributors`, `repo.goals`, `repo.graphql`, `repo.mcp`, `repo.model`, `repo.test-runner`, `repo.tickets`, `repo.todos`. The os product root and two framework modules became addressable as well.
- **Resolution of the 9 rows:**
  - 6 now cite `schema://<scope>/<Export>`: tickets `TicketDocument`, contributors `ContributorDocument`, goals `GoalDocument`, todos `Todo`, model `Repo`, print `CatalogEntry`.
  - **graphql SDL ×2.**
    - The committed SDL is the executor's own rendering, and no client reads it. `🔗️graphql/🧬️schema/🔣️schema.graphql` had never existed, which is why the graphql crate's unit test did not compile.
    - I rendered it with the Go `RenderSDL(BuildSchema())` into `🔗️graphql/🧫️fixtures/📜️served-schema/🔗️.graphql`.
    - Both cases now cite `shared://📜️served-schema/🔗️.graphql`, and the unit test reads the same file.
  - **mcp `descriptions.json`.** It is authored data, not a schema. It moved to `🔌️mcp/🖼️assets/🔣️descriptions.json`, and the feature, `surface.json` and the Rust `include_str!` were updated.
- **Schema URI format selector.**
  - `schema://<scope>/<Export>?format=<format id>` now selects a non-normative implementation. The taxonomy `uriPattern`, `parseSchemaUri`, `schemaFormatKey` and `resolveSchemaExport` are updated, with a unit expectation in `🧬️schema-invariants`.
  - Nothing uses it yet, but the SDL question showed the grammar could not address a scope's GraphQL format at all.
- **Newly visible `schema check` findings on the new scopes**, which are schema-owner debt:
  - the 2020-12 dialect where the check wants draft-07;
  - non-PascalCase root titles;
  - print's TS format lacks declarations of its exports.

  See `schema-check-1-summary.txt`.
- **Parity after the change:** tickets 59/59, graphql 34/34, model 14/14, mcp 22/22, contributors 11/11, goals 13/13, todos 11/11. The remaining print errors are §1.3.

## 3. gltf scene / buffer / mesh

- **Oracle** (`♾️any/🔮️oracles/🦀️.rs`, region `🔖️StructureKinds`).
  - All 76 kinds are implemented over the json-rust tree. For every top-level family change, the reference remapping is re-derived from the format:
    - scene roots, children, skin joints and skeleton, and animation targets point at nodes;
    - node meshes point at meshes;
    - primitive attributes, indices and morph targets, inverse-bind matrices and sampler input/output point at accessors;
    - accessor and image buffer views point at buffer views;
    - a buffer view's buffer points at buffers.
  - Required references refuse the change. `create-buffer` writes a base64 `data:` URI (own RFC 4648 codec).
  - New `restore_members` provides the inverse of kinds whose payload cannot carry the undo.
  - The projection is now normalized with the format's defaults. It covers every node, scene, mesh (semantic maps as ordered pairs), accessor, buffer view and buffer (decoded bytes), plus animation sampler `interpolation` defaulting to `LINEAR`. That default was the one real writer-freedom difference found.
- **Cases.** Three new cases were authored by `gltf/author.py` from the leaf payload structs (`gltf/survey.json`) and a parameter plan derived from each fixture (`gltf/plan.py`, `plan.json`):
  - `🎬️scene/🧪️tests/🎬️mutate-gltf-2-0-scene`
  - `💿️buffer/🧪️tests/💿️mutate-gltf-2-0-buffer`
  - `🕸️mesh/🧪️tests/🕸️mutate-gltf-2-0-mesh`

  Each has a `@mutations-gltf-2-0-<subset>` feature and a Rust adapter.
  - The spec in each scenario names its fixture and its inverse. The adapter is generic except for the subject dispatch through each leaf's typed `apply()`.
  - Every kind has a `mutate-` and an `inverse-` scenario.
- **Fixture fix.**
  - `delete-morph-target` is impossible on the committed base: production requires empty mesh-level weights, and mesh 0 has `[0.5]`.
  - The recipe in `♾️any/🏭️generator/📜️script.ts` now starts from a mesh without default weights. I regenerated that fixture with `generate --only delete-morph-target-applied`; only its two files changed. I then refreshed the manifest's sha256, bytes and notes.
- **Registry.**
  - `json-rust-gltf-2-0-mutate` now also declares `gltf-2-0-mutate`. The contract requires the feature capability to equal the catalog capability, and the oracle to declare it.
  - Its kind stays `cross-semio-implementation`, so the `third-party-library` requirements stay honestly unmet (T5).
  - The stale "seven kinds" paragraph of its rationale is rewritten, and so is the module doc.
- **Observation (not changed).** The existing camera, skin, animation, asset and material cases tag `@oracle-three-gltf-2-0-mutate-reader` but ship no TypeScript adapter. Their parity is therefore 0/0 ("needs a typescript adapter"; `parity-🎞️animation-gltf0.txt`). The artifact-root `🧊️mutate-gltf-2-0` adapter still references the removed `GltfMutationLeafDescriptor`/`DESCRIPTOR`.

## 4. T1 leftovers

**Fixed:**

| Row | Fix |
|---|---|
| os mcp ×4 (`🧪️tests/<case>/🏃️execution/🟦️.ts`) | These are `BundleScript` runners, so they are scripts. They moved into the os mcp `📜️script.ts` router, which imports `runOwnedCommand` from its module. The four modules were deleted. The router was smoke-tested |
| os dev ×6 (`🧑‍💻dev/🧪️tests/⚖️parity/*`) | This is the renderer-parity harness the dev router drives, not a test. It moved to `🧑‍💻dev/⚖️parity/*`: <br>• relative imports rewritten and checked (`check-relative-imports.py`) <br>• importers, both `project.json`, `staging-root.json` and `os-dev-composition-ownership` updated <br>• taxonomy: the `os-dev-parity-tests` member kind was removed, because `⚖️parity` under a module already resolves to the existing `repo-test-parity` kind, and its six children now hang off that kind |
| vscode `🧪️tests/🧩️extension/🎚️config/🟨️.mjs` | Moved to the canonical config case `🧪️tests/🎚️config/🟨️.mjs`. The router, both `project.json` and the `tool-configuration-ownership` fixture were updated. The config loads and resolves the same package root |
| library `🎨️styling-outputs/🐍️python/🟦️.ts` (depth + self-test) | Became its own case, `🧪️tests/🐍️styling-python-outputs/🟦️.ts`, and the `cache-contracts` import was updated |
| presentation react, first vitest block | Moved into `🧪️tests/📝️owned-markdown-compiler`, which the react config already includes. 12/12 pass |

**Open, with the reason:**

- **The presentation react `🟦️.tsx` still has 3 in-source blocks.** One of them is 3,832 lines over internals, so the row stays; `includeSource` is kept so those tests still run. In that suite, 11 `presentation interaction geometry` tests fail independently of this change.
- **hub ×4:** `foundation-source`, `live-sign-in` and `socket-grant-command-source` runners, plus `🧬️schema/🛂expectation`. The hub foundation-source law pins the router import strings and fixture paths. Moving them means editing the 17,000-line hub `📜️script.ts`, which the h-slices own.
- **flow ×2 and its self-test:** an ownership law pins the `🏷️ownership/🧪️tests/🟦️.ts` tokens.
- **stdio composition self-test.**
- **ui react (11,676 lines), `💻️os/🟦️.ts` and `🖥️server/🟦️.ts` vitest/registration:** in-source suites in large production files.

**Pre-existing failures seen, not caused here:**

- **os-dev-composition** `engine-publication: buildEngineWasm` and the launch-seed route.
- **command-composition** launch route missing from `.vscode/launch.json`.
- **tool-configuration**: Playwright lists 12 demonstrator tests where the law expects 7, and a wgpu dist artifact is missing.

## 5. dashboard (open)

- `semio-framework-repo-dashboard`'s crate root declares 9 command modules under `🎛️dashboard/<x>/`. Only `🌀️daemon` and `🌳️command-tree` exist.
- Commit 623 (`bb961413d4`) created the crate and deleted `🎮️commands/🎛️terminal-dashboard/🦀️.rs` (619 lines), but never moved the command modules.
- `⌨️cli/🦀️.rs` still includes its own copies from `🎮️commands/*`, plus the deleted `🎛️terminal-dashboard`. The cli crate also lacks the `repo_cli` module its Rust test adapters call.
- **Plan for the owner:**
  - Move the 6 commands from `🎮️commands` into `🎛️dashboard/{🌊️workflow,🔌️plugin-registry,🛝️playground-session,⌨️usage,📇️playground-catalog,📜️root-delegation}`.
  - Restore `🖥️terminal` from `bb961413d4^`, adjusting `crate::command_tree_discovery` → `crate::command_tree`.
  - Drop the superseded `🖥️terminal-dashboard-daemon`/`🌳️command-tree-discovery` wrappers and the `🎮️commands` collection, including its taxonomy members.
  - Make the cli crate dispatch into dashboard instead of duplicating `args`/`daemon`/`catalog`.

  This is a structural move across two crates and the taxonomy, so I did not do it half-way.

## 6. Contract delta

`contract-2.txt`: 318 high rows, by T1's classes and mine (`classify.py`).

| Class | T1 `contract-2` | T4 | Δ | Cause |
|---|---|---|---|---|
| fixture-unresolved | 9 | 0 | −9 | §2 |
| catalog-unclaimed (gltf) | 3 | 0 | −3 | §3 |
| test-source-depth | 18 | 6 | −12 | §4 |
| self-test-declaration | 3 | 2 | −1 | §4 |
| rust-test-wiring | 5 | 0 | −5 | already fixed by T2's renames (stale at T1's run) |
| stub-serializer | 106 | 0 | −106 | peers (T3) |
| wire-record drift rows | ~90 | 0 | −90 | peers |
| no-runtime-inventory | 175 | 172 | −3 | T5 territory |
| everything else | — | — | — | unchanged: vocabulary-without-catalog 48, fixture-dependency 31, vitest 4, registration 4 |
| **Total** | **573** | **318** | **−255** | |

## Processes (pids)

- T2 loop 66328: exited.
- Mine (all detached, all exited):
  - print parity 99341
  - owner loop 18179 (`parity-owners.sh 2`)
  - animation probe 50956
  - gltf loop 58912 (`gltf1`); `gltf2` ran in the foreground
  - contract 59125 and 72670, the latter followed by the test-platform run
  - crate checks 46938
- No servers, no ports, no fleet-mutex work: no wasm32 or `semio-hub` builds.
- **Note.** I ran one `git mv` (`🚪️entrypoint-contract.json`). It staged a rename in the index before I stopped using git for moves. The working tree matches, and nothing else was staged.

## Files changed (T4)

- **Test platform:**
  - `🧪️test/🖥️host/🏗️materialization/🟦️.ts`
  - `🧪️test/🟦️.ts` (taxonomy type, schema URI format)
  - `📚️library/🔍️discovery/🟦️.ts`
  - `📚️library/🔣️taxonomy.json` (`testSubjectBoundaryOwnerKinds`, `product-root`, `uriPattern`, parity member kinds)
  - `🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`, `🧬️schema-invariants/🟦️.ts`, `📐️test-layout/🟦️.ts`
  - `🧪️test/🧫️fixtures/🧱️command-composition-source/🔣️.json`
- **print:**
  - `🔨️modules/🖨️tectonic-template-compilation/🟦️.ts`
  - `🔨️modules/🧪️viz-probe/🟦️.ts`
  - `🧬️schema/🔣️.json` (`$id`)
  - `🧪️tests/📚️catalog-coverage/🥒️.feature`
- **Schemas:** `$id` in tickets, contributors, goals, todos, model, graphql (plus PascalCase `$defs`), mcp and test-runner. Regenerated `📚️library/🔣️schema-catalog.json` and `📓️schema-catalog.md`.
- **Cases:** the features and `🟦️.ts` of the 9 fixture rows; graphql `🧫️fixtures/📜️served-schema/🔗️.graphql` and unit test.
- **mcp:**
  - `🔌️mcp/🖼️assets/🔣️descriptions.json` (moved), `🔌️mcp/📦️packages/🦀️rust/🦀️.rs`, `🧫️fixtures/📋️surface.json`
  - `🧫️fixtures/🚪️entrypoint-contract.json` (moved from `💻️client/🔌️mcp`)
  - `💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go`
- **gltf:**
  - `♾️any/🔮️oracles/🦀️.rs`, `♾️any/🔮️oracles/🔣️.json`, `♾️any/🏭️generator/📜️script.ts`
  - `🕸️mesh/🧫️fixtures/🧬️morph-target/🗑️delete/*`
  - the 3 new cases
- **T1 leftovers:**
  - os mcp `📜️script.ts` (4 runner modules deleted)
  - `🧑‍💻dev/⚖️parity/*` (moved), dev `📜️script.ts`, 2 dev tests, dev and library `project.json`, `staging-root.json`, `os-dev-composition-ownership/🔣️.json`
  - vscode `🧪️tests/🎚️config/🟨️.mjs` (moved), its router, `tool-configuration-ownership/🔣️.json`
  - `⚡️caching/🧪️tests/🐍️styling-python-outputs/🟦️.ts` (moved), `⚡️cache-contracts/🟦️.ts`
  - presentation react `🟦️.tsx` and `🧪️tests/📝️owned-markdown-compiler/🟦️.ts`

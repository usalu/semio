# WP-T1: Test Placement, Oracle Purity, Go In-Package Tests

Slice: T1 (session 10). Captures: `.tmp-ticket/wp-t1/generated/`. Ticket inputs (kept): `.tmp-ticket/wp-t1/*.py|*.ts|*.sh`.
Inherits: R5 §1/§4 (contract-3: 3434 rows), R3 (json-rust envelope oracle).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. go-test-in-package (638) | **DONE: 638 → 0.** Schema-first rule, fixture law, `go list` cross-check. No Go file left its package | `contract-0.txt` → `contract-1.txt`, test-layout suite 77/78 |
| 2. oracle-in-production (217) | **DONE: 217 → 0.** serde_json ×9 migrated to json-rust and regenerated through their generators. node-crypto / ajv / typescript decided case by case (§2b) | `purity-2.txt` (0 hits), `fem-regenerated-vs-head.txt`, `draw-regenerated-vs-head.txt` |
| 3. rust-outside 218 / inline 30 / depth 24 / filename 13 / registration 11, legacy-filename 138, obsolete-category 37 | rust-outside **0**, inline **0**, filename **0**, legacy **0**, obsolete **0**, case-name 12→**0**. Still open: depth 19, registration 9, vitest-wiring 7, rust-wiring 5, self-test 3, delivery 1 (§3) | `contract-delta.txt`, `check-repo-1.txt`, `check-plugins-1.txt`, `check-plugins-2.txt` |
| 4. test-platform suite | R5 102/11 → **111 pass / 2 fail** (run 5, 94 s). Remaining: contract-zero (t2/t3/owners, expected) and mutation-without-fixture (87 declared mutations with no vectors: block 41, cad 19, wfc bitmap 10, gif 9, equation 9 — owner debt, not placement) | `test-platform-5.txt` |
| 5. contract rerun (after T2 finished) | **3434 → 573** high rows. All of T1's classes are at 0, except the leftovers listed in §3 | `contract-2.txt`, `contract-delta.txt` |

### Contract per class (`classes.py contract-0.txt contract-2.txt`)

| Class | Before | After |
|---|---|---|
| go-test-in-package | 638 | 0 |
| rust-test-outside-canonical | 218 | 0 |
| oracle-in-production | 217 | 0 |
| legacy-test-filename | 138 | 0 |
| obsolete-test-category | 37 | 0 |
| rust-inline-test-module | 30 | 0 |
| test-source-depth | 24 | 18 |
| vitest-wiring | 14 | 4 |
| adapter-filename | 13 | 0 |
| test-case-name | 12 | 0 |
| test-registration | 11 | 4 |
| delivery-scope-owner | 8 | 0 |
| rust-test-wiring | 7 | 5 (all 5 are new block-5d `#[path]` lines, from a peer) |
| self-test-declaration | 3 | 3 |
| everything else (t2/t3/owners) | 2064 | 539 |

## 1. Go in-package tests + legacy filenames

**The rule is schema-first.** A new taxonomy key, `testInPackageImplementations`, lives in `📚️library/🔣️taxonomy.json`:
`{id: go-package-test, implementation: go, fileKindId: go-source, filenameSuffix: "_test.go", rationale}`. `go-test-suffix` is
removed from `testLegacyFilenamePatterns`, because it contradicted the new key.
- **Discovery validation** (`📚️library/🔍️discovery/🟦️.ts`) checks four things:
  - the implementation is registered;
  - the file kind is a test file kind;
  - the suffix extends one of that kind's extensions;
  - no legacy pattern overlaps the suffix.

  `testImplementationIds` is now typed there as well.
- **Canonical in-package test.** `inPackageTestImplementation` in `🧪️test/🟦️.ts` treats a `_test.go` file as a canonical in-package test only when a production `.go` sits beside it.
  - An orphan `_test.go` is still reported as an inline test body.
  - A `_test.go` inside a `🧪️tests` case is still reported as a wrong filename.
  - The purity scan treats in-package tests as test-owned.
- **The Go test signal was wrong in two ways.**
  - It flagged production `TestSelector(string)` and `TestVerbLines`. It now requires `*testing.T|B|F`.
  - It reported the line of the preceding blank line, because `^\s*` crossed the newline. The pattern is now `[ \t]*`, which also fixes the Python and .NET patterns.
- **Fixture law `go-in-package-tests`** (`🧪️test/🧫️fixtures/📐️test-layout/🔣️.json`) is cross-checked against a third party. The new test `Go in-package test classification agrees with go list` compares our canonical set with `go list -json` `TestGoFiles ∪ XTestGoFiles`. It passes.
- **The Go files stayed in their packages.** 8 `🧪️_test.go` were renamed in place:
  - events, search, yaml, identity → `🔬️_test.go`
  - workspace → `🔎️glob_test.go`
  - graphql → `📜️parse_test.go`
  - languages → `🗂️table_test.go`
  - providers → `📼️recorded_test.go`

  `go test ./...` passes in 7 of 8 packages (`go-renamed-tests.txt`). graphql fails in `TestFixApplyAutofixes`, which reads the missing fixture `📜️statutes/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx`. That failure predates this slice.
- **110 story-file rows.** The taxonomy's own story kind is `🧪️.story.tsx`, and its own pattern `^🧪️` flagged it. A registered file-kind filename is now never a legacy test filename. Fixture law: `registered-story-kind-is-not-a-legacy-test-filename`.
- **Runner configs.** A new taxonomy key, `testRunnerConfigurationCaseName: "🎚️config"`, marks a package's vitest config case. It configures exactly one delivery package, so it may sit under `🎯️targets`/`📦️packages`, matching the existing vitest-ownership law. Fixture law: `test-runner-configuration-configures-its-delivery-package`. The Nx cross-check skips config cases, because they are not test cases.
- **The whole `.🧬semio` root is now excluded** from layout discovery (`getSemioRoot`). Before, only `🦑️repo` was. The hub's runtime data dir `🌐hub/…/stdio-target/*/examples` was showing up as 6 obsolete-category rows.
- Test-layout suite: **77 pass / 1 fail**. The failure is `Nx hashes semantic-owner cases…`. It reads `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📋️project.json`, which a peer removed.

## 2. Oracle purity

### 2a. serde_json → json-rust (9 oracles, 121 rows)

Each carrier engine is a standalone crate on `json = "0.12"` (json-rust, cached, `--offline`). Values are built from their JSON lexemes, so the committed number spelling survives, and output is key-sorted like the committed files.

| Oracle (`serde-json-*` → `json-rust-*`) | Before | Regenerated through | Result |
|---|---|---|---|
| equation | serde_json | `✳️any/🏭️generator/📜️script.ts generate` + `manifests` | 1 pair, byte-identical |
| fem2d, fem3d | first-party `pack` (a peer had swapped serde for our own JSON: no independence at all) | `carrier` + `carrier-manifests` | 104 files. `0.0`→`0`; pack's `6.97e-8` → the seed's lexeme. **104/104 semantically equal to HEAD** through the new reader |
| draw drawing | serde_json | `generate` + `manifests` | 6 files, `0.0`→`0`; 6/6 equal to HEAD |
| semio cad, mesh, drawing, brep | serde_json, unwired | **new `carrier` command** in each `🏭️generator/📜️script.ts` | cad, drawing byte-identical; mesh, brep `0.0`→`0` |
| semio document | serde_json | `carrier` | byte-identical |

- **Generator drift, fixed by hand.**
  - equation wrote 10 kinds into a nonexistent `✳️any/🧫️fixtures`. It now writes the one witnessable kind into `➗️equation/🧫️fixtures/🎚️change-coefficient/`.
  - draw wrote under a nonexistent `🖍️drawing` plugin path. It now writes into `🏷️metadata`/`🎨️style` and refreshes both registries.
  - mesh and brep now write their reviewed emoji directories.
- **Registries: 13 files.** Changed fields:
  - oracle id, package `json`/`0.12`, engine;
  - source and homepage;
  - probes and pipelines;
  - requirements and fixture provenance;
  - prose, including a stale path in the cad rationale.

  The base envelope attributions now carry the new ids. These edits were made by the ticket scripts `migrate-registry.py` and `refresh-carrier-manifests.py`, and I reviewed every diff. The scripts keep number lexemes and formatting.
- **Readers verified** both ways: before==before → true, before≠after → false.
- **`🔒️dependencies.json`.**
  - The engine manifests moved from `serde_json` users to `json` users.
  - The `serde_json` entry no longer carries oracle ids.
  - The `json` oracle ids come from the live registry.
- Not migrated: `🎬️sequence`'s engine. Its `serde-json-sequence-carrier-reader` is **not registered** (8 contract rows, owner sequence).

### 2b. node-crypto, ajv, typescript (and d3-force, image, three-mesh-bvh), decided per case

**The gate was measuring the wrong scope.** Your definition: an oracle must be a library that *the production path under test* doesn't use. `oracleImportsInProduction` instead scanned the whole repository for every oracle.
- Every hit left (node:crypto 66, ajv 22, typescript 5, d3-force, image, three-mesh-bvh) was **outside** the owner that registers the oracle.
- The owners' subjects are:
  - hand-rolled SHA-256 in events, coordinator, mcp, statutes;
  - the Go/Rust definition parsers in languages;
  - print, tickets, goals, model, todos, events and contributors' schemas for ajv;
  - stdio codecs for image;
  - semio mesh for three-mesh-bvh.

  None of these subjects import their oracle.

**Change.** An oracle's imports are now judged only inside the owner(s) that register it. The same applies to external host packages and their owners. A core-registry oracle, which has no owner, is still scanned repo-wide.
- Whether a package may be production-reachable at all is still enforced at the declaration: the baseline must classify it test-oracle, or the oracle must record `productionDebt`. The tests `recorded production debt` and `dependency ratchet` cover that.
- The synthetic ownership law, where an owner imports its own oracle, still reports as before.
- Result: **0 purity hits** (`purity-2.txt`).

Per case:

| Package | Decision |
|---|---|
| node:crypto (4 oracles) | A runtime builtin. It is never a dependency, and no owner's production imports it. Nothing to record |
| typescript | The compiler toolchain. `typescript-compiler` now states `productionReachable: true`, gives a rationale (it had none), and records `productionDebt` for its 5 real paths: browser-bundle parse, dependency inventory, hub type-check |
| ajv | Every one of the 22 users is verification code in scripts or test helpers, none in a subject. **Finding for os/plugin owners:** these are test commands living in production dirs, and as tests they would be test-owned. The existing `ajv` debt list is stale: 33 of its 94 paths no longer exist, and 0 overlap the current 22. I left print's record as is |
| image, d3-force, three-mesh-bvh | Outside their owners' subjects. `image` and `d3-force` already record debt |

### 2c. Dependency baseline reconciled with the live declaration scan

The committed baseline had **49 stale records** that claimed more reach than the declarations show. Examples:
- `quick-xml`, `lopdf`, `gif`, `tobj`, `ruststep`, `dxf`, `tiff`, `riff`, `rust_xlsxwriter`, `markup5ever_rcdom`, `gltf`: `production-runtime`, while the scan finds them test-oracle only. Their users had moved to `🔬️probes/📖️reader`.
- `csv`: production-runtime, while the scan finds it test-oracle.
- d3-*, markdown-it, remark/unified: `repository-tooling` vs test-oracle.
- python numpy, openstudio, honeybee-energy, ladybug-core, deepdiff: `test-runner`, while only oracles link them.

All 49 were shrunk to the scan's classification with `baseline-shrink.ts` (log: `baseline-shrink.txt`). The script never widens a record.

**Genuine finding (not fixed).** `architect-program-zip-reader` is not independent. Its subject, `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program`, links `zip = "6"` in production. The source probe misses it, because the probe matches `use zip`, not `zip::` paths. The other zip oracles (stdio zip/bcf/docx/pptx) lack debt for the os-host reachability that the `zip` oracle records. **Owner: architect** (use stdio's zip codec or another oracle) **and stdio** (record debt).

## 3. Test placement (Rust, then TS/JS)

**Rust inline test modules → canonical cases.** Each body was moved verbatim with `extract-test-module.py`, which resolves the brace block on masked source and wires the new file by `#[path]`:
- 15 repo modules (tree, dashboard, tickets, workspace, events, mcp, search, graphql, coordinator, languages, providers, yaml, hooks, identity, move) → `<module>/🧪️tests/🔬️unit/🦀️.rs`.
- dashboard 🌀️daemon, 🌳️command-tree.
- wfc grid2d txt serializer (`🔁️dsl-txt-round-trip`) and grid3d io.
- procedural 2d, raster set-active-example.
- fem 2d/3d transform, layout canvas-drop/gumball.
- hub auth command, os db driver-runtime.
- ui wgpu host/chrome → `🖱️ui/🧪️tests/📋️clipboard-content`, `🎨️foreground-on-fill` (the owner sits above `🎯️targets`).
- The draw editor's nested `args_bridge::tests` → `🧪️tests/🌉️args-bridge`. Its `#[path]` could not pass through an inline module's nonexistent directory, so it is declared at editor level with `use super::args_bridge::command_from_action`.

Other moves and renames:
- procedural generation3d: `🔬️serial/🧪️tests/🔬️unit` (tests nested in a test) → sibling case `🔒️serial-lock-discipline`.
- os renderer directory/socket door tests under `🎯️targets/🧊️wgpu/…/🧪️tests` → `🧑‍🎨engine/🧪️tests/📇️wgpu-directory-door`, and the existing `🔌️wgpu-socket-door` case. Its include path was rebased.
- hub auth `credential-source-order/🔮️oracles/🦀️.rs` → the case's own `🦀️.rs`. The hub lib, `📋️project.json`, the foundation-source test and fixture were updated.
- wfc grid3d fill `🎪️tests`/`🎫️fixtures` → `🧪️tests`/`🧫️fixtures`.
- 4 probe crates `🔬️probes/🔮️oracle` → `🔬️probes/📖️reader` (equation, draw, semio cad, semio drawing). References updated in the generator, the probes scripts, note codec docs and the baseline. `cargo check` passes on all four.
- os dev benchmark `🧪️stub` → `📏️measured-row`, with the taxonomy registry member, the ownership fixture, project inputs and the importer updated.
- Case names with generic or missing emoji:
  - `📂️retained-section-collapse` → `🪗️retained-section-collapse` (ui tests/fixtures/schema, os renderer), with ui's `♿️.rs` split into its own case `♿️retained-section-accessibility/🦀️.rs`;
  - `📄️ticket-document-codec` → `🎫️…`;
  - `📄️goal-document-codec` → `🎯️…`;
  - `persistence-data-class` → `🗃️persistence-data-class`.

**Compile evidence** (`cargo check --tests`, `CARGO_INCREMENTAL=0`):
- **15 repo crates** (`check-repo-1.txt`). All pass except 3, and none of the 3 comes from the move:
  - mcp: `include_str!("../../🧫️fixtures/🚪️entrypoint-contract.json")`. The file lives in `💻️client/🔌️mcp`, not `🔨️modules/🔌️mcp`, and was missing at HEAD.
  - graphql: `🧬️schema/🔣️schema.graphql` does not exist at HEAD.
  - dashboard: `pub mod workflow` points at a missing `🌊️workflow` (peer, in progress).
- **Plugins, ui, os-db, os-renderer-wgpu, hub** (`check-plugins-1.txt`): Finished, 0 errors.
- **The same crates with `component-app-assembly`** (`check-plugins-2.txt`): wfc grid2d/grid3d, fem 2d/3d and procedural 3d lib tests pass. Two errors come from peers:
  - procedural 2d: `io/🦀️.rs:162` is missing `.await`, so its moved `🔬️unit` test is not yet verified;
  - procedural 3d: the integration test `io-round-trip` points at a png round-trip file that is not there.

**TS/JS.**
- **19 stray `tsc` emits** (`📜️script.js`, `🟦️.js` beside their `.ts`, including 13 inside test cases) had no references and were deleted: demonstrator, actor lifetime/patch, ui styling vite, os plugin store installation. List: `stray-js-mine.txt`. They caused all 13 adapter-filename rows and 7 vitest rows.
- presentation `pdf-canvas-port` and the owned-markdown in-source vitest blocks → `🎤️presentation/🧪️tests/🔌️pdf-canvas-port`, `📝️owned-markdown-compiler`. The react config now `include`s them instead of `includeSource`. **11/11 pass** in vitest. The ownership-law projection hashes were updated for this config and os's (`vitest-projection.ts`, both match).
- world3d shading pixel-oracle driver `🧪️tests/🎨️world3d-scene-shading/📜️script.ts` → `🧑‍🎨engine/🔮️oracles/🎨️world3d-scene-shading/📜️script.ts`, at the same depth, with the importer updated.

**Still open in row 3**, with the plan for each:
- In-source vitest blocks in the big files:
  - presentation `🟦️.ts` (2014), react `🟦️.tsx` (3 blocks);
  - `💻️os/🟦️.ts` (5096 lines, 3 blocks, staged by a peer);
  - `🖥️server/🟦️.ts` (staged by a peer);
  - ui react `🟦️.tsx` (11676 lines).

  Each needs its blocks cut into cases that import only exports, or the `run(deps)` wiring. That is 16 rows.
- 19 depth rows: helper modules nested in cases, such as `🏃️execution/🟦️.ts` and `⚖️parity/*`. They need their own semantic scopes.
- 3 self-tests: stdio composition build, flow browser ownership, library styling.
- presentation `📦️packages/🟦️typescript/🧪️tests/🧭️slide-glob-assembly`.

## 3b. Final session additions
- **Presentation core.** The in-source vitest block (55 tests) moved to `🎤️presentation/🧪️tests/📽️presentation-core/🟦️.ts`, which imports exports only. `slide-glob-assembly` moved out of `📦️packages` to `🎤️presentation/🧪️tests/🧭️slide-glob-assembly`. The config and the project inputs were updated. Vitest: **56/56 pass**. The ownership projection hash was refreshed.
- **indexed-generated-output.** The bare `🧪️tests/🟦️.ts` moved to `🧪️tests/🗂️indexed-generated-output/🟦️.ts`. 1/1 passes.
- **Oracle purity walk.** Every oracle now has an owner, so the scan enters only the owner subtrees. That took it from ~140 s to 20 s. It still finds 0 hits, and the purity and ownership-law tests pass. The two purity tests that timed out under load now pass.
- **Production debt now recorded to match the live baseline.**
  - 6 zip oracles (os host `zip`);
  - 6 repo ajv oracles;
  - `modelcontextprotocol-sdk` (os mcp);
  - `image-bmp-3-mutate-reader`;
  - `manifold-mesh-measure` (three-mesh-bvh);
  - `manifold3d-three` (three).

  The spec files are `debt-spec-*.json`, applied with `record-debt.py`.
- **Dependency baseline.** Rewritten with the platform's own `dependency write-baseline`. That added the 27 oracle-linked packages that were missing, such as pillow, pypdf, jszip and yauzl.
- **External host packages.** The host-package test now excuses a host package whose oracle records debt, consistent with the oracle-package test.
- **Profile names fixed.**
  - energy honeybee oracle: profile `ordered-json-v1`, not its pipeline id. The feature tag now names the registered oracle, and the energy case is backed.
  - brepjs-occt: profile `semantic-brep-kernel-edit-v1`, not its pipeline id.
- `validateTaxonomy()` → 0 problems.

## 4. Test-platform suite

- R5 left it at 102/11.
- Run 1 (`test-platform-1.txt`): 102/11.
- Run 2 (`test-platform-2.txt`, before §2c): **104/9**. Passing now:
  - oracle purity;
  - the "only the recorded paths are excused" and "rationale" tests (except energy);
  - csv.

  Two of the 9 were timeouts under peer load: discovery at 5 s, narrowing at 60 s.
- Run 3: 104/9. Run 4: 109/4. **Run 5 (final): 111/2** (`test-platform-5.txt`).

Failures that are not T1's, with owners:

| Test | Reason | Owner |
|---|---|---|
| every committed case satisfies the frozen contract | the 647 rows | t2/t3/owners |
| oracle coverage | energy `🏛️simulate-bestest-energyplus` names an unknown oracle. It was 4 at R5, and the wfc 3 are fixed by peers | energy |
| mutation without fixture | 87 declared mutations without vectors: block 41, cad 19, wfc bitmap 10, gif 9, equation 9 | those owners |
| rationale / profile | `energyplus-25-2-0-via-honeybee-openstudio` names the pipeline id as a profile | energy |
| recorded production debt | zip oracles, see §2c | architect, stdio |

## Processes (pids)

- contract 26510 and 62176: detached, exited.
- test-platform runs 52421, 72278, 89758, 1184 and contract-2: all exited.
- cargo checks 36199, 37815, 40121, 48076: exited.
- No servers.

## Files changed (T1)

- Taxonomy and platform:
  - `📚️library/🔣️taxonomy.json` (3 keys, the go legacy pattern, the benchmark member);
  - `📚️library/🔍️discovery/🟦️.ts`;
  - `🧪️test/🟦️.ts`;
  - `🧪️test/🧬️schema/🔣️.json` (unchanged net);
  - `🧪️test/🧫️fixtures/📐️test-layout/🔣️.json` (+3 laws);
  - `🧪️test/🧪️tests/📐️test-layout/🟦️.ts`;
  - `📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`;
  - `📚️library/🧫️fixtures/🧑‍💻os-dev-composition-ownership/🔣️.json`;
  - `🔒️dependencies.json`.
- Go: 8 renames in `🦑️repo/🔨️modules/*/📦️packages/🐹️go/`.
- Carrier engines, generators and registries: see §2a.
  - equation `✳️any/🏭️generator/**`, `➗️equation/🔮️oracles/🔣️.json`
  - fem `◻️2d|🧊️3d/…/🌐️any/🏭️generator/**`, `…/🔮️oracles/🔣️.json`, 104 fixtures
  - draw `✳️any/🏭️generator/**`, `✳️any|🏷️metadata|🎨️style/🔮️oracles/🔣️.json`, 6 fixtures
  - semio `📐️cad|🔺️mesh|🖊️drawing|🧊️brep|📑️document/🏭️generator/**`, their `🔮️oracles/🔣️.json`, `📑️document/🧫️fixtures/🔣️.json`, `✉️base/🔮️oracles/🔣️.json`, 4 fixtures
  - languages `🔮️oracles/🔣️.json`
- Placement: every source and new case listed in §3, plus the 19 deleted `.js`.

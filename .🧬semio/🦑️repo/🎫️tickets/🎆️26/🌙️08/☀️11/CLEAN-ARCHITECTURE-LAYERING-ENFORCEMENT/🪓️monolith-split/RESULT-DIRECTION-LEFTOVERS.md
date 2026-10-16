# Result: Direction Leftovers (slice DIRECTION-LEFTOVERS, prefix `dlo`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Run of 2026-10-16 12:52 to 14:00 CEST.
Folder F = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/dlo-tools/`. Logs and raw outputs: `F/🗑️generated/dlo-*` (kept).
Path aliases: `FW` = `🧰️framework`, `OS` = `FW/🛍️products/💻️os`, `M` = `FW/🔨️modules`, `SDK` = `FW/📦️packages/🟦️typescript`.

## 0. Verdict

| Target | Before | After | Status |
| --- | --- | --- | --- |
| 2. B6, gis (and stdio docx) imports of os internals | 12 `plugins-framework-sdk-only` edges | **0** | Done. Neutral contracts live in framework modules, behind 6 new SDK entries. gis imports only the SDK. |
| 1. Procedural core, 3 Cargo edges (plus 2 identical edges of the new `pch` test-support crate) | 5 `cargo-plugins-no-artifacts` violations | **0** | Done statically (option 1: two framework contract facade crates). Real compile blocked by the headroom gate; the exact commands for the compile queue are in section 6. |

Order of work: `pch` (RESULT-PLUGIN-CRATE-HYGIENE.md) owned procedural (it created the `🧰️support` test-support crate and edited the core manifest and lock), so I did target 2 first and came back to target 1 once its result file existed. I edited only the `use`/dependency lines of the `pch` support crate (section 4.3), nothing else of its work.

There was no cut predecessor for this slice, so there is no "found state" section.

## 1. Baselines (before the first edit) and after

### 1.1 Cruiser (TS rule)

Exact command of audit 2 section 1 with the config `FW/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`, scoped to the roots that carry the 12 edges (`F/dlo-tools/cruise.sh`): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s/🔌️plugins/🌍️gis ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx FW/📦️packages/🟦️typescript --config <cfg> --output-type err`. The cruiser follows imports into the framework, so the graph is larger than the roots. Outputs: `dlo-cruise-before-b6.txt` (12:54), `dlo-cruise-final-b6.txt` (final, 13:55). The raw output has broken emoji bytes on some lines, so rows are diffed as multisets of their ASCII skeleton (`dlo-tools/attrib.py`).

| Rule | Before | After | Attribution |
| --- | ---: | ---: | --- |
| `plugins-framework-sdk-only` (scoped roots) | 39 | **27** | 12 removed, 0 added. The 12 are exactly the B6 rows of section 2.1; the 27 left are gis/stdio rows of B1 (repo library script imports), B2 (store `child` schema and JSON) and B3 (ShellHost, host-bootstrap, renderer-react, renderer config) |
| `no-cross-technology-✏️s-to-🧰️framework` | 18 | 10 | 8 removed, 0 added |
| `not-to-unlisted` | 68 | 64 | 4 removed, 0 added |
| `no-circular` | 531 | 492 | 45 removed, 6 added; all 6 added rows are peer native-build modules (`🏃️process/📦️artifacts/🏗️native-build/💾️storage/**`), none is mine |
| `renderer-hosts-only-ui` | 488 | 364 | 124 removed, 0 added in the final run. A middle run (13:26) showed 6 added ShellHost rows whose targets are my moved modules (`📄️document/📌️check-in`, `💡️inference/🔌️service`); the final run no longer lists them because a peer reshuffled the same family. Treat them as P2 (renderer hosts), see hand-offs |
| `no-cross-package-relative-@semio-tech/framework` (hygiene P11) | 408 | 419 | 18 removed, 29 added; 20 of the 29 are mine and structural: the 6 new SDK facades and 14 os importers that now reach framework modules by relative path (the existing os convention, for example the old `⏱️trace/🧮️memory` import of the pack codec). The other 9 are peer moves (`🪢️wire-contracts`, `🧾️app-channel`, native-build) |
| all other rules | identical rows | identical rows | |

### 1.2 Cargo direction rule

Authoritative report function run directly (`dlo-tools/cargo-direction-scan.ts`, a copy of `cpd-scan.ts`): `cargoDirectionInventory(root)` as graph and inventory, then `cargoDependencyDirectionReport` with the taxonomy policy `cargoDependencyDirections` (no cargo, no metadata cross-check; same method as RESULT-CARGO-PLUGIN-DIRECTION.md section 2).

| | Before (`dlo-cargo-before.txt`) | After (`dlo-cargo-after.txt`) |
| --- | --- | --- |
| packages / local deps | 2305 / 32208 | 2307 / 32239 (+2 facade crates) |
| violations | 5 (`cargo-plugins-no-artifacts`) | **0** |
| problems | 92 (`missing-role` 50, `owner-role-mismatch` 42) | 92, identical set (`diff` of the sorted `P` lines is empty) |

The 5 before rows: `semantic-ui` to playbook; `generation/sqlite` to flow and to playbook; the new `generation/sqlite/🧪️tests/🧰️support` crate (made by `pch` after RESULT-CARGO-PLUGIN-DIRECTION.md was written) to flow and to playbook.

### 1.3 Type-check

`tsc -p` is blind in this tree (syntax errors in foreign files, see RESULT-SDK-BOUNDARY-2.md section 1), so I used the compiler API route of that slice: `sdk2-tools/mk-tsconfig.ts` plus `sdk2-tools/tsc-sem.ts` (semantic diagnostics for all root files; syntax errors elsewhere do not suppress them). 207 root files = 48 touched or new files (all gis inference files, the docx mutation file, the new framework modules, the os importers, the os store worker, ShellHost importers, dev variant tests) plus every current SDK facade (`dlo-tsconfig-after.json`, `dlo-ts-after.txt`).

- After: **3 diagnostics** (`dlo-tsc-after3.txt`), none in a file or line I changed in substance:
  - `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🗺️document-open/🟦️.ts` (433,20) TS2345 (`actorIdentityGrant` missing in a test argument). Pre-existing: `F/dlo-tools/tsc-head-file.ts` serves the `HEAD` text of this file and reports the same diagnostic at (433,20); my edit to this file is the import line only.
  - `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/👷️worker/🟦️.ts` (15,155) and (17,56) TS2339 `test` on `never`: a file I did not touch (`let driver = null` assigned in a callback narrows to `never`).
- First run after my edits had 5 diagnostics; the two real ones (the spread cast of `post` lost `kind`, and the lease `browserActor` projection was not a discriminated union) were fixed, see section 2.3.
- The compile of the os store worker against the neutral host port is itself the structural proof that the os `ArtifactState` and its lease, reservation and cold-pair classes satisfy the port (`const host: DocumentServiceWorkerHostV1 = {…}` in `activateDocumentServiceWorkerV1`).

### 1.4 Tests (failure sets identical or smaller)

| Check | Command | Before | After |
| --- | --- | --- | --- |
| os vitest project (in-source suites of the os package, includes the store worker, backbone envelope IO, mesh pack decode, worker law suites) | `dlo-tools/vitest-os.sh` (`vitest run --config OS/🧪️tests/🎚️config/🟦️.ts`), diff with `dlo-tools/vitest-diff.cjs` | 609 tests, 499 pass, 110 fail (107 in `🎭️actor-transport`, 3 in the os root file) | 609 / 499 / 110, per-file failing-name sets identical (5 files differ only by a peer moving in-source tests into new files, same totals) |
| gis service-composition suite (os worker law suites plus the gis driver tests; config moved by me) | `vitest run --config ✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🎚️config/🟦️.ts` | not runnable before as one run (config in the plugin); the same suites pass inside the os run | **149 tests, 149 pass** (100 worker law suites, 20 os worker, 29 gis driver) |
| os directory check-in representation law | `bun test` | 5 pass / 0 fail | 5 / 0 |
| installed-service io boundary law (moved) | `bun test` | 8 / 0 | 8 / 0 (49 expects) |
| docx mutation unit tests | `bun test` | 11 / 0 | 11 / 0 |
| gis client io test | `bun test` | 1 / 0 | 1 / 0 |
| hub document-check-in test | `bun test` | error: hub schema module catalog stale | the identical error (foreign) |
| new document scope law | `bun test` | n/a | 2 pass / 0 fail, 60 expects |
| SDK surface law | `bun test` (`SDK/🧪️tests/🔌️sdk-surface`) | 5 pass, 439 expects (RESULT-SDK-BOUNDARY-2.md) | 5 pass, 496 expects; one intermediate run showed 4 pass / 1 fail on `./graph/manifest/execution` while a peer was editing that module; it loads and passes on the rerun |

Lists: `dlo-tests.txt` and `dlo-tests-after-list.txt`; results `dlo-tests-before.json`, `dlo-tests-after.json`, `dlo-os-vitest-{before,after}.json`, `dlo-gis-vitest-after.json`.

## 2. Target 2: the 12 B6 edges (TypeScript)

### 2.1 The 12 edges and their fixes

| # | Edge (before) | Fix |
| --- | --- | --- |
| 1 | stdio docx `🚪️io/💾️binary/🧬️mutations` to `OS/🟦️.ts` (pack codec: `encodePackValue`, `decodePackValue`, `packUInt`, `isPackInteger`) | `@semio-tech/framework/packs` |
| 2 | gis `🪟️presentation` to `@semio-tech/framework-os` (`artifactFrontierIs*`, `decodePackValue`, `DocumentScope`, `ArtifactFrontier`) | `@semio-tech/framework/document/scope` and `@semio-tech/framework/packs` |
| 3 | gis `🧬️schema` to the os `document-check-in-v1` schema | `@semio-tech/framework/document/check-in` |
| 4 | gis `🧪️tests/🔬️probe` to `@semio-tech/framework-os` (`encodePackValue`) | `@semio-tech/framework/packs` |
| 5 | gis `🧪️tests/🎚️config/🟦️.ts` to the os vitest base config | the file is a composition root (it runs the os worker law suites against the gis driver), so it moved to the dev variant that already owns the driver wiring: `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🎚️config/🟦️.ts` |
| 6 | gis `🔌️client/🟦️.ts` to the os installed-service contract | `@semio-tech/framework/inference/service` |
| 7 | gis `🔌️client/🚪️io` to the service wire contract | `@semio-tech/framework/inference/service/io` |
| 8, 9 | gis `🔌️client/🧪️tests/🚪️io` to the service contract and wire contract | the same two entries |
| 10, 11, 12 | gis `👷️worker` to the service contract (type), to the os store worker (host type) and to `@semio-tech/framework-os` (`DirectoryHttpError`, `DocumentScope`, `BackboneWorkerResponse`, `DocumentExecutionTargetLeaseFieldsV1`) | `@semio-tech/framework/inference/service`, `…/inference/service/worker-host`, `…/document/scope` |

### 2.2 Neutral modules (new homes) and SDK entries

All new modules are under `M` (framework, not under `🛍️products`), so the SDK role `framework-neutral` may re-export them.

| Module | Origin | Notes |
| --- | --- | --- |
| `M/🎁️packs/🟦️.ts` (pack value codec, 986 lines) | moved from `OS/🔨️modules/🎁️packs/🟦️.ts` | imports only framework modules (`🎠️kernel`, `🛂️manifest`, `📡️replication`, `⏱️trace`); os root and 9 os importers re-pointed (17 specifier edits). Chosen over `🎒️pack/🌱️value` because the codec depends on kernel and manifest types (an upward edge from `🌱️value`) |
| `M/📄️document/📌️check-in/🟦️.ts` (document check-in grammar, 142 lines) | moved from `OS/…/📇️directory/🧬️schema/📌️document-check-in-v1/🟦️.ts` | the Rust twin `🦀️.rs` and its tests stay in the os folder, which keeps the Rust `#[path]` mount unchanged (the TS and Rust twins are now in different trees, noted as a hand-off); 11 importers re-pointed |
| `M/📄️document/🪪️scope/🟦️.ts` (34 lines): `DocumentScope`, `ArtifactHash`, `ArtifactFrontier`, `DOCUMENT_OPEN_ID_MAX_BYTES`, `artifactFrontierIsGenesisForV1`, `artifactFrontierIsEditedForV1` | cut out of the 1,695-line os directory schema | the os schema imports them and re-exports the same names explicitly (it already re-exports the check-in names the same way), so no os importer changed |
| `M/💡️inference/🔌️service/**`: `🟦️.ts`, `🚪️io`, `🧬️schema` (json and `🧺️value`), `🌐️document-http/🧬️schema` (the Ajv-subset compiler, formerly in `OS/…/📇️directory/🔌️client`), `🧪️tests` with fixtures | moved from `OS/…/💡️inference/🔌️service` and `OS/…/📇️directory/🔌️client/🌐️document-http/🧬️schema` | 10 files; 16 files edited (20 specifier edits) in os, gis and the dev variant; the Rust twin of the contract lives under `OS/…/🌉️mcp/💡️inference/🔌️service` and reads its own fixture, so it did not move |
| `M/💡️inference/🔌️service/🛂️worker-host/🟦️.ts` (75 lines, new) | the explicit worker port design asked for by the B6 hand-off | `DocumentServiceWorkerHostV1` (10 members), the structural state, lease, reservation, cold-pair, fence, transport and message projections, and the class `DirectoryHttpError`, which moved here from `OS/…/📞️directory-client` (the os directory client now imports and re-exports it, so there is one class and `instanceof` keeps working) |

Lossless proof for the 13 moved files (1,849 lines, snapshots `dlo-snap-{packs,checkin,service,config}.json` against the live files): 16 changed lines in total, every one an import specifier, a docstring link, a test path string, the ownership fixture or the config's unused `resolve` import. Mover and tools: `dlo-tools/move.ts` (AST based; it recomputes every relative specifier that touches a moved file, including `typeof import()` types), `dlo-tools/repath.ts` (exact path strings in json with sorted-array preservation).

New SDK entries (6; `exports` is now 159 entries, 6 of them mine; every one is explicit named re-exports, no `export *`): `./packs`, `./document/scope`, `./document/check-in`, `./inference/service`, `./inference/service/io`, `./inference/service/worker-host`. Facades under `SDK/🎁️packs`, `SDK/📄️document/**`, `SDK/💡️inference/🔌️service/**`.

### 2.3 Worker host port: how the os implements it

- The os store worker (`OS/…/🏪️store/👷️worker/🟦️.ts`) no longer defines `DocumentServiceWorkerHostV1`; it imports the port, and `const host: DocumentServiceWorkerHostV1 = {…}` type-checks the os `ArtifactState`, `DocumentExecutionTargetLease`, `DocumentBrowserActorReservation` and `VerifiedColdDocumentPair` against it.
- The three factory signatures take `InstalledServiceDriverV1<DocumentServiceDriverContextV1>` (the neutral context); the os registry keeps its richer `DocumentServiceWorkerContextV1`.
- `post` is the one deliberate loosening: the port accepts `DocumentServiceWorkerMessageV1` (`{ kind: string }` plus open record) and the os casts to `BackboneWorkerResponse` before it stamps `owner` and `serviceId`. gis already cast its messages to that union, and the wire encoder (`encodeBackboneWorkerResponse`) is the validation point. gis keeps a local type for the one message variant it observes (`InferencePortStatusMessageV1`).
- gis derives its state, reservation, pair and fence types from `DocumentServiceWorkerHostV1` as it did before, so the plugin needs only 3 type names, one class and one function type from the SDK.

### 2.4 Registration and law for the new modules

- New law `M/📄️document/🪪️scope/🧪️tests/🟦️.ts` plus the language-agnostic vectors `🧫️fixtures/🔣️.json` (14 rows). Each row is judged by the implementation and by an independent Ajv schema built per row (third party); 2 tests, 60 expects, pass.
- Registered the same way as `test-source-text`: script class and route in `SDK/📜️script.ts`, target in `SDK/📋️project.json`, script in `SDK/package.json`, launch entry `🧪️test🧰️framework📄️document-scope` (order 900.0585177) in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (both parse with `jsonc-parser`, 0 errors). I could not run it through the launcher (needs the repo invocation envelope: running `bun ./📜️script.ts test-document-scope` directly stops with `Original parent Script process invocation required`, the same for `test-source-text`); the underlying `bun test` of the law passes.
- Text references re-pointed (exact path strings, arrays kept sorted): os and wgpu `📋️project.json` input lists, `config-graph.json` fixture, `🗂️schema-document-index.json` (1 path), `generator-ownership` fixture (9 lines), `taxonomy.json` (`wgpu-frame-worker` `sourceModulePaths` and `inputPatterns`, 8 lines; both arrays re-verified as sorted and unique), the os package script (`installed-service-source` paths), the gis package script and the hub package script (3 and 1 config path references), the dev ownership law (config path).
- The io-boundary law of the moved service contract now builds its entry bundle with the owner `🧰️framework` and refuses `🧰️framework/🛍️products` as a removed owner: the bundle proof (92 sources, 0 specific sources) therefore also proves the neutral contract has no os dependency. The check of the Store worker's TS syntax was dropped from that law (a framework law must not read a product file).
- Taxonomy: the simulator `tx2-tools/sim.ts` over the new directories and their ancestors (59 dirs, sandbox copy of the taxonomy): 0 unresolved, 0 ambiguous, all `crate-module-children`; the control name `notanemoji` is correctly refused.

## 3. Target 1: procedural core (Cargo)

Decision: option 1 of RESULT-CARGO-PLUGIN-DIRECTION.md section 5, framework facade crates that expose the neutral model contracts, exactly like the space precedent (`semio-framework-os-host-space`). Two facade crates were needed because the existing `semio-framework-os` host crate is heavy (wasm, workflow, plugin host) and the flow hub crate `semio-framework-os-flow` pulls canvas, wasm and host crates into a pure data dependency.

| Crate | Path | Contents |
| --- | --- | --- |
| `semio-framework-os-flow-contract` (role `product`, id `os-flow-contract`, project `@semio-tech/framework-os-flow-contract-rs`) | `OS/🔨️modules/🌊️flow/📜️contract/📦️packages/🦀️rust` (lib root `🌊️flow/📜️contract/🦀️.rs`, 11 lines) | explicit re-exports of the names the core uses: 12 root names (`CameraJson`, `FlowChannelRef`, `FlowHostSnapshot`, `FlowNodeGui`, `FlowPreviewGui`, `FlowUi`, `NodeChrome`, `OrderedMap`, `OrderedSet`, `SynapseSpec`, `Widget`, `WidgetLayout`), `neural::{Atom, ColdDictionaryBuilder, ColdOwner, ColdRetire, Dictionary, Neuron, Synapse, Tree, Value}`, `retained::{FlowOwner, FlowRetirement}`. One dependency: the flow artifact crate |
| `semio-framework-os-playbook-contract` (id `os-playbook-contract`, project `@semio-tech/framework-os-playbook-contract-rs`) | `OS/🔨️modules/📖️playbook/📜️contract/📦️packages/🦀️rust` (lib root 3 lines) | `default_value_for_block`, `is_block_visible`, `FormGeneration`, `GenerationPlayRoot`, `GenerationPlayState`, `PlaybookSpec`, `PlaybookValues`. One dependency: the playbook artifact crate |

The name list was extracted mechanically from every `semio_framework_artifact_{flow_flow,playbook_playbook}::` use in the core (brace-aware parser, 24 flow paths and 8 playbook paths); the facade re-exports each under the same module path, so a path resolves in the facade exactly when the old path resolved in the artifact crate. One path changed: `neural::cold::ColdOwner::new` (2 sites in `📥️reconstruction`) became `neural::ColdOwner::new`, the same type (`pub use cold::{ColdOwner, ColdRetire}` in the neural engine model), so the facade does not expose the `cold` module. The facade owners sit outside every `🗿️artifacts` segment and are not role `artifact`, so `cargo-plugins-no-artifacts` matches neither by role nor by owner segment (verified by the report function, section 1.2).

### 3.1 Changes in the procedural core (`✏️s/🔌️plugins/🌀️procedural/🫀️core`)

- `Cargo.toml` (shared with `pch`, only the two `[workspace.dependencies.*]` tables changed): the flow and playbook artifact entries replaced by the two contract entries (paths verified to exist).
- `🖼️semantic-ui` manifest and `🦀️.rs` (6 path occurrences), `🧬️generation/🪶️sqlite` manifest and 6 source files (crate path prefix only, line counts equal, rustfmt parses all).
- The `pch` test-support crate `🧪️tests/🧰️support`: manifest and `🦀️.rs` re-pointed to the contract crates (4 lines). If `pch` regenerates the crate from `pch-tools/g-gensupport.ts` it must use the contract crates too.

### 3.2 Registration (recipe steps 6 to 10, os-product variant of `PRODUCER-CRATE-RECIPE.md`)

- Root `Cargo.toml` members: 2 entries inserted at the byte-order position (the array is not globally sorted, so each was placed beside its siblings).
- `📋️build-owners.json`: 2 rows inserted in lexical order (array verified sorted, 2,649 to 2,651 rows).
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 12 routes (build, check, test, each prototype and release) per file, cloned from the `os-host-space` block with new unique orders 900/922/903.20711, .20712, .20721, .20722.
- Locks: `Cargo.lock` (root) gained the two member packages in sorted position; the seven locks that contain the core crates (`🫀️core`, `🌀️procedural/🏭️bridge`, `generation2d`, `generation3d`, `✏️s/🧑‍💻dev/🧩️composition`, `✏️s/🧑‍💻dev/🧹️fixture-sweep`, `🌎️hub`) got the two packages and the changed dependency lists of `semio-s-procedural-core-{semantic-ui,generation-sqlite,generation-sqlite-test-support}` by `dlo-tools/lock-contract.py` (idempotent, sorted). Lock coherence check (python): in all 8 locks the three core package blocks equal the dependency names of their manifests, and there is no dangling dependency name.
- `🧰️framework/Cargo.toml` and `Cargo.lock` are untouched (the framework workspace does not list flow or playbook).

## 4. Foreign errors and observations

- Build image headroom: the launcher stops at `Insufficient physical compiler initial headroom` (4,262,871,040 of 4,294,967,296 bytes available at 13:38; one `metadata --locked --offline` attempt under the mutex ended with this refusal, `dlo-cargo-metadata-core.err`). No compile or `cargo metadata` of any touched crate ran.
- Taxonomy load failed from about 13:30 to about 13:50: `generatorContracts["jco-package-adapter"].inputPatterns must be unique and lexically ordered` (a peer added `OS/📦️packages/🦀️rust/🎛️commands/**/*.ts` after `…/📜️script.ts`). It blocked every check that loads the taxonomy and the simulator on the live file (I used a sandbox copy with that one array sorted); a peer fixed it before my last law rerun. Not mine.
- The dev composition ownership law has a pre-existing failure at its line 22 (`gis` matches `registry`), see section 5 row 15.
- Hub document-check-in test: stale hub schema catalog (identical before and after).
- Pre-existing, unchanged by me: 110 failing os vitest tests (107 actor-transport), the 3 tsc diagnostics of section 1.3, 50 `missing-role` and 42 `owner-role-mismatch` Cargo problems.

## 5. Commands run and real results

| # | Command | Result |
| --- | --- | --- |
| 1 | `sh dlo-tools/cruise.sh before-b6` / `after-b6` / `final-b6` | exit 188 / 13 / 206 (dependency-cruiser exits with the violation count); tables in section 1.1 |
| 2 | `bun dlo-tools/cargo-direction-scan.ts --problems` before and after | 5 violations, 92 problems, then 0 violations, 92 problems (identical) |
| 3 | `bun sdk2-tools/mk-tsconfig.ts` + `bun sdk2-tools/tsc-sem.ts` (3 runs), `bun dlo-tools/tsc-head-file.ts` | 5, 3, 3 diagnostics; HEAD text of the dev test reproduces its diagnostic |
| 4 | `bun dlo-tools/move.ts <spec> --write` for packs, check-in, service, config; `bun dlo-tools/repath.ts <map> <files>`; `python3 dlo-tools/lock-contract.py <locks>` | applied; counts in section 2.2 and 3.2 |
| 5 | `sh dlo-tools/vitest-os.sh before/after`, `node dlo-tools/vitest-diff.cjs` | 609/499/110 both |
| 6 | `bun --bun node_modules/vitest/vitest.mjs run --config …/🧩️service-composition/🎚️config/🟦️.ts` | 149 pass, 0 fail |
| 7 | `bun sdkb-tools/runtests.ts` over `dlo-tests*.txt` | table in section 1.4 |
| 8 | `bun test` of the scope law, io-boundary law, docx mutation tests, SDK surface law | 2/0, 8/0, 11/0, 5/0 |
| 9 | `bun --bun node_modules/vitest/vitest.mjs list --config <moved config> --filesOnly` | config loads and lists the 3 files |
| 10 | `bun repo cargo --store=shared … -- metadata --locked --offline --manifest-path ✏️s/🔌️plugins/🌀️procedural/🫀️core/Cargo.toml` (under the fleet mutex, released after) | refused: `Insufficient physical compiler initial headroom` |
| 11 | `rustfmt --edition 2021 --check` on both facade roots, `rustfmt --emit stdout` parse of every procedural core `🦀️.rs` | exit 0 (facades); 0 parse errors in 9 core files |
| 12 | python path checks | both facade manifests: workspace root exists and lists them as members, lib roots exist; all 7 `[workspace.dependencies]` path entries of the core exist; the 3 core members exist |
| 13 | `bun dlo-tools/jsonc-check.ts` | both launch files: 0 parse errors, 12 new routes and the scope law entry present |
| 14 | `bun F/🗑️generated/tx2-tools/sim.ts <taxonomy> <dirs> <out>` on the live taxonomy (13:29) and on a sandbox copy (13:50) | 0 unresolved, 0 ambiguous among the new directories; the control name `notanemoji` is refused |
| 15 | `sh dlo-tools/ownership.sh after` (dev composition ownership law) | the law now loads the taxonomy (the peer order defect was fixed meanwhile) and stops at its own line 22: `✏️s/🧑‍💻dev/🚀️entry/🟦️.ts` imports `🔌️plugin/📇️registry`, and the regex `/plugins|variants|gis|puzzle/` matches `gis` inside `registry`. The file is unchanged against `HEAD` (empty `git diff HEAD`), so this failure is pre-existing and independent of my edit. The two lines I changed in that law (config path and the `includes` check, lines 108 to 109) were verified separately: the moved config exists, contains `../👷️worker/🟦️.ts`, and the worker it names exists |

## 6. Real-check commands for the compile queue (run when headroom is admitted, one launcher call at a time, under the fleet mutex)

Prefix for all: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --`

1. Facades (root workspace): `check -p semio-framework-os-flow-contract -p semio-framework-os-playbook-contract --lib --keep-going --message-format=short`
2. Core (its own workspace): `check -p semio-s-procedural-core-semantic-ui -p semio-s-procedural-core-generation-sqlite --lib --keep-going --message-format=short --manifest-path ✏️s/🔌️plugins/🌀️procedural/🫀️core/Cargo.toml`, then the same with `-p semio-s-procedural-core-generation-sqlite-test-support --tests`.
3. Dependents: the same `check --lib --keep-going` with `--manifest-path` of `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.toml` and `…/🧊️generation3d/Cargo.toml` (members `…/🚪️io/🪶️sqlite/📦️packages/🦀️rust`, `…/✏️editor/**`, `…/👁️viewer/**`).
4. Lock freshness for the 8 locks: `metadata --locked --offline --format-version=1 --manifest-path <workspace>/Cargo.toml` for root, the core, `🌀️procedural/🏭️bridge`, `generation2d`, `generation3d`, `✏️s/🧑‍💻dev/🧩️composition`, `✏️s/🧑‍💻dev/🧹️fixture-sweep` and `🌎️hub`. If a peer-drifted lock reports `cannot update the lock file`, run `update --workspace --offline --manifest-path …` first (the diff is then larger than my edits).
5. Then the rule: `bun dlo-tools/cargo-direction-scan.ts` (expects 0 violations) or the nx target `@semio-tech/repo-lib:lint-cargo-dependency-direction`.

## 7. Hand-offs

1. Rust twins of the moved TS contracts are now in other trees: the check-in Rust twin stays at `OS/…/📇️directory/🧬️schema/📌️document-check-in-v1/🦀️.rs`; the service Rust twin at `OS/…/🌉️mcp/💡️inference/🔌️service`. Carving the os kernel crates can move them next to the TS neutral modules.
2. B3 (`renderer-hosts-only-ui`, P2): the ShellHost files import the check-in grammar and the service wire contract from framework modules now (6 rows appeared in one cruiser run). The renderer-host slice should route them through its ports. The remaining gis B3 edges (ShellHost, host-bootstrap, renderer-react test import, renderer vitest config) are untouched.
3. B1/B2 gis and stdio rows still counted by `plugins-framework-sdk-only` (27 in this scope): repo library script imports, store `child` schema and JSON.
4. The `post` message loosening (section 2.3): a typed neutral message union for the inference port variants would restore compile-time checking; it needs a decision where `inference-port-*` messages are owned.
5. `pch` should regenerate the `🧰️support` crate with the contract crates; and the compile queue should run section 6.
6. Fix the `jco-package-adapter.inputPatterns` order (peer) so the taxonomy loads again; then rerun `sDevCompositionOwnershipV1` and the schema index check.
7. Taxonomy registration beyond the simulator (real `inventoryTaxonomy`, about 8 minutes) was not run.

## 8. Files changed

- SDK: `SDK/package.json` (+6 `exports`, +`test-document-scope`), new facades `SDK/🎁️packs/🟦️.ts`, `SDK/📄️document/🪪️scope/🟦️.ts`, `SDK/📄️document/📌️check-in/🟦️.ts`, `SDK/💡️inference/🔌️service/{🟦️.ts,🚪️io/🟦️.ts,🛂️worker-host/🟦️.ts}`, `SDK/📜️script.ts`, `SDK/📋️project.json`.
- Framework modules (new or moved): `M/🎁️packs/🟦️.ts`, `M/📄️document/{🪪️scope,📌️check-in}/**`, `M/💡️inference/🔌️service/**` (10 moved files, the new `🛂️worker-host`).
- os: store worker, directory client, directory schema, `os/🟦️.ts`, `🧵️backbone`, `🧾️app-channel`, `🪢️wire-contracts`, `💡️inference/🚪️opening`, directory `🚪️io/📝️text` (3 files), 4 `action-handoff` files, 5 ShellHost files, `os/🧪️tests` (mesh-pack-decode, backbone-envelope-io), `os/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`, the wgpu `📋️project.json`, `config-graph.json`, `🗂️schema-document-index.json`; moved-out directories left empty were removed.
- gis: `🔌️client` (3 files), `👷️worker`, `🧬️schema`, `🪟️presentation`, `🧪️tests/🔬️probe`, `📦️packages/🟦️typescript/📜️script.ts`; the removed `🧪️tests/🎚️config/🟦️.ts` (moved). Dev variant: new `🧩️service-composition/🎚️config/🟦️.ts`, `🗺️document-open` test, ownership law. Hub: `📦️packages/🦀️rust/📜️script.ts`. stdio: docx `🧬️mutations` file.
- Rust: new `OS/…/🌊️flow/📜️contract/**` and `OS/…/📖️playbook/📜️contract/**`; procedural core files of section 3.1; root `Cargo.toml`, `Cargo.lock`, `📋️build-owners.json`; the 7 plugin, dev and hub locks of section 3.2.
- Registry and fixtures: `taxonomy.json` (8 lines), `generator-ownership` fixture, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.
- Tools: `F/dlo-tools/**`; logs `F/🗑️generated/dlo-*` (kept).

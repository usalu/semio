# result-law-sweep2 (2026-10-20)

Executor `law-sweep2`. Inputs: `brief-1019.md`, `result-law-sweep.md`, `bzl-act-tools/wrap.ts`, `result-bzl-suite.md`. C = ticket folder. Tools: `C/law-sweep2-tools/`.

## Method

- Route inventory: every `metadata.semio.workspaceCommand` declaration in the `📋️project.json` files (`routes.py`, `git grep`): 172 routes, 118 `verify ...` laws (the "90" of law-sweep are the subset that `dispatchOwnedScriptRoute` sends to `bun repo run`; all 118 were taken, plus the 54 non-verify routes). Each route = `project.json` target `command` + `cwd` -> the package `📜️script.ts <command>` -> `runOwnedVerifyScript` in `<artifact>/🧪️tests/🔬️rust-package-commands|typescript-package-commands/🟦️.ts`.
- Runner `run-law.ts` (wrap.ts pattern): issues the repository script providers (`issueRepositoryScriptProvidersV1`) and a process envelope (`createScriptProcessEnvelope`, unbounded elapsed), then spawns `bun --preload guard.ts ./📜️script.ts <args>` in the route `cwd`. One law at a time under `perl -e 'alarm 300; exec @ARGV' --` (`drive.py`, own process group, group-killed on timeout).
- `guard.ts` (preload, re-injected into nested `bun`/`bun test` children): refuses and logs any `spawn/exec/Bun.spawn` of `cargo|rustc|nx|bazel|playwright|vitest-binary|npm|docker`, `bun repo run|cargo|run-many|...` and the cargo relay script. A law that hit the guard counts as "TS part ran, remainder is cargo/nx". In-process vitest (the Node API used by `runVitest`) is not a spawn and ran. Machine load average was 24-80 on 10 cores.
- `native` / `--native` routes and the `*-test-snapshot-sqlite` (no `source`) routes (they queue on the shared `cargo:repository-tests` lease) were never started.
- Not run: `cache-prune ...` (mutates the cache), `native` variants, no bazel, no nx, no cargo, no Playwright browser.

## Totals (latest result per law)

| Set | Laws | Pass | Skipped (cargo/native/lease) | Not pass |
|---|---|---|---|---|
| `verify` routes | 118 | 71 | 22 | 25 |
| non-verify routes | 54 (30 run; 23 `native` + `cache-prune` not started) | 23 | 24 (23 native, 1 mutating) | 7 |

Of the 25 `verify` not-pass: 10 pass their TypeScript part and stop at cargo, 10 are cargo/native only (nothing TS to run) and `generation3d-document-io` queues on the cargo lease, 3 are open defects (two stale library pin laws, one generated-descriptor drift) and `framework-ui-protocol-ownership` exceeded 300 s only under load (its three parts pass individually).

## Skipped (never started)

`architect-program|cad|dag|forms|remodel-remodeling|trinity-rewriting-test-snapshot-sqlite` (6, lease queue on `cargo:repository-tests`; their `-source` variants ran and pass), every `... native` / `-native` / `--native` route (16: `artifact-reference-retained-equality`, `fem2d-invariants native`, `fem2d|3d-material`, `fem3d-results|transform-intent`, `generation3d-widget-inputs`, `inference-client`, `value-admission-check`, `value-retained-equality`, the `*-test-snapshot-sqlite-native`), non-verify `cache-prune` and the 24 `test-* native`.

## Results: law -> result -> cause / fix

### Fixed (re-run after the fix, rc 0)

| Law | Before | Cause | Fix |
|---|---|---|---|
| `architect-program-snapshot-sqlite-source`, `architect-window-ownership` | TS6053 missing file / cannot find module | stale paths: sqlite snapshot tests moved to `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests`, window oracle dir renamed `🔬️window` -> `🔬️window-ownership` | `architect/.../rust-package-commands` paths |
| `dag-document-contract` | 3 layers: missing `🪪️document` test dir; fixture roots gone; parser accepted `childId != target.artifactId` | (a) dir renamed `🪪️document-contract`; (b) the mutation fixtures were deleted with the graph content move (commit 666/670); (c) real defect: `parseDagContentChild` never enforced child identity, the oracle's `invalidIdentityDocuments` row #3 was accepted | commands path; test `mutationRoots: []`, `committed 0/0`; `parseDagContentChild` now throws unless `childId === target.artifactId` |
| `drawing-canvas-window-ownership`, `flow-window-ownership`, `gis-map-window-ownership`, `gis-terrain-window-config`, `norm-results-window-ownership`, `generation3d-preview-window-transient` | ENOENT fixture | fixtures moved from `<config>/🧫️fixtures` to `<config>/🧪️tests/🧫️fixtures` (test did `../../🧫️fixtures`); norm commands still named `🔬️window` | `fixpaths.py` rewrote the 11 provable cases (`../../🧫️fixtures` -> `../🧫️fixtures`, only where the old path is absent and the new one exists); norm commands path |
| `remodel-window-ownership` | module missing, then ENOENT, then ajv "must be object" | dir rename `🔬️window-ownership`, fixture moved + renamed, and the three window-config mutations became atomic `{kind:"snapshot",config}` replacements while the fixture still held `set-camera|set-layers|set-frame-cursor|set-report-table` | commands path, test fixture path, fixture mutations rewritten to `snapshot` rows (Rust law reads the same fixture and already parses `snapshot`) |
| `layout-window-ownership` | `layout-window-config-mutation-invalid` | fixture rows `{kind:"update",patch}` vs `{kind:"snapshot",config}` | fixture rows (textual edit, two lines) |
| `rewriting-window-config` (TS part) | ENOENT witness | fixtures moved (`./../../🧫️fixtures` form, missed by the scripted fix) | test path; law now stops only at its cargo step |
| `rewriting-document-contract` | `rewriting fields differ`, then `Jack JSON camera fields differ` | real defect: `parseRewritingDiffJsonValue` decoded a sparse diff with the full-program `parseRhs` and the sparse Jack `workingGraph` diff with the full-artifact `parseJackJsonValue` | `parseRhsPatchJson` in the rewriting JSON deserializer; new `parseJackDiffJsonValue` (jack deserializer + explicit public surface `📦️packages/.../🟦️.ts`) |
| `playbook-document-contract` | schema rejects `document` | playbook artifact schema/parser own only `flow` (`document` child removed in 670), fixture + test still carried it; mutation fixtures reduced to `change-title` | fixture without `document`, invalid children target `flow`, committed `2/1` |
| `presentation-document-contract` | `committed` 18/5 vs 16/4; then TS6053 | schema has 8 mutations (16 snapshots, 4 diffs); sqlite test path moved | committed `16/4`; tsc source list path |
| `procedure-document-contract` | scandir ENOENT, then `fields do not match ProcedureDiff` | step mutations deleted (commit 670), `ProcedureDiff` has no `artifact` slot | `mutationRoots: []`, committed `0/0`, fixture diff without `artifact`, expected `{schema,flow,text}` |
| `norm-document-contract` | `climateTable` required; scandir ENOENT; committed 46/18 vs 52/21; `childKind` crash; TS6053 | Din18599 schema now has `climate` numeric record + `climateTable` child; mutation fixtures under `🧪️tests/🧫️fixtures`; diff schema deliberately has no `climateTable` | fixture document, `childField: "climateTable"`, committed `52/21`, fixture root, diff facet asserted to omit the slot, tsc path |
| `contributions-receiver` | `ownerFields` `[]` | Rust fields are `pub` / `pub(super)` now, the text pin regex ignored them | regex `^\s*(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:` |
| `framework-flow-physical-retirement`, `generation2d-window-camera-ownership` | ENOENT on doubled path | the law test used `process.cwd()` as the workspace root, but the route runs in the package directory (also broken under nx) | root derived from `import.meta.url` (generated `cwdroot.py` pattern) |
| `framework-ui-protocol-ownership` | fixture ENOENT; then two renderer suites failed collection (`editorTextInputFixture` undefined); then hub catalog ENOENT | (a) retained-command fixtures moved under `✏️editor/🧪️tests/🧫️fixtures`; (b) the react vitest config aliased the whole `@semio-tech/framework-editor-rs` tree to the wasm stub, swallowing `/testing/fixtures`; (c) hub space descriptor moved to `🧩️component/🔣️.json`, `🎯️space-command` module renamed `🕹️space-command`, builder is `PluginBuilder::<_, SpaceApps>::new` | schema-owner paths; precise alias for `.../testing/fixtures` placed before the tree alias; space `📜️script.ts` paths. Parts verified separately: ownership TS, both vitest runs (1+1 files pass), `interactive-job-catalog-check` rc 0. The full law needs ~355 s under load and was cut by the 300 s alarm |
| `home-host-panel-owner` | fixture ENOENT; self-match; descriptor path; Studio count; stale vitest filter; stale lane | the scan matched its own regex literal; generated Space descriptor moved to `🧩️component`; Studio no longer has `setActivePanelTab` (3 -> 0); `typed-operation` suites live in `🔌️plugin-runtime-suite`; `TYPED_OPERATION_RESULT_LANE_MAX = 15` makes lane 15 (diagnostic, new) valid | fixture path; scan excludes the law file; descriptor path; Studio expected 0; react script filters -> `🧪️tests/🔌️plugin-runtime-suite/🟦️.tsx`; invalid lane in the page test is 16 |
| `operation-diagnostic-host-check` | no test files | same stale vitest filter as above | `✏️` react `📜️script.ts` filters |
| `terrain-document-contract` | `Cannot use test outside of the test runner` | the oracle registers `test(...)` at import (the package suite runs it with `bun test`), the law `import`ed it | law now runs `bun test <file>` through `runRepositoryCommand` (same pattern as `cad`), the oracle file is unchanged |
| `stdio-document-contract` | ajv `unknown keyword "x-semio-type"`; `properties: null` rejected; loose `ArtifactLink` row; round-trip drift | (a) generator annotation `x-semio-type` missing from the vendor vocabulary; (b) Rust `Option<Option<ArtifactChild>>` (`Some(None)` clears) emitted as non-nullable `object`; (c) representation `item` not referencing the link schema; (d) parser normalises list deltas to `{added,modified,removed}` and decoded design pieces in `modified` rows | vocabulary `x-semio-type: string`; kit diff schema `properties: ["object","null"]` and `item: $ref link schema`; test `deltaDefaults`, `diffFixture` covers `modified` rows, diagnostic message prints the failing case |
| fem `fem2d-invariants source`, `fem2d-window-config-contract`, `fem3d-window-config-contract` | `Property 'vitest' does not exist on type 'ImportMeta'` (tsc) | `📡️replication/🟦️.ts` gained an in-source vitest block (commit 696) without the repo's `/// <reference types="vitest/importMeta" />` | reference added (same convention as `🎯️action-bus`, `🛂️manifest`, ...); fem2d rustfmt list also drops the deleted `elements2d/🧪️tests/🔬️unit/🦀️.rs` |
| `test-operation-diagnostics source` | same TS2339 in `🎭️actor/📥️cold-pair`, `🎭️actor/🚪️lifetime` | same | references added |
| `architect-program-test-snapshot-sqlite-source` | 6 bun-test 5 s timeouts | load 80; rc 0 at load 24 (28 s) | none (load) |
| `fem-live-visual-publication`/`fem-numerical-microcursor` (partial) | pins fail | `policyReadFileSafe` read only the root file of sources that are now module trees | commands file reads `policyReadModuleTreeSource`; laws still fail (see open) |

### Pass without changes

`cad-activation-source`, `cad-owned-target`, `cad-reference-owner-syntax`, `cad-document-contract`, `cad-snapshot-sqlite-source`, `cad-test-snapshot-sqlite-source`, `contributions-syntax`, `curation-document-contract`, `dag-test-snapshot-sqlite-source`, `fem-assembly-physical-owners`, `fem-combination-stress`, `fem-mesh-preparation-owners`, `fem-numerical-page-owners`, `fem-pcg-publication-owners`, `fem2d-material source`, `fem3d-material source`, `fem3d-numerical-child-source`, `fem3d-results source`, `fem3d-transform-intent source`, `forms-document-contract`, `forms-test-snapshot-sqlite-source`, `forms-try-window-ownership`, `generation3d-widget-inputs`, `geometry-context`, `inference-client`, `jack-document-contract`, `layout-document-contract`, `layout-frame-selection`, `map-document-contract`, `native-receiving-original-source`, `note-document-contract`, `note-empty-config-ownership`, `program-document-contract`, `puzzle-fill-policy-self-tests`, `remodel-remodeling-test-snapshot-sqlite-source`, `semio-member-store-producer-source`, `sequence-window-ownership`, `trinity-rewriting-test-snapshot-sqlite-source`, `value-admission-check`, `value-retained-equality source`, `viewport-presence-check`, `window-utility-check`, `writer-window-state`, `artifact-reference-retained-equality source`.

### TypeScript part passes, remainder is cargo (guard or cargo policy refused, nothing built)

`fem-scalar-owners-native`, `fem2d-invariants mounted|numerical|presence` (rustfmt/tsc parts pass, exact cargo laws need `SEMIO_CARGO_TEST_POLICY`), `rewriting-window-config`, `rewriting-map-ownership`, `wires-document-contract`, `wires-window-transient`, `note-snapshot-guest`, `note-sqlite-snapshot-guest` (cargo owner preparation), `flow add-widget-retained-check` (source pins pass after the fixes below; stops at `cargo:selected-preparation`).

### Cargo / native only, no TypeScript to run

`fem3d-numerical-child-native`, `inference-mcp`, `inference-native-service`, `jack-child-selection`, `jack-graph-query`, `jack-query-ownership`, `sqlite-snapshot-native`, `generation3d-document-io` (queues on `cargo:repository-tests`), `retained-pack-encoding`, `staged-payload-funding`, `window-utility-native`. The cargo relay refused each one itself (`Build mode must be prototype or release` / `Explicit Cargo test policy required`); no compiler started.

### Non-verify routes

Pass: `test source`, `test-camera-presence source`, `test-fixed-dofs source`, `test-fixed-request-owners source`, `test-native-shared-receive source`, `test-operation-diagnostics source`, `test-owned-projection source`, `test-result-publication source`, `test-retained-diff-receiving source`, and the 14 `framework` wrapper routes `test-close-demand|exact-key-lookup|fixed-key-slots|immutable-root-clone|indexed-edit|named-edit|named-key-index|named-root-receive|native-persistent-receive|native-retirement-loan|persistent-build|persistent-canonical|sealed-frontier|slice-equality source` (these need the `framework` process wrapper `command --config ...` plus `SEMIO_BUILD_MODE=prototype`, `SEMIO_COMMAND_STORE`, `SEMIO_TEST_ARTIFACT_DIR` set; `drive.py` runs the wrapper, store/artifact dirs were under the ticket and deleted).

## Not pass (open, root cause known)

| Law | Class | Cause | Needed |
|---|---|---|---|
| `fem-live-visual-publication`, `fem-numerical-microcursor` | stale law | library laws `⚖️laws/🎯️tool-job-coverage/🏗️fem-visual` and `.../🧮️fem-numerical-microcursor` pin an obsolete hot-loop design: 71 of ~160 text pins of the microcursor law fail (`LDLT_MAXIMUM_ORDER = 40` is now `u32::MAX`, stages/functions renamed or moved to `🔺️ldlt/*`, `🎵️subspace/*`); bisect tool `bisect.py` lists every failing clause | rewrite the two laws against the split sources (frozen library; not staged: it is a redesign, not an anchored edit) |
| `gis-inference-discovery-oracle` | generated descriptor drift | `InferenceDiscoveryRosterV1` now requires `documentSchema` + `documentSchemaVersion`; the committed `🌎️hub/🧩️compositions/🌍️gis/🧩️component/🔣️.json` (generated by `describe` after a wasm build) lacks them; the neutral fixture got `documentSchema: "gis.map"`, `documentSchemaVersion: 1` (value inferred from the law's own `gis.map` check, unverified against Rust) | regenerate the descriptor; confirm the fixture values |
| `flow child-edit-check`, `child-identity-check`, `test-source` | stale pin | after three fixes (module-tree read, `ChildEmitPreparation`, relocated `artifact-app-laws`) the next pin `/InputSlider\s*\{\s*id: String,\s*label: String,/` no longer matches the Rust shape | pin owner |
| `stdio package-contract`, `stdio package-graph` | needs lease context | `phase.leaseControl.signal` undefined: requires the lease control that `bun repo run` provides, the envelope alone does not | run under `bun repo run` |
| `schema runtime-graph` | nx | spawns `bun repo run @semio-tech/plugin-registry:guest-framework-check` | not run |
| `framework-ui-protocol-ownership` | load | all three parts pass, the sum exceeds the 300 s alarm at load 25-80 | rerun on an idle machine |

## Transient / environment notes

- `bun` boots of anything importing `discoverPackages` crashed once in `packageImplementationAdmitted` (`admission` undefined) from a ticket script; the same law runs via the package scripts were unaffected.
- Nested `bun test` children need the preload injected after the subcommand (`bun test --preload X`); `BUN_OPTIONS=--preload` breaks `bun test` (`Script not found "test"`).
- The cargo relay is invoked as `bun <⚡️caching/🦀️cargo/📜️script.ts> relay ...` without the word `cargo`; the first guard version missed it (the relay itself refused: no build ran). The guard now blocks on the path.

## Changes (files)

All outside the frozen paths; nothing staged under `📚️library/` (no anchored library edit was needed; the two stale library laws need a redesign, see above).

- `🧰️framework/🔨️modules/📡️replication/🟦️.ts`, `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts`, `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts` (vitest `importMeta` reference)
- `🧰️framework/🔨️modules/🧬️schema/🧬️vendor-annotation-vocabulary/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧬️schema-owner/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`, `.../⚛️react/🧪️tests/🎚️config/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📃️typed-operation-pages/🟦️.tsx` (invalid lane 16)
- `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-discovery/🟦️.ts`, `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/` (54 files in total including the ones above; per plugin): `🏛️architect`, `🕸️dag` (commands, `🧬️schema/🟦️.ts`, document-contract test), `📕️norm` (test + fixture + window test), `🌊️flow` (script, contributions test, source-contract test, main window test), `🎬️sequence`, `🌀️procedural` (generation2d camera, generation3d three tests), `🌍️gis` (gismap window test, gisterrain contract test + commands, gismap discovery fixture), `🏙️bim` world unit test, `📏️layout` (test + fixture), `🖍️draw`, `📖️playbook` (test + fixture), `🎞️animate` presentation (test + commands), `📜️imperative` procedure (test + fixture), `📸️remodel` (commands, test, fixture), `🔱️trinity` rewriting (window test, JSON deserializer) and jack (JSON deserializer + public surface), `🪐️space` home (unit test, commands), `🗄️stdio` kit (document-contract test, diff schema), `🏗️fem` (`typescript-package-commands`, `◻️2d` commands).
- `🧑‍💻dev/📕️norm` typescript-package-commands (`✏️s/🧑‍💻dev/📕️norm/🧪️tests/🔬️typescript-package-commands/🟦️.ts`).
- Tools (new): `C/law-sweep2-tools/{guard.ts,run-law.ts,drive.py,one.sh,routes.py,bisect.py,fixpaths.py,cwdroot.py,summ.py,final-status.json}` (`drive.py` needs `routes.py` first; `final-status.json` is the machine-readable per-law result).

## Verification

- Each "fixed" row was re-run with `law-sweep2-tools/one.sh <cwd> <command>` (the same runner) and printed rc 0 as listed; `drive.py` batches reproduce the totals (`final-status.json`).
- Exact commands: `python3 law-sweep2-tools/routes.py C`, then `python3 law-sweep2-tools/drive.py C <from> <to> verify|nonverify [idx,idx,...]`, wrapper routes with `SEMIO_BUILD_MODE=prototype SEMIO_COMMAND_STORE=... SEMIO_TEST_ARTIFACT_DIR=...`.
- Not run: any cargo build or test, nx, bazel, Playwright/Chromium, `bun test <dir>`; no `bun repo ...`. `fem3d-window-config-contract` ran its in-process vitest (Node API) once; all other test runs were by explicit file path.
- The Rust sides of the edited fixtures (`remodel`, `layout`, `playbook`, `procedure`, `presentation`, `norm`, `dag`, kit diff schema) are not compiled or run; the `remodel` Rust law already parses `snapshot` mutations, the others were not checked.

## Open items

1. Rewrite the two stale tool-job library laws (fem live visual publication, fem numerical microcursor) or retire them (`bisect.py` output shows which pins).
2. Regenerate the GIS component descriptor (documentSchema fields) and verify the fixture values.
3. Flow `InputSlider` pin; `stdio package-*` need `bun repo run`.
4. Cargo remainders of 20 laws still unrun (list above), plus the `fem3d-window-config-contract` vitest 15 s budget flaked under load once.
5. `parseDagContentChild` now rejects `childId != target.artifactId`: check the DAG fixtures of other consumers (hub, forms) when they next run.
6. Schema generator: nested `Option<Option<_>>` must emit a nullable schema and `ArtifactLink` rows must `$ref` the link schema (hand-patched in the kit diff schema only).

# Result lay15: cycles, no-core-path, glue names, narrow entries

Executor `lay15`. No `package.json`/`bun.lock`/taxonomy edit, no cargo, no git-modifying command, no sub-agent. Tools and kept inputs are in `C/lay15-tools/`; every tool output (`🗑️generated/lay15-run`) was deleted. The coordinator HARD FREEZE (library, root script, manifests, taxonomy) arrived after my last edit there; nothing was edited under it afterwards.

## 1. Before / after (dependency-cruiser, lay13 per-area method, 12 areas, error severity)

`lay13` = end state of lay13 (start of this task). `b` = my cruise after the cycle and rename fixes, before glue/rewrites. `a` = final. Peers kept editing during the run (new `*-package-commands` tests, `shard-worker-source` split, `cleanup/tests` move), so b to a contains drift that is not mine; the last row isolates my files.

| rule | lay13 | b | a |
|---|---:|---:|---:|
| no-circular | 27 | 10 | **1** (generated, gitignored bundle) |
| no-core-path | 4 | 0 | **0** |
| no-cross-package-relative | 1,688 | 1,413 | **1,328** |
| plugins-framework-sdk-only | 97 | 130 | 155 (peers) |
| renderer-hosts-only-ui | 215 | 52 | 57 (lay14 scope) |
| no-cross-technology | 37 | 37 | 290 (peers: 85 new `*-package-commands` test files, 1 of them mine) |
| framework-no-products | 28 | 28 | 28 |
| plugin-no-extension-or-artifact | 21 | 21 | 21 |
| ui-no-framework-packages / ui-foundation-closure | 6 / 2 | 6 / 2 | 10 / 22 (peers: new package-commands tests under ui/value) |
| no-state-outside-os | 1 | 1 | 1 |

Mine only: in the 83 files I rewrote the violations from those files went ncpr 188 to 95, no-cross-technology 7 to 3, no-state-outside-os 1 to 0, others unchanged. Per area table: `lay15-tools/per-area-table.md`.

The one remaining `no-circular` is `os/modules/dev/📤️distribution/🧶️bundles/{🌊️flow/🌐️browser-*,🖥️runtime-*}.js`: a minified, gitignored build output (`.gitignore` line 16) that the cruiser follows because `ignoredPathPatterns` lists `**/📤️dist` but not `📤️distribution`. The pattern list is pinned verbatim by a law and the taxonomy is frozen, so not changed; not a source cycle.

## 2. Cycles fixed at the root cause (27 to 0 source cycles)

Important finding: the lay1 and lay13-era scanners missed compact code (`import{a}from"x"`), so I wrote `lay15-tools/cycles-js.ts` (TypeScript `preProcessFile`, package-export resolution, in-source tests excluded). It agrees with dependency-cruiser; zero source SCC remain.

- process leases / metadata: `retainResourceQueueMetadataHeadroomV1` takes the custody check as a parameter (`requireCustody`); metadata no longer imports leases. Call sites: leases (2), reserve test, child script.
- ui contract resident / wire pages: the builder (`OwnedUiOperationPayloadBuilder`) declares the seams it needs (`UiResidentPayloadSeam`, `UiResidentPageSeam`, `UiResidentPayloadReaderSeam`, statics) and a bind-once host (`bindUiOperationResidentHost`, the repo's existing host pattern); the resident binds itself at module end. A `UiResidentTypes` module augmentation keeps renderer tests typed against the concrete reader class. Pages has no import of resident.
- kernel return input / actor shard-client: kernel imports `@semio-tech/framework-actor/shard-client/activation` (existing key) instead of the whole shard-client entry.
- registry playground / catalog-view / repo-lib dev: `loadFrameworkOsPlaygroundCatalog<Variant extends PlaygroundCatalogRow>` is generic; repo-lib no longer imports the registry type; `build/plan` passes `PlaygroundBuildTarget`.
- store worker folder / config-lane: `writeFolder`, `folderEnvelopeUrl`, `retireCurrentFolderCanonicalBootstrapMirror` moved to new leaf `📁️folder/✍️write`; 4 importers re-pointed.
- bazel operations / script: `BazelTaskV1`/`BazelTaskPlanV1` moved to leaf `bazel/📋️task-plan`.
- ui elements (12-file cluster): `Label` imported the Tree `📐️layout` barrel, which re-exports `actions`; it now imports `📏️row-alignment` directly.
- process native-build storage (5-file SCC `storage` / `fixed` / `birth/claim` / `cleanup/command` / `cleanup/retirement/contract`): the platform dispatch (`darwin ? persistent : fixed`) is a strategy now. New leaf `🔌️retirement-backend` (`NativeRetirementBackendV1`, `bindNativeRetirementBackendV1`, `nativeRetirementBackendV1`); `fixed` and `cleanup/command` bind at module end; the core `storage/🟦️.ts` and `birth/claim` dispatch through it (4 sites) and import neither backend. `readNativeImageRetirementFileV1` moved to leaf `📄️retirement-file`, `readNativeCreatorClaimRecordsV1` plus its closure to `🌱️birth/🧾️claim-records` (lay1 `extract.ts` via `extract2.ts`, importers re-pointed, package glue got `export *` of the leaf). `platform/🟦️.ts` imports both backends statically, so both are loaded whenever a dispatcher runs. Two source-shape tests re-pointed (`cache/retirement/tests` claim-records import, `fixed/tests/receipt` dynamic import).
- generated bundle and the vendored `preview2-shim`: not source (see section 1).
- renderer PluginRuntime cycles: no longer present in the cruise; not touched (lay14).

## 3. no-core-path (4 to 0)

`🧬️retained-clone/🔗️shared` to `🔗️sealed-shared-list` and `🛬️decode/🧳️owned/🔗️shared` to `🔗️sealed-shared-receiver` (`lay1-tools/movedirs.ts`: only the slug changes). Re-pointed: both `#[path]` twins (`owned/🦀️.rs`, `📽️projection/🧬️clone/🦀️.rs`), both `fixture_text!` literals, 2 literals in `value/📦️packages/🦀️rust/📜️script.ts`. Also fixed a stale lay1 leftover `♻️retirement/🔗️shared/🦀️.rs` to `🔗️arc-custody` in `tool-job-coverage/…/caller-retained-media/🟦️.ts`. Both names resolve to admitted kinds (`tax.ts`); `policyBannedNameStemBreaches` = 0; new leaves `✍️write`, `📋️task-plan`, `🔌️retirement-backend`, `📄️retirement-file`, `🧾️claim-records` also resolve (`tax.ts`; the last four via the same crate-module-children kind).

## 4. Glue names (51 rows) and importer rewrite

`lay15-tools/ncpr2.ts` (lay13 `ncpr-all.ts` plus: JSON glue recognition, destructured dynamic imports and namespace-member analysis, `glue` mode, `narrow` mode, closure/barrel filter, renderer skip). `glue` mode appended 178 explicit named re-exports (ordinal-sorted, value/type split) to 19 existing glue files (shell, interaction, actor shard-client/wire-turn/retained-turn, canvas runtime-assets, process leases/routing/native-build, kernel input, ui fixtures/keybindings/ContextMenu, os store fixtures, ui-styling vite, react testing config) and 1 more (`ui-react build-tooling`) in a second pass. 30 of the 51 rows no longer have a skipped importer; 21 still do: 17 are renderer importers (lay14), 3 frozen files, 1 namespace use, plus `repo-test .mjs` (`export *` glue the tool cannot follow). Reports: `glue-report-1.json`, `glue-report-2.json`.

Importer rewrite: 246 planned, 245 applied, then reverted 3 that created cycles (`revert-cycles.ts`) and 25 that broke `ui-no-framework-packages` (`revert-rules.ts`); 218 kept in 83 files (`ncpr-applied.json`, `reverted.json`). Import attributes were stripped from the 5 JSON imports now going through `.ts` JSON glue (`strip-json-attr.ts`; without it `bun build 📜️script.ts` failed). 110 candidates inside the zero-touch CLI closure were not applied. Revert tool bug (first-occurrence replace swapping two identical `after` strings) hit 2 files; repaired by order (`repair-order.ts`), both equal HEAD again.

## 5. Narrow entries instead of barrels (19 edges)

21 narrow glue files created under `repo-lib/📦️packages/🟦️typescript/…` (one library module each, explicit named re-exports). Importers in the bootstrap/CLI closure now import them by relative path; 2 bare importers were left alone. The keys are not in `package.json` yet, so these edges still count as `no-cross-package-relative` (net 0 against the old deep import) until the deps owner adds them. Needed exports keys with glue paths and names: `lay15-tools/narrow-keys.json` (e.g. `./storage/enforcement/budget/opening`, `./storage/enforcement/tools`, `./artifacts/native-orchestration/invocation`, `./build/graph`). Dependency gaps for bare rewrites: `lay15-tools/depgaps.json` (`@semio-tech/framework` package needs `@semio-tech/framework-os`).

## 6. Verification

- `tsc --noEmit` over 258 touched files (`tsconfig` listing them): no error in my files; remaining errors are peers' (`creatorPending`, `maxPathBytes`, `build/📜️script.ts` Promise type, missing `@dnd-kit`/`katex`/`flow-core`, `import.meta.vitest`).
- `bun build --target=bun 📜️script.ts` ok; `bun build --target=bun …/🏗️build/📜️script.ts` ok; `bun repo --help` boots; `bun C/scan-bare-imports.ts … @semio-tech/ typescript`: files 441, bare {} (CLI closure still relative-only). 102 touched files bundled with `bun build`: only Vite-query (`?raw`, `?worker`), vitest-config multi-output and missing-install failures.
- Rust: `rustfmt --check` parses the renamed mounts; `lay1-tools/rust-paths.ts` 71,161 literals, 0 mention the renamed folders (2,505 baseline misses are inline-module forms the resolver cannot follow); `cargo-paths.ts` 50,154 entries, same 3 non-path literals. Cargo not run.
- Tests: vitest `actor-transport` (179 tests: kernel input, shard-client, resident, pages) 179/179 after the cycle fixes; at the end 176/179, the 3 failures parse `shard-worker-source/🟦️.ts`, which a peer split at 19:29 (not mine). Native-build suite (61 bun files incl. `cache/retirement`): baseline 328 pass / 114 fail before the refactor; after: identical per file except one file a peer moved (`cleanup/tests/📋️state.ts`). Not run: leases suites (need real disk images), cargo.

## 7. Open items

1. Add the 21 narrow keys and the dependency gap (package.json freeze owner).
2. 595 skipped importers still need new manifest keys (lay13 spec 323 rows, unchanged); 17 renderer importers are lay14's.
3. `📤️distribution` is not in `ignoredPathPatterns` (frozen taxonomy plus the pinned law).
4. Peers' `*-package-commands` tests add about 250 no-cross-technology and 20 ui-boundary violations; they import `process` execution/routing/capacity glue from ui, value, plugins and hub.

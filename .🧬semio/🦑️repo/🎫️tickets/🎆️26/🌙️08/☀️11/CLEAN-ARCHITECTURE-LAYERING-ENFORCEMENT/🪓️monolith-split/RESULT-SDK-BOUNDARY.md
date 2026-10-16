# Result: SDK Boundary (slice SDK-BOUNDARY, prefix `sdkb`)

Status: neutral-module part DONE, product part (about 520 repo-lib edges, about 250 os edges) NOT SDK-able and handed off (section 5). The exact-command cruiser numbers are in section 7.

Target: rule `plugins-framework-sdk-only` (`DEPENDENCY-DIRECTION-AUDIT-2.md` section 3, P1): 1,763 cruiser edges, 1,235 outside tests, at 04:59.
Tools: `sdkb-tools/` (this folder). Logs and raw outputs: `🗑️generated/sdkb-*` (kept).

## 1. Verdict

- 1,875 plugin import statements resolved into `🧰️framework/` without hitting an SDK export target at 05:20. 1,180 of them reached neutral framework modules. All 1,180 now go through `@semio-tech/framework/<subpath>` or, for bootstrap entries, through a relative path to the SDK facade.
- Plugin-side violating import statements: 1,875 before, 799 after. Of the 799, 23 are in layout artifacts (16 neutral-module and 7 product imports; another slice owns them) and 776 reach product code (`🛍️products/🦑️repo`, `🛍️products/💻️os`, presentation). No neutral-module import is left outside layout.
- Exact cruiser command: `plugins-framework-sdk-only` 1,763 to 725 edges (1,235 to 569 outside tests); `no-cross-technology-✏️s-to-🧰️framework` 1,807 to 470. Every remaining sdk-only edge points at product code or at layout-owned files (section 5).
- The SDK grew from 30 to 125 `exports` entries (95 new facade files, 348 explicitly named re-exports, 0 `export *`).
- Two regressions were caught by running tests and fixed (section 4.3). Test and type-check failure sets are identical before and after.
- One policy line changed (section 3.3): `no-cross-technology-*` no longer fires on workspace-aliased (package specifier) imports.

## 2. Found state and rule semantics

No cut predecessor (new slice). Baseline = audit 2 (`dda2-*`, 2026-10-16 04:59).

- The rule is `pluginsFrameworkSdkOnlyRule` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`. It flags every edge from `^✏️s/🔌️plugins/` to a resolved path inside `🧰️framework/` (or to a framework workspace package) unless the target is an authored `exports` target of the package named by `taxonomy.dependencyDirections.publicApi.sdkPackage`. That package is `@semio-tech/framework` (`🧰️framework/📦️packages/🟦️typescript`).
- `publicApi.toolingRoles` must be `[]` (the loader throws otherwise), so `📜️script.ts`, tests and stories get no exemption. Bootstrap entries are exempt only from the `no-cross-technology-*` family.
- `authoredPackageExportAuthority` accepts only export targets that are files inside the package directory (no `..`). The public surface can therefore only be facade files in the SDK package. The 30 existing entries (`./process/*`, `./schema`, `./filesystem`, `./mesh/polygon`, `./value/*`) follow exactly that pattern.
- The SDK package is role `framework-neutral` (`framework-no-products`). A facade must not re-export `🛍️products/**`. This is what separates SDK-able code from product code in section 5.
- Specifier resolution: `enhancedResolveOptions` use `exportsFields: ["exports"]`. `@semio-tech/framework/<sub>` resolves to the facade through the root `node_modules/@semio-tech/framework` workspace link (verified with a real cruiser run, section 7, and with Vite and bun, section 4).

## 3. SDK design (recorded before the plugin edits)

### 3.1 Principles

1. One entry per framework module that plugins consume. Facade path mirrors the module taxonomy: `🧰️framework/📦️packages/🟦️typescript/<module path>/🟦️.ts`, `exports` key `./<module path without emoji>`. A module's own `📦️packages/🟦️typescript` segment is dropped.
2. A facade holds only explicit `export { … } from` and `export type { … } from` lines. No `export *`, no code, no renames (except JSON default re-exports). The name set is exactly the union of what plugins import today (AST scan), so every name has a real consumer. Value versus type is decided with the TypeScript checker, including type-only re-exports of upstream barrels (`isolatedModules` safe).
3. Test helpers sit under a `testing` subpath. The facade folder is `🧪️testing` and never `🧪️tests`, so test discovery cannot collect facades. React testing-library re-exports are a separate entry (`./ui/targets/react/testing`) because importing it registers test hooks.
4. Schema-first assets that plugins import are named exports of their module entry: JSON schemas as `<module>Schema` (default import becomes a named re-export of the JSON default), shared conformance fixtures as `<module>Fixture` under `testing`. The portable SQLite metadata DDL is `sqliteSnapshotMetadataSql` from a new TypeScript accessor beside the `.sql` file.
5. Reuse before creating: if an existing facade already re-exports the same declaration (symbol identity, not name equality), the codemod points there and creates nothing. Existing facades are extended only with missing names (`./process/routing` got `createIssuedResourceControlV1`).
6. The root barrel `.` is never a codemod target (it loads the whole framework). The 82 existing plugin imports of `@semio-tech/framework` stay valid.
7. Bootstrap entries (`📜️script.ts`, `🏗️builder/🌐️vite/🟦️.ts`, `🧪️tests/🎚️config/🟦️.ts`, `vite|vitest.config.*`) keep relative paths to the SDK facade file. They run before `bun install` can be assumed and are exempt from the cross-technology rule. All other plugin files use package specifiers.

### 3.2 Surface (95 new entries; 348 named re-exports)

Counts are `v` values, `t` types, `json` schema or fixture defaults.

- `2d`: `2d` (4v+4t), `2d/booleans` (1v+1t), `2d/booleans/paths` (1v+2t), `2d/booleans/paths/schema`, `2d/booleans/schema`, `2d/compute/testing/consumer` (+`/execution`, `/host`), `2d/path/flatten` (+`/schema`), `2d/path/stroke`, `2d/retire` (+`/schema`), `2d/text`, `2d/text/font` (2t), `2d/text/font/catalog` (4v+2t), `2d/text/font/run`, `2d/text/font/shape/unicode`, `2d/trace` (+`/schema`)
- `3d`: `3d` (3v+8t)
- `compiler`: `compiler/syntax/module-tree`, `compiler/syntax/rust` (4v+1t)
- `graph`: `graph/manifest/execution` (3v), `graph/manifest/value` (6v+1t), `graph/manifest/value/schema`
- `hash`, `identity/grapheme`, `interaction` (2t), `canvas/runtime-assets`, `action-bus/wire-retirement/testing/wire-retirement`
- `io`: `io/base64`, `io/ports` (pre-provisioned for layout), `io/schema` (4v+1t+json), `io/sqlite-snapshot` (10v+5t), `io/sqlite-snapshot/artifact` (17v+2t), `.../artifact/keys`, `.../ieee754` (5v+2t), `.../ieee754/testing`, `.../refusal/testing`, `.../schema` (DDL accessor), `.../schema/relational`, `.../testing/semantic-extent`, `io/text/ieee754`, `io/text/artifact-reference/binding/testing`
- `job/reconcile/schema`, `kernel` (5v+1t), `kernel/clipboard` (+`/schema`), `machine` (8v+8t), `manifest` (3v+7t), `manifest/media`, `mesh-engine/schema/polygon`, `pack/value/testing/preflight/pack`, `replication` (+`mutation/map/schema`), `surface/paint/testing`, `tool-machine/schema/node-graph-edit-rows`
- `pixels`: `compositing` (+`/frames`, `/layers`), `coverage`, `editing`, `image/decode` (+`/schema`), `png/encode`, `sampling/affine` (+`/testing/oracles`), `selection`
- `schema`: `schema/artifact-reference` (2v+2t+json), `schema/oracles/document`, `schema/oracles/validator`, `schema/record` (+`/testing/unit`)
- `test`: `test/adapter`, `test/gherkin`, `test/mutation/inventory/execution`
- `ui`: `ui/scene` (+`/testing`), `ui/styling`, `ui/targets/react` (45v+19t), `ui/targets/react/testing` (5v), `ui/viewport/2d/schema`, `ui/viewport/3d/schema`
- `value`: `value/decode`, `value/dictionary` (+`/native-json`), `value/retained-clone/contract`, `value/retained-clone/wire` (+`/schema`), `value/schema` (+`/intrinsic`, `/intrinsic/json`), `value/text`, `value/type`

Entries that no non-layout plugin uses yet because the layout slice still imports them by path: `./io/ports`, `./value/dictionary`, `./value/dictionary/native-json`, `./manifest/media`. They are pre-provisioned so the layout owner can switch with the same codemod.

Candidate for narrowing later: `./ui/targets/react` mirrors the 64 names that CAD's editor renderer takes from the UI React barrel, including re-exported `three` and React values. They are re-exported explicitly (AGENTS: re-export explicitly what the client needs).

### 3.3 Policy decision: `no-cross-technology-*`

Measured with the real cruiser on one file: an import through `@semio-tech/framework/...` gets `dependencyTypes: aliased, aliased-workspace, local, import`. `crossTechnologyRules()` matched `local`, so every SDK import was still reported as `no-cross-technology-✏️s-to-🧰️framework`, contradicting the rule's own comment ("Runtime relative imports must not cross top-level technology folders"). The two rule families could not both be satisfied.

Change (one line, `🏗️construction/🟨️.cjs`): `to` gains `dependencyTypesNot: ["aliased-workspace"]`. Relative imports stay forbidden. Package specifiers are governed by the package-aware rules (`plugins-framework-sdk-only`, `no-cross-plugin-*`, `framework-no-implementation`, the declared role rules). Test: `🚀️bootstrap/🧪️tests/🟦️.ts` asserts the shape on all 20 rules (19 pass, 0 fail with `SEMIO_TEST_ARTIFACT_DIR` set; construction 5/5, pattern-safety 2/2, selection 2/2). Revert this one line if the coordinator prefers to keep the family as is; nothing else depends on it.

## 4. Codemod, batches, resolution

### 4.1 Tools (`sdkb-tools/`)

| File | Purpose |
|---|---|
| `lib.ts` | TypeScript-AST import extraction (import, export-from, dynamic, require, import types) and bundler-style resolution |
| `scan2.ts`, `usage.ts` | violating-import scan and per-target usage table |
| `plan.ts` | derives facades and `exports` from usage using the TS checker (symbol identity, value or type) |
| `apply-surface.ts` | writes facades and `package.json` `exports` (dry-run by default) |
| `rewrite.ts` | rewrites specifiers, re-scans each file right before editing, writes a pre-edit snapshot, skips no-op patches |
| `verify-rewrite.ts` | checks every rewritten file against its snapshot (same line count, only import lines changed) |
| `cruise.sh`, `cruise-batch.sh`, `cruise-chain.sh` | cruiser runs with retry on peer churn (ENOENT, `ENFILE`, duplicate package name) |
| `runtests.ts`, `testdiff.ts`, `tscdiff.ts`, `table.ts`, `errsum.ts`, `ruletable.ts`, `consumers.ts`, `owners.ts` | before/after comparison and reporting |
| `script-smoke.ts`, `facade-load.ts`, `vite-resolve-check.mjs`, `split-react-testing.ts` | runtime and bundler checks, one-off split |

### 4.2 Batches

Dry run first (`DRY: files=806 edits=1540`), then per plugin with a snapshot in `🗑️generated/sdkb-snap/`. Totals: 805 files, 1,539 edits in the 35 plugin batches; 2 stragglers (imports added by peers after the scan); 3 files re-pointed to `ui/targets/react/testing`. `verify-rewrite.ts`: 805 files, 1,451 changed lines, all import lines; the 2 flagged lines are a peer's later edit in a GIS test.

Layout artifacts (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/**`) were skipped as instructed. Their 16 neutral-module imports are left for that slice.

Per-plugin table (cruiser edges before are audit-2 numbers; AST imports are my scan, which counts statements, so the numbers differ slightly; "after" cruiser edges come from the scoped batch runs b1 to b7 in `🗑️generated/sdkb-b*-err.txt`):

| Plugin | Cruiser edges before (audit 2) | AST imports before | AST imports after | Cruiser edges after (batch run) |
|---|---:|---:|---:|---:|
| 🗄️stdio | 582 | 604 | 193 | 187 |
| 🖍️draw | 136 | 139 | 12 | 11 |
| 📕️norm | 120 | 136 | 44 | 44 |
| 🌍️gis | 73 | 81 | 46 | 38 |
| 📐️cad | 59 | 68 | 25 | 22 |
| 🔱️trinity | 56 | 60 | 21 | 17 |
| 🧩️puzzle | 50 | 54 | 109 | 95 |
| 🀄️wfc | 53 | 53 | 28 | 28 |
| 🏗️fem | 37 | 44 | 28 | 21 |
| 📏️layout | 39 | 41 | 28 | 4 |
| 🧱️block | 34 | 41 | 21 | 17 |
| 🖨️raster | 36 | 36 | 5 | 5 |
| 🌀️procedural | 32 | 35 | 13 | 12 |
| 🌊️flow | 32 | 33 | 20 | 19 |
| 💠️lowpoly | 29 | 32 | 7 | 7 |
| 📋️forms | 30 | 32 | 8 | 7 |
| 🪵️sourcing | 30 | 32 | 15 | 13 |
| 📸️remodel | 24 | 30 | 20 | 14 |
| 🪐️space | 25 | 26 | 14 | 14 |
| 📜️imperative | 23 | 25 | 14 | 12 |
| 🏛️architect | 21 | 24 | 12 | 9 |
| 💡️reasoning | 21 | 24 | 10 | 8 |
| 🎞️animate | 22 | 23 | 12 | 11 |
| 🎬️sequence | 20 | 22 | 11 | 9 |
| 🏙️bim | 21 | 21 | 3 | 3 |
| 🔋️energy | 19 | 20 | 9 | 7 |
| 🕸️dag | 18 | 20 | 10 | 9 |
| 🗒️note | 17 | 19 | 11 | 9 |
| 🏭️process | 18 | 18 | 8 | 8 |
| ✒️writer | 16 | 17 | 9 | 8 |
| 📖️playbook | 17 | 17 | 9 | 9 |
| 🎥️shooting | 15 | 15 | 8 | 8 |
| ➗️mathematical | 13 | 13 | 8 | 8 |
| 🎪️demonstrator | 10 | 11 | 5 | 5 |
| 🌿️vcs | 8 | 9 | 3 | 3 |
| ��️energy | 0 | 0 | 0 | 1 |
| ��️trinity | 0 | 0 | 0 | 1 |
| ��️stdio | 1 | 0 | 0 | 1 |
| ��️dag | 1 | 0 | 0 | 0 |
| ��️norm | 1 | 0 | 0 | 0 |
| ��️cad | 1 | 0 | 0 | 0 |
| ��️procedural | 1 | 0 | 0 | 0 |
| **total** | 1761 | 1875 | 799 | 704 |

Note on puzzle: the plugin gained 81 new os-dev test imports (`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/`, untracked, created by a peer after my scan). They are product-bucket B4, not caused by this slice. Rows with a mojibake name are rule-name artifacts of audit 2 (same plugin, damaged emoji in the path).

### 4.3 Verification

| Check | Before | After | Result |
|---|---|---|---|
| `tsc --noEmit` on the 806 touched files (+ closure; `🗑️generated/sdkb-tsconfig-touched.json`) | 477 errors | 479 | 0 attributable. `tscdiff.ts`: 1 message reworded (union print order), 2 new errors in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🚪️session/🫴️refusal/🧪️tests/🟦️.ts`, a peer's file (os plugin slice) |
| `tsc` on all 101 planned facades | n/a | 1 diagnostic: the `.sql` import (fixed by the TS accessor), 1 pre-existing foreign error | clean after fix |
| `bun test` 14 pilot files (11 plugins) | 131 pass, 42 fail | 131 pass, 42 fail | identical per file |
| `bun test` 54 wide files (23 plugins, 3 smallest per plugin) | 601 pass, 163 fail | 601 pass, 163 fail | identical per file after the two fixes below |
| Edited `📜️script.ts` (58) loaded with no arguments | n/a | 0 module-resolution failures | all fail only at "original parent Script process invocation required" |
| All 123 non-root SDK entries imported by specifier in fresh bun processes (`facade-load.ts`) | n/a | 123 ok, 0 fail | |
| SDK surface law test (new, section 4.5) | n/a | 5 pass | |

Regressions found by running tests (both fixed, both would have passed the cruiser):

1. `.sql` text import: bun ignores import attributes on `export … from`, so `export { default as X } from "x.sql" with { type: "text" }` yielded a file path, not the text. Replaced by a TypeScript accessor `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🟦️.ts` (`sqliteSnapshotMetadataSql`, reads the same `🗄️.sql` the Rust side includes) and one plugin support file re-pointed.
2. A namespace import used through a cast (`(intrinsic as Record<string, any>).parseIntrinsicValueControlled`) had its member hidden from the static scan. `parseIntrinsicValueControlled` was added to `./value/schema`. The scan had flagged `membersUnknown` for that import; I had checked it with a grep that missed the cast. Lesson recorded: treat `membersUnknown` as blocking.

### 4.4 Bundling and resolution

- Vite string aliases match by prefix. `{ find: "@semio-tech/framework", replacement: <barrel file> }` turns `@semio-tech/framework/io/sqlite-snapshot` into `<barrel>/io/sqlite-snapshot`. Proven with a real Vite plugin container (`sdkb-tools/vite-resolve-check.mjs`): old alias resolves `🟦️.ts/io/sqlite-snapshot`; the exact-match alias and no alias both resolve to the facade.
  Fixed in four configs by `find: /^@semio-tech\/framework$/u` (the repo Vite builders are loaded with Node strip-only TS, a regex literal is fine):
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`, `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts`, `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts`, `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts`.
  Storybook's alias record cannot hold a regex, so the two `"@semio-tech/framework": …🟦️.ts` entries in `.storybook/📖️stories/🧭️coordination/🟦️.ts` were removed (node resolution gives the same root file; verified in the "no alias" case above).
- bun: resolves through `node_modules/@semio-tech/framework` (workspace link) and the `exports` map; proven by the tests above.
- tsconfig: root `moduleResolution: bundler` resolves `exports`; per-package `paths` entries that name `@semio-tech/framework` exactly (quiz packages) do not shadow subpaths.
- Not touched and still correct: `createWorkspaceViteResolveConfig` aliases (other package names), cad vitest config alias of `@semio-tech/framework-3d-js` (now unused by plugin code, harmless).

### 4.5 New test and registration

`🧰️framework/📦️packages/🟦️typescript/🧪️tests/🔌️sdk-surface/🟦️.ts` plus language-agnostic cases in `🧫️fixtures/🔣️.json` (10 facade-source cases). It asserts: every export target is a file inside the package; every non-root facade is explicit re-exports only (fixture cases classify sources; the checker is the independent oracle); the TypeScript program has no diagnostic on any facade (names exist, values are values); every entry loads by specifier in a fresh process. Result: 5 pass, 388 expects, 26 s.
Registered as `test-sdk-surface`: `📜️script.ts` (`SdkSurfaceScript`), `📋️project.json`, `package.json` script, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (entry `🧪️test🧰️framework🔌️sdk-surface`, group `9_gates`, order 900.0585175). `bun repo run @semio-tech/framework:test-sdk-surface` itself was not run (the launcher route is unreliable under the current machine load).

## 5. What is left, and why (not SDK-worthy)

Remaining plugin-side violating statements (scan4, 799 in 7,067 plugin TS files):

| Bucket | Edges | Files | Why not SDK | Proper design |
|---|---:|---:|---|---|
| B1 repo library tooling in plugin `📜️script.ts` and tests: `runArtifactRustPackageMain` 104, `runArtifactTypeScriptPackageMain` 81, `runRepositoryMutationBridgeScriptV1` 42, `runCmd`/`getWorkspaceRoot`/`runVitest`/`runCargo`/`cargoTargetDirectory`… 253 | 480 | about 330 | `🛍️products/🦑️repo` is a product; the SDK is `framework-neutral` and `toolingRoles` must stay empty | Extract the runner kernels into `🔨️modules/🏃️process` (the neutral half already lives in `🏗️native-build`, `🕸️wasm-build`, `routing`) and inject the product parts (cargo admission, cache storage, network policy: `repositoryCargoNetworkV1`, `runRepositoryCommand`, `buildRepositoryCargoArtifacts`) as ports implemented by the repo product. Then add `./process/artifact-*` entries and rerun this codemod. Owner: repo build infra. `getWorkspaceRoot` (40) is a smaller candidate: the SDK already has `findWorkspaceRoot` with slightly different fallback semantics |
| B2 os store schemas `🏪️store/🪆️child`, `🔗️link`, `📦️blob` (`ArtifactChild`, `parseArtifactChild`, `contentId`, `ArtifactLink`, JSON schemas) | 82 | 67 | pure schema modules (they import only `🔏️hash` and `🗿️artifact-reference`) but located in the os product; a facade would add `framework-no-products` errors | Move the TS+JSON+GraphQL+proto schema triples next to `🧬️schema/🗿️artifact-reference` and re-point the Rust twin. 191 files reference the old path (Cargo, build owners, schema catalog), so this needs the Store owner |
| B3 os renderer and infinite-world hosts (`World3dHost`, `Canvas2dHost`, `Board2dHost`, `UiDocumentStore`, r3f world kit) used by plugin stories, tests, wasm session loaders | 66 | 39 | host components of the os renderer | renderer injects hosts through the UI contract (audit P2); plugins should render declarative UI nodes |
| B4 os dev test kits (`🧑‍💻dev/🧪️tests/🧪️time-travel/**`) | 80 | 10 | os-internal test kit; 81 of these appeared from a peer's new untracked puzzle tests after my scan | publish the kit through a test entry once it is stable |
| B5 repo test kit `@semio-tech/repo-test` (`currentPlatform`, `committedArtifact`, `digest`) | 39 | 36 | repo product | same as B1 (neutral test-artifact helpers into `🏃️process` testing) |
| B6 os other (store worker, kernel types, inference service, mcp, plugin fault, pack, directory) | 26 | 16 | os internals | per-case: most are fixtures or tests that belong in the os tree |
| B7 presentation product | 3 | 2 | product | `@semio-tech/framework` has no presentation entry; belongs to the presentation owner |
| L layout artifacts (other slice) | 23 | 10 | skipped by instruction | rerun the codemod there: all 16 neutral imports already have SDK entries |

Also visible but not an import edge, so not counted by the cruiser: 48 `new URL(...🧰️framework...)` or `readFileSync(...🧰️framework...)` source-text reads in plugin tests (fixtures, schema JSON, `.rs` text). They need a read API (`readModuleTreeSource` or an SDK fixture entry) in a follow-up.

The 112 `no-cross-package-relative-*` edges that originate in the new facades (hygiene family P11): every SDK facade imports module files that belong to a workspace package whose `exports` do not list them. This is structural (existing facades show the same: 28 in the same run). Before this slice the same deep modules were reached directly by the plugin files, so each facade edge replaces many plugin edges; the family total falls (section 7). The long-term fix is the second tier: each module package exports its public entry files and the SDK facade re-exports those (P11).

## 6. Foreign changes and errors observed

- `tsc`: 2 new errors in the os plugin tests tree (peer, os plugin slice). Pre-existing: `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts(77,183)` TS2345.
- Cruiser churn: `ENOENT` on files deleted or renamed by peers (`🧪️test/🟦️.ts`, `🎠️kernel/🫧️transient/📜️script.ts`, `✏️s/🧑‍💻dev/…`), `Dependency policy package name has distinct owners` during the `rtv-*` rename (`@semio-tech/repo-test`, `repo-vscode`), and `ENFILE: file table overflow` under fleet load. All handled by retry in `cruise*.sh`; none is caused by this slice.
- `bun test <path>` without a leading `./` treats the path as a filter and scans the whole repo (2 GB, segfault). Use `./`-prefixed paths.
- Shared files edited concurrently by peers while I worked: `📜️script.ts`, `package.json` and `📋️project.json` of the SDK package (a peer added `test-io-ports`), `.vscode/launch.json`, the os dev Vite builder. My edits were anchored and re-read.

## 7. Cruiser, exact command

Command (audit 2 section 1, unchanged): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (alarm 2400, run by `sdkb-tools/cruise.sh after full`).

- After: exit 163 (13,475 error lines mod 256), 446 s, 17,149 modules, 46,531 dependencies. Raw: `🗑️generated/sdkb-after-err.txt`; tables: `sdkb-ruletable-after.txt`, `sdkb-origin-delta.txt`.
- Before: audit 2 at 04:59 (16,421 errors, 16,748 modules, 44,465 dependencies, same command, `🗑️generated/dda2-depcruise-err.txt`). A fresh "before" run on the untouched tree was not obtainable: two attempts died on peers deleting files mid-run (`ENOENT`), then on a duplicate package name during the `rtv-*` rename, then on `ENFILE`; by the time the machine was stable I had to start editing. The AST scan at 05:20 (1,875 violating statements) and the unchanged audit numbers bracket the starting point.
- Peers changed other areas between 04:59 and 06:46, so only the rows whose origin is `✏️s/🔌️plugins` or the SDK package are attributable to this slice. The origin-split diff (`sdkb-origin-delta.txt`) is the attribution.

Rules this slice moves (total error lines / outside `🧪️tests`):

| Rule | Before | After | Note |
|---|---:|---:|---|
| `plugins-framework-sdk-only` | 1,763 / 1,235 | **725 / 569** | all origin `✏️s/🔌️plugins`; remaining edges are the buckets of section 5 |
| `no-cross-technology-✏️s-to-🧰️framework` | 1,807 / 997 | **470 / 180** | package imports no longer match (section 3.3); the rest are relative imports into products, layout files and non-plugin `✏️s/🧪️tests`, `✏️s/🧑‍💻dev` |
| `no-cross-package-relative-*`, origin plugins | 3,180 | 2,084 | plugin to deep module files replaced by facade imports |
| `no-cross-package-relative-*`, origin SDK package | 20 | 132 | the new facades (section 5); net −984 across both origins |
| `not-to-unlisted`, origin plugins | unchanged | unchanged | no new unlisted dependency from `@semio-tech/framework` imports |
| `no-cross-plugin-*` | 4 / 3 | 0 / 0 | peers (RESULT-PLUGIN-CROSS-EDGES wave 3) |

Rows changed by peers or untouched by this slice (for completeness): `no-circular` 626 to 715 (origin framework +123, none through a new facade: checked by path), `no-core-path` 880 to 962, `renderer-hosts-only-ui` 661 to 707, `framework-no-products` 131 to 147 (none from the SDK package), `ui-no-framework-packages` 256 to 260, `no-state-outside-os` 86 to 93, `plugin-no-extension-or-artifact-🗄️stdio` 0 to 4 (a peer's new `🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts`), `no-cross-technology-🌎️hub-to-✏️s` 16 to 0, total error lines 16,421 to 13,475.

Scoped batch runs (same config, roots = the batch plugins plus the SDK package, `sdkb-b1..b7-err.txt`): sdk-only edges after = 28, 111, 73, 172, 122, 11, 188 (sum 705). b4 contains puzzle's 81 new os-dev test imports (peer).

## 8. Commands run

| # | Command (abbreviated) | Result |
|---|---|---|
| 1 | `bun sdkb-tools/scan2.ts` (AST scan of 7,055 then 7,067 plugin files) | 2,614 framework imports, 739 already allowed, 1,875 violating; after: 2,696 / 1,897 / 799 |
| 2 | `bun sdkb-tools/plan.ts` (TS program of 2,374 files) | 112 targets, 101 facades; 1 problem (namespace members, resolved by hand, later fixed again after a test) |
| 3 | `bun sdkb-tools/apply-surface.ts --write` | 93 facades created, 95 `exports` added, 0 collisions |
| 4 | `tsc -p sdkb-tsconfig-facades.json` | facade diagnostics: `.sql` TS2307 (fixed), 1 foreign error |
| 5 | `bun sdkb-tools/rewrite.ts --write` per plugin (35 batches) | 805 files, 1,539 edits; `verify-rewrite.ts` clean |
| 6 | `cruise-batch.sh b1..b7` and `cruise.sh after full` | section 7 |
| 7 | `bun sdkb-tools/runtests.ts` before and after (14 pilot, 54 wide files) | identical failure sets |
| 8 | `tsc -p sdkb-tsconfig-touched.json` before and after | 477 and 479, 0 attributable |
| 9 | `bun sdkb-tools/script-smoke.ts` (58 edited scripts) | 0 resolution failures |
| 10 | `bun sdkb-tools/facade-load.ts` (123 entries) | 123 ok |
| 11 | `node sdkb-tools/vite-resolve-check.mjs` | old alias breaks subpaths, new alias and no alias resolve |
| 12 | `bun test ./…/🚀️bootstrap/🧪️tests/🟦️.ts` (+ construction, pattern-safety, selection) | 19 pass; 5; 2; 2 |
| 13 | `bun test ./…/📦️packages/🟦️typescript/🧪️tests/🔌️sdk-surface/🟦️.ts` | 5 pass, 388 expects |
| 14 | `bun sdkb-tools/consumers.ts` | 125 entries; 4 new ones layout-only, rest used |

## 9. Files changed

- SDK package `🧰️framework/📦️packages/🟦️typescript/`: `package.json` (95 new `exports`, `test-sdk-surface`), 95 new facades (`<module path>/🟦️.ts`), `🏃️process/🧭️routing/🟦️.ts` (1 name), `📜️script.ts`, `📋️project.json`, `🧪️tests/🔌️sdk-surface/{🟦️.ts,🧫️fixtures/🔣️.json}`.
- Framework module: `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🟦️.ts` (new).
- Policy: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs` (+1 line), `🚀️bootstrap/🧪️tests/🟦️.ts` (+1 assertion).
- Bundler: 4 Vite builders (section 4.4), `.storybook/📖️stories/🧭️coordination/🟦️.ts` (−2 lines).
- Registration: `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (1 entry each).
- Plugins: about 810 files under `✏️s/🔌️plugins/**` (import specifiers only; list in `🗑️generated/sdkb-snap/*.json` keys and `sdkb-touched-files.txt`).
- Tools and result: `sdkb-tools/**`, this file.

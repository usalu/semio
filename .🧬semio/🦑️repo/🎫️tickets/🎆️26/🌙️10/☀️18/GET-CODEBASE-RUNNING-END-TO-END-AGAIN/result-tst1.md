# Result tst1: TypeScript suites (course and repo library)

`C` = this ticket folder. No manifest (`package.json`, `bun.lock`) was touched (freeze respected). No cargo ran except one accidental `cargo metadata` inside `workspace-contract` (see below). No modifying git command.

## 0. Blocker: `bun repo run` never got through

- `bun repo run @teaching/architecture-quiz:test` and `repo:test-javascript-storage` fail after about 8 min with `Quiescent pre-pin cache storage admission required`. Cause: `prepareNxGraphCacheStorageV1` (nonwaiting EX on the `graph` image) while `.cache/b/g/m/nx/cache` does not exist yet (first-time pool creation), and any other `bun repo` process (the coordinator's 80-min `show projects`, the bzl `--tool=bazel run --dry-run` loop, `dependencies lock --all`, cc's `repo cargo check`) holds the graph or dependency partition.
- Last attempt (13:00) was still in the loading stage after 27 min (installer work), no verdict.
- Scripts cannot run without the CLI (`Original parent Script process invocation required`). So every verdict below is a DIRECT run: vitest / `bun test` / tsc, with the missing packages emulated (see 2). CLI-only suites are listed in 5.
- Tools: `C/tst1-tools/` (`run-lib-test.sh`, `quiz-vitest.config.ts`, `quiz-tsconfig.json`, `lib-tsconfig.json`, `runtime-tsconfig.json`, `store-paths.json`, `missing-root-declarations.json`, probes).

## 1. Systemic finding (for the deps executor)

Since commit 693 `bunfig.toml` uses `linker = "isolated"` with no hoisting. Files that sit outside a package directory (all of `📚️library/**` except `📦️packages`, `🎓️teaching/**/🧪️tests`, `🖱️ui/🎯️targets/⚛️react/🌐️i18n`, ...) only resolve what root `package.json` declares. Undeclared (or declared only in the package) imports are unresolvable: this is the dominant cause of failures. Nothing here is caused by the splits.
Missing, with versions that already sit in the store (`C/tst1-tools/missing-root-declarations.json`):
`jsonc-parser`(59 importers), `minimatch`, `pathe`, `string-argv`, `commander`, `diff`, `yargs-parser`+`@types/yargs-parser`, `ignore`, `smol-toml`, `es-module-lexer`, `mdast-util-from-markdown`, `@noble/hashes`, `zod`, `resolve.exports`, `postcss`, `iconv-lite`, `fflate`, `fast-check`, `yaml`, `immer`, `fast-json-stable-stringify` (a peer already added the last two), `i18next`, `react-i18next`, `@xyflow/react`, `@react-three/fiber`, `@tailwindcss/vite`, `@vitejs/plugin-react`, `@types/react-dom`, `@types/three`, `@types/lodash`.
Not installed anywhere (new installs): `execa` (process-budgets), `postcss-load-config` (tool-configuration-ownership).

## 2. Method

Emulation of the declarations without touching manifests: vitest `resolve.alias` / tsc `paths` / `bun test --tsconfig-override` with `paths` to the `.bun` store dirs (bun test ignores NODE_PATH and plugins). `SEMIO_TEST_ARTIFACT_DIR=<repo>/.🧬semio/🦑️repo/⚡️cache/🧪️tests/tst1` (the default route; the ticket path exceeds the 256-unit capacity of dependency-direction).

## 3. Before / after per suite (direct runs)

| Suite | Before | After |
| --- | --- | --- |
| quiz vitest (catalog, deploy, local-stack, host-document, pet-cast) | without declarations 2 files fail to load (`yaml`, `@tailwindcss/vite`); emulated: 253/254 (1 stale CI-install assertion) | emulated 254/254 |
| quiz tsc | `react-dom` types missing + 8 undeclared modules + 1 real TS error (proctor manifest-closure) | emulated 0 errors |
| quiz `check` | not run (cargo); real defect: `checkQuizCatalog` spawned raw `cargo run` (refused by the ingress gate) | rewritten, see 4 |
| repo-lib tsc | 180 errors (126 TS2307 missing modules) + 19 real | emulated: 5 left, see 5 |
| inference tests | 5/5 | 5/5 (types fixed) |
| readiness tests | 18/18 | 18/18 (types fixed) |
| dependency storage tests | CLI only: direct 16 pass / 25 fail (envelope) | not verifiable without the CLI |
| repo-source-ownership | 5/6 | 6/6 |
| root-artifact-schema-law-source | 10/11 | 11/11 |
| command-source | 9/11 | 11/11 |
| script-async-runners | 1/2 | 2/2 |
| artifact-empty-facet-authoring | 0 (jsonc-parser), emulated 39/43 | 43/43 |
| source-services | 3/4 | 4/4 |
| dependency-direction | 18/18 | 18/18 |
| root-inference-law-source, root-schema-field-source, cargo-transaction-command-source, canonical-execution, crate-tree(119), path-emoji-statutes(37), readme-reviewed-fixture-inputs, registry-import-language | all pass | all pass |
| root-surface-abstraction-law-source | 7/9 | 8/9 (see 5) |
| workspace-contract (18 groups) | 712 pass / 43 fail | 5 groups fixed (below); remaining see 5 |
| transaction-v2 | 0/62 without run identity env; emulated env 58/62 | 59/62 (+ language-neutral golden) |
| taxonomy-pattern-compiler-reuse | 71/72 | unchanged (1 fail, not triaged) |
| frozen-markdown-coordinates | 35/36 | unchanged, pre-existing |
| process-budgets, tool-configuration-ownership | blocked by `execa`, `postcss-load-config` | blocked |

## 4. Changes (file list, relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` = LIB unless stated)

Real defects fixed:
- `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🧪️deploy/🟦️.ts`: CI install step expectation `--scope .` to `--all` (the quiz package is not in the root scope; the workflow already says `--all`).
- `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/🟦️.ts`, `📦️packages/🟦️typescript/📜️script.ts`: `checkQuizCatalog(repoRoot)` no longer spawns raw `cargo`; builds the release proctor via `buildProctor` (cargo launcher) and runs `check` through `launchProctor`; removed unused `cargoTargetDirectory` import and `invocation` parameter. Not run (cargo-backed, cc).
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🕸️manifest-closure/🟦️.ts`: `dependencyTables` typed loop (TS7053).
- `LIB/🏗️authoring/📦️crate-tree/🧰️testkit/🟦️.ts`: defined the missing `ExternalGateReader` type.
- `LIB/🏗️authoring/🗿️artifact-tree/🟦️.ts`: io facet scaffold order kind-major (matches the `artifact-empty-facet-authoring` fixture; the tool was rep-major since 10-06).
- `LIB/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts`: narrow `duplicate` out of `RestateReason` check; its `🧪️tests/🧩️split-leaves` (schema type, `validate(...) as boolean`).
- `LIB/🔍️discovery/🎁️package-projection/🟦️.ts`: `Object.values(... as Record<string, unknown>)`.
- `LIB/⚡️caching/🕸️graph/✅️verification/🟦️.ts`: explicit `BuildTargetV1 | undefined` (circular implicit any made the command-source typecheck law fail).
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/.../🧬️schema/🔣️.json`: title `Rewriting Artifact` to `RewritingArtifact` (title must equal the type name, like `JackArtifact`).
- `LIB/🧪️tests/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json` + `LIB/🧪️tests/🔄️transaction-v2/🟦️.ts`: `builderPath` for the helpers that the P81 split moved.

Stale tests / fixtures after today's splits and rewrites:
- `LIB/🧪️tests/🦑️repo-source-ownership/🟦️.ts` (package barrels may be multi-line), `LIB/🧪️tests/🧫️fixtures/🧱️root-artifact-schema-law-source/🔣️.json` (new source-access exports), `LIB/⚡️caching/🧪️tests/🧫️fixtures/🧱️command-source/🔣️.json` (consumer moved to the inference generator output), `LIB/🧹️normalization/🧪️tests/🏗️source-services/🟦️.ts` (root deleted: assert absence and that no engine owner re-exports owner types), `LIB/🧪️tests/🧪️script-async-runners/🟦️.ts` (assignment counts as bound), `LIB/🧪️tests/🪶️artifact-empty-facet-authoring/🟦️.ts` (`repo:script` executor).
- `LIB/🧪️tests/🔬️workspace-contract/🌐️static-sites-and-boundaries/🟦️.ts` (legacy-alias scan excludes the whole suite directory; renderer host scan covers the barrel children; playground ports now storybook/projektetage/architecture-quiz), `.../📖️taxonomy-loading/🟦️.ts` (`buildTargetRecord` stub, "adjacent exact build project manifest").
- Type-only: inference tests (`as boolean` etc.), readiness tests (`strip` return type, `probe.port as number`), `LIB/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative` (stale `../../🟦️.ts` type import), dependency storage `👓️reading` test (`expect<string|null>`), `LIB/🧪️tests/❄️frozen-markdown-coordinates` (leftover `discovery.` namespace).

## 5. Still open

- CLI-only suites unverified: dependency storage tests (`repo:test-javascript-storage`), transaction-v2 child-process cases (3 fails, "Plan has unresolved blocking violations": children lack the emulated resolution), everything cached through `bun repo run`, `@teaching/architecture-quiz:check|deploy-check|test-e2e|docker-*` (cargo/docker).
- repo-lib tsc residuals: `@types/yargs-parser`, `postcss-load-config` (deps); `🪞️unmirror` (peer edit in flight: `referencedIdents`, `Universe` members); `🔍️discovery/🕸️runtime/🧪️tests` line 496: `cargoStreamingStatus` now needs `control, storage, sourceBoundary` (cc's launcher API; cargo-backed helper, not changed).
- root-surface-abstraction-law-source: 18 plugin artifacts whose representations drift from the schema (probe `C/tst1-tools/probe/abs.ts`): process3d/generation2d/generation3d/remodeling TS `Artifact` declarations, en1996/en1999/en1993/din16798 Rust/graphql/proto, program/wires diff TS+graphql, puzzle3d Rust. Not caused by today's work.
- workspace-contract remaining (not split related, groups run separately): package-language-handoff (5; test selects 3 WeakMap consts as functions, was failing before), mutation-taxonomy (3) and taxonomy-loading (2): tests still say `io/text/mutations` is undeclared while the taxonomy declares it, generated-outputs (4: missing `dist/dev` outputs), package-discovery (marker role `hub` vs `plugin`), registry-and-cargo-providers (1), ticket-protection (ticket paths from `r7c-tools` beyond the Windows limit), `resolveCargoPackageName` (165 s, ran `cargo metadata`; do not run that group again).
- frozen-markdown-coordinates: the 10-14 commit moved four contracts out of the taxonomy while the seal ledger still lists them (pre-existing). taxonomy-pattern-compiler-reuse: 1 fail at line 187 (not triaged).
- Cleanup: `🗑️generated/tst1-logs` and `.🧬semio/🦑️repo/⚡️cache/🧪️tests/tst1` are deleted at the end.

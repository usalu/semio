# Result c6repo2: repo library TypeScript monolith splits (discovery, normalization, P79, P80, P81)

Executor `c6repo2`. TypeScript only, so there is no crate to compile. All paths below are relative to
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` (`LIB`) unless stated. Tools live in
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c6repo2-tools/` (`F/c6repo2-tools`).

## 1. Generated check (discovery, normalization)

Both files are hand-authored, not generated. Neither has a generated header, no generator writes them, and a text search for
writers finds none. The census "parser signature" heuristic (`validateAgainst` / `(value: unknown)`) is a false positive for both.
Nothing to report as a generator.

## 2. Verdict per file

| File | Before | After | Children | Largest child |
| --- | ---: | ---: | ---: | ---: |
| `LIB/🔍️discovery/🟦️.ts` | 11,929 | removed | 37 sibling dirs, 12,036 lines in total | `🔐️taxonomy-authority` 1,805 (see §6) |
| `LIB/🧹️normalization/🟦️.ts` | 10,787 | removed | 45 sibling dirs, 11,365 lines in total | `🚀️apply` 905 (`applyTaxonomyPlan` is 870 lines) |
| P79 `LIB/📦️packages/🟦️typescript/📜️script.ts` | 1,055 | 901 (kept as the router entry) | 6 command modules under `LIB/🎛️commands/` | n/a |
| P80 `LIB/🧪️tests/🔬️workspace-contract/🟦️.ts` | 7,535 | 38-line suite entry | 18 test groups plus `🏗️shared-fixtures` | 954 (`🧱️taxonomy-normalization`) |
| P80 `LIB/🧪️tests/📦️crate-tree/🟦️.ts` | 1,996 | 12-line suite entry | 12 test groups plus `🏗️virtual-crate` | 374 |
| P81 `LIB/🧪️tests/🔄️transaction-v2/🟦️.ts` | 1,110 | 546 (describe block kept in place) | 3 helper dirs | 297 |

Every new file except `🔐️taxonomy-authority` is under 1,000 lines. The census flagged `discovery` and `normalization` as `PARENT`
owners, and both parents are deleted (no compatibility barrel, no re-export).

Children, discovery (all under `LIB/🔍️discovery/`): `📜️taxonomy-contracts`, `📑️taxonomy-document`, `🧭️workspace-taxonomy-locator`,
`🔐️taxonomy-authority`, `📄️implementation-findings`, `🗂️file-kind-resolution`, `🛠️generator-commands`, `✍️cli-preparation`, `📂️semantic-directory-kinds`,
`🗃️owned-file-projection`, `🧱️path-projection-contracts`, `🪞️path-projection`, `🎯️taxonomy-path-matching`, `⚖️fixed-contract-resolution`, `🧾️schema-scope-catalog`,
`👥️mutation-owner-rules`, `🪪️mutation-owner-identity`, `📦️package-projection-contracts`, `🎁️package-projection`, `✅️owned-file-approvals`, `🦀️rust-fact-contracts`,
`🧷️rust-path-references`, `🔀️rust-non-repo-join-bases`, `🏃️rust-runnable-tests`, `🏛️rust-structure`, `🌐️rust-module-graph`, `🔎️rust-source-identities`,
`🛡️mutation-codec-ownership`, `🪧️owner-markers`, `📇️registry-catalog-inputs`, `🧲️registry-catalog-projection`, `🏷️package-source-classification`, `🛣️ecma-command-routes`,
`🔭️repository-scan`, `🚚️cargo-provider`, `🧵️mutation-metadata-source`, `🧩️semantic-census`.

Children, normalization (all under `LIB/🧹️normalization/`): `📜️plan-contracts`, `🧾️plan-parsing`, `📃️plan-document`, `🏷️file-kind-matching`, `📸️repository-capture`,
`📏️fixed-contracts`, `🔗️reference-inventory`, `📥️incoming-references`, `🔤️reference-tokenizers`, `🧷️typescript-reference-authority`, `🧮️reference-digests`, `🔬️reference-analysis`,
`🦀️rust-reference-graph`, `✂️reference-edits`, `🧭️reference-resolution`, `🧱️inventory-leaf-stamps`, `🕸️reference-graph`, `🪞️artifact-reference-projection`,
`👥️mutation-projection`, `🗺️artifact-projection-catalogs`, `📂️owned-file-projection`, `🏭️generator-inputs`, `🔍️inventory-planning`, `➰️symlink-targets`,
`🪢️symlink-target-edits`, `❄️frozen-evidence`, `🛫️embedded-roots`, `🗃️affected-state`, `🧠️planning`, `🌳️tree-digests`, `✅️verification`, `💾️durable-journal`,
`🔄️journal-reconciliation`, `🔐️transaction-lease`, `🔑️transaction-lease-acquisition`, `🧐️preimage-assertions`, `🗑️removal-references`, `🚿️transaction-cleanup`, `🩹️recovery-authorities`,
`🛟️recovery`, `♻️projection-staleness`, `⏪️rollback-reconciliation`, `↩️rollback-transaction`, `🏛️repository-authority`, `🚀️apply`.

P79 command modules moved out of the package directory: the normalizer law `package-implementation-file` ("Package implementation must move to the
library domain") flagged them under `📦️packages/🟦️typescript/`, so they live in `LIB/🎛️commands/{🪁️typecheck,🕸️dependency-lint,🗂️schema-read-inventory,
🖼️repository-ui,🧱️fixture-ownership,🗺️general-execution-ownership}/🟦️.ts`, next to the existing `🧹️code-style`, `🧪️test`, `✅️verify` command modules.
`📜️script.ts` keeps `TestScript` (an 866-line class that cannot be split losslessly), the router and the entry statement.

Test suites keep their original entry path, so every runner command, `-t` filter and `bun test <path>` string stays valid:
`🟦️.ts` of `workspace-contract` and `crate-tree` is a pure side-effect-import entry (plus the original side-effect imports);
`transaction-v2/🟦️.ts` keeps the `describe` block and the 14 title-counted `test(` calls the harness regex counts.
The crate-tree group files wrap their tests in the original `describe("artifact crate carve", …)` name, so test ids are unchanged.

## 3. Method

`F/c6repo2-tools/{lib,split,rewrite,check,sites}.ts` (TypeScript compiler API 5.9.3, copied to `/Users/ueli/.cache/c6repo2/typescript`):

1. Parse into top-level units; declaration merges and cross-unit assignment of top-level `let` are glued; the dependency graph is resolved with the checker
   (scope-accurate, no name matching).
2. Strongly connected components are atomic; user-named semantic groups are scheduled so that the module graph is acyclic by construction
   (fragments are merged or named when a group had to split). Result: no import cycle among the new children.
3. Per child: only the needed imports (explicit named, `type` kept), `export` added only to declarations used across children, relative specifiers
   (static, dynamic `import()`) re-based, `import.meta.dir`/`url` in the workspace-contract suite re-based through one exported `contractRoot`/`contractUrl`.
4. Importer rewrite: 135 importers of discovery and 30 of normalization were edited by AST (named imports split by new owner, namespace imports and
   `await import()` namespace variables converted); `export …from` re-exports of the old parents were resolved to their origin. 19 residual sites were handled by hand.
5. Before any file was written, the generated children were type-checked in a virtual overlay (0 diagnostics).

## 4. Oracles run (all static or local, no cargo/nx/bazel/`bun repo` build)

| Oracle | Result |
| --- | --- |
| Virtual-overlay TS check of every child before writing | 0 diagnostics (codes 2304/2305/2307/2339-on-modules/2614/2724 and syntax) |
| TS check of 326 touched files after writing (`checkfiles.ts`) | 0 diagnostics |
| `bun build --packages=external` of 326 touched entry points | links; only failures are 6 pre-existing/unrelated (3 old parents/roots removed, `crate-tree/🗺️plan` and `📚️library/🟦️.ts` moved by executor `c6repo`, `package-integration` assigns an import in `shard-client`) |
| Load of all 82 new discovery/normalization children with the bun runtime | 93 ok (82 new + 11 others), 3 fail on pre-existing bare `@semio-tech/compiler-syntax-ecma-ts` (not mine) |
| Lossless statement multiset (original vs union of children) | discovery: 621/622 identical, the 1 difference is the intended `__dirname` depth edit; normalization: 571/572, the difference is the intended `new URL("../🧵️digests/…")` edit |
| Old monolith vs new children, `inventoryTaxonomy` + `planTaxonomy` (old parents restored for minutes, then removed) | identical SHA-256 of entries, violations and plans on `crate-tree`, `discovery/🧬️schema` and `workspace-cleanup`; identical entries and violations on the whole library (4,665 entries, 174 violations); the whole-library plan comparison was stopped after more than ten minutes |
| The repo's own taxonomy inventory on my scopes | 0 new violations (naming, emoji kinds, path length); remaining violations are pre-existing, see §6 |
| `bun test` crate-tree | 116 pass, 0 fail (identical test count to the original) |
| `bun test` of 60 touched suites (individually, 100 s cap each) | 13 fully green (e.g. `taxonomy-scaling-laws` 8, `run-vitest-config-argument-tokens` 9, `nx-workspace-root-file-reference` 7, `cargo-target-discovery-skip` 2, `rust-mount-emitters` 1, `plugin-publication-source-ownership` 7); the rest stop on a missing package or missing `SEMIO_TEST_ARTIFACT_DIR`, or on the pre-existing failures in §6 |
| `bun repo --help` after each stage (discovery, normalization, script, tests, configs) | boots every time, relative imports only; the library `node_modules` is a dangling symlink until executor `deps` finishes |

## 5. Text readers and configuration re-pointed

Importers rewritten by AST: 135 files for discovery, 30 for normalization, 1 for the removed library-root `export *` (see §6 coordination).

Tests that read the old parents as text or load them by path, re-pointed in the same slice (declarations are found by AST in the owning child(ren)):
`taxonomy-pattern-compiler-reuse`, `artifact-empty-facet-authoring` (identity pins now enumerate the leaves), `artifact-empty-facet-authority`,
`rust-finite-target-consumption`, `cargo-discovery-exclusions`, `nested-cargo-collision-authority`, `taxonomy-leading-grapheme`, `registry-catalog-gitlink-boundary`,
`rust-physical-reference-context`, `cargo-target-discovery-skip`, `root-script-compiler`, `rust-divergence-callback`, `registry-import-language`, `frozen-markdown-coordinates`,
`historical-document-evidence`, `fixture-payload`, `reference-coordinate-progress` (the unused discovery pin was dropped), `readme-move-source-authority`, `transaction-recovery-authority`,
`typescript-path-collection`, `bare-reference-sibling-precedence`, `rust-writable-path-authority`, `reference-reach`, `planner-blockers`, `run-vitest-config-argument-tokens`,
`nx-workspace-root-file-reference`, `json-reference-owner-lookup`, `gherkin-description-inline-code`, `markdown-inline-references`, `reference-coverage-selection`, `preflight-reference-basis`,
`transaction-sentinel-policy`, `path-emoji-statutes`, `taxonomy-scaling-laws`, the renderer `package-integration`, the dev `🎮️session` and `browser-ownership` tests,
`caching/🔏️receipt`, `descriptor-verification`, `rust-mount-emitters` (emitter list), `workspace-contract` (self-inspection split across group and shared fixtures).

Shared helper `LIB/🧹️normalization/🧪️support/🏗️source-services/🟦️.ts`: the callers now pass the normalization **directory**; new
`NORMALIZATION_ENGINE_OWNERS` and `normalizationEngineDeclarations` (declarations without `export`, dependency order); `normalizationSourceFiles/Declarations` enumerate the 45 engine owners instead of the root file.

Configuration and data re-pointed: `nx.json`, 14 `📋️project.json` (discovery/normalization inputs are now `**/*.ts` globs; the library package project also gets the moved
command modules, the test sub-suites and the explicit `🚀️apply` / `🔐️taxonomy-authority` entries the cargo-transaction law requires), `🔣️taxonomy.json` (two `inputPatterns`
arrays: wgpu-frame-worker and jco-package-adapter, now `*/🟦️.ts` globs, re-sorted in UTF-16 order), `🏗️build/🔣️policy.json`, fixtures `root-artifact-dependency-source`,
`readme-reviewed-fixture-inputs` (copies list now the 37 + 45 leaves), `html-source-pair`, `config-graph` (deny list), flow `ownership`, `cargo-transaction-command-source`,
`tool-configuration-ownership`, `discovery-schema-handoff`, `root-clean-scaffold-source`, `root-taxonomy-workflow-source`, the transaction provenance identity paths, the
browser-bundle first-party source list (`repo/normalization/<owner>/🟦️.ts` for every engine owner) and four doc-comment references.

## 6. Open risks and facts for the coordinator

1. **`🔐️taxonomy-authority` is 1,805 lines.** It holds `validateTaxonomy` (1,281 lines, not splittable losslessly) and its 23-unit mutual-recursion component
   (`loadTaxonomy` → `loadCatalogTaxonomy` → `validateTaxonomy` → helpers that call `loadTaxonomy` again; ~450 lines). A cycle-free layout cannot separate them; splitting would need
   a module cycle, which I did not introduce.
2. **Dependency image is missing**, so most suites that import `jsonc-parser`, `minimatch`, `ajv`, … could not run; I verified them by type-check, bundling, load and the old-vs-new
   differential instead. Re-run after `deps sync` goes green: `workspace-contract` (the entry runs all 18 groups; some `-t` filters need `SEMIO_TEST_ARTIFACT_DIR`), `transaction-v2`,
   `taxonomy-pattern-compiler-reuse`, `path-emoji-statutes`, `frozen-markdown-coordinates`, `readme-reviewed-fixture-inputs`, `registry-import-language`.
3. Failures that are **pre-existing and unrelated** (checked against the same code at HEAD): `historical-document-evidence` (the test fixture uses the identical literal for `source` and
   `final`, so no move exists), `rust-divergence-callback` registration rows expect `nx:run-commands` but the project now uses `repo:script`, `lib-wiring-budget` (Rust line budgets),
   `artifact-support-leaf-authority` (fixture vs `🧪️fixtures` directory name drift in the vectors), `package-integration` (`shard-client` assigns an import), the
   `fixed-source-disposition-unresolved` finding for `📜️script.ts` (HEAD has it too), `playground-session` (needs `SEMIO_TEST_ARTIFACT_DIR`).
4. **Behavior-visible choices.** `taxonomy-pattern-compiler-reuse` now types one test per owner file (more titles than before, because the 1 root became 45 + others);
   `workspace-contract` and `crate-tree` register their `describe` blocks in group order (all `-t` names unchanged); wgpu/jco generator contracts list globs
   (`🔍️discovery/*/🟦️.ts`, `🧹️normalization/*/🟦️.ts`) instead of the two old file paths (they now also cover the pre-existing children, a superset).
5. **Coordination with `c6repo`.** The library root `🟦️.ts` (P73) carried `export * from "./🔍️discovery/🟦️.ts"`: I removed that line and rewrote its only consumer
   (an `os/renderer` script) to import from the owning children; `c6repo` has since removed the root. `c6repo` also rewrote the `🔣️taxonomy` type imports inside my new
   normalization children after splitting P78 (imports only; checked, no semantic change). I did not touch any `c6repo` or `deps` file other than those anchored edits.
6. Backups of the replaced importers and the old parents (outside the repo): `/Users/ueli/.cache/c6repo2/backup/`. The four kept-parent files (`script.ts`, `workspace-contract`,
   `crate-tree`, `transaction-v2`) were backed up after being overwritten (tool order bug, fixed afterwards); their pre-split text is recoverable from `git show HEAD:<path>`.

## 7. Crates to compile

None (TypeScript only).

# Result c6repo: monolith census 6, repo TypeScript slice (P73 to P78)

`C` = this ticket folder. Nothing was built through `bun repo run`, `nx`, `cargo` or `bazel`. No modifying git command ran. `bun repo --help` was run after every step that touched the CLI import closure and still boots (last run after the final edit). Tools live in `C/c6repo-tools/` (`ast.ts`, `split.ts`, `repoint.ts`, `classsplit.ts`, `run-split.ts`, `plans/*.ts`, `lossless.ts`, `tscheck.ts`); plans for every file are in `C/c6repo-tools/plans/`.

## Verdict

| Pkg | File | Before | After | Verdict |
| --- | --- | --- | --- | --- |
| P73 | `📚️library/🟦️.ts` | 5,182 | deleted; 51 leaf modules, largest 352 | split, barrel removed |
| P73 | `📚️library/📦️packages/🟦️typescript/🟦️.ts` (`export *` glue) | 1 | deleted, `exports["."]` removed from its `package.json` | removed |
| P74 | `🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts` | 1,530 | deleted; 9 children, largest `🧮️pack` 935 (holds the 900-line `planArtifactCratesOnce`, not splittable) | split |
| P75 | `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` | 1,054 | leaf 607 + `🧾️command-inputs` 457 | split |
| P76 | `🎛️commands/✅️verify/🟦️.ts` | 1,410 | leaf 656 (`run`) + 4 abstract lane classes (250, 218, 100, 221) | split |
| P77 | `⚖️laws/…/🔁️live-replay` | 1,156 | leaf 616 + 3 children (max 208) | split |
| P77 | `⚖️laws/…/🗄️artifact-store` | 1,500 | leaf 286 + 3 children (max 548) | split |
| P77 | `⚖️laws/…/🖥️mounted` | 1,337 | leaf 662 + 1 child 665 | split |
| P77 | `⚖️laws/…/🗄️database` | 1,042 | leaf 374 + 2 children (466, 180) | split |
| P77 | `⚖️laws/…/🎯️mutation-outcome` | 1,028 | leaf 313 + 2 children (246, 516) | split |
| P77 | `⚖️laws/…/🗺️taxonomy-shape` | 1,139 | leaf 416 + 3 children (max 321) | split |
| P77 | `⚖️laws/…/🚪️artifact-io` | 1,493 | leaf 39 + 6 children (max 435) | split |
| P77 | `⚖️laws/…/🧬️schema-overhaul-s2` | 1,040 | leaf 125 + 4 children (max 428) | split |
| P78 | `🧹️normalization/🔣️taxonomy/🟦️.ts` | 1,244 | leaf 822 (`parseTaxonomy` + `loadTaxonomy`, parser stays private) + 6 type modules (max 99) | split |

Every produced file is under 1,000 lines (max 934), longest repo-relative path 127 characters.

## P73 layout (new directories under `📚️library`)

`🏗️build/📥️inputs/🧊️stable-file`, `🗂️workspaces/📁️data-directories`, `🚨️policy/{🔖️breach,🖥️cli,🧩️linters,🛡️dependency-boundary,🔒️allowlist,🏃️runner,🧭️entrypoints}`, `🧪️testing/{🎚️levels,🐹️go-plan,🎛️repository-command,📊️coverage,🗄️lcov,🧪️vitest/📋️arguments,🎭️playwright}`, `🏃️process/{🦀️cargo/{🔍️package-names,🧹️lint,🏃️run},👹️daemon,🍞️bun,🧭️routing/🔗️script-url}`, `🎮️playground/{🖥️asset-server,📋️example-argv,🧊️wgpu-dev}`, and the commit family under `🎛️commands/📝️commit/{🌳️git,📊️metrics/{🏷️types,🛤️path-rules,🔢️uloc,💾️scan,🔀️deltas,🎨️format,🧮️summary},🔣️emoji-presentation,👤️contributor,🔢️counter,🔸️bullets,📄️prepared-message,🪝️hooks,🪙️micro-commit,🪜️steps,📋️prepare-commands,🧾️bundle-model,🏷️labels,🔖️tags,🧭️path-prefixes,📅️dates,📈️attribution,📜️body,📤️emit,🔀️run-commit}`. The module graph has no cycles. All 51 modules load under bun and type-check clean (only the `Bun` global noise that the original also had).

Decisions:
- The root was the package barrel plus implementation. A barrel is a compatibility surface, so it is gone. `export *` and name re-exports of other modules (`discovery`, `workspaces`, `process`, `playground`, `builder`, vite, cargo, wasm, routing) were not kept: each importer now imports the owning module directly.
- Dropped legacy aliases `resolveLintScriptEntity` and `runLintScript` (aliases of `resolvePolicyScriptEntity` and `runPolicyScript`; no importer).
- `getLibRoot` still returns `…/📦️packages` (its `__dirname` anchor was adjusted to the new depth). Relative dynamic `import()` specifiers inside moved bodies were re-relativised.
- Orphan docstrings left by earlier partial extractions in the root (documenting removed re-exports) were not carried along.
- Names that came through the four `export *` re-exports and the discovery names were mapped to the module that defines them at write time (discovery is being split by another executor; its current definers were used, the tool `definerIndex` in `exports.ts` can be re-run).

## Importers rewritten

Mechanical (TypeScript AST, one import statement per owning module, named bindings kept verbatim): P73 474 files (including 151 `await import(...)` destructurings and 1 `import("…").Type`), P74 5, P75 1, P76 0 (class stays in place), P77 51 files in total, P78 36. By hand: the `External01` namespace bag in `🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🧰️test-kit` (now spreads `cargoProfileDir` and `getWorkspaceRoot`), the three unused `library = await import(...)` bindings and the `indexPath` in `🧪️tests/🔬️workspace-contract` (now `🏃️process/🟦️.ts`), `🦑️repo-source-ownership` (`getLibRoot`), `🧪️script-async-runners` (async runners are now collected from `export async function` in library sources, because there is no barrel to enumerate), `⏱️process-budgets` (per-symbol module paths for the mocked modules), `🪶️artifact-empty-facet-authoring` (identity list no longer names the deleted entry).

## Text readers and registries re-pointed

- Nx inputs naming the deleted entry or root now use one brace glob over the new families (`{🏗️build/📥️inputs/🧊️stable-file,🗂️workspaces/📁️data-directories,🚨️policy,🧪️testing,🏃️process,🎮️playground,🎛️commands/📝️commit}/**/*.ts`): `✏️s/🧑‍💻dev/🗄️stdio` project and its `🏃️command-ownership` fixture, flow wasm ownership fixture, flow core rust project, `🧑‍💻dev` project, `⚡️caching/📋️project.json`, `📦️packages/🟦️typescript/📋️project.json` (7 targets), `🧪️test/📋️project.json`, `os-dev-composition-ownership` fixture. `test-tool-configuration-ownership` points at `⚡️caching/🌐️vite/🌐️service/🟦️.ts` instead. The two cache-contract inputs gained the new child.
- `🔣️taxonomy.json` generator `inputPatterns` (3 lists): entry/root replaced by seven `**/*.ts` family patterns.
- `🧪️tests/🦑️repo-source-ownership` test and fixture: library `package-reexport` entry and owner chain removed (22 to 21 entries, 3 to 2 package entries, 2 to 1 chains).
- `🎚️tool-configuration-ownership` test and fixture: ESLint probe file, `librarySource` and the input assertion now use existing modules. The fixture token `"--config", opts.config` no longer exists anywhere (stale before this change) and was dropped, `config: string;` is kept on the vite service.
- `🧱️dependency-direction` fixture (11 synthetic paths), print `command-boundaries` fixture, `📊️metrics/🔮️oracles` (`countUnifiedLocForFile` now points at `🔢️uloc`), `🧑‍💻dev` `⚙️config-graph.json` deny list, `adapter-ownership` fixture (`removedExports` row for a file that no longer exists), `🚧️authority-bans` raw-spawn exemption (dead), `⚡️caching/🧪️tests/🧊️native-runtime` guard, one Rust doc comment, `🔌️plugin/🌐️browser-bundle/📜️script.ts` first-party digest (`repo/execution` now digests the stable-file module).
- Kept on purpose as regression guards against a returning barrel: `endsWith("📚️library/🟦️.ts")` in the rust-intent test and the coordinator `forbidden` list. `🧼️remaining-package-purity-authority` holds a historical migration record and was not touched.
- P76: `🏛️canonical-execution` test reads `runCanonicalArchitecture` from `🏁️gate-lanes`; fixtures `root-artifact-schema-law-source`, `root-inference-law-source`, `root-surface-abstraction-law-source` list each owner under the lane file that now imports it.
- P75: `cargo-transaction-command-source` consumer moved to `🧾️command-inputs`.
- P77: `root-inference-law-source`, `root-surface-abstraction-law-source` and `root-schema-field-source` consumers relocated to the files that import their owners.
- P78: `🧹️normalization/🧪️support/🏗️source-services` owner list, `🧪️tests/🏗️source-services` test (closed 6-owner literal and size now 12, `LoadedTaxonomy` and `TaxonomyContentFacts` read from `📚️loaded`) and its fixture (owners with exact dependencies).

## Verification (exact tools)

| Check | Command | Outcome |
| --- | --- | --- |
| CLI boots | `bun repo --help` after every step | prints usage, `stage: complete` |
| CLI closure | 622 files reachable from `🏗️build/📜️script.ts`, TypeScript 5.9.3 from the bun cache over the 363 `.ts` files | no unresolved relative module or missing export; 0 bare `@semio-tech/*` imports |
| Types, all new files and all rewritten importers | `bun c6repo-tools/tscheck.ts <list> <out>` (strict, bundler resolution, `@types/node` 22) and diffed against a baseline taken before the P73 write | 0 new diagnostics from this change. Remaining TS2307 and similar are workspace packages missing from `node_modules` or another executor's discovery split (`🧪️tests/🔬️workspace-contract` still names `🔍️discovery/🟦️.ts`) |
| Lossless | `bun c6repo-tools/lossless.ts <orig> <new files>` (top-level declaration texts, class members for P76) | root 413/413, P74 52/52, P75 11/11, P78 54/54, P77 all equal; the only differences are re-relativised `import()` paths, the `getLibRoot` anchor, `private` to `protected` in P76 and a concurrent edit of the `🖥️mounted` file by another executor |
| Runtime load | `bun -e import()` of each new module | 51/51 for P73, 7/7 P78; P75 and P77 children fail only on `@semio-tech/*` bare packages that are not installed (same as the originals) |
| Sizes and paths | script over all produced files | max 934 lines, longest path 127 characters |

## Open risks

- No test ran: `node_modules` is not resolvable in this tree (dependency image), so every bun test fails to import `typescript`/`ajv`. Static checks only.
- Another executor is splitting `🔍️discovery` and `🧹️normalization`. Names that used to come through the root barrel from `discovery` were bound to the definer present at write time; if those children move again, their importers (now my direct imports) move with the usual tools. `🧪️support/🏗️source-services` and the `source-services` test/fixture are shared with the normalization split; my edits are the six `🔣️taxonomy` entries only.
- Pre-existing, left alone: `⚡️caching/🧪️tests/⚡️cache-contracts/📋️bindings` looks for a `testCacheContracts` function that no longer exists; `🧪️tests/📦️crate-tree` imports `declaredNames` twice; `🏗️authoring/📦️crate-tree/🟦️.ts` imports `ExternalGateReader`, which `🧰️testkit` does not export.
- The `@semio-tech/repo-lib` package now has no `.` export. The four former bare importers use relative paths.

## Coordinator: what to run

No cargo crate is affected. Suggested order once dependencies are installed: `bun repo run @semio-tech/repo-lib:typecheck`, then the targets `test-repo-source-ownership`, `test-tool-configuration-ownership`, `test-root-artifact-schema-law-source`, `test-root-inference-law-source`, `test-root-surface-abstraction-law-source`, `test-root-schema-field-source`, `test-cargo-transaction-command-source`, `test-source-services`, `test-dependency-direction`, plus the caching command-source, canonical-execution, process-budgets, script-async-runners, artifact-empty-facet-authoring and cache-contracts tests.

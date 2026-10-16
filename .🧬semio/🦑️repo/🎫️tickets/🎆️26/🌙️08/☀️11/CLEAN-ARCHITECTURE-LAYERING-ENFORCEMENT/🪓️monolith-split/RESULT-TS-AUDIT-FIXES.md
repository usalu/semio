# Result: TS audit fixes (slice TS-AUDIT-FIXES, prefix `taf`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. 2026-10-16 11:40 to 12:40. `F` = this folder. `R` = repo root. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `OS` = `🧰️framework/🛍️products/💻️os`. `K` = `🧰️framework/🔨️modules/🎠️kernel`. `IA` = `L/⚖️laws/🔬️interactivity-audit`. Tools: `F/taf-tools/`. Logs: `F/🗑️generated/taf-*` (the source copies and the vitest caches were removed at the end; no `[DEBUG]` log was added).

## Verdict

Findings 1, 2, 3, 4, 5, 6, 8 and 10 are fixed and verified. Finding 7 was left to the parent. Two sub-items are deliberately not done and justified: the 20 undocumented peer files (finding 10) and three Rust comments under `🔌️plugin/🖥️host` (finding 8). Test sets are identical or smaller everywhere, the type-check error sets show nothing in my files, and the no-circular counts fell.

| Finding | Result |
| --- | --- |
| 1 launch entry | Re-pointed in both launch files; the six paths of the entry all exist. |
| 2 worker cycles | The worker imports no test. 44 cycles gone; the worker tree has 1 cycle left (pre-existing, below). |
| 3 kernel cycles | The six kernel children import no test. 10 in-tree cycles became 1 (a pre-existing, unrelated one). |
| 4 fixture rows | Both rows re-pointed to `🗂️taxonomy`; no reference to the deleted harness is left in the tree. |
| 5 oversized modules | 1,836 / 1,803 / 1,539 lines became at most 1,400 (table below). |
| 6 docstring emoji | 60 modules checked: 60 distinct first emoji, none repeated inside its own file. |
| 8 stale comments | 15 files fixed; 3 Rust comments in `🔌️plugin/🖥️host/**` left (owned by the host carve slice). |
| 10 directives | The six `/// <reference>` directives are gone, because the code that needed them left. |

## Baseline and after: dependency-cruiser `no-circular`

Command (the repo policy config reduced to the `no-circular` rule, `F/taf-tools/nocycle.cjs`; `F/taf-tools/cycscan.sh <phase>`): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs <roots> --config F/taf-tools/nocycle.cjs --output-type err`. The scan follows the whole dependency closure of its roots, so totals include cycles outside the tree. Summaries: `F/🗑️generated/taf-cyc-{before,after,final}-summary.txt`, per-tree lists `taf-cyc-*-touching.txt`. Before = recorded before my first source edit; after and final = after all edits (final reran after the last file change).

| Tree (roots) | Violations before | after | Cycles touching the tree before | after | Cycles entirely inside the tree before | after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Kernel `K` | 114 | 94 | 79 | 59 | 10 | 1 |
| OS worker (`worker`, `worker-law-suites`, `worker-law-fakes`) | 193 | 129 | 45 | 1 | 2 | 0 |
| Root script (`L/🎛️commands`, `L/⚖️laws`) | 184 | 164 | 0 | 0 | 0 | 0 |

- Cycles through the worker-law test modules: 44 before, 0 after. Cycles through the turn-outcome test: 10 inside the kernel plus the same module's loops through the closure, 20 in total in the kernel scan, 0 after.
- Remaining worker-tree cycle: `worker/🟦️.ts -> 🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts` (type import back). It exists at the audit base. Removing it needs the 130 module-private names that `registerBackboneParityTests` takes from the worker, so it is a separate refactor (hand-off).
- Remaining kernel in-tree cycle: `📤️return/📦️content/🧪️tests/…/🟦️.ts -> 📤️return/📦️content/🟦️.ts`. Pre-existing, in a file I did not touch.
- The root-script trees have no cycle of their own; the 20 fewer closure cycles are the kernel ones that the closure reaches.

## Findings 2 and 3: test registration moved out of the source

Rule applied: tests import the source; the source never imports tests. Type imports count too (`tsPreCompilationDeps: true`), so removing the dynamic import from the source was the only real fix.

- **Kernel.** The in-source `if (import.meta.vitest)` blocks and their regions were deleted from `K/{📡️turn-outcome-broadcast,🧭️app-router,🎛️activation-registry,🗂️plugin-registry,🚪️io-router,🔌️plugin-source}/🟦️.ts`. `K/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts` is now a plain vitest test file: it imports the six children and calls `registerTests1..6` with the same dependency bags. `directory` is computed with `dirname` calls (a `new URL("..")` fails under jsdom, as `RESULT-INTERPRETER-KERNEL-TS.md` found). Its `NonNullable<ImportMeta["vitest"]>` became `typeof import("vitest")`. Both configs now list it in `include` and drop the six children from `includeSource` (`K/🧪️tests/🎚️config/🟦️.ts`, `🧰️framework/🧪️tests/🎚️config/🟦️.ts`; the comment about `include` was reworded).
- **OS worker.** The two registration lines (`registerWorkerLawTests`) were deleted from the worker's in-source block. `OS/🧪️tests/📚️worker-law-suites/🟦️.ts` is now a plain vitest test file that imports `documentWorkerTestDependenciesV1` and `documentWorkerTestSourceV1` from the worker and registers the 21 suites in their old order. `workerLawSourceV1` moved to `OS/🧪️tests/🧰️worker-law-fakes/🟦️.ts`, so the gis worker (`✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/👷️worker/🟦️.ts`) no longer imports the aggregator. The OS config and the gis config (`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧪️tests/🎚️config/🟦️.ts`) list the aggregator in `include`, so both routes still run the 100 law tests. Launch entry `⚖️test-worker-law-suites🖥️os🟦️` (both files) now filters on the aggregator.
- Vitest only defines `import.meta.vitest` for the module that is the test file itself, so importing the worker from the aggregator does not re-run its in-source block (no double collection; checked by the identical test counts below).
- A first run of the aggregator hung and failed 4 tests: I had mistyped the relative URL in the moved `workerLawSourceV1` (`../../` instead of `../../../`), which sent fixture reads to a doubled path. Fixed; the standalone entry then passed 100 of 100.

## Finding 4: fixture rows

- `L/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json`: `owner` and `glue` of the `package-reexport` row now name `🧪️test/🗂️taxonomy/🟦️.ts` and `../../🗂️taxonomy/🟦️.ts` (the anchor `export const TAXONOMY_REL_PATH` lives there; the package entry imports exactly that specifier).
- `L/🧪️tests/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json`: the row's destination path now names `🗂️taxonomy/🟦️.ts`, and the integer after it is `destinationPathBytes` (82 became 98), not a line number.
- `git grep` over tracked and untracked non-ticket files for the old path finds nothing. That purity fixture is pinned by hash in two other fixtures (`🧭️fixture-placement/📜️enrollment.json`, `🏺️historical-package-owner-identity/🔣️.json`: sha `c3e39ed5…`, size 239,319); the file already differed before my edit (size 239,399), so those pins were stale and I did not touch them.

## Finding 5: size splits

Tool: `F/taf-tools/split-module.cjs` (TypeScript checker based; moves a trailing run of top-level statements into a child folder, rebuilds imports, closes and reopens split regions, re-points importers). Children sit in a sub-folder of the parent's folder, like the kernel children.

| Module | Before | After | New child | Child lines |
| --- | ---: | ---: | --- | ---: |
| `IA/🗄️database` | 1,836 | 1,016 | `IA/🗄️database/🌱️database-lifecycle`: catalog bootstrap, create catalog, compaction, sync-hello, artifact history, DB CLI, MCP HTTP transport | 824 |
| `IA/🖥️mounted` | 1,803 | 1,337 | `IA/🖥️mounted/🖼️mounted-surface`: surface lifetime, prepared render, surface lane | 470 |
| `L/🎛️commands/✅️verify` | 1,539 | 1,400 | `L/🎛️commands/✅️verify/🎨️storybook-freshness`: the five Storybook and Tailwind freshness methods, now module functions taking `repoRoot` | 148 |

- Importers re-pointed in the same step: 9 test files and `🏁️run` and `verify` for `database`, 4 for `mounted`, `verify` for `storybook-freshness` (`checkStorybookFreshness(this.root)`). The unused `existsSync`, `readdirSync`, `Dirent`, `dirname` imports of `verify` were pruned.
- Losslessness (`F/taf-tools/lossless.py`, trimmed line multiset against the original text): database and mounted differ only by the two docstrings, one import, and the two synthetic umbrella region markers `🔖️InteractivityAudit` (the umbrella region now closes in the parent and reopens in the child). Verify differs only by the changed signatures, the `this.root` to `repoRoot` replacements, the one extra `../` of the `.storybook` import, the new docstring and the two synthetic `🔖️VerifyScript` markers. No region marker was lost (the region count of the moved text is unchanged apart from the synthetic markers).
- A first draft of the verify child named the parameter `root`, which the loop variable `for (const root of scope.sourceRoots)` shadows (`join(root, root)`). `tsc` cannot see that. I caught it in the line-multiset check and renamed the parameter `repoRoot`.
- Behaviour proof for the verify child: the original methods (verbatim, wrapped in a class) and the new function ran against the real repo and against a temporary root with a broken stylesheet; both runs produced the same exit code and the same stderr (`IDENTICAL` twice; the real repo reports 6 stale Storybook paths in both).
- Import and load: all seven touched modules load under bun (no TDZ), `bun ./📜️script.ts --help` gives the baseline usage error with exit 1.
- Not done by design: `VerifyScript.run` is still a 630-line `if` chain (the module is under the limit); the orphan fixture `L/🧪️tests/🧫️fixtures/🚏️router-import-boundary/🔣️.json` names `VerifyScript.checkStorybookFreshness` as a consumer row. Nothing reads that fixture (`git grep` finds only ticket documents), so I left it.

## Finding 6: docstring emoji

`F/taf-tools/doc-emoji-fix.cjs` assigns each root-split module the emoji of its own folder when that emoji is free, and an explicit fitting fallback when it is already used by another module or by a docstring inside the same file. Checked after applying (`F/taf-tools/doc-emoji.cjs`, list `taf-docemoji-after.tsv`): 60 modules (the 57 root-split modules plus the 3 new children), 60 distinct first emoji, 0 repeats inside a file. Before, 41 modules opened with 🧭, 6 with 🎯 and 3 with 📐. Fallbacks used: `🔎 verify`→✅, `rust-warnings`→⚠, `storybook-freshness`→🖌, `examples`→📖, `stdio`→📟, `release`→📦, `os`→💻, `schema`→🧾, `stdio-ledger`→📒, `self-test-lanes`→🧫, `factory`→🔩, `live-replay`→🔂, `all-app`→🛰, `interactivity run`→🏃, `tool-run-and-history`→⏳, `database`→🛢, `allowlists`→📃, `crate-shape`→🪚, `subset-conformance`→🔖, `artifact-io`→🛂, `dissolved-kernels`→🫧. The two edited parent docstrings of `database` and `mounted` now say where the moved content lives. The new kernel test file got 🧰 (its first draft emoji 📡 repeated in the file). The first character of every docstring is the emoji plus U+FE0F.

## Finding 8: stale `🎠️kernel/🟦️.ts` references

Fixed (each now names the child that owns the named item; `F/taf-tools/stale-kernel-path.py`): `.cursor/plans/realtime_interaction_history_c71a5401.plan.md`; `K/🎞️media-jobs/🦀️.rs` (5 comments: `🛠️spawned-job`, `🎬️media-export`, `🎥️video-render-program`, `🗄️file-open-import`); `K/🖼️ui/🦀️.rs` and `K/🧪️tests/🐢️ui-dirty-scope/🦀️.rs` (`🎯️ui-dirty-scope`); `K/🧪️tests/{⬇️media-export-encoding,📤️file-open-import,🧵️spawned-job-drive}/🦀️.rs`; two generation3d Rust tests (`🧾️contributions-json`); `🖱️ui/📜️declaration/📐️layout/🦀️.rs` (`🧭️app-router`); `K/🧬️schema/🔣️history-patch/🔣️.json` (`🕰️history`); the renderer test `🛰️framework-plugin-runtime` (2 comments) and fixture `🪟️app-mode-layouts/🔣️.json`; plus the TS comments in `⚖️laws/🧩️plugin-consistency/🔌️plugin-architecture` (`🫧️transient`) and in `verify` (`🎬️media-export`). `git grep -F '🎠️kernel/🟦️.ts'` now finds only: `🧰️framework/📦️packages/🟦️typescript/package.json` (the SDK subpath `./kernel`, a different file) and the three skipped comments below.

Skipped: `OS/🔨️modules/🔌️plugin/🖥️host/🎠️activation/🦀️.rs:12`, `…/🖥️host/🧪️tests/🔬️app-router/🦀️.rs:178`, `…/🖥️host/🧭️routers/🦀️.rs:1709`. The plugin-host crate belongs to the plugin-host carve slice; the right targets are `🚦️activation-reason`, `🧭️app-router`, `🧭️app-router`.

## Finding 10: directives and undocumented files

- The `/// <reference types="vitest/importMeta" />` line existed only to type `import.meta.vitest` in the six in-source blocks. Those blocks left the six children, so the directive was removed instead of reordered, and each docstring is now the first thing in its file.
- The 20 files without an emoji docstring (`F/🗑️generated/audit5t-nodoc-list.txt`: retention, leases, caching, `🧵️gis-inference-bridge`, `🎟️carried-phase`, `🔢️leb`, spatial-kernel bindings, browser-host staging contract) are not fleet-created: none appears in any RESULT file, they are in a compact peer style (`import{test,expect}from"bun:test"`), and 12 of them were modified between 11:04 and 11:54 today, so a peer is writing them right now. I did not touch them; a docstring added now would collide with the writer.

## Verification

| Check | Before | After |
| --- | --- | --- |
| Kernel vitest (`K/🧪️tests/🎚️config`, hand-built `SEMIO_VITEST_POLICY` as in `ikt-tools/vitest.sh`; `F/taf-tools/vtk.sh`) | 14 files, 91 pass | 9 files, 91 pass; status+title sets identical |
| Framework vitest (`🧰️framework/🧪️tests/🎚️config`) | 11 files, 306 pass | 6 files, 306 pass; sets identical (the six in-source kernel files became one) |
| OS vitest (`OS/🧪️tests/🎚️config`, `F/taf-tools/vt.sh`) | 9 files, 609 tests, 499 pass, 110 fail | 10 files, 609 tests, 499 pass, 110 fail; status+title sets identical (the 110 failures are the actor-transport file, unrelated) |
| Gis vitest (gismap inference config) | 2 files, 149 pass | 3 files, 149 pass; sets identical |
| Standalone worker-law entry | n/a | 1 file, 100 pass |
| bun tests, 19 files (`F/taf-tools/bun-tests.txt`: root-script readers, `root-*-source`, outcome and diff-only gates, bootstrap, fixture-placement, historical identity, repo-source-ownership, workspace-contract) | `workspace-contract` 619 pass / 137 fail / 83 skip; others as in `taf-bun-before.counts` | identical per file except `workspace-contract` 621 / 135 / 83: two package-boundary guard tests that failed before now pass (a peer's change, not mine); no test fails that did not fail before |
| 24 interactivity `*SelfTests` run through the moved functions (`F/taf-tools/selftests.ts`) | 10 pass, 14 fail | identical output line by line (the 14 failures are the existing repo defects named in `RESULT-ROOT-SCRIPT-SPLIT.md`: missing source paths and drifted Rust sources) |
| `tsc` root-script closure (`taf-tsc-root-*`, 60 files plus closure) | 30 error lines | 32; the 2 extra are in peers' in-flight `native-build` files (`🧩️component-pass`, `📦️retention`); none in a file I touched or created |
| `tsc` kernel, worker, suites, gis worker, configs (15 files plus closure, `taf-tsc-os-*`) | n/a (no closure baseline taken before the kernel and worker edits) | 97 error lines, identical to the 97 recorded by `RESULT-MISPLACED-E2E-TESTS.md`; 0 in the kernel children, the kernel test, the aggregator, the configs; the 5 in `worker-law-fakes` and the gis worker are the old `actorIdentityGrant` and `never` errors with shifted lines |
| Launch files | n/a | both parse as strict JSON; the six paths of `🧪️test📜️history🔮️six-family-source` and the `⚖️test-worker-law-suites🖥️os🟦️` target exist |
| Max new path | n/a | 165 UTF-8 bytes (limit 240) |

## Hand-offs

1. Taxonomy registration (held file `L/🔣️taxonomy.json`): `🌱️database-lifecycle`, `🖼️mounted-surface`, `🎨️storybook-freshness`; same unresolved class as their siblings.
2. The one remaining worker cycle (`backbone-parity` type import) and the 630-line `VerifyScript.run`.
3. Three plugin-host Rust comments (finding 8) and the 20 peer files without a docstring (finding 10), once their writers are done.
4. `RESULT-MISPLACED-E2E-TESTS.md` and `RESULT-INTERPRETER-KERNEL-TS.md` describe the old in-source registration; their run instructions now apply to the test entries above.
5. Finding 7 (policy file) untouched.

## Files

Created: `IA/🗄️database/🌱️database-lifecycle/🟦️.ts`, `IA/🖥️mounted/🖼️mounted-surface/🟦️.ts`, `L/🎛️commands/✅️verify/🎨️storybook-freshness/🟦️.ts`.
Rewritten: `K/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts` (now a test file), `OS/🧪️tests/📚️worker-law-suites/🟦️.ts`.
Edited: the six kernel children, both kernel vitest configs, the worker (2 lines), `worker-law-fakes`, the OS and gis vitest configs, the gis worker (1 line), `IA/🗄️database`, `IA/🖥️mounted`, `verify`, `🏁️run`, the interactivity test importers named in finding 5, the docstring first emoji of the root-split modules, two fixtures, the comment files of finding 8, `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (2 entries each).
Temporary: `F/taf-tools/` (scripts, specs, tsconfigs), `F/🗑️generated/taf-*` (logs and JSON reports).

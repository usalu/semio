# Result: TS-FILE-SPLITS-8 (`tf8`)

Executed 2026-10-16 by the Sonnet executor, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `MR` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`, `T` = `MR/🧪️test`, `E` = `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine`. Tools: `F/tf8-tools/` (split, lossless, repoint, titles, differential, runners, configs in `cfg/`). Logs and original snapshots: `F/🗑️generated/tf8-*` (`tf8-orig/{orch,tp,prev}.ts` are the pre-edit copies).

## 1. Verdict

All three splits are applied. Statement sets are lossless, the test-title multiset is identical, the failure sets are identical, `tsc` shows no new error, and no new cycle exists. The one open point is the taxonomy: it needed no edit (section 6).

| Target | Before | After |
| --- | --- | --- |
| `T/🧬️schema/📋️orchestration/🟦️.ts` | 2,609 lines, 162 statements | 66 lines (`SchemaScript`, genuinely its own) plus 9 children, 65 to 596 lines |
| `T/🧪️tests/🧪️test-platform/🟦️.ts` | 1,562 lines, 124 tests, 30 describes | deleted. 4 topic test files (260 to 574 lines) plus 1 shared support module (60 lines) |
| `E/🧮️preview/🟦️.ts` | 1,258 lines, 97 statements | 225 lines (the two model-diff builders and `PreciseSpatialKernelMath`, the part that faces the `🗺️spatial` contract) plus 7 children, 49 to 282 lines |

New folders (all admitted by the taxonomy as is):

- `T/🧬️schema/📋️orchestration/`: `📚️schema-documents` (JSON documents and pointer helpers), `🦀️rust-tokens` (Rust lexer and structural readers), `🌳️mutation-tree` (leaves, aggregates, Rust source walk), `🎛️mutation-input-ui`, `⚖️mutation-payload-parity`, `🐘️mutation-caps`, `🏷️mutation-labels`, `📢️fault-notices`, `✏️mutation-editability`.
- `T/🧪️tests/`: `🧰️repository-scan` (the once-per-process repository scan, with the license header region), `🔍️discovery-contract`, `⚖️comparison-profiles`, `🔮️oracle-governance`, `🦠️mutation-catalogs`.
- `E/🧮️preview/`: `🔷️brep-records`, `📏️vec3`, `🌀️curve-sampling`, `🔺️face-geometry`, `📦️aabb`, `⚓️anchor-placement`, `🎚️preview-params`.

Each new file starts with a docstring whose first character is the folder emoji (21 emoji, none repeated inside a sibling set). The longest new path is 147 UTF-8 bytes (limit 240).

## 2. Baseline

Importers (`importers.mjs`, TypeScript resolver over git-grep candidates plus untracked files):

- Orchestration: `T/📜️script.ts` (`SchemaScript`, unchanged), the five tests `🧪️fault-notices-gate`, `🧪️mutation-caps`, `🧪️mutation-history-gates`, `🧪️mutation-input-declarations`, `🧪️mutation-payload-parity`, and one dynamic import in `MR/📚️library/🎛️commands/✅️verify/🚦️gates/🟦️.ts`.
- Test platform: no importer. Path readers: `T/📋️project.json` (nx inputs), the package `📜️script.ts` (file list), `T/🧪️tests/🧱️command-composition-source` (test plus fixture JSON) and a library comment.
- Preview: 15 files (`E/🧠️semio`, one spatial test, two brepjs oracle files, and 11 cad plugin files, four of them `.tsx`).

| Command | Baseline result |
| --- | --- |
| `bun test` of the five orchestration test files (`F/tf8-tools/run-tests.sh base`) | caps 5 pass; input-declarations 31 pass; payload-parity 43 pass; history-gates 43 pass, 1 fail; fault-notices-gate 2 pass, 1 fail |
| `bun test` of `🧪️test-platform` as the repo runs it | the file does not load: `Original finite test artifact callback required` (module-scope scan needs the storage scope, the same in `RESULT-REPO-TEST-VSCODE-SPLIT.md`), 0 tests |
| the same with a bun preload that mocks only the storage root (`tf8-tools/preload-storage.ts`, `run-tp.sh`) | 125 tests (the 124 plus 1 unnamed load error), 110 pass, 15 fail |
| `vitest` cad-js project, in-source tests of the spatial kernel (`vt.sh`) | 12 tests, 10 pass, 2 fail; 10 suites do not load (missing `brepjs` and `@semio-tech/cad-js-module-*` packages) |
| `tsc --noEmit` over the closure of all touched files and importers (`tsconfig.tf8.json`, 25 files plus closure) | 24 error lines, none in a file I changed except the existing `TS7016` for `../../🟨️.mjs` in the test platform file and `TS18047` in `mutation-history-gates` |
| `dependency-cruiser` `no-circular` over the new trees | taken after the split (section 4); the one preview-related cycle was shown pre-existing |

Why the storage mock: the platform file cannot run under bare `bun test` here, so without it the only dynamic evidence would be "fails to load, before and after". The mock replaces `repositoryTestArtifactRootV1`, `repositoryTestArtifactPathV1` and `repositoryTestArtifactEnvironmentV1` with a plain scratch directory outside the repo. Nothing else is stubbed.

## 3. How the files were split

`F/tf8-tools/split.mjs` (the `rtv-tools` splitter, extended by line-range assignment and a license-header option) uses the TypeScript checker to resolve every identifier, builds the statement graph, assigns statements to modules, regenerates imports (relative specifiers recomputed for the new depth, `type` kept), and adds `export` only to declarations that now cross a module boundary. `eval.mjs` proved each module graph acyclic before anything was written (type-only edges included).

- Orchestration: the assignment follows the region names, except that the shared base moved down: JSON helpers into `📚️schema-documents`, the Rust lexer and readers into `🦀️rust-tokens`, and the leaf/aggregate walk into `🌳️mutation-tree`. Module order, low to high: `schema-documents`, `rust-tokens`, `mutation-tree`, then the six gates (`editability` imports `labels` once), then `SchemaScript`. 31 declarations received `export` (the five `run*` entries, the lexer and JSON helpers, `MUTATION_LABEL_APP_TRAITS`, and so on).
- Test platform: the 28 describe blocks were grouped by topic, kept byte for byte and in source order, so titles are untouched. The module-scope scan (`repoRoot`, `repoContributions`, `repoRegistry`, `repoCases`, `repoOracleHits`, `repoWideBreaches`, the area helpers, `relativeToRepo`) lives once in `🧰️repository-scan`, which the four topic files import. Each topic file therefore still measures its time budgets over an already finished scan, as the original comment demanded.
- Preview: the regions were unbalanced in the original (`#region 🧮️SpatialKernelMath` closed by `#endregion 🧮️Vec`, `#region 🌀️EdgeGeometry` never closed, `export { kernelGeometry }` inside the adapter region). The tool ran on a normalized copy (`F/🗑️generated/tf8-norm/prev.ts`) that adds `#region 🧮️Vec` and `#endregion 🌀️EdgeGeometry` and moves the re-export into `🧱️kernelGeometry`. Every original marker is kept. The two model-diff builders stay in the parent (see section 4).

## 4. Verification

| Check | Result |
| --- | --- |
| Statement set (`lossless.mjs`, TypeScript statement text, `export` prefix stripped) | orchestration 162 of 162, test platform 42 of 42, preview 97 of 97, 0 mismatches. `F/🗑️generated/tf8-lossless-{orch,tp,prev}.log` |
| Line multiset | missing lines are only the `export`-prefixed declarations (31, 11 and 5) and re-generated imports; added lines are docstrings, imports and repeated region markers. No marker is missing in any of the three |
| Region markers | none lost. Repeated names: orchestration `⚖️MutationPayloadParity` (+3) and `🦀️RustSources` (+1); test platform `🔌️Adapters` (+4) and `🧪️Tests` (+3); preview `🔌️Adapters`, `🧮️SpatialKernelMath`, `🌀️EdgeGeometry` (each +5 to +7). Added to balance: `#region 🧮️Vec` and `#endregion 🌀️EdgeGeometry` |
| Test-title multiset (`titles.mjs`, AST of describe and test, 154 rows) | identical before and after |
| Test platform, same-time run (`tf8-base2-tp.log` = the original copied to a temporary sibling folder, `tf8-after2-tp.log` = four new files, run concurrently with the same preload) | both 125 tests, 110 pass, 15 fail, 9062 expect calls; the sorted fail lists are identical (the 15 are existing environment and registry failures, for example the live registry laws and the Rust host laws). A first sequential after-run matched too (110/15). The temporary folder was removed |
| Orchestration `bun test` after (`run-tests.sh after`) | identical to baseline file by file: 5, 31, 43 pass; 43 pass + 1 fail; 2 pass + 1 fail, with the same failing titles |
| Preview vitest after | 12 tests, the same 10 pass and 2 fail, the same suite-level failures (`FAIL` lines identical) |
| Preview differential (`preview-diff.ts`, old snapshot vs new modules, seeded) | 1,672 calls (every pure function with 40 random inputs, plus a built box model through faces, anchors, fuse and AABB), 0 mismatches. The class members are identical. Sensitivity: a deliberate change of `vec3Add` in the old copy gives 300 mismatches |
| `tsc --noEmit` over the same closure after (`tsconfig.tf8-after.json`, final run `tf8-final-tsc.log`) | 24 error lines, the same set; the one `TS7016` for `../../🟨️.mjs` moved with its import from `🧪️test-platform` to `🔍️discovery-contract`. A transient 2-line error from a peer's cargo preparation file (`🛠️preparation/📋️request`) appeared in one run and was gone in the next |
| `no-circular` (`dependency-cruiser` with the `taf-tools/nocycle.cjs` policy over the new trees) | 0 cycles in the orchestration children, the test-platform files and the preview children. The only cycle that touches the touched trees is the existing `spatial test -> 🧮️preview parent -> 🗺️spatial -> spatial test` (the in-source `registerTests1` block of `🗺️spatial`, plus the parent's type import of the spatial contract). A first version with `🧊️preview-model-diff` as a child created a second, longer variant of that cycle, so the diff builders were moved back into the parent. 138 violations became 137 in the whole cruise |
| Source-text readers: `T/🧪️tests/🧱️command-composition-source` after | 10 pass, 1 fail. The failure (`prioritizes each declared Python source`) points at the missing directory `T/🧫️fixtures/🧱️command-composition-source` and does not involve my files. The "typechecks an acyclic owner graph" test timed out once at 41 s under load 57 against its 30 s limit; the same program run directly gives 0 diagnostics (`tf8-owner-graph.log`), and a rerun passed |
| Launch files | both parse as strict JSON; 4 entries added to each |
| Dead references | `git grep` for the three old paths (`🧪️test-platform`, `🧬️schema/📋️orchestration/🟦️.ts` as import target, `🧮️preview/🟦️.ts` as a function source) finds only the parent files that still exist for their own content |

Not run: the Nx routes (`bun repo run @semio-tech/repo-test:test`, per `RESULT-REPO-TEST-VSCODE-SPLIT.md` it blocks for more than 20 minutes in `build:selected-compile-imports`) and the `brepjs`-dependent cad suites (their packages are missing here, before and after).

## 5. Importers and readers updated

- Orchestration: the five test files (each import split by owning child, `repoint.mjs`), the dynamic import and the `@see` in `MR/📚️library/🎛️commands/✅️verify/🚦️gates/🟦️.ts` (now `🐘️mutation-caps`), the `$comment` of `T/🧪️tests/🧫️fixtures/🧫️mutation-payload-parity/🔣️.json`, and `T/📋️project.json` (`commandCompositionSources` lists the nine children so a child edit invalidates the cache). `T/📜️script.ts` is untouched: the parent still owns `SchemaScript`, and the command-composition fixture row for it stays valid.
- Test platform: `T/📋️project.json` (the platform inputs, five paths), `T/🧪️tests/🧱️command-composition-source/🟦️.ts` (expected input list), the fixture `T/🧪️tests/🧫️fixtures/🧱️command-composition-source/🔣️.json` (consumer row now names `🔍️discovery-contract`, the file that holds both required owners, host materialization and scenario), the library comment in `MR/📚️library/🎛️commands/🧬️schema/🟦️.ts`, and the package `T/📦️packages/🟦️typescript/📜️script.ts` (aggregate file list plus four single-file segments `platform-<topic>`).
- Preview: the 15 importers (`E/🧠️semio/🟦️.ts`, the brepjs oracle and its test, `core-vec`, `core-model-commit-mesh`, and the cad plugin tests and renderer files). Duplicate imports from the parent were merged by hand in two files. Rust comments with line numbers into the old file were re-pointed to `🌀️curve-sampling` (`✏️editor/⚙️engine/🧲️picking/🦀️.rs`, three comments; `🧬️schema/📐️geometry/🦀️.rs`, one) and a doc comment in the brepjs file.
- No barrel and no compatibility export was added. The package entry of the repo test platform is unchanged.

## 6. Entry points and taxonomy

- Nx targets `test-platform-discovery-contract`, `-comparison-profiles`, `-oracle-governance`, `-mutation-catalogs` in `T/📦️packages/🟦️typescript/📋️project.json` (shape of `test-fixture-isolation`, uncached because the laws read the live registry).
- Launch entries `🧪️test🦑️repo🧪️test🔍️discovery-contract`, `…⚖️comparison-profiles`, `…🔮️oracle-governance`, `…🦠️mutation-catalogs` in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, group `9_gates`, orders `900.057632` to `900.057635`, placed after `…🚷️discovery-boundaries` with the same environment. Other people edit both files; the insertion was one anchored text edit per file.
- Taxonomy: none of the 21 new folders needs a row. The library's own `semanticDirectoryKindId` with ancestor context (`tf8-tools/taxsim.ts`, `validateTaxonomy` 0 problems) resolves the five test folders to `members-of-tests` and the other 16 to `crate-module-children` through their slug-pattern rows (log `F/🗑️generated/tf8-taxsim.log`, 0 unresolved). So `🔣️taxonomy.json`, which a peer edited minutes ago, was not touched.

## 7. Hand-offs

1. `🗺️spatial` registers its tests from inside its own source (`import.meta.vitest` block). That in-source import is the root of the remaining cycle through `🧮️preview`; moving the registration into a test file (the `taf` rule) removes it.
2. `T/🧪️tests/🧫️fixtures/🧱️command-composition-source` is referenced by `🐍️.py`-related code as `T/🧫️fixtures/…`, which does not exist; one command-composition test fails on it before and after.
3. The test platform file only loads under a finite-artifact scope. Under plain `bun test` all four topic files fail at load exactly as the old file did; the preload in `tf8-tools/preload-storage.ts` is how to exercise them.
4. The shared scan docstrings in `🧰️repository-scan` still say "this file" (kept verbatim for losslessness).

## 8. Files

Created: the 9 + 5 + 7 child modules above (`T/🧬️schema/📋️orchestration/*/🟦️.ts`, `T/🧪️tests/{🧰️repository-scan,🔍️discovery-contract,⚖️comparison-profiles,🔮️oracle-governance,🦠️mutation-catalogs}/🟦️.ts`, `E/🧮️preview/*/🟦️.ts`).
Rewritten: `T/🧬️schema/📋️orchestration/🟦️.ts`, `E/🧮️preview/🟦️.ts`. Deleted: `T/🧪️tests/🧪️test-platform/🟦️.ts` and its folder.
Edited: the five orchestration tests, `MR/📚️library/🎛️commands/✅️verify/🚦️gates/🟦️.ts`, `MR/📚️library/🎛️commands/🧬️schema/🟦️.ts`, `T/📋️project.json`, `T/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`, `T/🧪️tests/🧱️command-composition-source/🟦️.ts`, two fixture JSONs, the 15 preview importers, two Rust comment files, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.
Temporary (kept per rules): `F/tf8-tools/`, `F/🗑️generated/tf8-*`. No `[DEBUG]` line was added.

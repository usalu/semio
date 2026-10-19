# Result tax: whole-tree taxonomy validation, findings before/after, root-cause fixes

`C` = this ticket folder. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Tools: `C/tax-tools/` (all read-only drivers except the three `edit-*.py` anchored text editors).
Nothing was committed, staged, stashed or reverted. No file touched by another agent in the last 20 minutes was edited (every edited file was at least 40 minutes old; the one exception is `L/🔣️taxonomy.json` and `.gitignore`, edited with anchored inserts after a fresh read).

## 0. Verdict

| lane | before | after |
| --- | ---: | ---: |
| `validateTaxonomy` (20 rule groups over `🔣️taxonomy.json`) | 0 problems | 0 problems |
| `verify taxonomy report --scope <any>` (planning phase) | **crashed**: `Registry implementation import is not schema-owned: @semio-tech/process/routing` | completes (teaching scope: 36 s, `clean=false errors=1` before the last fix, now 0) |
| `verify taxonomy implementation report` (leaf-name census) | 58,013 errors, 7 min, walked `.cache` (47,864), `.lake` (9,021), generated `🧑‍💻dev` bundles | **151 errors**, 36-70 s |
| scoped inventories (sum over 9 top-level scopes) | 24,620 violations | 23,950 (see 2) |
| `🎓️teaching` scope | 68 | **0** |

The whole-repo `verify taxonomy report` (no `--scope`) was not completable: 9 minutes in `inventory/references` (28,418 of 129,486 reference files) and the incoming-reference scan reads 129k files; on a hot tree it also fails with `Incoming reference changed during its snapshot: 📋️build-owners.json`. Per-scope inventories (`C/tax-tools/inventory.ts`, `inventoryTaxonomy`) are the usable instrument (teaching 21 s, framework 80 s, `✏️s` 150 s).

## 1. Root causes found and fixed

1. **Registry catalog closure crash (blocked every `verify taxonomy report`).** `registryCatalogInputPaths` only accepts exact workspace package names (`audit-generator-contracts` enforces `^@scope/name$`), but the glue sweep had rewritten `L/🏗️build/🟦️typescript/🟦️.ts` to subpath specifiers (`@semio-tech/process/typescript-check`, `@semio-tech/process/routing`) that the dev script's static import closure reaches. Restored the two relative imports (the file's third import already used that form). Tracer `C/tax-tools/registry-unbound.ts` walks the closure and lists every non-schema-bound specifier: now 0, closure 232 files.
2. **Architecture-quiz to architecture-course rename never reached the taxonomy** (57 unregistered directories in the course, 20 concept pets, ventilation topic, proctor `📚️course` module). `L/🔣️taxonomy.json`: new kinds `teaching-course`, `teaching-course-stack`, `teaching-course-content`, `teaching-course-e2e`, `teaching-course-e2e-learner` (replace the four `teaching-quiz-*` kinds, which had no directory left), `teaching-ventilation`; `teaching-quiz` no longer lists `teaching-architecture` as parent (topic `❓️quiz` dirs unchanged); new member row `members-of-teaching-course` (29 names: deploy, fixtures, sessions, mode); `members-of-teaching-pets` +21 pets; `members-of-teaching-energy`/`-topics` +ventilation; `members-of-modules` +`📚️course`. Two pet directories used non-emoji prefixes (`ƒ`, `∞`) that the member validator rejects (`has invalid exact member`): renamed `ƒ️formula` to `➗️formula` and `∞️continuous-parameter` to `♾️continuous-parameter` (dir, `🐾️pets/🔣️.json` species list, `🐾️pets/🟦️.ts` imports; the only references).
3. **Course router rejected by the root-script law** (`fixed-source-disposition-unresolved`): top-level `async function`, `throw Error(` without `new`, and a second `router.register(...)` statement. Moved the build function to `buildCourseReleaseSite` in `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts`, `throw new Error`, one registration chain. Same mechanical `throw Error(` to `throw new Error(` flip applied to 44 more scripts that the validator accepts afterwards (list in 4).
4. **Workspace-root tool files had no fixed contract** (`package.json`, `bun.lock`, `tsconfig.json`, `Cargo.toml`, `Cargo.lock`, `.config/nextest.toml` of the nested workspaces `🎓️teaching`, `🌎️hub`, `🏢️semio-tech`, `♻️mit-bestand`, `✏️s`, `🧰️framework` and standalone crate workspaces; the planner wanted to rename them to `⚙️.toml`, `🔒️.lock`, `🔣️.json`). Six `fixedFilenameContracts` (`workspace-package-manifest` `*/package.json`, `workspace-bun-lock`, `workspace-typescript-config`, `workspace-cargo-manifest` `**/Cargo.toml`, `workspace-cargo-lock`, `cargo-nextest-config`), all `path-pattern` scope (lowest specificity, so the existing package-root contracts still win).
5. **Declaration companions flagged as foreign implementation leaves** (`🟨️.d.mts`, `🗄️.d.ts`, `🗄️.sql.d.ts`: declarations named after their non-TypeScript sibling) and nested Go command module manifests (`📦️packages/🐹️go/<cmd>/go.mod`). Four fixed contracts (`javascript-module-declaration`, `sql-module-declaration`, `sql-module-extension-declaration`, `go-command-module-manifest`) plus three `packageSourceDispositions` rows (the validator requires one per source-format contract).
6. **JSON re-export glue unclassifiable** (`export { default } from "x.json" with { type: "json" }`): `L/🔍️discovery/🏷️package-source-classification/🟦️.ts` stripped neither import nor export import attributes. `package-role-unresolved` 72 to 20 in the framework. Test added with TypeScript compiler cross-check and negative control (see 3).
7. **Implementation census counted git-ignored output** (58k findings). `taxonomyImplementationFilesystemFindings` got an `admitted(path, nodeKind)` option (`L/🔍️discovery/📄️implementation-findings/🟦️.ts`, `L/🔍️discovery/🗂️file-kind-resolution/🟦️.ts`); the lane (`L/🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts`) passes `git ls-files --cached --others --exclude-standard` visibility. Also `implementationLeafPolicy.ignoredPathPatterns` += `**/.cache`, `**/.lake` (taxonomy and the exact-list validator `L/🔍️discovery/📎️audit-leaf-and-reference-policies/🟦️.ts` changed together). Test added.
8. **Leaked artifacts.** 50 bun lockfile snapshots (`.` plus seven hex digits, 640-660 KB JSON starting `{"lockfileVersion"`) at `/`, `🎓️teaching`, `🌎️hub`, `🏢️semio-tech`, `♻️mit-bestand`, `✏️s` (all older than 2 h; list in 4) deleted; `.gitignore` gets the matching pattern so they leave the taxonomy inventory and commits. The leaked test temp dir `extension-host-build-7a5OFB/` (Oct 13, tracked; `L/🏗️build/🕸️graph/🟦️.ts` already special-cases that prefix) deleted.

## 2. Findings per scope and code (scoped inventory, before to after)

| scope / code | before | after |
| --- | ---: | ---: |
| 🎓️teaching (all) | 68 | 0 |
| 🌎️hub (all) | 51 | 33 |
| - file-kind-unresolved (stray lockfiles) | 13 | 0 |
| - semantic-stem-unresolved | 7 | 2 |
| 🏢️semio-tech (all) | 21 | 8 |
| - file-kind-unresolved | 11 | 0 |
| - semantic-stem-unresolved | 4 | 2 |
| ♻️mit-bestand (all) | 111 | 104 |
| - file-kind-unresolved | 5 | 0 |
| - semantic-stem-unresolved | 45 | 43 |
| 👴️leutwiler | 1 | 1 |
| 🧫️fixtures | 1 | 1 |
| 🧪️tests, 🧬️schema | 0 | 0 |
| 🧰️framework (all) | 1,550 | 1,408 |
| - package-role-unresolved | 72 | 20 |
| - taxonomy/kind-only-basename | 120 | 80 |
| - fixed-source-disposition-unresolved | 274 | 233 |
| - semantic-stem-unresolved | 311 | 306 |
| - file-kind-unresolved | 14 | 10 |
| ✏️s (all) | 22,817 | 22,395 |
| - semantic-stem-unresolved | 465 | 96 |
| - taxonomy/kind-only-basename | 75 | 33 |
| - file-kind-unresolved | 22 | 15 |
| - fixed-source-disposition-unresolved | 237 | 234 |

Scoped inventories exclude `.🧬semio` and the repo-root files. Attribution of the remaining 23,950 to "today" is unreliable by file time (a git operation at 12:29 reset the birth time of everything); by `git log --diff-filter=A` since 10-18 plus untracked files the new-in-this-window share was 568 before the fixes (333 `package-implementation-file` warnings, 57 teaching directories, 36 JSON glue, 20 router bodies, 13 basename, 12 directory, stray lockfiles), of which all errors except the items in 5 are gone.

## 3. Verification (all run here)

- `bun C/tax-tools/validate.ts` (`validateTaxonomy(loadCatalogTaxonomy())`): `{"problems":0}` after the last edit.
- `bun ./📜️script.ts verify taxonomy report --scope 🎓️teaching`: before the router fix `clean=false errors=1`, 36 s; scoped inventory afterwards 0 violations.
- `bun ./📜️script.ts verify taxonomy implementation report`: `errors=151` (was 58,013).
- `bun C/tax-tools/registry-unbound.ts`: visited 232, unbound 0, missing 0.
- Tests (explicit files only): `L/🧪️tests/🦑️repo-source-ownership/🟦️.ts` 7 pass (new test; negative control with the regex fix reverted fails with `Expected "declaration", Received "unresolved"`); `L/🧪️tests/🌳️kind-only-basename/🟦️.ts` 8 pass (new `admitted` test); `L/🧪️tests/🌐️registry-import-language/🟦️.ts` 16 pass; `L/🧪️tests/🔬️workspace-contract/📖️taxonomy-loading/🟦️.ts` 54 pass, 2 fail (both pre-existing: `artifactFacetPathIsDeclared` contradicts itself at lines 551-552; the General publisher preview command `playground-session` exits 1), vtax reported 4 failures there before; `.../🗂️workspace-taxonomy-authority/🟦️.ts` 4 pass; `.../🔭️package-discovery/🟦️.ts` 19 pass.
- Course: `vitest run --config 🧪️tests/🎚️config/🟦️.ts 🧪️tests/🐾️pet-cast/🟦️.ts` in the course package: 359 pass after the two pet renames. `bun build --target=bun` of the site module OK; of the course `📜️script.ts` it stops at `Could not resolve "@semio-tech/course"` (pre-existing deps4 link item, unrelated to the edit).
- `bun repo --help` boots; `bun build --target=bun 📜️script.ts` bundles (24.22 MB).
- Not run: whole-repo `verify taxonomy report`, `verify taxonomy enforce`, the Rust/Cargo side (no Rust edited), `launch.json` (no new command).
- Two transient windows of 2-3 minutes in which `🔣️taxonomy.json` failed `validateTaxonomy` (two pet names with non-emoji prefixes; then three source-format contracts without `packageSourceDispositions` rows). Both fixed immediately; any agent that loaded the taxonomy in those windows saw `Invalid taxonomy schema`.

## 4. Files changed

- `L/🔣️taxonomy.json` (anchored inserts only, formatting preserved): teaching kinds and members, `members-of-modules`, 10 `fixedFilenameContracts`, 3 `packageSourceDispositions`, `implementationLeafPolicy.ignoredPathPatterns`.
- `L/🔍️discovery/📎️audit-leaf-and-reference-policies/🟦️.ts`, `L/🔍️discovery/🏷️package-source-classification/🟦️.ts`, `L/🔍️discovery/🗂️file-kind-resolution/🟦️.ts`, `L/🔍️discovery/📄️implementation-findings/🟦️.ts`, `L/🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts`, `L/🏗️build/🟦️typescript/🟦️.ts`.
- Tests: `L/🧪️tests/🦑️repo-source-ownership/🟦️.ts`, `L/🧪️tests/🌳️kind-only-basename/🟦️.ts`.
- Teaching: `🎓️teaching/🏛️architecture/🐾️pets/🔣️.json`, `.../🟦️.ts`, directories `➗️formula`, `♾️continuous-parameter` (renamed); `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts`, `.../🚀️deploy/🌐️site/🟦️.ts`.
- 44 scripts with `throw Error(` to `throw new Error(`: list kept in `C/tax-tools/router-flips-applied.txt`.
- `.gitignore` (+2 lines at the end).
- Deleted: 50 stray lockfile snapshots (`C/tax-tools/strays-deleted.txt`), `extension-host-build-7a5OFB/` (4 files plus `out/`).
- Tools (inputs, kept): `C/tax-tools/{inventory,scopes,validate,classify,bisect-rust,router-variants,router-census,registry-unbound}.ts`, `edit-{teaching,workspace-roots,companions,dispositions}.py`, `router-flips-applied.txt`, `strays-deleted.txt`. Generated output `C/🗑️generated/tax-run/` deleted.

## 5. Open items (not fixed, with cause and owner)

1. **Command-router law backlog, 495 scripts** (`fixed-source-disposition-unresolved`, 233 framework, 234 `✏️s`, 16 hub, 8 mit-bestand, 3 semio-tech, 1 leutwiler): `ecmaCommandRouterModule` accepts only one routing terminal with registered classes. 257 of them are not even parseable by `@semio-tech/compiler-syntax-ecma-ts` (type aliases, top-level functions, `try`/`if` at top level, import attributes `with {type:"json"}`), the rest are multi-class or helper-bearing scripts. These contain implementation and need logic moved into modules, not edits.
2. **`✏️s` mutation/fixture/path budget, about 21,000** (`path-too-long` 15,763: the taxonomy budget is 240 UTF-8 bytes, no path reaches 256 characters, the longest has 314 bytes; `mutation-fixture-unpaired` 3,177; `mutation-pair-path-budget` 1,163; `projection-catalog-unrealized` 821; `mutation-payload-schema-authority-invalid` 574; `projection-catalog-coverage` 223; `mutation-fixture-invalid` 122; `projection-member-unresolved` 109). Concentrated in `🏙️bim` 4,297, `🔋️energy` 3,643, `🏛️architect` 3,438 paths; scoped-inventory artefacts are possible for the pairing codes (a scope sees one side of a pairing); needs `verifyTaxonomyScopesClosed` on single plugins at a quiet moment.
3. **151 leaf-name findings** (`verify taxonomy implementation report`): 66 `*.ts`, 27 `*.test.ts`/`*.test.tsx` (`🟦️.test.ts` needs the vitest `include` change), 13 css, 11 js, 10 `*.rs`, 9 `.d.ts`; mostly test helpers such as `🧪️tests/🍎️image.ts` (importers must be re-pointed), 48 in `♻️mit-bestand` (a saved web page `🌐️public/…_files/*` and presentation slides, content not code), 3 `🏢️semio-tech`. The canonical targets are in the planner output (`inventory.ts` prints `moves`); renaming needs the normalization transaction at a quiet moment, not text edits.
4. **Rust and Go package forms** (`package-role-unresolved` 33, `package-implementation-file` 666 warnings): Rust entries with `artifact_retire_leaf!` macros or multi-line `pub use` and Go files with receiver methods in `📦️packages/…` do not match the glue grammar (macro registry is hard-coded in `classifyRustPackageSource`; the Go function regex ignores receivers). Law decision: either widen the grammar or move the code out of `📦️packages`.
5. `semantic-stem-unresolved` 449: 306 framework (Unicode data `*.txt`, LaTeX `📓️print/🖋️latex` 76, markdown fixtures 30), 96 `✏️s`, 43 mit-bestand (saved web page assets); third-party or fixture bytes that need a data-kind or an opaque exclusion.
6. `directory-kind-unresolved` 36 (framework fixture dirs named by data such as `0-0-0`, `1standard`, `a/deep`, `🔣️json/<case>` glue dirs, `🧑️client/escaping-return`), `file-kind-unresolved` 25 (`*.orig` merge leftovers from Sep 29 `🌉️mcp`, `go.mod` under `🏖️sandbox`, `*.ops`, `*.wat`, `*.icc`), `nested-cargo-authority-invalid` 2, `symlink-absolute-target` 1, `path-emoji-*` 8.
7. `🌎️hub/📦️packages/🦀️rust/.semio/hub/{directory.db,db/catalog/root.bin}`: tracked runtime state from Aug 17 (also 2 unresolved directories). `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets` violates `target-inside-package-boundary`.
8. **Writer of the lockfile snapshots is unidentified.** Their content is older versions of the sibling `bun.lock`; clusters at 17:21, 19:29, 23:19 (10-18), 12:29, 14:36-14:45 (10-19), one per workspace, none since 14:45 although syncs ran at 16:56 and 17:07. The deps owner should confirm the dependency lock step stops leaking them; the ignore rule hides them, it does not stop them.
9. Process: the registry closure only accepts exact package names, so any new `@semio-tech/<pkg>/<subpath>` import reachable from `✏️s/🧑‍💻dev/📜️script.ts` (for example by the glue sweep) breaks every scoped `verify taxonomy` again; `registry-unbound.ts` is the 10-second check.

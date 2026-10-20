# Result tax6: per-scope taxonomy re-inventory, root-cause classification, first repairs

`C` = this ticket folder. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Tools (inputs, kept): `C/tax6-tools/`. Generated output (delete when the ticket closes): `C/🗑️generated/tax6/`, `C/🗑️generated/tax6-sim/`.
Nothing was committed, staged, stashed or reverted. Cargo, `bun repo run`, sync and `show projects` were not started. The TOTAL WRITE PAUSE (coordinator, 12:43) arrived while the library patch was still being measured: **the library/taxonomy patch is staged, not applied**. The only live repo writes are 6 pollution deletions made before the pause (section 3).

## 0. Verdict (validator, before, after)

"Before" = live inventory run after apply-wave2 (tax4 + tax5 staged sets applied), `C/tax6-tools/run.sh` per scope, 2026-10-20 ~12:00. "After" = the same live tree inventoried with the patched engine copy (`C/tax6-tools/sim-rebuild.sh` = real copy of `L` + `staged/apply.py`, siblings linked) plus the 6 live deletions. The root scope `.` (186k files) does not fit one 10-minute call; its content is the union of the scopes below.

| scope | before | after |
| --- | ---: | ---: |
| `🧰️framework` | 985 | **864** |
| `♻️mit-bestand` | 101 | 101 |
| `🌎️hub` | 22 | **21** |
| `🏢️semio-tech` | 6 | 6 |
| `👴️leutwiler` / `🧫️fixtures` | 1 / 1 | 1 / 1 |
| `🎓️teaching`, `🧬️schema`, `🧪️tests` | 0 | 0 |
| **all scopes** | **1,116** | **994** |

By validator, `🧰️framework` (the other scopes only move `semantic-stem-unresolved` 2 to 1 in hub and 4 in `directory-kind` etc. as listed in the diff output):

| code | severity | before | after | what changed it |
| --- | --- | ---: | ---: | --- |
| `semantic-stem-unresolved` | error | 319 | 206 | +76 print `.sty` and +14 Unicode UCD exact contracts (-90); `policy-document` directory kind (-22); test pollution (-1) |
| `package-implementation-file` | warning | 372 | 371 | destination now unique (see 2.2); 1 pollution file removed |
| `fixed-source-disposition-unresolved` | error | 83 | 84 | +1 is a new peer script (`💻️client/🔌️mcp/📜️script.ts`), not a regression |
| `taxonomy/kind-only-basename` | error | 44 | 44 | |
| `opaque-reference-target` | warning | 40 | 40 | |
| `directory-kind-unresolved` | error | 34 | 31 | 3 pollution directories removed |
| `mutation-fixture-unpaired` / `mutation-payload-schema-authority-invalid` | error | 29 / 26 | 29 / 26 | `🎚️config` owner decision (4.6) |
| `package-implementation-destination-unresolved` | error | 13 | 12 | pollution file removed |
| `package-role-unresolved` | error | 10 | 10 | |
| `file-kind-unresolved` | error | 10 | 6 | 3 `.orig` + `doc.spr` removed |
| other (`nested-cargo-authority-invalid` 2, `target-inside-package-boundary` 1, `symlink-absolute-target` 1, `fixed-source-content-unreadable` 1) | error | 5 | 5 | |

Per scope other than framework: hub `semantic-stem-unresolved` 2 to 1 (`policy`), `directory-kind-unresolved`/`package-implementation-*` unchanged except as noted in `tax6-tools/diff.py findings-before findings-sim3`; mit-bestand and semio-tech unchanged (4.4).

## 1. Method

- `C/tax6-tools/run.sh <abs list file> <parallel>` runs `inventory.ts` (live `inventoryTaxonomy`) per scope under `perl alarm 580`; `TAX6_SIM=tax6-sim TAX6_SCRIPT=sim-inventory.ts` runs the patched copy. `aggregate.py <dir> <out>` sums, `diff.py <a> <b> [scope]` diffs by (code, path). Raw files: `C/🗑️generated/tax6/{inv0,sim1,sim2,sim3}`, `findings-{before,sim1,sim2,sim3}.json`.
- Environment facts: `@semio-tech/framework-transient` is missing from the installed `node_modules` (installed generation predates the manifest), so any inventory/plan that loads `registry-catalog-inputs` fails. Workaround for tools only: `tax6-tools/tsconfig-shim.json` (`--tsconfig-override`, a `paths` entry to the source entry; a Bun plugin shim crashed Bun in the inventory workers). Running 5 scopes in parallel under the fleet load crashed Bun (SIGSEGV/SIGBUS/137); one scope at a time is reliable (framework 183 s).
- Diagnostics: `explain.ts <path>` (file kind, ancestor directory kinds, stem match per context), `router-check.ts <file>...` (command-router validator), `plan.ts` (inventory + `planTaxonomy`, optional patched engine).

## 2. Staged patch `C/tax6-tools/staged/apply.py [--check] [--root <L copy>] [--only <op>]`

Guarded read-modify-write of `L/🔣️taxonomy.json` (byte-exact JSON round trip, sha256 guard, atomic replace) and one anchored text edit; idempotent. `--check` against the live library: all 5 ops "would write". `validateTaxonomy` over the patched document: 0 problems.

1. `print-latex-package-contracts`: 76 exact `fixedFilenameContracts` `print-latex-<stem>` for the `.sty` files without one (same authority, reason, verification as the 95 existing ones; the schema audit demands one literal basename per contract, so a wildcard contract is invalid by design). -76.
2. `unicode-ucd-source-contracts`: 14 exact contracts `unicode-ucd-<name>` for the vendored UCD files (authority Unicode Consortium UCD, names recorded with url and sha256 in the sibling `🔣️provenance.json`). -14.
3. `policy-document-directory-kind`: global structural kind `policy-document` (emoji of `repo-test-policy`, slug `^policy$`). The 24 `🔣️policy.json` documents (library caching/build/workspaces/discovery, repo coordinator/test, hub stdio, process) get the canonical destination `⚖️policy/🔣️.json` instead of an unresolvable stem. Existing contextual `policy` kinds still win; none of the 24 destinations collides. -23 findings; the 24 moves need the transaction (4.1).
4. `package-implementation-destination-keeps-module-path`: rule bug in `packageImplementationDestination` (`L/🧹️normalization/📏️fixed-contracts/🟦️.ts`). The hoist onto the owner dropped every module directory between the package root and the file, so all `📦️packages/🦀️rust/<module>/🦀️.rs` modules of one crate got the same destination `<owner>/🦀️.rs` (documented as a known limitation in `💥️generic-stem-collision-resolution`). The destination now re-roots the file's canonical path below the package root onto the owner. Measured: 372 of 372 `package-implementation-file` destinations are distinct (before: heavy collapse).
5. `package-implementation-destination-test`: language-agnostic test + fixture (`L/🧹️normalization/🧪️tests/📦️package-implementation-destination/🟦️.ts`, `.../🧫️fixtures/📦️package-implementation-destination/🔣️.json`; 8 cases, each also re-derived with `node:path` posix `relative` as the independent implementation). `bun test` on the patched copy: 9 pass, 0 fail.

Apply when the pause is lifted and no `bun repo` process runs: `python3 C/tax6-tools/staged/apply.py`, then `bun test ./L/🧹️normalization/🧪️tests/📦️package-implementation-destination/🟦️.ts`, `bun C/tax-tools/validate.ts` (expect 0 problems), `bun C/tax6-tools/run.sh` on `🧰️framework` (expect 864). Editing `taxonomy.json` and the library invalidates a running dependency sync, which is why it is not applied during the pause.

## 3. Applied live (before the pause, outside L)

Test pollution, tracked and unreferenced (`git grep` found no reader), list in `C/tax6-tools/deleted-pollution.txt`: 3 `*.orig` patch backups in `💻️os/🔨️modules/🌉️mcp` and 3 test artifacts written into the `💻️os/📦️packages/🦀️rust` package root by tests whose `SEMIO_TEST_ARTIFACT_DIR` pointed there (`command-text-transfer/input`, `command-transport-neutral/input`, `retained-spr-command-context/doc.spr`; their directories are gone). Effect: -9 findings (`file-kind` 4, `directory-kind` 3, destination 1, stem 1). Not touched: the dependency-store capsule residue `.p6af50af4/{i,r}` and `.pbdb5915c/{i,r}` under `📚️course` (state of an interrupted sync, owned by the dependency tool).

## 4. Root-cause classification of what remains (framework unless noted)

1. **Transaction blocker (new, affects every rename batch).** `clean taxonomy plan` for `🪝️hooks` (8 moves, all correct and distinct with fix 2.4) ends with 67 unresolved decisions that block `apply` for ANY scope: 66 `reference-syntax-unsupported` (`rust-path-join-unproven`/`rust-path-join`) in 24 Rust test files (`owner.join("🦀️.rs")` style joins whose base the analyzer cannot prove: 38+6 in `✏️s` plugin tests, 21 in `🧰️framework` incl. `🗣️dsl/✨️derive/🧪️tests` 5, `🪝️hooks` 3 files, print `chart-mutations`, 1 in hub) and 1 `generator-preview-invalid` (`plugin-registry`, `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`). Fix options: rewrite the joins to a proven `Path::new(env!("CARGO_MANIFEST_DIR"))` base, or teach `inspectRustNonRepoJoinBaseSpans` the `let owner = ...` provenance. Rust edits, not verifiable without cargo, therefore not made. Plan cost: about 25 minutes per plan under fleet load (repo-wide incoming-reference scan); plan file `C/🗑️generated/tax6/plan-hooks.json`.
2. **`package-implementation-file` 371 (warnings).** Rust module files of root crates (`🦑️repo` modules 177, `💻️os` 101, presentation TS 19, hub stdio 7, ...) and TS/CSS/HTML package files. After fix 2.4 each has a unique owner destination, so moves are plannable; applying them rewrites `#[path]`/Cargo literals and must not run during the cargo window (`cargo-plan-tools/run.sh`) or before item 1 is cleared. Recommendation: one batch per module, TS/CSS/HTML first, Rust after the window with `C/tax4-tools/rs-path-check.py` before/after.
3. **`fixed-source-disposition-unresolved` 84 (+ mit-bestand 5, hub 5, semio-tech 1, leutwiler 1): `📜️script.ts` rejected by the command-router law.** `router-check.ts` reproduces all 95 rejects. Classes: router with logic in classes 60, birth/main programs 15, re-export-only files named `📜️script.ts` 5 (public surface entries `./script`; rename to `🟦️.ts` plus the `package.json` export key), worker-thread entries 3, other programs 12. The recorded diagnoses (`C/tax2-tools/remaining-router-law.tsv`) are no-module-slot, `import.meta` use, exports in the script. A tool exists (`C/tax-stage-tools/lift.ts`, `C/tax2-tools/restructure.ts`, shape owner `C/lift-fix-tools/shape.ts`); each of these 95 needs its own lift or rename, and the test-spawned child programs need the `📦️child/🟦️.ts` convention.
4. **Authored-document stems (`semantic-stem-unresolved` 206, `kind-only-basename` 44+).** Doctrine (`_treePurityComment`): kind-only basenames, semantic concern in a registered `emoji+slug` directory. The unresolved remainder: print `.tex` fixtures 79 and presentation `.md` fixtures 30 (case ids are file names; default destination collides: 30 files to one `📝️.md`, 79 to 22 `📐️.tex`), print `🎬️render-scene/🔣️<name>.json` 4, hashed Vite bundles under the gitignored generator output `📤️distribution/🧶️bundles` 38 (`dev-distribution-bundle` admits them as ignored generator output although its contract states a "hashed bundle partition"; they cannot be exact contracts), jcoprobe shims 5, module-root `package.json`/`tsconfig.json`/`project.json` 21 (manifest batch by a peer in progress), mit-bestand slides/web page 101 (`mit-bestand-document-children` pattern only admits the first level: its `ownerKindIds` must also list the pattern kind itself, then the 40 slide scripts rename to `<emoji><stem>/🟦️.ts`), semio-tech root files 6. Decisions needed, in this order of leverage: (a) corpus documents: directory per case with a pattern member kind under `fixtures` (renames plus feature/URI re-points), or a case table like tax5's `leafCaseTable`; (b) a `names: content-addressed` flag on `generatorContracts[].outputRoots[]` that exempts hashed partitions from naming laws (loader, audit, two type copies, inventory); (c) the mit-bestand pattern ownership.
5. **`directory-kind-unresolved` 31, `file-kind-unresolved` 6, `package-role-unresolved` 10, `package-implementation-destination-unresolved` 12, `opaque-reference-target` 40:** fixture directories named by data (`0-0-0`, `1standard`, `a/b`, `🧑️client/escaping-return`) need an emoji+slug rename or a pattern kind; unregistered fixture formats `.wat`, `.ops`, `.rb`, `.unknown` need `fileKinds` rows; the `node_modules` symlink inside the gitignored generator output `🕸️bindings` is admitted because the generator walk does not prune dependency directories; role-unresolved are Rust macro registrations, `.csproj`, `.cjs`; opaque references are warnings from `compose`/`recherche` mentions in comments, gherkin descriptions and law tests (a documentation mention is not a dependency; the scanner could skip comment/description contexts).
6. **`mutation-fixture-unpaired` 29 and `mutation-payload-schema-authority-invalid` 26:** the `🎚️config` module mutations are TypeScript-only shard directories without a `🔣️.json` descriptor and the `💻️os/🎚️config` fixtures have no canonical implementation case. Feature work (16 descriptors plus implementation cases), owner decision.
7. Other peers' in-flight work seen in the inventory and left alone: the module-root `package.json` batch, `💻️client/🔌️mcp/📜️script.ts`.

## 5. Verification (commands, outcome)

| command | outcome |
| --- | --- |
| `C/tax6-tools/run.sh` (live, one scope at a time) for 9 scopes | rc 0 each, numbers in section 0 |
| `TAX6_SIM=tax6-sim TAX6_SCRIPT=sim-inventory.ts ... run.sh` on framework, hub, semio-tech, mit-bestand | rc 0; framework 985 to 864, all scopes 1,116 to 994; diff shows 0 added findings from the patch |
| `bun test` of the new destination test on the patched copy | 9 pass, 0 fail |
| `validate-sim.ts` (patched) / `bun C/tax-tools/validate.ts`-equivalent (live) | `{"problems":0}` both |
| `python3 C/tax6-tools/staged/apply.py --check` on the live library | all 5 ops would write; nothing written |
| `plan.ts` with the patched engine on `🪝️hooks` | 8 moves, all distinct and correct; 67 unresolved (item 4.1) |
| `git grep` for readers of the 6 deleted files | none |

Not run: `apply`, any `clean taxonomy apply`, cargo, `bun repo ...`, the whole-repo `verify taxonomy`, the root scope `.`, the existing library law suites against the patched library (only the new test), `bun build` of the root script (untouched). The patch was not measured against the exhaustive laws that pin `fixedFilenameContracts` counts.

## 6. Open items

1. After the pause: apply the staged patch (section 2), re-run the framework inventory (expect 864), run `taxonomy-loading`, `path-emoji-statutes`, `package-boundary-classification` by explicit path.
2. Clear the 67 unresolved Rust joins (4.1); then plan and apply per-module move batches (4.2, the 24 `policy.json` moves, kind-only renames), TS first.
3. Decisions: corpus documents (4.4a), generated hashed partitions (4.4b), mit-bestand pattern ownership (4.4c), router-law remainder (4.3), `🎚️config` descriptors (4.6).

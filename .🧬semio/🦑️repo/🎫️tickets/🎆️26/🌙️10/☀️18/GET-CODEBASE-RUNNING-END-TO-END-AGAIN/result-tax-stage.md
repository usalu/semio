# Result tax-stage: staged library patch (router-law backlog, directory renames, dangling references, law-sweep edits)

`C` = this ticket folder, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `T` = `C/tax-stage-tools`. Nothing under the HARD FREEZE was edited (no `L`, root `📜️script.ts`, `package.json`, `🔣️taxonomy.json`); the patch is **staged, not applied**. Nothing was committed, staged in git, stashed or reverted.

## 0. Verdict

| item | before | after the staged patch (mirror) |
| --- | ---: | ---: |
| library `📜️script.ts` rejected by the command-router law | 43 | **3** |
| taxonomy inventory of `L` (166 findings before) | 166 | **122** (0 new, 44 removed: 40 `fixed-source-disposition-unresolved`, 4 `directory-kind-unresolved`) |
| emoji-only test directories | 4 | 0 |
| dangling references from tax2's renames (3 queued + 1 cluster found) | 3 + 9 | 0 |
| law-sweep queued edits | 2 | 2 applied |
| operations in the manifest | | 168 (144 write, 24 delete), 1 of them `package.json` |

The 3 remaining rejected scripts are process-birth entry points that the grammar cannot express (section 4). Everything else of the 43 is handled (section 2).

## 1. How to apply (exact order)

`T/staged/ops.json` is an anchored manifest: every operation carries the sha256 of the file it was derived from, `apply` refuses if any target changed or a dependency sync runs. Disjointness proof: no file of this patch is a target of tax3 `apply-1..4` (checked against their `rep(...)` paths) or of walkperf (`💾️storage/🟦️.ts`, `🏗️build/🚀️bootstrap/🟦️.ts`, `🗂️workspaces/🟦️bun/🟨️.cjs|🟦️.ts`, `🧾️generation/**`).

1. Sync stopped and freeze lifted (`apply.sh` refuses while `dependencies sync` / `deps4-*.sh` run).
2. walkperf: `sh C/walkperf-tools/apply.sh apply --dry-run`, then `apply`, then its `run-tests.sh real` (independent, any position).
3. tax3 `apply-1..4`: **already applied in the live tree** (coordination 01:40; `🧱️package-implementation-admission`, `🥒️mutation-fixture-coverage` exist). The mirror below was built from that tree, so this patch is verified on top of tax3. On a tree without tax3 the order tax3 then this patch (or any order, the files are disjoint).
4. `sh C/tax-stage-tools/apply.sh check` (expect `ready 168`), `sh C/tax-stage-tools/apply.sh apply --dry-run`, then `sh C/tax-stage-tools/apply.sh apply --restage`. `--restage` re-derives the manifest from the current tree first (peers edit hot files such as `🏗️build/🧪️tests/🕸️graph` all the time: two conflicts appeared and were cleared by restaging during this session). Restage reuses the reference index `T/index/` (8,697 candidate files, taken 00:40); rerun `T/refs.ts` after a repository-wide `git grep` for a fresh one when many files moved.
5. The single `package.json` operation is `L/⚡️caching/🚀️bootstrap/🛠️tools/package.json` (`./bazel/script` and `./script` export targets follow the renamed modules). It is coupled with the bazel module rename: apply it in the same window as the other dependency-manifest edits (the sync reloads it). `apply --skip-package-json` exists but leaves the two exports pointing at removed files.
6. After apply: `bun C/tax2-tools/census.ts` (expect 3 library scripts rejected), then the explicit-path tests of `T/verification-tests.txt` (85 files, never a directory, at most 2 concurrent) with `T/run-tests.sh live ...`.

Mirror/diff helpers: `apply.sh diff [substring]`, `apply.sh mirror DIR --copy <L>` (real copy of the library + patched files, symlinks elsewhere), `--no-ops` builds the pristine twin used as baseline.

## 2. What the patch contains

**Dispositions of the 43 library scripts** (`T/library-43.txt`; detail in `T/staged/log.json`):

| disposition | n | what |
| --- | ---: | --- |
| test-spawned child programs renamed | 18 | `<dir>/📜️script.ts` becomes `<dir>/📦️child/🟦️.ts` (the dir already named `📦️child` for the rustc child); relative imports rebased, every spawn site, source read, fixture and `bun ./📜️script.ts` command literal follows (bazel `📤️arguments` corpus: 7 literals). They are programs taking positional fixture arguments, not command scripts, so a router would change their argv; renaming keeps behaviour byte-identical (no-argument smoke of all 18 old vs new: identical first error line). |
| module-scripts renamed to `🟦️.ts` | 2 | `⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel` (+ `package.json` exports) and `🏗️build/🔧️tools/🏗️bazel` (97 KB, ~40 referencers: imports, source-reading laws, `callers.json`, action fixtures). |
| lifted: skeleton stays in `📜️script.ts`, classes/functions/programs move to a module | 19 | module = sibling `🟦️.ts` when free, otherwise `🧪️tests/🔬️script-commands/🟦️.ts` (precedent: tax2b; merging into a light shared `🟦️.ts` such as `⚡️caching/🟦️.ts` would have pulled bare imports into the CLI closure of every importer, so it is never done); packages script: `L/🧪️tests/🔬️typescript-package-commands/🟦️.ts`. `import.meta.dir|url|path` become module constants of the script location (exact semantics, e.g. the Cargo endpoint's `argv[1]` identity check); `new ScriptRouter(getWorkspaceRoot())` becomes `import.meta.dir` with `this.root`/`this.repoRoot` replaced by `getWorkspaceRoot()`/`findWorkspaceRoot(getWorkspaceRoot())` (env override preserved); `registerLazy` of the packages script becomes a delegating class with a dynamic import; the bounded-scope main of the native preparation script became two registered classes that open `withRepositoryCargoPreparationV1` per command. |
| edited in place | 1 | `🧪️tests/🐹️executable-owners` (router built once, no second argument). |
| not staged | 3 | section 4. |

Importers of moved exports are re-pointed (import position only); source-reading laws (`readFile(new URL("../📜️script.ts"…))`, AST laws) are re-pointed line by line to the module when the base directory is provable (`new URL(.., import.meta.url)`, `join|resolve(import.meta.dir, ..)`); the rest are explicit sites in `T/plans/lifts.ts` (`forceReads`, 12 sites incl. 3 fixture JSON `routerPath`). Spawn references stay on the script. Two law tests that encoded "the router exports its command classes" were updated: `🏗️build/🧪️tests/🕸️graph/🟦️.ts` + `🧫️fixtures/🕸️graph/🔣️.json` (new fixture field `commandModule`).

**Four directory renames** (library, the fifth `🧫️harness` was already fine): `🛂️` to `🛂️sdk-fences`, `🌿️` to `🌿️runtime-selection`, `📐️context/🦀️` to `🦀️rlib-context`, `🦀️profile/🧪️tests/🦀️` to `🦀️profile-precedence`; referencers re-pointed (capacity fixture ×2, dashboard authority test, `callers.json`, the profile test's `new URL("../🦀️.ts")` ×2 which had been dangling since tax2, profile `sourceOwnership.files`).

**Dangling references:** the 3 queued (`🏗️build/📥️inputs/💾️storage/📜️script.ts` ledger, `⚡️caching/📜️script.ts` image and scope literals, `🦀️.ts` context test + profile json) plus a cluster the queue missed: tax2's rename of `⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts` to `🟦️.ts` left 9 references (tools package `./script` export, 7 test reads/imports, `🔏️authorizers` scope test). Fixing them turned previously failing tools tests green (tools suite 0/8 to 3/5, cleanup 0/1 to 1/0, authorizers scope 2/1 to 3/0).

**law-sweep edits:** `🧷️composed-child-refs` skips `🎫️tickets`; `framework-viewport-ownership|projection` parse the `.graphql` files with `graphql@17` (`parseViewportGraphqlSchemasV1`, dynamic import) instead of `prettier --check`.

## 3. Verification (mirror at `/private/tmp/tsm`, pristine twin `/private/tmp/tsb`, both deleted at the end)

Mirror = real copy of `L` + the staged files + two real root manifests, everything else symlinked/copied; short path because tests enforce 256-character budgets. Baseline = the pristine twin (not the live tree: live-vs-mirror differences such as symlinked ancestry were mirror artifacts and were excluded by this twin).

- Router law over the mirror library (`T/verify-router.ts`): 50 scripts, **3 rejected** (the section-4 ones).
- Imports (`T/verify-imports.ts`): 133 changed files, 2,121 imports resolved, every named import checked against its target's exports; plus a global pass over 1,446 indexed files for importers of any `📜️script.ts`: 0 problems except 2 pre-existing (`@semio-tech/nx-bootstrap-tools/script` is not installed in the sandbox; `type BazelTaskV1` is not exported at HEAD either).
- Import-time smoke (`T/smoke-import.ts`, 50 scripts/modules, one process each): all load; 5 expected refusals (3 scripts execute their terminal at import without `import.meta.main`, as before; 2 hit the "real authority ancestry" check because the mirror is a symlinked path, identical in the pristine twin).
- Taxonomy inventory of `L` (`T/inv.ts`): 166 to 122 findings, 0 new.
- Tests, explicit files, pristine twin vs patched mirror (`T/compare-all.sh`, `T/cmp-tests.py`; failing-name sets compared): bazel 15 files identical (46/29 in the big file on both sides), caching groups 27 files identical except improvements from the dangling fixes, 17 law/fixture files for the packages script and fixtures identical, graph test identical up to its own flakiness, process/watch/headroom/retention identical. Regressions found by the method and fixed before this report: bare `join(import.meta.dir, "📥️purpose/📜️script.ts")` reads (bazel purpose), a base-directory mismatch (`resolve(owner, "../📜️script.ts")` resolves to the bootstrap script, so it is deliberately not rewritten), array-element source reads, three `routerPath` data vectors, the `moduleReceiving` law above.
- Not attributable / not proven: `🎛️dashboard/🧪️tests/🧭️authority` (9 failures; 3 reproduce in the pristine twin, the 4th walks the whole repository and times out in the twin, fails fast in the mirror on `AGENTS.md`); tests that need cargo, nx, Bazel, real Store leases or elevated disk were not run (cargo/nx/Bazel not started, per brief); Windows/Linux not run.

## 4. Not staged (needs a decision, no workaround inside the current grammar)

1. `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📜️script.ts`: the Cargo `rustc` refusal shim referenced by `.cargo/config.toml` must refuse **before** any process envelope exists and with `policy.refusal` verbatim (the admission test asserts it); a router needs `receiveScriptProcessInvocation`, which would fail first with a different message.
2. `⚡️caching/🚀️bootstrap/📜️script.ts` (57 KB) and `🏗️build/📜️script.ts` (the `bun repo` CLI): process-birth entry points that mint their own envelope (`nxBootstrapEnvelopeV1`, `runBuildCliV1`); the grammar only accepts terminals that receive one. The root `📜️script.ts` is the same class. Proposal: a birth terminal in the grammar and the routing module (`runScriptBirth(router, mint)`), plus an envelope-free refusal terminal for (1). Their implementation could then move exactly like the lifted ones (the 57 KB and 30 KB bodies are already modules in everything but name; 15 source-reading laws point at `🏗️build/📜️script.ts`).

## 5. Open items and caveats

- Concurrency: the manifest is guarded, but `restage` is needed after peer edits; run `apply.sh apply --restage` in the apply window. A fresh reference index is only needed if peers move or add files that mention `📜️script.ts`.
- Docstrings of generated program classes use a rotating emoji list; uniqueness across the repo is not enforced by this tool (law backlog `docstrings emoji-unique` already has 37k findings).
- New convention introduced: `🧪️tests/🔬️script-commands/🟦️.ts` (script command module) and `<dir>/📦️child/🟦️.ts` (spawned child program); the inventory accepts both without registration.
- Peer note: `C/walkperf-tools` and tax3 are untouched; `C/tax2-tools/restructure.ts` is superseded by `T/lift.ts` for library scripts.

## 6. Files

- Kept inputs: `T/` (`stage.ts`, `plans/{simple,children,modules,ghosts,lifts,laws}.ts`, `lift.ts`, `vfs.ts`, `refs-lib.ts`, `refs.ts`, `apply.py`, `apply.sh`, verifiers `verify-router.ts`, `verify-imports.ts`, `smoke-import.ts`, `dangling-scan.ts`, `inv.ts`, test drivers `run-tests.sh`, `batch.sh`, `compare-all.sh`, `cmp-tests.py`, lists `library-43.txt`, `t-children.txt`, `verification-tests.txt`, reference index `index/`, manifest and blobs `staged/`).
- Report: this file. Generated output (`C/🗑️generated/tax-stage`, `/private/tmp/tsm`, `/private/tmp/tsb`, test artifacts) deleted.

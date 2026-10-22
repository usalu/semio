# result-loader-own (executor loader-own, wave 1022a)

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Tools: `C/loader-own-tools/` (probe.ts, run-closure.ts, computed-table.ts, apply-computed-table.py, patch_*.py = the anchored patches that were applied live, live-log.txt). Generated: `C/🗑️generated/loader-own/`.

## Design (root fix, derived instead of hand-kept)
1. Literal sites are derived at capture time by one module, `L/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/` (schema-first: policy + EN/DE refusal Messages in its `🧬️schema/🔣️.json`): scanner imports + text sites `createRequire(..)("x")`, `createRequire(..).resolve("x")`, `require("x")`, named createRequire bindings, `import.meta.resolve("x")`.
2. Third-party packages are no longer a hand-kept list (`["typescript"]`): `captureNxAuthorizerSourcesV1(..., {installed:true})` resolves every bare site; a package resolved into `node_modules` becomes the import-map target `installed:<name>@<installed version>` (declared dependency image, version pinned into the closure identity). The loader loads it natively and verifies the installed version at startup and per use (EN/DE `installed-drift`).
3. Computed sites: declared once in the schema table `ComputedImportSites` (owner, count, roots); roots become import-map entries `semio-root:<path>` so a computed `import(abs)`/require of a declared root is served from the captured image. `importRevision` (bootstrap) lost its hand-kept thunk map `capturedRevisionImportersV1`; its 11 roots are the row.
4. Loader semantics: `createRequire(base).resolve()` is data-only resolution (never refuses; physical resolution, re-entrancy guarded), loading through a base outside the closure is refused (`declaring-module`); schema reads are refused only when the owning module is captured and the schema is not (analysis data of uncaptured modules and `package.json` stay readable); all refusals carry EN/DE text naming the site and the fix.
5. Node loader (`🟢️node/🟨️.mjs`) accepts `installed:` targets.
6. Laws: `authorizers/🧭️sites/🧪️tests` (fixture of sites vs TypeScript parser + independent Python token oracle; EN/DE texts), `closure/🧪️tests/🧭️sites` (real stream loader on a fixture workspace: package, subpath, root import, outside base, schema owner, drift; EN and DE), `authorizers/🧪️tests/🧭️closure-law` (all 5 kernel closures: no literal site the loader would refuse, computed table = TypeScript-AST count per file, roots admitted, installed markers match node_modules). The old `🧭️computed-imports` law (hand-picked scopes) is replaced by it.

## Live changes (03:22 - )
- 03:22 authorizers/🟦️.ts, closure/🟦️.ts, closure/🟢️node/🟨️.mjs, closure/🏗️compiler/🟦️.ts (installed:true) patched; new `🧭️sites` module.
- 03:23 `ComputedImportSites` bootstrap row (count, 11 roots); bootstrap/🟨️.mjs importRevision loads by `import(fileURLToPath(url))`.
- 03:30 `.resolve` data-only; `package.json` exempt from schema-read guard (coordinator fault 1).
- 03:3x `ComputedImportSites` regenerated from the closure AST (16 rows; 10 stale rows of files outside every command closure dropped).
- 03:5x `RuntimeDataCompanions` reduced 24 -> 2 rows (only the 3 data paths the capture cannot derive); the two worker/entry modules moved into `ComputedImportSites` roots (count 0 allowed for a worker spawn). `authorizers/🧪️tests/🧭️computed-imports` deleted (superseded by `🧭️closure-law`).

## Verification
| check | result |
|---|---|
| `bun test` `authorizers/🧭️sites/🧪️tests/🟦️.ts` (cwd = scratch, never the repo cwd) | 3 pass: sites == TypeScript parser == independent Python token oracle (8 fixture cases), EN/DE texts of all 7 codes, specifier classes |
| `bun test` `closure/🧪️tests/🧭️sites/🟦️.ts` | 2 pass: real stream loader on a fixture workspace (derived package/subpath/static import, resolve data-only, outside base refused, declared vs undeclared root import, owner-aware schema read, package.json under a schema dir, EN and DE texts, installed version drift refused) and the Node loader serving `installed:` targets |
| `bun test` `authorizers/🧪️tests/🧭️closure-law/🟦️.ts` | 3 pass over all 5 kernel closures (1,058 files for `runtime`): zero literal sites the loader would refuse, ComputedImportSites == AST count per file, roots admitted, markers == installed versions, no derivable data companion |
| `bun test` `closure/🧾️diagnostics/🧪️tests/🟦️.ts` (pins the loader guard conditions) | 2 pass with the new loader |
| `closure/🧪️tests/🕸️resolution`, `🟢️node/🧪️tests`, `🌊️stream` | need the repo test runner (invocation envelope): NOT RUN |
| `bun repo tools setup nx` | LIVE exit 0 (04:09, `Nx Version Local v23.2.0`) |
| real closure, resolve probe (`C/loader-own-tools/run-closure.ts` + `resolve-entry.ts`, observation module in the real 239-file closure) | `createRequire(<non-closure base>).resolve("typescript")` answers, missing modules fail with the natural MODULE_NOT_FOUND, no refusal |
| `bun repo show projects --json` (pid 73877, log `🗑️generated/loader-own/show-projects-5.log`), `show targets --json` (pid 3106, `show-targets-1.log`) | QUEUED 03:36 / 04:11 until 05:10 (then killed by me to free the 5 GB capacity grant; re-run both) behind `repository-graph-facts` EXCLUSIVE held by the cold play generation (pid 4325, 8 GB); run 2/3/4 (03:23-03:32) failed on faults that are fixed now: `Authenticated captured build command entry required` = a peer edited the loader text between parent start and child capture (transient), `Uncaptured implementation import refused ... process/routing` from `observedRequireResolve` = `.resolve` not yet data-only (fixed 03:30, slice-play added `__physicalResolveV1` in parallel; both work) |
| `@semio-tech/semio-tech-play:activate-prototype --dry-run` | not mine to run twice: peer slice-play's cold dry-run (pid 4821/4325) uses the new loader; I did not start another |

## Coordinator faults
1. package.json under `🧬️schema/` dirs: exempt in the guard; more generally a schema read is refused only if the owning module (parent of the nearest `🧬️schema` segment) is a captured code module and the schema is not captured (live 03:30, proved by the fixture cases `package-json-under-schema-dir`, `analysis-schema-of-uncaptured-owner`, `owned-schema-not-captured`).
2. `Captured require declaring module required` for `⚡️caching/🦀️cargo/📜️script.ts`: `createRequire(base).resolve()` is data-only for every base (image or not); only loading through a base outside the closure is refused.
3. `Authenticated captured build command entry required`: the check compares the parent's loader text (built from the physical modules at process start) with the child's; an edit of the loader file between those two instants fails the run by design. Not a fault of the end state: my last loader edit was 03:30, the last peer edit of it 03:33.

## How a future executor stays out of trouble
- A new `require("x")`, `createRequire(import.meta.url)("x")`, `import("x")`, `require.resolve("x")` with a literal specifier needs nothing: the capture derives it (relative -> image, package -> `installed:`). Install the package; a bare literal that is not installed is skipped by the capture and listed by the law.
- A computed specifier: add/adjust the owner row in `ComputedImportSites` (`C/loader-own-tools/apply-computed-table.py` regenerates counts from the TypeScript AST over all kernel closures, keeping roots) and list the modules it may load as `roots`.
- Refusals name site and fix in EN and DE (language from `SEMIO_LANGUAGE`/`LANG`, both when unresolved).

## Open
- Live `show projects` / `show targets` / dry-run: only the queue is missing; re-run them once `repository-graph-facts` is free (`C/loader-own-tools/live-log.txt` has the timeline of live changes).
- Redundant after this work: slice-play's `__physicalResolveV1` bypass in `🏗️build/👁️observation/🟨️.mjs` and the closure module (my `.resolve` is already data-only); remove after the live run confirms.
- Bun 1.3.14 segfaults a loader child whose workspace root is a symlinked path (macOS `/var/folders`): the tests use `realpath`.

## 06:52 Coordinator urgent: v3 cold run died in build:infer
- Fault: `import(new URL(...).href)` in `🧪️test/🟨️.mjs` (computed, absolute path) -> `Uncaptured implementation import refused ... inference/🟨️.mjs` with `declaringImage:true, declared:false, targetImage:true`. Why my law missed it: the file IS in the runtime kernel closure and its computed site is counted in `ComputedImportSites` (count only); the law verifies counts, not that the executed target is declared, so a counted computed `import()` of a captured image could still be refused at run time. A peer made the site literal at 06:51 (`loadInferenceModuleV1`).
- Root fix (06:52, `closure/🟦️.ts` `declaredTarget`): a computed import/require of an absolute path or `file:` URL that is a captured image is served without a per-importer declaration (the loader's invariant is "only captured images execute", the image set is the capture); roots stay the capture-admission mechanism and still decide what enters the closure; a non-image target is still refused with the EN/DE text. New fixture case `computed-import-of-captured-image` (4 tests pass incl. diagnostics guard pins).
- Per-project v3 records persisted: `.cache/b/g/m/inference/selections/8d8f21d7.../np` holds 25,434 node-project records and `rr` 1,887 result records (+ `members.jsonl`), 27,319 files written since 05:00 (42,679 total); a rerun reuses them.
- Relaunched ONE detached graph run: `bun repo show targets --json`, pid 35824, log `C/🗑️generated/loader-own/v3-show.log` (06:52). Deliberate handover: do not kill; I start no other graph run.

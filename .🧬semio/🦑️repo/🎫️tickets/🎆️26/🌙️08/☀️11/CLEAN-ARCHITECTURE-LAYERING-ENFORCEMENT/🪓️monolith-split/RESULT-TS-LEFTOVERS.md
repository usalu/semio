# Result: TS-LEFTOVERS (prefix `tsl`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. 2026-10-16 21:46 to about 22:30. `F` = this folder. `R` = repo root. `OS` = `🧰️framework/🛍️products/💻️os`. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Tools: `F/tsl-tools/`. Logs and JSON: `F/🗑️generated/tsl-*`. Pre-edit snapshots of every changed file: `F/🗑️generated/tsl-snap/`. No `[DEBUG]` line was added.

## Verdict

DONE for F-03, F-05 (a) and (b) and F-12. One item could not be verified as asked: the Nx launcher route (`bun repo run workspace:verify-typescript`) fails inside the launcher before it reaches the script (foreign fault, section 6); the script route that the Nx target calls was run and works.

| Item | Result |
| --- | --- |
| F-03 permanent route | `bun ./📜️script.ts verify typescript <project\|check>` generates a relative-path `tsconfig` from the live checkout (`git ls-files` plus the root `exclude` list), so no root list can go stale. Whole workspace: 13,577 roots, 1,919 diagnostics in 552 files, 352 s. |
| F-03 stale lists | Regenerated from the live tree as `F/🗑️generated/tsl-roots-{psc,sdk2,tte-psc-live,tte-sdk2-live}.txt` plus generated projects. The 4 "dropped" roots are not coming back (section 3). |
| F-05 (a) worker cycle | Cut. The worker no longer imports `backbone-parity`. No neutral module was needed (section 4). |
| F-05 (b) loops H, I, J | All three cut with the self-registering convention. |
| F-12 | `⚖️retirement-law` paths are within the AGENTS 256-character limit: no rename. `assert_eq!*(`: 0 sites. |

## 1. Baselines (recorded before my first source edit)

| Check | Baseline |
| --- | --- |
| `no-circular`, OS modules plus `OS/🎚️config` (`F/tsl-tools/cruise.sh before`, rule reduced to `no-circular` as in `F/otc-tools/nocycle.cjs`) | 475 violations; `OS/🔨️modules` without the renderer: 4 (H, I, J and the gitignored vite bundle K); worker tree: 1; `OS/🎚️config`: 0 |
| `no-circular`, worker closure (`F/tsl-tools/worker-cruise.sh before`: `worker`, `worker-law-suites`, `worker-law-fakes`, `backbone-parity`) | 106 violations (closure-wide), 2 lines naming `backbone-parity`/the worker |
| vitest `OS/🧪️tests/🎚️config` (`F/tsl-tools/vt.sh`, hand-built `SEMIO_VITEST_POLICY`, level `long`) | 22 files, 609 tests, 499 pass, 110 fail (the actor-transport file) |
| vitest window-kits config | 6 files, 13 pass |
| vitest plugin-store config | 1 file, 9 pass |

## 2. F-05 cuts

### (a) Worker to `backbone-parity`

Cause: two edges. The worker's in-source block did `await import("../🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts")` and passed ~130 names, and the test imported the types `ArtifactState` and `BackboneWorkerTestDependencies` back from the worker.

The names do not need a new module: the worker already exports `documentWorkerTestDependenciesV1()` and the type `BackboneWorkerTestDependencies`, which are exactly that bag (the `worker-law-suites` aggregator uses them). So the seam existed and the cut is the self-registering convention of `RESULT-TS-AUDIT-FIXES.md` finding 2:

- `OS/🔨️modules/🏪️store/👷️worker/🟦️.ts`: the two lines (dynamic import and the 130-name registration call) are deleted from the in-source block. Nothing else in the worker changed; the other two in-source registrations stay.
- `OS/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts`: imports `* as vitest from "vitest"` and `documentWorkerTestDependenciesV1`, types `Vitest` as `typeof import("vitest")`, and ends with `await registerBackboneParityTests(vitest, documentWorkerTestDependenciesV1());`.
- `OS/🧪️tests/🎚️config/🟦️.ts`: the test file is an `include` entry (not in `includeSource`, so it is not collected twice).

No store, actor, member or media internals were touched beyond those two worker lines; no neutral module exists, so nothing is listed instead.

### (b) Loops I and J

`F/otc-tools/insource/📜️script.ts <module> apply` (the OTC converter, dry run first). It removed the `if (import.meta.vitest)` block and its region from the module, appended the registration to the test file with `import * as ModuleUnderTest`, constants for `import.meta.dir/url`, and `await import("vitest")`.

- I: `OS/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts` (6 lines out) and `…/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts` (15 lines in). Config `…/🏪️store/🧪️tests/🎚️config/🟦️.ts`: `include` now lists the test, `includeSource` only `🟦️.ts`.
- J: `OS/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts` (4 lines out) and `…/🎬️media/🧪️tests/🧪️rendermedia/🟦️.ts` (8 lines in). Config `…/🪟️window-kits/🧪️tests/🎚️config/🟦️.ts`: `include` gained the test, `includeSource` only `📝️text/🟦️.ts`, the docstring now says only the text kit is still in-source.
- `git diff --stat` per file was proportional after each step. No launch entry referenced these modules (checked with `/usr/bin/grep` in `.vscode/launch.json`).

## 3. F-03

### Why the old lists failed

`tsc` stops before semantics on any stale `files` entry (TS6053). The old lists were absolute-path JSON, frozen at one moment. Also, `psc-tsconfig-after2.json` extended the root `tsconfig.json` without `include: []`, so the root `include` (`**/*.ts`) applied: the "psc" baseline of 2,722 and later 2,454 diagnostics in `RESULT-TSC-UNBLIND.md` and `RESULT-TS-TYPE-ERRORS-1.md` was effectively a whole-workspace check (880 files carried diagnostics from a 322-file root list). `sdk2` had `include: []` and was a real 256-root check.

### The permanent route

New module `L/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts`, dispatched from `L/🎛️commands/✅️verify/🟦️.ts` (7 lines: import and the `typescript` branch). Commands (run from `R`):

```
bun ./📜️script.ts verify typescript project [flags]   # writes the tsconfig, prints roots=N
bun ./📜️script.ts verify typescript check   [flags]   # writes it, runs tsc, counts diagnostics, exits 1 above --max-diagnostics (default 0)
flags: --scope <path>…  --changed  --files-from <list>  --out <file>  --heap-mb <n>  --timeout-minutes <n>  --max-diagnostics <n>  --list
```

- Roots = `git ls-files --cached --others --exclude-standard` for `*.ts`/`*.tsx`, present on disk, minus the root `tsconfig.json` `exclude` list (read from the file, matched like TypeScript does: a pattern also drops everything below a matching folder), minus a `name.d.ts` shadowed by `name.ts`/`name.tsx`, sorted by code point. `--scope` keeps path prefixes at a segment boundary, `--changed` keeps files that differ from `HEAD`, `--files-from` keeps only listed paths and prints the listed roots it dropped as missing or excluded (so a saved list can no longer break `tsc`).
- The generated project uses relative paths only (`extends`, `files`, `paths`), `include: []`, `types: ["bun","node"]`, the same three ambient `paths` shims as the old lists (kept only while their target exists), so it is portable across macOS, Linux and Windows. Default home: gitignored `.cache/semio/typescript/` (`--out` overrides; the log is `<out>.tsc.txt`).
- `check` runs `node --max-old-space-size=<75% of RAM, 4 to 24 GiB> node_modules/typescript/bin/tsc -p <project> --noEmit --pretty false`, prints a heartbeat every 60 s on stderr, forwards SIGINT/SIGTERM to `tsc` (cancellation), kills it after `--timeout-minutes` (default 180), and distinguishes diagnostics (exit 2) from a crash (for example 134 for heap exhaustion).
- Wiring: `📋️project.json` targets `verify-typescript-project` and `verify-typescript` (same shape as `verify-generated-corruption`, `forwardAllArgs`), and the launch entries `📦️check🌳️workspace🟦️types` (206.1641) and `📦️generate🌳️workspace🟦️tsconfig` (206.1642) after the `generated🔬️corruption` entry, identical text in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`. Both files still parse (`json.loads`, `Bun.JSONC.parse`).

### Test (language-agnostic table plus two independent oracles)

`L/🧪️tests/🔷️typescript-project/🟦️.ts` with fixture `L/🧪️tests/🧫️fixtures/🔷️typescript-project/🔣️.json` (schema id `semio.repo.library.typescript-project/v1`): 8 selection rows (whole, scope, two scopes, file scope, changed-only, intersection, empty, listed roots). Third-party validation:

- `ts.parseJsonConfigFileContent` over a scratch tree with the same `include`/`exclude` must resolve the same file set as our selection (it showed the `.d.ts` shadowing rule, which I then implemented).
- The generated document, parsed by TypeScript, must resolve to the same roots with relative paths only.
- A real `tsc` run on a scratch project with two known errors must give `{total: 2, files: 1, TS2322: 1, TS2304: 1}` through our summarizer.
- Strict flag parsing, root-list reading, and a live-workspace test (all roots exist, none under `node_modules`, `🗑️generated` or the tickets folder, and the exclude list equals what `ts.readConfigFile` reads).

`bun test ./L/🧪️tests/🔷️typescript-project/🟦️.ts`: 16 pass, 0 fail. `bun ./📜️script.ts verify docstrings emoji-unique --files-from <the two new .ts files>`: 0 repeats.

### Regenerated lists and new baseline

`F/tsl-tools/roots.py` converted the four old tsconfig lists (relative, live-only) and the route regenerated the projects:

| Old list | Roots | Stale entries dropped |
| --- | ---: | --- |
| `psc-tsconfig-after2.json` → `tsl-roots-psc.txt` | 318 → 313 live (312 after the route's exclude/shadow filter) | os `🧵️backbone/🟦️.ts`, and the 4 below |
| `tte-tsconfig-psc-live.json` → `tsl-roots-tte-psc-live.txt` | 313 | none (identical set to the live psc list) |
| `sdk2-tsconfig-after.json` → `tsl-roots-sdk2.txt` | 256 → 255 | gis `🧪️tests/🎚️config/🟦️.ts` |
| `tte-tsconfig-sdk2-live.json` → `tsl-roots-tte-sdk2-live.txt` | 255 | none |

The 4 roots of `tte-psc-dropped-roots.txt` are gone for good, not stale by accident: the directories `🧬️mutations/{📂️opening-config-mutation,📌️set-default-app,🧹clear-default-app}` hold no `🟦️.ts` any more (all 14 leaves under `OS/🎚️config/🧬️schema/🧬️mutations/*/` lack a `🟦️.ts`; the parent `🧬️mutations/🟦️.ts` was rewritten at 17:54 and no longer imports them) and `♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input` has no `🟦️.ts`. The originals in `F/🗑️generated` were left untouched.

New baselines (all with the permanent route, tree as of 22:09, `--max-diagnostics 1000000` so the run prints instead of failing):

| Project | Command | Roots | Diagnostics | Files | Time |
| --- | --- | ---: | ---: | ---: | ---: |
| sdk2 roots | `bun ./📜️script.ts verify typescript check --files-from F/🗑️generated/tsl-roots-sdk2.txt --out F/🗑️generated/tsl-sdk2.tsconfig.json` | 255 | **24** (was 52, then 61) | 9 | 89 s |
| psc roots only (`include: []`, true closure of the 312 roots) | same with `tsl-roots-psc.txt` | 312 | **57** | 25 | 96 s |
| **whole workspace** | `bun ./📜️script.ts verify typescript check --out F/🗑️generated/tsl-whole.tsconfig.json` | 13,577 | **1,919** (the old psc baseline: 2,454) | 552 | 352 s, 24 GiB heap, exit 2 |

Whole-workspace top codes: TS2345 358, TS7006 289, TS2769 277, TS2339 239, TS18046 94, TS2741 92, TS2307 90, TS2322 61. By area: repo modules 468, os modules 253, mit-bestand deck 157, stdio plugin 120, docklayoutstore test 79, process artifacts 72, print tests 56, os tests 50. Against `tte-psc-after.txt` (2,454): 1,826 diagnostics are the same (file, code, message), 628 gone, 93 new (peers' concurrent edits; not attributed one by one). The whole check is the number to compare from now on; it needs no list. Default gate behaviour: `verify typescript check --files-from tsl-roots-sdk2.txt` without `--max-diagnostics` exits 1 with "24 diagnostic(s) exceed the allowed 0" (`tsl-sdk2-gate.route.txt`).

## 4. Verification of F-05

| Check | Before | After |
| --- | --- | --- |
| `no-circular` OS modules plus config (`cruise.sh`) | 475 total; `OS/🔨️modules` without renderer 4; worker tree 1; touching `backbone-parity` 1, `📥️installation` 1, `🎬️media/rendermedia` 1 | 472 total; without renderer 2 (the vite bundle K and one renderer `ShellHost` pair, a peer's, which the cruiser prints with a damaged first emoji); worker tree 0; the three modules: 0 each |
| `no-circular` worker closure (`worker-cruise.sh`) | 106 violations | 105 violations; no line names the worker or `backbone-parity` (diff of the rest shows only peers' kernel/actor/manifest loops moving) |
| vitest OS config | 22 files, 609 tests, 499 pass, 110 fail | 23 files, 609 / 499 / 110; **status+title sets identical** (`tsl-cmp-os.diff` empty). The 10 backbone-parity tests (including "a mounted browser actor retracts a refused history step …") now come from the backbone-parity file |
| vitest window-kits | 13 pass | 13 pass, identical; `rendermedia` 3 tests now from their own file |
| vitest plugin-store | 9 pass | 9 pass, identical; all 9 from the installation-identity test file |
| `tsc` (new route, whole workspace, 13,577 roots) | n/a (route did not exist; whole-workspace baseline 1,919 is taken after all edits) | 0 diagnostics in any file I touched: worker, `backbone-parity`, installation module and test, media module and test, the three vitest configs, the verify module, the new test (`/usr/bin/grep -c` over `tsl-whole.tsconfig.tsc.txt` gives 0 for each). All of them are roots in `tsl-whole.tsconfig.json`. |

The `tsc` row has no "before" for the touched files: I did not revert my own edits to take one. The argument that the edits introduce no error is the zero count in files whose only change is removed or added import/registration lines, together with the unchanged vitest sets (a type error alone would not change vitest, so this part is `tsc`-evidenced only for the after state).

## 5. F-12

- (a) Three `⚖️retirement-law` test paths exceed 240 bytes: 248 bytes (`💡️reasoning/…/🔌️wires/…/🕸️canvas/…`, 163 characters, 176 UTF-16 units), 246 bytes (`📋️forms/…/▶️try/…`, 161 characters) and 241 bytes (`✒️writer/…/✒️main/…`, 159 characters). The AGENTS limit is 256 characters, so none breaks it. Over all 167,237 tracked and untracked paths (`git ls-files` plus untracked) the maximum is 207 characters / 226 UTF-16 units / 314 bytes; 0 paths exceed 256 characters or 256 UTF-16 units, also with a 28-character absolute root prefix. 11,373 paths exceed the fleet's 240-byte budget (3,059 exceed 256 bytes), so these three are not outliers. **No folder was renamed**, because a rename touches Rust `#[path]` references for no gain.
- (b) `git -c core.quotepath=false grep -n -F 'assert_eq!*(' -- '*.rs'` returns no match (exit 1); 27 untracked `.rs` files also contain none. The named files are fixed: `…/🐚️brep-topology/🧪️tests/🔬️unit/🦀️.rs` has 1 `assert_eq!(*` in the right form, and the 15 tracked files named `geometry/🧪️tests/🔬️unit/🦀️.rs` or `brep-topology/🧪️tests/🔬️unit/🦀️.rs` (the 7 geometry sites included) all parse in `rustfmt --check` (no `error` line; only formatting diffs such as import order, not touched).

## 6. Commands run (all from `R`) and foreign findings

1. `F/tsl-tools/cruise.sh before|after`, `worker-cruise.sh before|after`, `count.cjs` (dependency-cruiser, 1.5 to 2 min each).
2. `F/tsl-tools/vt-all.sh before|after` (vitest through `tsl-tools/vt.sh`), `taf-tools/vsum.cjs`, `diff` of status+title columns.
3. Python edits with asserted anchors and snapshots; `bun F/otc-tools/insource/📜️script.ts <module> dry|apply` for I and J.
4. `bun test ./L/🧪️tests/🔷️typescript-project/🟦️.ts` (16 pass); `bun ./📜️script.ts verify docstrings emoji-unique --files-from …` (0 repeats).
5. `F/tsl-tools/tsc-runs.sh` (sdk2, psc, whole) and the gate run `tsl-sdk2-gate.route.txt` (exit 1, 24 diagnostics).
6. Read-only git only (`ls-files`, `grep`, `diff`, `log`, `ls-tree`, `status`) with `-c core.fsmonitor=false` where useful (the fsmonitor daemon prints `error: daemon terminated` on stderr but results are correct).

Foreign (not mine, not fixed):

- `bun repo run workspace:verify-typescript-project --scope …` fails in the launcher before running the script: "Exact canonical persistent Native publication required" at `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:122` (native image cleanup, after an `[DEBUG] Actual exclusive lease image retired …` line that the repo itself prints), 10 minutes. The same launcher would run the new targets once it works; the `bun ./📜️script.ts verify typescript …` command each target calls was run directly.
- The TS leaves of `OS/🎚️config/🧬️schema/🧬️mutations/*` are gone, including the four contract leaves that OS-TS-CYCLES created (those folders are now empty); the parent was rewritten at 17:54. So the OTC cuts "3.2" of `RESULT-OS-TS-CYCLES.md` no longer exist as TypeScript, and F-05 (c) (Rust mirror) is moot for the TS side. Someone retired the TS twins on purpose; I did not verify who.
- The 1,919 whole-workspace diagnostics are the repo's: repo library (468), os (253), mit-bestand deck (157), stdio (120), docklayoutstore test (79), the missing workspace packages (90 TS2307). None is in my files.

## 7. Files

Created: `L/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts`, `L/🧪️tests/🔷️typescript-project/🟦️.ts`, `L/🧪️tests/🧫️fixtures/🔷️typescript-project/🔣️.json`.

Edited (mine): `L/🎛️commands/✅️verify/🟦️.ts` (+7), `📋️project.json` (+16, two targets), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (two entries each), `OS/🔨️modules/🏪️store/👷️worker/🟦️.ts` (−2 lines), `OS/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts`, `OS/🧪️tests/🎚️config/🟦️.ts`, `OS/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts`, `…/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts`, `…/🏪️store/🧪️tests/🎚️config/🟦️.ts`, `OS/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts`, `…/🎬️media/🧪️tests/🧪️rendermedia/🟦️.ts`, `…/🪟️window-kits/🧪️tests/🎚️config/🟦️.ts`. The `git diff --stat` of the launch files, `📋️project.json` and the worker also contains peers' edits (the launch files were already modified at the start).

Temporary: `F/tsl-tools/` (scripts), `F/🗑️generated/tsl-*` (cruise JSON/text, vitest JSON/rows/logs, tsc logs, generated projects, root lists, `tsl-snap/`). The scratch files I made in `/tmp` and the one `.cache/semio/typescript` project from my first manual run were removed.

## 8. Hand-offs

1. Taxonomy registration (held file `L/🔣️taxonomy.json`): `🔷️typescript-project` under `🎛️commands/✅️verify` and the test folder `🧪️tests/🔷️typescript-project`. The emoji `🔷️` is RGI, used as a folder prefix elsewhere, and unique among both sibling sets.
2. The launcher fault above blocks `bun repo run workspace:verify-typescript*`; re-run `bun repo run workspace:verify-typescript-project --scope <path>` when it is healthy.
3. Whether to wire `verify typescript check --max-diagnostics <n>` into a CI gate is a policy call (the repo has no baselines by rule): the gate is red at 1,919 today.
4. The missing workspace packages (90 TS2307) still need the install decision of `RESULT-TSC-UNBLIND.md` family 3; the route has no `paths` for them on purpose.

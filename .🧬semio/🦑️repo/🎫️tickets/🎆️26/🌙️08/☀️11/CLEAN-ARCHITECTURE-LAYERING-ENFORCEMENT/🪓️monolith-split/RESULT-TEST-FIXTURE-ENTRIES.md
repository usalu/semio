# Result: TEST-FIXTURE-ENTRIES (`tfe`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Run of 2026-10-16 16:20 to 19:00. `F` = this folder. `FW` = `🧰️framework`. `OS` = `FW/🛍️products/💻️os/🔨️modules`. Tools: `F/tfe-tools/` (scanner, generator, runners, probes, `orig/` pre-edit copies of every touched file plus `index.tsv`). Logs and raw outputs: `F/🗑️generated/tfe-*` (kept).

## 1. Verdict

Done for the free areas: 37 new and 1 extended public fixture entries (`<module>/🧪️tests/🧫️fixtures/🟦️.ts`), 196 test files re-pointed (about 750 call sites), 0 re-pointable cross-module fixture reads left in framework and os module tests. The ones left are named in section 7. The coordinator's late finding (12 renderer files failing with `The URL must be of scheme file`) was my defect in the first entry version; it is fixed (section 5).

| Metric | Before | After |
|---|---:|---:|
| Cross-module fixture or data reads by literal path in framework and os module tests, whose owner is a free area (my work set: relative and `🧰️framework/…` root literals, existing target, entry files excluded) | 726 sites | 7 sites, none of them a read (3 docstring mentions, 2 `catalog:` property strings, 2 logical-key strings) |
| Same metric, wide scan (`tfe-tools/scan.ts`, counts every relative or root literal naming another module's fixture, JSON or fixture folder, in `🧰️framework`, `✏️s`, `🌎️hub`, excluding `.rs`/`.toml`/`.md`/`.txt` targets and entry imports) | 1,211 | 481 |
| of which framework and os | 958 | 232 (plus 58 owned by `📚️library`) |
| of which `✏️s` plugins | 107 | 106 |
| of which `🌎️hub` | 88 | 85 |

(`✏️s` and `🌎️hub` moved by peers, not by me.) The remaining 232 are broken down in section 7.

## 2. Found state

No cut predecessor. The two residuals named in the brief were the starting point: the 48 ui retained and resident corpus paths and the 34 fixture paths of other modules read by the typedwire topics (UiDocumentStore) and the engine-contract topics. Both are done. The wide scan then showed the same pattern in about 150 more test files; those are done as well.

## 3. What changed

### 3.1 The entries

Entry path: `<owner module>/🧪️tests/🧫️fixtures/🟦️.ts`. Docstring first character `🧫`. Per target file, only the exports consumers need:

- static consumers: `export { default as <name> } from "<rel>.json" with { type: "json" };` (133 exports, 12 of them the pre-existing ui ones);
- `await import("<rel>.json")` consumers: `export const load<Name> = () => import("<rel>.json").then(module => module.default);` (152 lazy loaders, so large corpora are only read by the test that wants them; 3 `…Text` loaders use `?raw`);
- `new URL(lit, import.meta.url)` and `readFileSync(resolve(…, lit))` consumers (file-path needs, for example Rust twins reading the same file): `export const <name>Url = fixtureUrl(import.meta.url, "<rel>");` (83 URL exports).

Names are derived from the owner folder plus the last three path words and a kind suffix (`Fixture` for `🧫️fixtures`, `Schema` for `🧬️schema`, `Contract`, `Document`), unique per module. No file moved. Existing ui entry lines were kept verbatim.

| Owner | Entry |
|---|---|
| fw modules (27) | `🎭️actor` (also the pre-existing nested `🎟️retained-turn/🏛️bootstrap` entry stays), `🎠️kernel`, `🌱️value`, `🛂️manifest`, `🖱️ui` (extended: +120 exports), `🧊️3d`, `📡️replication`, `🧵️job`, `🚪️io`, `🗺️surface`, `🖼️assets`, `🖼️canvas`, `🪪️identity`, `🏃️process`, `🎒️pack`, `🏗️mesh-engine`, `🕹️interaction`, `🎓️onboarding`, `✍️editor`, `⚠️diagnostic`, `⏱️trace`, `⏯️tool-run`, `🌉️abi`, `🧩️action-argument-resolution`, `🛠️tool-machine`, `📁️filesystem` |
| os modules (11) | `📇️directory`, `🏪️store`, `♾️infinite`, `🌊️flow`, `🚪️io`, `📡️spr`, `🧠️neural`, `🛢️db`, `🌉️mcp`, `🖥️shell`, `🌿️vcs`, `🧑‍💻dev` |

The entry list is `F/tfe-tools/orig/index.tsv` (rows starting with `NEW`). Each entry imports one helper, `fixtureUrl`, from `FW/🔨️modules/🧪️test/📖️source-text/🟦️.ts` (the existing sdk2 module), where I added the function (section 5).

### 3.2 The consumers

196 test files (section 3.3 lists the renderer and ui ones by area), edited with the AST-aware generator `tfe-tools/gen.ts` (`work.ts` finds the literals, `forms.ts` classifies each as static import, `{ default: X } = await import(…)`, `(await import(…)).default`, `new URL(…, import.meta.url)`, or the path argument of `readFileSync`, `readFile`, `existsSync`, `statSync`, `Bun.file`; anything else is left for hand edits). Imports of one owner are merged into one `import { … } from "<entry>"` line; local names never collide (checked); every edited file passed a TypeScript syntax transpile before write; every file was re-read right before its write and refused if it changed meanwhile. About 30 sites were edited by hand (helper functions `read(path)`, `require(…)`, the three `?raw` imports, `Bun.file(literal)`).

By consumer area: renderer `🧑‍🎨engine` tests and elements (including all UiDocumentStore topics, Interpreter, AgentBridge, World3dHost), `🔌️plugin` tests (per the brief: its owner entries are in free areas), `🏪️store`, `🌊️flow`, `🚪️io`, `📡️spr`, `🧑‍💻dev`, `🖱️ui`, `🎠️kernel`, `🧵️job`, `🗣️dsl`, `🏃️process`, `🧪️tests` at the framework root and `💻️os/🧪️tests`.

### 3.3 Not touched

`🖱️ui/🧬️contract/**` (read only; the entry sits outside it), renderer TS non-test code, `🔌️plugin/**` as an owner (no entry there), `📚️library`, Rust files, `.vscode` (no new test entry points were created), taxonomy (see hand-offs).

## 4. Equivalence and verification

### 4.1 Every entry loads in a fresh process

`tfe-tools/verify-entries.sh` runs `entry-probe.ts` in one fresh `bun` process per entry: imports it, awaits every lazy loader, checks every URL exists and every eager export is defined. Final result (`tfe-entries-probe3.txt`): 38 of 38 entries, 0 bad (this run is after the `fixtureUrl` change).

Under vitest with jsdom (the environment that broke): `tfe-tools/jsdom-probe/all-entries.test.ts` imports all 39 entry files (the 38 plus the actor bootstrap entry), asserts each URL export is a `file:` URL that `readFileSync` reads, and awaits every loader: 39 of 39 pass.

### 4.2 Same file by construction, checked

`tfe-tools/equiv.ts` re-collects every original literal from the pre-edit copies (`orig/`, 726 literals) and checks that its target file is among the paths named by the owner's entry: 726 of 726 (`tfe-equiv.txt`). A missing export name or a wrong type would additionally show as a `tsc` error (none do, 4.4).

### 4.3 Tests (baseline taken before the first consumer edit, same commands after)

Runners: `tfe-tools/run.sh` (set 1), `run2.sh` (set 2), `run3.sh` (single vitest filters). bun: `bun test --timeout 120000 <abs files>` with junit and the `(fail)` lines; vitest: the renderer react config with `SEMIO_TEST_LEVEL=exhaustive`, JSON reporter, filters are path fragments.

| Set | Files | Command | Before | After | Result |
|---|---:|---|---|---|---|
| 1, bun | 66 | `run.sh base bun` / `run.sh final bun` | 614 tests, 511 pass, 102 fail, 1 skip | 624 tests, 520 pass, 103 fail | failure sets identical except 1 extra failure `neutral compiler cache retirement declares first preparation failure before observation`; it is in a file a peer re-pointed to an SDK entry at 18:09, and it passes on a solo re-run (not reproducible), so peer churn or load. The earlier run (before the flow-unit path fix) had 12 failures fewer only because one file did not load (fixed, 4.5) |
| 1, vitest | 20 files that load | `run.sh base vitest` / `run.sh final vitest` | 525 tests, 419 pass, 106 fail | 535 tests, 429 pass, 106 fail | titles identical; status diffs: 2 TypedWire tests fail to pass (`OwnedSceneGenericPack…`, `OwnedScenePackedField…`, both stable across 3 runs), and one `selection-focus` test differs per run (run 1 `menu filters and grid…`, run 2 `selection survives focus…`); that file passes 53 of 53 twice when run alone (`tfe-solosel*`), so load flakiness, 0 extra failures in the stable sense |
| 2, bun | 21 | `run2.sh base2` / `final2` and a re-run after the last hand edit (`final3`) | 296 tests, 223 pass, 73 fail | 296 tests, 223 pass, 73 fail | failure sets identical |
| 2, vitest | 16 filters | same | 1 file loads (`📏️modelling`, fails to resolve the wasm node-graph package) | same | identical |
| 3, bun (`💻️os/🧪️tests` and `🧪️tests`) | 17 | `bun test` on the list in `tfe-run-bun3.txt` before and after (`base3`, `final4`, `final7`) | 61 tests, 48 pass, 13 fail | 61 tests, 48 pass, 13 fail | failure sets identical, run twice after |
| flow wasm unit (the file I broke with a wrong relative path and then fixed) | 1 | solo | 12 failing names in the baseline | 22 pass, 12 fail, the same 12 names | identical |
| fixture-placement laws of the owners (value, pack, actor, process, ui, replication, manifest, surface) | 8 | `bun test` solo | no baseline without entries | value, manifest, replication pass; pack, actor, process, ui fail | every failure names a pre-existing misplaced fixture of another path (`pack/…/📨️wire/🧫️fixtures`, `job/…/🧪️testing/…/🧫️fixtures`, `process/🧪️testing/…`, `ui/🎬️scene/📐️math/📦️resource/🧫️fixtures`); none names an entry file, so the entries do not cause them |

An important coverage limit, measured: of the 101 vitest consumer files, only 17 to 20 load in the react config in this tree. The others fail at file level before and after (missing wasm node-graph package; at the end also a peer's `@semio-tech/assets/published-page` alias). For these the evidence is 4.1, 4.2, 4.4 and the diff of every changed line (`tfe-tools/showdiff.py <fragment>`), not test results.

The re-run after the `fixtureUrl` change is section 4.6.

### 4.4 Type-check

Scoped `tsc --noEmit -p` (extends the os tsconfig; roots = all consumer files plus all entry files; `tfe-tools/mktsconfig.mjs`). Baseline `tfe-tsc-base.log`: 106 errors, 102 of them in files I later touched. After, restricted to touched files and compared as normalized `file: error` multisets (line and column removed, `tfe-tsc-*.norm`): no error disappeared, none of "module has no exported member" or "cannot find module <entry>" remains. The "new" ones in `tfe-tsc-final-c.log` are:

- two new files added to the roots after the baseline (`🧪️docklayoutstore` 79 errors, `📡️scene-wire` 1): their errors are unrelated type errors of lines I did not edit (the diff of both files is only the fixture import);
- files in which a peer edited the same lines concurrently (`🏃️process …🧹️retirement`, `🏪️store …📏️retirement`, `🧑‍💻dev …ticket-owned-browser-host-staging`, `🎠️kernel …📦️package-assets`; each shows peer imports from `@semio-tech/framework/<x>/testing/fixtures` or `@semio-tech/framework/identity/path`);
- `🧵️job/🎟️context/🪙️loan`: its `unknown`-typed `caller`, `inputs`, `policies` errors are pre-existing: that file is a second-round consumer and was not a root of the baseline `tsc` run; the pre-edit copy type-checked alone (`tfe-tools/tsc-orig/`) shows the identical `TS18046` errors. My own defect there was a duplicate import line, merged. Round-2 files in general have no `tsc` baseline of their own, so their entries in `tfe-tsc-*.norm` read as new when they are not.

The last `tsc` run (after the `fixtureUrl` change and the loan fix) is in 4.6.

### 4.5 Defects found by verification and fixed

- flow wasm unit: import of the mcp entry used 4 instead of 5 `../` (module not found, the file did not load); fixed, solo run in 4.3.
- `🧵️job/🎟️context/🪙️loan`: duplicate import line (4.4); fixed.
- gen tool snapshot path was percent-encoded (`new URL(import.meta.url).pathname` in a path with emoji): a stray directory `.%F0%9F…` was created in the repo root; I moved its contents into `tfe-tools/orig/`, removed the empty directory and fixed the tool (`import.meta.dir`). Nothing else was written outside the ticket folder.

### 4.6 After the `fixtureUrl` change (final state)

- Entries: 38 of 38 fresh-process loads and 39 of 39 jsdom loads (4.1).
- Set 3 bun (`💻️os/🧪️tests`, root tests): 48 pass, 13 fail, failure set identical to the baseline.
- Set 2 bun (`run2.sh final6`): 296 tests, 224 pass, 72 fail; failure set equals the baseline minus one failure that no longer fails (`preparation request carries the actual installed typed retirement issuers`); no new failure.
- Set 1 bun (`run.sh final5`): 655 tests (the flow unit file now loads), 546 pass, 108 fail, 1 skip. Against the baseline failure set: the 12 flow-unit failures are the same names as before, and there are 6 failures not in the baseline: `cancelled original dispatch returns its interaction and in-flight event before paid closure`, `clipboard dispatch source remains owned across asynchronous intent handback and validates the actual incoming caller`, `registered file dependency preserves original mounted close loan and complete causes` (all three fail on `toContain`/`toBe` assertions over source text of other modules), and three tests in `🏃️process/…/🧹️retirement/🧪️tests/🟦️.ts` (`reads complete immutable original claim DATA`, `terminal close preserves actual dual failure order`, `journal boundary distinguishes replay…`; messages `Actual ticket DATA sink required` and `Original parent Script process invocation required`). Attribution: the retirement file is being edited by a peer right now (modified 23:32, last committed 18:19; the peer's edit added the SDK import and the ticket-sink requirement), and the other three assert over source text of files that other slices rewrote the same hours. None of the six involves a fixture entry or URL. I did not prove this by reverting, because the files are shared and live; the one test of that file that failed in the 18:20 run passed solo at 18:3x. Treat these 6 as unresolved-but-attributed, not as proven clean.
- `tsc` (`tfe-tsc-final-d.log`, 204 errors in the scoped closure; touched-file errors 199 against 102 in the baseline, 0 baseline errors gone): all additions are the groups listed in 4.4 (new roots, peer-edited files, second-round files without a baseline); no error names a missing export, entry or fixture.
- The 12 renderer files: section 5 below (0 `scheme file` errors).

## 5. The `The URL must be of scheme file` defect (coordinator message, 18:2x)

Cause (reproduced with `tfe-tools/jsdom-probe/old-vs-new.test.ts`, vitest 4.1.10 with `environment: "jsdom"`): under jsdom the vite asset plugin rewrites a literal `new URL("<path>", import.meta.url)` into `http://localhost:3000/@fs/…`, and the global `URL` is jsdom's class. My first entries exported exactly that form (`new URL("<rel>", import.meta.url)`), so every jsdom test importing an entry with a URL export got an `http:` URL, and `readFileSync(url)` (or a load-time `fileURLToPath`) threw `The URL must be of scheme file`. 12 renderer files that I edited in the second round (the ones with `resolve(repoRoot, "🧰️framework/…")` reads) failed to load. My own first vitest runs did not catch it because most renderer files do not load in this tree (4.3).

Fix: no entry spells the literal pattern any more. `fixtureUrl(base, path)` (new export of `FW/🔨️modules/🧪️test/📖️source-text/🟦️.ts`) computes `pathToFileURL(resolve(dirname(fileURLToPath(base)), path))`: path arithmetic on `import.meta.url` and a native Node `URL`, so it works in bun, node vitest and jsdom vitest. Applied to 83 URL exports in 38 entries (each entry imports `fixtureUrl`). The same file also had a top-level `fileURLToPath(new URL("../../..", import.meta.url))` that broke the import itself under jsdom; it is now `resolve(dirname(fileURLToPath(import.meta.url)), "../../..")` (same value; the module's own law `🧪️test/📖️source-text/🧪️tests` was not re-run: see section 7).

Verification: jsdom probe 39 of 39 entries pass; the 12 renderer files named by rhu2 re-run with rhu2's config and wrapper (`F/rhu2-tools/work/vitest.react.config.ts`, `rhu2-tools/vitest.sh renderer`, log `tfe-12.log`): 0 `scheme file` errors (was 12); 3 files pass, 9 fail, and every remaining failure is independent of the entries: 6 files fail at load on a peer's unresolved `@semio-tech/assets/published-page`, `diff-view-produced-surface` has an ENOENT on a ui schema path (it is the one file that already failed in rhu2's baseline), `icon-svg-lighting` fails on a WebGPU binding size, `browser-frame-transport` has 6 ENOENTs on `🧑‍🎨engine/🧫️fixtures/…` files (not my paths, none of my fixtures; its deadline test, which uses my URL, is not among the failures). The coordinator reports RENDERER-HOSTS-UI-3 saw the 12 load again with baseline results; that agrees with this.

## 6. Commands run

All from `/Users/ueli/Documents/semio`, all logs `F/🗑️generated/tfe-*`.

1. `git ls-files` lists plus `bun F/tfe-tools/scan.ts` (wide scan: before 1,753 raw rows, classified in `count.py`) and `bun F/tfe-tools/work.ts` (work set), `forms.ts`, `gen.ts plan|apply` (four apply passes: relative literals, hand-fixed cases, root-relative literals, `💻️os/🧪️tests` and root tests), `equiv.ts`.
2. Baselines and afters: `run.sh`, `run2.sh`, `run3.sh`, `bun test` on lists, vitest through the react config.
3. `node node_modules/typescript/bin/tsc --noEmit -p F/🗑️generated/tfe-tsconfig-{base,after,final}.json` (four runs).
4. `verify-entries.sh` (fresh bun per entry), `vitest` jsdom probe, rhu2 wrapper for the 12 files.
5. `bun test` of eight fixture-placement or ownership laws.
No git command that modifies anything, no ticket tool, no sub-agent, no `[DEBUG]` log left in files I wrote (the probes under `tfe-tools/jsdom-probe/` carry `[DEBUG]` and live only in the ticket folder).

## 7. What is left (section 1's 232 plus 58)

| Count | Reason | Hand-off |
|---:|---|---|
| 58 | consumers in `📚️library` (parent's) | parent: same recipe, entries exist for the framework and os owners |
| 50 | owner is `os/🔨️modules/🔌️plugin` (pi1) | pi1 creates `🔌️plugin/🧪️tests/🧫️fixtures/🟦️.ts`; `gen.ts` then re-points renderer, flow and dev consumers (tools reusable: run `bun F/tfe-tools/gen.ts apply <list> <report>`) |
| 3 | owner is the renderer (rhu2, ri2) | |
| 100 | consumers that are not modules: `📓️print`, `🐾️pets`, `❓️quiz`, `🦑️repo` modules, `📦️packages` (SDK facades), product-level os dirs | owners of those products; SDK facades are the public route already |
| 11 | wrong direction: a framework module test reads a product's file | needs a design decision (move the fixture or invert), not an entry |
| 7 + 12 | targets in `✏️s` or `🌎️hub` | plugin side needs SDK entries (the peer's `@semio-tech/framework/<x>/testing/fixtures` facades under way in `📦️packages` are the natural route; they can re-export these entries) |
| 22 | same product, sub-package granularity of my module rule | not cross-module |
| about 30 | helper-wrapped reads (`here(…)`, `read(…)`, `fixture(…)` taking a path string), directory reads (`document-identity-wire`, conformance corpus directories), docstring mentions | per-site; directory entries (`…FixtureDirectory`) are a small generator extension |
| 7 | not reads (comments, `catalog:` strings, logical keys) | none |

Other open points:

- Taxonomy: the 37 new `🧪️tests/🧫️fixtures/` folders under owners that had none were not registered in `🔣️taxonomy.json`, and the real `verify taxonomy report` was not run. The `🧫️fixtures` and `🧪️tests` names exist for the ui entry (rtt), so I expect the same resolution; unverified.
- The source-text law (`🧪️test/📖️source-text/🧪️tests`) and the SDK surface law were not re-run after I added `fixtureUrl` and changed `FRAMEWORK_ROOT`; `fixtureUrl` is not re-exported by the SDK entry `@semio-tech/framework/test/source-text` (it only re-exports named functions).
- Fixture-placement laws in pack, actor, process and ui fail for pre-existing misplaced files (4.3); worth a ticket.
- A coverage gap that predates this slice: most renderer vitest files do not load in this tree (missing wasm node-graph build and a peer's alias), so their tests cannot confirm anything either way.
- Reduced path lengths: the longest new or edited path is the ui entry (`FW/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts`, 76 bytes); all entry export strings are relative.

## 8. Files changed

38 entry files (37 new, `FW/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts` extended), 196 consumer test files under `FW/🔨️modules/**` and `OS/**`, `FW/🧪️tests/🧪️docklayoutstore/🟦️.ts`, `FW/🔨️modules/🧪️test/📖️source-text/🟦️.ts` (`fixtureUrl`, `FRAMEWORK_ROOT`), tools in `F/tfe-tools/**`, this file. Pre-edit copy of every touched file: `F/tfe-tools/orig/` (index `index.tsv`).

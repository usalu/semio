# 1028 result: e2eprep

Status: GREEN for static work (typecheck rc 0). unit `test` target also GREEN.

## Files changed
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorexamples/🟦️.ts`: new test per `catalogueRows` fixture row; asserts the acceptance edits fixture id equals the fixture row id and the id is present in the authored source.
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorexamples/🧫️cases.json` + `🧬️schema/🔣️.json`: language-neutral `catalogueRows` (required) for aggregator and koordinator.
- `1028-tools/e2eprep/tsconfig.json`: scoped typecheck config (tools only).

## Triage risks (audit §7 items 5, 6)
- Aggregator: the audit suspected the DOM id is folded. Verified not: puzzle catalogue panel `object_kind_item` uses `try_id(&kind_id)` with the raw DSL object `id`; builder `try_id` stores the string verbatim; Tree item renders `id={id}` unchanged. The raw selector `[id="Hexagonal Cut Concrete Forest Left"]` is correct (spaces are valid in an attribute selector). Now pinned statically: row id must appear as a raw first column of the `objects` table in the concrete-forest `🗣️.dsl.semio`.
- Koordinator: `cad-play-catalogue.<typology>` (cad catalogue panel `cad_tree_item(format!("cad-play-catalogue.{}", entry.typology)…)`), typology `spatial.shape.primitive.box` pinned against `TYPOLOGY_CATALOG` in the cad `🧰️kit`; test also asserts rowId == prefix + typology.
- Acceptance spec itself left unchanged (selectors were right).

## SEMIO_BUILD_MODE
Already done by demots: `serve`, `prepare-e2e`, `serve-e2e` set `SEMIO_BUILD_MODE=prototype` in `📋️project.json` (verified, lines 323/352/383).

## Typecheck
Route: `repo tools exec tsc -- -p C/1028-tools/e2eprep/tsconfig.json` (extends root tsconfig, `types: [node, bun]`; scripts use `import.meta.dir`, so bun types are required, without them 2 TS2339 on the two `📜️script.ts`, not code errors). Covers acceptance suite, examples test, e2e module (`🟦️.ts`, `📜️script.ts`, config) and runtime `📜️script.ts`. Result: 0 errors (rc 0).

## Test run
`repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:test` detached: rc 0. First run failed on my own wrong relative path to the edits fixture (ENOENT, fixed to `../🎭️acceptance/…`); second run passed including the new catalogue-row tests. Note: each run re-planned the graph (about 20 min) plus 15+ min capacity queue. `🔨️modules/🧪️e2e` and `🧩️runtime` have no server-free unit tests under this target beyond what `test` already runs (vitest includes only the console test plus includeSource files); runtime-catalog tests are the separate `test-runtime-catalog` target (not run, owned by a peer's run).

## SHARED CHANGE
none

## E2E environment prep (coordinator task 2)

Static dry-run findings (no activation needed):
- Browser install: `workspace:deps-browsers` runs `node node_modules/playwright/cli.js install chromium` with `PLAYWRIGHT_BROWSERS_PATH ??= repoCacheDirectory(repo,"tools","ms-playwright")` (`repoToolCacheEnv`). `test-e2e` sets the very same path for the Playwright child, so install and consumption agree. Caller env had no `PLAYWRIGHT_BROWSERS_PATH` (unset) so the default applies. Do not export it elsewhere.
- Config load: `🎚️config/🟦️.ts` throws unless `PLAYWRIGHT_BASE_URL` is set; `TestScript` sets it from the published service, so only `repo run …:test-e2e` works (not a bare `playwright test`).
- GPU flags on darwin: `--use-angle=metal --enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu` (verified in `browserLaunchArgs`). Under devcontainer set `DEMONSTRATOR_E2E_GPU=swiftshader` (verfolgen map cases then cannot pass).
- Port: `serve-e2e` binds `127.0.0.1:0` (ephemeral, published via the service session); 6029 only for `serve`. No collision possible.
- Output path guard: with `SEMIO_TICKET_DIR` = this ticket dir the output path is 153 chars + 102 reserved = 255 <= 256 (1 char of headroom). Do not set `SEMIO_TICKET_DIR` to a longer path; leaving it unset uses `dist/reports/e` (207, safe).

### STATUS at CHECKPOINT 22:55
- `repo run workspace:deps-browsers` was started detached (not my recorded pid; log `C/🗑️generated/1028/e2eprep-br.txt|.rc`) and is STILL QUEUED on machine capacity (about 40 min, 7 ahead): Chromium is NOT yet verified installed. If the log shows rc 0, check `ls "$(repo path-of-cache)"`: expected dir is `<repo>/.cache/**/tools/ms-playwright/chromium-*` plus `chromium_headless_shell-*` (the `.cache` layout is a mounted image; locate with `find .cache -maxdepth 8 -name 'ms-playwright'`). If the process died with my turn, rerun the command below.
- No source edits were made for the env task; nothing to fix was found statically.

### Exact final command sequence (cwd `/Users/ueli/Documents/semio`, each step detached with `nohup … & ` and a `.rc` marker, per Addendum 17:15)
1. Browsers (once): `repo run workspace:deps-browsers`
2. Activation (coordinator): `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype`
3. Official e2e (pulls the chain activate-prototype -> deps-browsers -> prepare-e2e -> serve-e2e itself):
   `SEMIO_TICKET_DIR=<ticket dir, at most the current 153-char length> repo run @semio-tech/mit-bestand-demonstrator:test-e2e`
   (unset `SEMIO_TICKET_DIR` to write to `dist/reports/e`; on a GPU-less host add `DEMONSTRATOR_E2E_GPU=swiftshader`, verfolgen cases then fail by design). If you want the stages separate: `repo run …:prepare-e2e`, then `repo run …:serve-e2e` (continuous, keep alive in its own detached shell, binds 127.0.0.1:0), then `repo run …:test-e2e`.
4. Manual serve + probe (no Playwright runner): `repo run @semio-tech/mit-bestand-demonstrator:serve` (port 6029, `MIT_BESTAND_DEMONSTRATOR_PORT` overrides), then with `PLAYWRIGHT_BROWSERS_PATH=<the ms-playwright dir from step 1>` run `bun C/🔍️landing-eight-pane-probe.ts` (env `PROBE_BASE_URL`, default `http://127.0.0.1:6029/`).
All of these must be started from the main session (subagent-started servers die with the turn).

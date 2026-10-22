# result-buildlib-reds (wave 1022a, executor buildlib-reds): library root suites

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Logs: `C/🗑️generated/buildlib-reds/`; tools: `C/buildlib-reds-tools/` (`sweep.sh`, `sweep-env.sh`, `markers.ts`). Every run is one explicit file (`bun test ./<file>`, envelope helper `C/lease-proof-tools/run-test.ts` where plain bun refuses, `C/bzl-act-tools/wrap.ts` + `SEMIO_TEST_BAZEL_BINARY` for the real-Bazel laws); at most 2 concurrent; no graph command, no kill of foreign pids, no sub-agents.

## Table (pass/fail)
| suite | before | after |
|---|---|---|
| graph stamps `I/🔖️stamps/🧪️tests` | 16/1 (`🔌️managers/🟨️.cjs` unclaimed) | 17/0 |
| output stamps `I/🧪️tests/🔖️output-stamps` | 3/1 (target-policy output moved under /1) | 4/0 |
| generations load `G/🕰️generations/🔌️load/🧪️tests` | 43/1 (compact inputs capacity) | 44/0 |
| inputs `L/🏗️build/📥️inputs/🧪️tests` | 3/1 (`lstatSync is not defined`) | 4/0 |
| dep generation `…/📦️dependencies/🧾️generation/🧪️tests` | 10/2 (keying oracle `bun` field, `projectListing(` 6 vs 2, anchor pin) | 12/0 |
| dep incremental (scan ledger) | 14/1 plain bun (needs the process envelope) | 15/0 under the envelope helper |
| dep ingress / followup / concurrent / managers / main | 10 / 7 / 16 / 87+1 / 9+9 | 10 / 7 / 16 / 88 / 18 (envelope) |
| closure-law `…/🔏️authorizers/🧪️tests/🧭️closure-law` | 1/2 | 3/0 (run again after step 7) |
| closure sites + authorizer sites | 2 / 3 | 2 / 3 |
| bzl execution contract `…/🏗️bazel/🏃️execution/🧪️tests` | 14/2 | 16/0 |
| bzl hermetic class law `…/🔒️classes` (real Bazel) | fail (`bun: command not found: repo`, then no language) | pass (67 s) |
| bzl hermetic law `…/🔒️hermetic` (real Bazel) | not run | pass (65 s) |
| bzl shim / strategies | 2 / 2+1 skip | 2 / 2+1 skip |
| ci-report `L/🏗️build/🧾️ci-report/🧪️tests` | 0/1 (`.github/actions/bun-repo` deleted) | 18/0 |
| operations `L/🏗️build/⚙️operations/🧪️tests` | 10/5 | 15/0 |
| observation `L/🏗️build/👁️observation/🧪️tests` (envelope) | 43/9 | roster test fixed (+1); 8 open (env-bound, below) |

## What changed (all LIVE, anchored edits, re-read before edit)
1. **Stamps.** `I/🔖️stamps/🧬️schema/🔣️.json`: `⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs` is a source of `graph-assembly` (scope all); `target-policy` stamp `semio.graph-inferrer.target-policy/2` (bump: re-infers every project record once; stale-impl-guard handles running processes); output record `I/🧪️tests/🔖️output-stamps/🔣️.json` refreshed with `SEMIO_UPDATE_OUTPUT_STAMPS=1`. The two stamp tests no longer hardcode a stamp number (`next` is derived; the row check reads the protocol).
2. **Load capacity.** `G/🕰️generations/🔌️load/🧪️tests/🟦️.ts`: the "generous" call used `maximumBytes: limit*4`, larger than one capture (the production host uses `limit/4`); now `limit`. No production change.
3. **Inputs.** `L/🏗️build/📥️inputs/🧪️tests/🟦️.ts`: the vm host of the owner-discovery program gets `lstatSync` (the walk uses it since the no-follow change).
4. **Dep receipt.** `…/🧾️generation/🧪️tests/🐍️keying.py` + `🔣️.json` + `🧬️schema/🔣️.json`: runtime field `manager` (was `bun`), messages `runtime-changed.manager` EN/DE ("package manager version"); `🟦️.ts` pins: `projectListing(` count 6 (walk has 6 call sites), typed anchor formula. Scan-ledger test is envelope-only (plain `bun test` is refused by design).
5. **Closure law.** `…/🔏️authorizers/🧬️schema/🔣️.json` `ComputedImportSites`: +7 rows (runtime esbuild 2, native 1, playwright 1, conformance command 1, storybook execution 4, units command 3, doctor ports 1; all `roots: []`), `🔨️modules/🧪️test/🟨️.mjs` 5 -> 4, three stale rows removed (process leases, cargo descriptor catalog, inputs ledger; the law names rows without a site), observation 6 -> 4 after item 7. Test: the `installed:` marker manifest is located through the module system (bun's isolated install has no `node_modules/nx`).
6. **Bazel.** Fixtures + independent Python parser + law of `🏃️execution` follow the production `instance` (`main`, env `SEMIO_REMOTE_EXECUTION_INSTANCE`; two new rows: override, malformed refused) and the flags `--remote_instance_name=<instance>`, `--noremote_cache_compression` (law: no cache flags except those, instance flag exactly when remote). Production fix in the hermetic runner (`🏃️execution/🏭️runner/🍞️bun/🟦️.ts`): a `repo` shim (the declared Bun) first on the action `PATH`, because command-form targets are `repo ./x.ts verb`; `SEMIO_LANGUAGE=en` fixed in the action environment like `LC_ALL=C` and `TZ=UTC` (diagnostics only, never an output; a hermetic action has no user language). Real-Bazel class and hermetic laws green.
7. **CI.** `.github/workflows/build-system.yml` bench job used the deleted `./.github/actions/bun-repo` (real defect, would fail at job start): now `repo-launcher`; header comment; `.github/dependabot.yml`: `repo-launcher` + `devcontainer-repo` directories instead of `bun-repo`. `ci-report` test: composite list, and the pin law now says: the launcher action holds no runtime pin, the launcher registry's Bun equals `packageManager`, shared archive hashes equal the official ones, `NODE_VERSION` equals `engines.node`.
8. **Operations.** Test drift only: `bun repo` vs `repo` laws (`includes("bun repo")`), `--service` global option, duplicate flag name `--for` (service/units) resolved per operation, `noteCount` 44 (independent count), `cache serve` -> `remote-cache serve` row, `capacity run` example.
9. **`__physicalResolveV1` bypass removed** from the observation module (`import{createRequire}from"node:module"`, plain `createRequire(base).resolve`) and from the closure module (`🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts`); `.resolve` is data-only in the loader. Closure-law 3/0, sites suites green after removal. Observation `declarationExports` fixture roster +6 exports (graph-receipt added them to `.d.mts`).

## Open (not mine / not provable here)
- Observation suite under the envelope helper: 8 failures (`non-blocking validation agrees with reuse: *` x5, memoized file record, corrupted body, TOML surrogate): all "hold stays true after a change" in a scratchpad artifact dir; graph-receipt called these "need the repo test target" (custody of the artifact dir). Plain bun fails earlier ("Original observation physical ancestry required"). Run through the project test target once the graph rebuild is done.
- Inputs storage suites red under the envelope: `💾️storage/🧪️tests` main 15, `📒️ledger` 12, `🎛️phase` 8 ("Original process admission closed"), `♻️scoped` 5+3, `🚪️admission` 2, `🪪️identity` 1, `🔏️closure`, `🔒️lifetime`, `🎟️custody`, `🎯️precision` 4, `🎯️selected` snapshot/rust publication/main, `📋️data` 6, `🔏️revision` 2. Dependency storage: `👓️reading` 39 (7 skip), `📏️paths` 2, main test cannot find `publishJavascriptWorkspaceExportsV1` (function moved), `🪞️adopt` 1. Sweep tables: `C/🗑️generated/buildlib-reds/sweep-inputs*.tsv`, `sweep-dep*.tsv`. Owners: storage/dependency-storage peers.
- The loader-own resolve probe (`C/loader-own-tools/run-closure.ts` + `resolve-entry.ts`) now stops with `Uncaptured implementation module refused: /private/tmp/integrate-r7/ov-r6/…/📁️descriptor/🧬️schema/🔣️.json`: a stale absolute overlay path from a persisted capture/analysis row (no file of the repo contains it). Looks like a cache row that stores an absolute resolved path; closure-law (same capture) is green. Not diagnosed further.
- Flake seen twice in closure-law "declared roots": `Authorizer DATA directory changed` while peers edit the tree (re-run passes).
- Not run: `bun repo` graph commands (the stamp bump costs one re-inference of all project records on the next graph command).

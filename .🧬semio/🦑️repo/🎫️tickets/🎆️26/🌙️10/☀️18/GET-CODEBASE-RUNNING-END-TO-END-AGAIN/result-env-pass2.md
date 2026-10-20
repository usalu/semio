# Result env-pass2 (2026-10-20, executor env-pass2)

Task: run the 118 envelope-gated suites (`tst4-tools/envelope-gated.txt`) plus the 48 `HARNESS-LAUNCHER` suites of `result-tst3c.md` directly under a minted process envelope, one file at a time, 2 concurrent, classify, fix real defects and stale laws.

## Outcome

- Baseline (`wrap.ts`, envelope only): 0 of 118 green (2 empty). Final: **13 of 118 green** (A094 has no tests) and **11 of 48** of the tst3c set (list `env-pass2-tools/list2.txt`; `🧫️.ts` fixture entry dropped). Counts are from the last completed run per suite (`summary-f1/f2.tsv`, `summary-g1.tsv`); the final f2 sweep was cut at 92/118 by the write pause, the remaining rows come from the f1 sweep run before the last edits (A013, A024, A110 re-run individually).
- 5 list entries pointed at non-existing files (`…/🍎️physical.ts`, `🍎️image.ts` x2, `🛂️.ts`, `📒️ledger.ts`): the suites are `<name>/🟦️.ts`; corrected list `env-pass2-tools/list.txt`.
- The failure mass is NOT test code: the majority needs the live stack that only `bun repo run` builds (see classes). A wrapper that emulates it cannot be honest: the envelope gates (`Original bounded Nx namespace`, `SEMIO_NX_TOOLING_IDENTITY`, nested native images) are real runtime checks.

## Harness (what finally ran)
- `env-pass2-tools/wrap3.ts`: mints the process envelope (`createNativeBuildCallerCapabilitiesV1` + `withScriptProcessEnvelope`), enters the authentic `withRepositoryTestArtifactsV1` scope (diagnostics image, `SEMIO_TEST_ARTIFACT_DIR`, TMPDIR/caches from `repositoryTestArtifactEnvironmentV1`), adds `SEMIO_BUILD_MODE=prototype`, `SEMIO_TICKET_DIR`, `SEMIO_REPO_ROOT`, `SEMIO_WORKSPACE_ROOT`, `SEMIO_BUILD_TOOL=nx`, then spawns `bun test --preload overlay.ts ./<file>`. `overlay.ts` + `apply.sh overlay` verify STAGED (frozen-path) patches without touching the file (Bun `onLoad` plugin; child processes are not covered).
- Drivers: `rr5.py <tag> <budget> <list> [idx…]` (resumable, 2 workers), `one5.sh`, `first.py`/`digest.py`/`ctx.py` (first non-cascade error per suite). Cascade errors (`Original process admission closed`, `birth already received`) are consequences of the first failure: one rejected issued operation closes the shared Original of its process.

## Cause classes
| class | meaning | suites |
|---|---|---|
| TOOLING | `bun repo tools setup nx` not done (`.cache/b/t/m/nx` absent), no SDK identity | 14: A003, A022, A023, A025, A026, A036, A051, A052, A073, A074, A079, A092, A096, A113 |
| REPO-RUN | needs nx graph / bounded Nx namespace (`SEMIO_NX_FIXED_STORAGE`, `NX_CACHE_DIRECTORY`) / repo-run caller | 16: A002, A015, A016, A018, A033, A034, A055, A056, A057, A072, A078, A091, A095, A105, A106, A108 |
| NATIVE-IMAGE | nested persistent images need the diagnostics volume `.cache/b/o/m` mounted with owners (message tells to remount) / parent scope | 21: A004, A005, A006, A008, A010, A011, A020, A027, A029, A048, A066, A070, A076, A111, A112, A117, B010, B027, B037, B039, B040 |
| STALE-LAW | test or law drifted from the library (structural controls, missing bindings, renamed functions, moved fixtures) or a real defect | 75: A007, A009, A013, A014, A017, A019, A021, A028, A031, A032, A035, A037, A038, A039, A040, A041, A042, A043, A044, A045, A047, A053, A054, A058, A060, A063, A064, A065, A068, A069, A075, A080, A081, A082, A085, A086, A087, A088, A089, A090, A093, A099, A100, A102, A104, A107, A109, A114, A115, A116, A118, B002, B006, B007, B008, B009, B011, B012, B013, B014, B021, B022, B023, B025, B026, B029, B033, B034, B035, B038, B041, B042, B043, B046, B047 |
| NEEDS-CARGO | launches a real cargo build / ingress | 8: A067, A071, A098, A101, B001, B003, B024, B048 |
| LOAD/TIMING | deadline-bound under load 45-60 | 6: A046, A049, A050, A103, B015, B032 |
| DEPS | undeclared dependency (package.json is frozen) | 2: B004, B005 |

## Fixes
### Applied directly (non-frozen paths)
- `📓️print/🧪️tests/🧬️native-chart-grammar/📥️emission/🧪️tests/🟦️.ts` (A001 green)
- `🎛️dashboard/🧪️tests/🌀️control-plane/🟦️.ts` (fixture path)
- `🖥️server/🎛️coordinator/🏗️builder/🎟️admission/🟦️.ts` (REAL DEFECT: structural control) and its `🧪️tests` (B/A110 green)
- R codemod, AST-planned (`plan_r.ts`): `receiveScriptProcessInvocation(process.env, …)` -> `withBuildTestInvocationV1(…)` in `🎛️dashboard/…/🧊️execution`, `🖥️server/…/📥️inputs`, `…/📥️inputs/🔌️nx`, `…/💾️storage/🧪️tests/🔌️nx`, `🧪️test/🧪️tests/🧾️artifact-root-source`
- process module tests: `🐹️go-build/🧪️tests`, `📁️filesystem/🛡️durability/🧪️tests` (issued controls), `🎟️admission/🧪️tests` (history store), `🧩️component-pass/🧪️tests/📥️inputs` (bindings, refinement)
### Staged under `env-pass2-tools/staged/patches/{R,T}.json` (frozen library paths; `apply.sh check|overlay|apply [--dry-run]`, idempotent, span/anchor checked, refuses while a sync runs)
- R.json: 38 library test files, same codemod (greens A012, A030, A083; A006/A075 +1 pass)
- T.json: A084 fingerprint binding (74/74), A059/A061/A062 pruning laws (issued controls; green), A007 phase output env, A013, A068/A069, A024 (+child script), others

## Per-suite results
Format: id, file, pass/fail, class, first non-cascade error, note.

### Envelope-gated list (118)
| id | suite | p/f | class | first error | note |
|---|---|---|---|---|---|
| A001 | `P/📓️print/📦️artifacts/📥️material/🧪️tests` | 42/0 | GREEN |  | FIXED (direct): native-emission law now reads `runNativeGrammarEmissionScript` in rust-package-commands (script delegates there) |
| A002 | `RM/🎛️dashboard/🧪️tests/🌀️control-plane` | 10/2 | REPO-RUN | error: Original bounded Nx namespace required | FIXED (direct): control-plane fixture path (`../🧫️fixtures`, moved by tst3c); remaining 2 need bounded Nx namespace |
| A003 | `RM/🎛️dashboard/🧪️tests/🧊️execution` | 4/10 | TOOLING | error: Original actual Nx SDK required |  |
| A004 | `L/⚡️caching/🌐️vite/💾️storage/🧪️tests` | 2/6 | NATIVE-IMAGE | error: Exact original parent child allocation required |  |
| A005 | `L/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🧪️tests` | 4/2 | NATIVE-IMAGE | error: Nested persistent image requires actual parent scope |  |
| A006 | `L/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🕸️units` | 7/5 | NATIVE-IMAGE | error: /usr/bin/hdiutil attach -nobrowse -noautoopen -owners on -mountpoint /Users/ueli/Do |  |
| A007 | `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📥️phase` | 4/3 | STALE-LAW / DEFECT | error: expect(received).toContain(expected) | STAGED: `SEMIO_CACHE_PHASE_OUTPUT` (no producer anywhere) -> `SEMIO_TEST_ARTIFACT_DIR`; child-mode tests need re-run after apply |
| A008 | `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests` | 11/7 | NATIVE-IMAGE | error: Nested Native image cannot attach inside the parent mounted without owners at /User |  |
| A009 | `ching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🧪️tests/🏗️native-build` | 6/13 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A010 | `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/📐️layout/🍎️physical` | 0/2 | NATIVE-IMAGE | error: Nested persistent image requires actual parent scope |  |
| A011 | `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧪️tests` | 1/2 | NATIVE-IMAGE | error: Nested persistent image requires actual parent scope |  |
| A012 | `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧪️tests` | 3/0 | GREEN |  | FIXED (staged R): direct `receiveScriptProcessInvocation` per test -> `withBuildTestInvocationV1` |
| A013 | `L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent` | 32/16 | STALE-LAW / DEFECT | error: mac:sync_hooks:62:22) | STAGED: missing `/` in source path + `ports` lacks image-information exports; children spawn the unpatched file, verify after apply |
| A014 | `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation` | 2/3 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A015 | `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/📥️purpose/📜️script.ts` | 0/2 | REPO-RUN | error: Exact graph purpose context required |  |
| A016 | `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests` | 18/30 | REPO-RUN | error: Exact carried graph purpose receipt required |  |
| A017 | `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests` | 11/8 | STALE-LAW / DEFECT | AssertionError: Expected values to be strictly deep-equal: |  |
| A018 | `L/⚡️caching/📥️inference/🧪️tests/🔍️imports` | 4/15 | REPO-RUN | error: Original bounded graph writer custody required |  |
| A019 | `L/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🧪️tests` | 0/2 | STALE-LAW / DEFECT |  |  |
| A020 | `L/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical` | 0/16 | NATIVE-IMAGE | error: Nested persistent image requires actual parent scope |  |
| A021 | `L/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body` | 0/9 | STALE-LAW structural control/invocation | error: Live Original process issuance required | missing imports in test body (`createScriptInputCache`, `requireIssuedResourceControlInvocationV1`) + structural invocation |
| A022 | `L/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests/🍎️image` | 0/1 | TOOLING | error: Original immutable SDK required |  |
| A023 | `L/⚡️caching/📦️artifacts/🔒️bounded/🧪️tests` | 6/1 | TOOLING | error: Original immutable SDK required |  |
| A024 | `L/⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands` | 6/0 | GREEN |  | STAGED: structural invocation -> issued envelope per row; child `📜️script.ts` passed a plain object as `commands` (ScriptRouter required) -> router; verify after apply (child process not overlay-able) |
| A025 | `L/⚡️caching/🕸️graph/🏁️startup/🚦️service/🧪️tests` | 0/2 | TOOLING | error: Original loaded SDK root required |  |
| A026 | `L/⚡️caching/🕸️graph/🏇️waiter/🧪️tests` | 0/1 | TOOLING | error: Original installed tooling identity and ticket artifact directory required |  |
| A027 | `L/⚡️caching/🕸️graph/📤️publication/🧪️tests` | 0/6 | NATIVE-IMAGE | error: Exact original parent child allocation required |  |
| A028 | `L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/📥️scope/🧪️tests` | 7/11 | STALE-LAW / DEFECT | error: Required original cache consumer unavailable |  |
| A029 | `L/⚡️caching/🕸️graph/🔏️seal/🧪️tests` | 0/4 | NATIVE-IMAGE | error: Exact original parent child allocation required |  |
| A030 | `L/⚡️caching/🚀️bootstrap/📥️admission/💾️storage/🧪️tests` | 2/0 | GREEN |  | FIXED (staged R) |
| A031 | `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/👓️reading` | 42/38 | STALE-LAW / DEFECT | error: expect(received).toEqual(expected) |  |
| A032 | `L/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests` | 26/20 | STALE-LAW / DEFECT | TypeError: undefined is not an object (evaluating 'file.statements.find((node) => ts.isFun | stale text law: extracted function names (`publishJavasc…`, `inspectJavascriptDependencyInputPlansReadV1`) no longer exist |
| A033 | `L/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests/📥️native-caller` | 0/1 | REPO-RUN | error: Original bounded Nx namespace required |  |
| A034 | `L/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧪️tests` | 1/5 | REPO-RUN | error: Original bounded Nx namespace required |  |
| A035 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📥️purpose/🧪️tests` | 1/3 | STALE-LAW / DEFECT | error: Exact same-wire toolkit refusal receipt required |  |
| A036 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child` | 0/2 | TOOLING | error: Original SDK and artifacts required |  |
| A037 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🧪️tests` | 1/4 | STALE-LAW / DEFECT | error: Exact same-wire toolkit refusal receipt required |  |
| A038 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative/⚠️failure` | 0/1 | STALE-LAW / DEFECT | SyntaxError: Export named 'preparedBinaryen' not found in module '/Users/ueli/Documents/se |  |
| A039 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests/📤️derivative` | 4/3 | STALE-LAW / DEFECT | SyntaxError: Export named 'preparedBinaryen' not found in module '/Users/ueli/Documents/se |  |
| A040 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🎟️capacity/🧪️tests` | 2/6 | STALE-LAW / DEFECT | SyntaxError: Export named 'preparedBinaryen' not found in module '/Users/ueli/Documents/se |  |
| A041 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🧪️tests` | 0/1 | STALE-LAW / DEFECT | SyntaxError: Export named 'preparedBinaryen' not found in module '/Users/ueli/Documents/se |  |
| A042 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🧪️tests` | 0/1 | STALE-LAW / DEFECT | SyntaxError: export 'preparedBinaryen' not found in '@semio-tech/nx-bootstrap-tools/wasm/s |  |
| A043 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests/🛂️` | ?/? | STALE-LAW / DEFECT |  |  |
| A044 | `L/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🧪️tests` | 0/1 | STALE-LAW / DEFECT | SyntaxError: export 'preparedBinaryen' not found in '@semio-tech/nx-bootstrap-tools/wasm/s |  |
| A045 | `L/⚡️caching/🚀️bootstrap/🛠️tools/🧪️tests` | 2/7 | STALE-LAW / DEFECT | error: Cannot find module '../📜️script.ts' from '/Users/ueli/Documents/semio/🧰️framework/🛍 | R-patch bug found+fixed (`routing.receive…` namespace form); remaining `Cannot find module ../📜️script.ts` = stale path |
| A046 | `L/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📏️headroom/🧪️tests` | 1/2 | LOAD/TIMING | error: Original resource loan deadline exhausted |  |
| A047 | `L/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧪️tests` | 15/34 | STALE-LAW / DEFECT | AssertionError: Expected values to be strictly deep-equal: |  |
| A048 | `L/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📦️generation/🧪️tests` | 1/2 | NATIVE-IMAGE | error: Nested persistent image requires actual parent scope |  |
| A049 | `L/⚡️caching/🦀️cargo/📦️units/📋️plan/🧪️tests/🔗️binding` | 6/2 | LOAD/TIMING | error: Build test law body deadline exhausted |  |
| A050 | `L/⚡️caching/🦀️cargo/📦️units/📋️plan/🧪️tests` | 26/7 | LOAD/TIMING | error: Build test law body deadline exhausted |  |
| A051 | `L/⚡️caching/🦀️cargo/📦️units/🧪️tests` | 12/16 | TOOLING | error: Original Nx SDK required |  |
| A052 | `L/⚡️caching/🧪️tests/🌍️caller-environment` | 1/9 | TOOLING | error: Original immutable SDK required |  |
| A053 | `L/⚡️caching/🧪️tests/🎮️playground-preparation/🧪️tests` | 1/3 | STALE-LAW / DEFECT | error: expect(received).toContain(expected) |  |
| A054 | `L/⚡️caching/🧪️tests/📦️native-dependencies/🧪️tests` | 1/5 | STALE-LAW / DEFECT | TypeError: prepareDependencies is not a function. (In 'prepareDependencies(row.kind, root, |  |
| A055 | `L/⚡️caching/🧪️tests/🔗️import-edges/🏛️graph/🔑️authority` | 0/1 | REPO-RUN | error: Original bounded Nx namespace required |  |
| A056 | `L/⚡️caching/🧪️tests/🔗️import-edges/📁️paths` | 17/8 | REPO-RUN | error: Original bounded graph writer custody required |  |
| A057 | `L/⚡️caching/🧪️tests/🔗️import-edges/🔁️context` | 2/11 | REPO-RUN | error: Original bounded graph writer custody required |  |
| A058 | `L/⚡️caching/🧪️tests/🗃️compiler-state` | 15/2 | STALE-LAW / DEFECT | error: 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🏗️builder/🏃️executio |  |
| A059 | `L/⚡️caching/🧹️pruning/⏱️continuation/🧪️tests` | 4/0 | GREEN |  | FIXED (staged): structural control -> issued control |
| A060 | `L/⚡️caching/🧹️pruning/🏪️budget/📜️history/🧪️tests` | 1/2 | STALE-LAW / DEFECT | error: Original historical control belongs to another caller |  |
| A061 | `L/⚡️caching/🧹️pruning/🔒️incremental/🧪️tests` | 1/0 | GREEN |  | FIXED (staged) |
| A062 | `L/⚡️caching/🧹️pruning/🔒️store/🧪️tests` | 1/0 | GREEN |  | FIXED (staged) |
| A063 | `L/⚡️caching/🧹️pruning/🧪️tests/⏱️recency` | 0/2 | STALE-LAW / DEFECT | error: Exact finite scanner fixture output required |  |
| A064 | `L/🏃️process/🎛️owned-execution/🧪️tests/📥️original-control` | 0/2 | STALE-LAW structural control/invocation | error: Live Original process issuance required | STALE-LAW needs redesign: hand-made ScriptInvocation cannot observe publish/yield or abort the issued signal any more (events via stderr, abort via deadline) |
| A065 | `L/🏃️process/🎛️owned-execution/🧪️tests` | 4/2 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A066 | `L/🏃️process/🦀️cargo/🎯️exact/🧪️tests/🍎️image` | 0/1 | NATIVE-IMAGE | error: Original issued native storage reservation required |  |
| A067 | `L/🏃️process/🦀️cargo/🏭️inventory/🧪️tests` | 2/4 | NEEDS-CARGO | TypeError: undefined is not an object (evaluating 'manifest.profile.prototype') |  |
| A068 | `L/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption` | 2/1 | STALE-LAW / DEFECT | TypeError: undefined is not an object (evaluating 'options.pkg.files') | STAGED partial: 5-arg `execute` call, 3 missing bindings; wasm row needs binding rewrite (buildRepositoryWasmWebV1 grew pkg/consume ports) |
| A069 | `L/🏃️process/🦀️cargo/📥️dependencies/🧪️tests` | 3/1 | STALE-LAW / DEFECT | TypeError: undefined is not an object (evaluating 'options.pkg.files') | same as A068 (shares the consumption law) |
| A070 | `L/🏃️process/🦀️cargo/🚪️launcher/🧪️tests` | 2/3 | NATIVE-IMAGE | error: expect(received).toBeDefined() |  |
| A071 | `L/🏃️process/🦀️cargo/🧪️testing/📦️executable/🧪️tests` | 0/1 | NEEDS-CARGO | error: Cannot find module './🌿️/🟦️.ts' from '/Users/ueli/Documents/semio/🧰️framework/🛍️pro |  |
| A072 | `L/🏗️build/🏃️execution/🧪️tests` | ?/? | REPO-RUN | error: Original prepared execution inventory required |  |
| A073 | `L/🏗️build/👁️observation/🧪️tests/⚡️incremental` | 2/1 | TOOLING | error: Original artifact namespace and immutable SDK required |  |
| A074 | `L/🏗️build/👁️observation/🧪️tests/📦️projects` | 0/2 | TOOLING | error: Original artifact namespace and immutable SDK required |  |
| A075 | `L/🏗️build/📊️reporting/🧪️tests` | 4/2 | STALE-LAW / DEFECT | error: Original report reader failure |  |
| A076 | `L/🏗️build/📖️storybook/💾️storage/🧪️tests` | 1/3 | NATIVE-IMAGE | error: Exact original parent child allocation required |  |
| A077 | `L/🏗️build/📥️caller/🧪️tests/📥️birth` | 9/0 | GREEN |  |  |
| A078 | `L/🏗️build/📥️inputs/🎯️selected/📦️snapshot/🧪️tests` | 1/1 | REPO-RUN | error: Original snapshot fixture required |  |
| A079 | `L/🏗️build/📥️inputs/🎯️selected/🧪️tests` | 0/1 | TOOLING | error: Original immutable native inventory SDK required |  |
| A080 | `L/🏗️build/📥️inputs/💾️storage/🧪️tests/🎛️phase` | 0/8 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A081 | `L/🏗️build/📥️inputs/💾️storage/🧪️tests/📒️ledger` | 4/12 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A082 | `L/🏗️build/📥️inputs/💾️storage/🧪️tests` | 2/5 | STALE-LAW / DEFECT |  |  |
| A083 | `L/🏗️build/📥️inputs/📂️membership/🧪️tests` | 2/0 | GREEN |  | FIXED (staged R) |
| A084 | `L/🏗️build/🔧️tools/🏗️bazel/🧪️tests` | 87/0 | GREEN |  | FIXED (staged): `taskInputs` binding renamed to `bazelTaskInputsV1` |
| A085 | `L/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests` | 2/2 | STALE-LAW / DEFECT | TypeError: The "paths[0]" property must be of type string, got undefined |  |
| A086 | `L/🏗️build/🧪️tests/🎟️phase` | 8/8 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A087 | `L/🏗️build/🧪️tests/🏗️native` | 3/2 | STALE-LAW structural control/invocation | error: expect(received).not.toHaveProperty(path) | CONFLICT: law says `.cargo/config.toml` has no `[profile.*]`, file still has `[profile.prototype|release]` (also A067: cargo rejects `profile.prototype` from config) -> coordinator decision |
| A088 | `L/🏗️build/🧪️tests/🕸️graph/🔄️candidate` | 0/3 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A089 | `L/🏗️build/🧪️tests/🕸️graph` | 102/87 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) | aggregate of several suites (graph/*); cascade of Nx-namespace/graph-writer gates |
| A090 | `L/🏗️build/🧪️tests` | 31/29 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A091 | `L/🏗️build/🧭️invocation/🧪️tests` | 2/1 | REPO-RUN | error: $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo  |  |
| A092 | `L/🏗️build/🧭️selection/📥️closure/🧪️tests` | 0/3 | TOOLING | error: Original Nx SDK required |  |
| A093 | `L/🔍️discovery/📤️schema-registry/🧪️tests` | 3/1 | STALE-LAW / DEFECT | error: Original native caller capabilities required |  |
| A094 | `L/🔍️discovery/🕸️runtime` | 0/0 | GREEN |  |  |
| A095 | `L/🔍️discovery/🕸️runtime/🧪️tests/🏗️build-producer` | 0/1 | REPO-RUN | error: /Users/ueli/.bun/bin/bun repo run runtime-probe:run --tool=bazel -- exact value exi |  |
| A096 | `L/🕸️dependencies/🟦️typescript/⚙️configured/🧪️tests` | 2/5 | TOOLING | error: Original Native hasher SDK required |  |
| A097 | `L/🕸️dependencies/🟦️typescript/🧪️tests` | 1/0 | GREEN |  |  |
| A098 | `L/🗂️workspaces/🦀️cargo/🏃️command/🧪️tests` | 3/1 | NEEDS-CARGO | error: process didn't exit successfully: `bun '/Users/ueli/Documents/semio/🧰️framework/🛍️p |  |
| A099 | `L/🗂️workspaces/🦀️cargo/🛠️preparation/🔁️replan/🧪️tests` | 0/7 | STALE-LAW / DEFECT | error: /Users/ueli/.bun/bin/bun /Users/ueli/Documents/semio/.cache/b/d/m/j/ecafa95add9056a | DEFECT: `tsc --ignoreConfig` unknown option (pinned TypeScript 5.9.3) -> use a supported flag |
| A100 | `L/🗂️workspaces/🦀️cargo/🛠️preparation/🧪️tests/📥️original-control` | 0/1 | STALE-LAW / DEFECT | error: Original default preparation artifact and storage declarations required |  |
| A101 | `L/🗂️workspaces/🦀️cargo/🧪️tests/🏎️nextest-configuration` | 1/1 | NEEDS-CARGO | error: error: process didn't exit successfully: `bun '/Users/ueli/Documents/semio/🧰️framew |  |
| A102 | `L/🗂️workspaces/🦀️cargo/🧪️tests/📥️original-control` | 3/9 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| A103 | `L/🗂️workspaces/🦀️cargo/🧪️tests/🕰️queued-preparation` | 0/1 | LOAD/TIMING | error: Original Script deadline refused |  |
| A104 | `L/🗂️workspaces/🦀️cargo/🧪️tests` | 0/1 | STALE-LAW / DEFECT | error: Original short preparation custody storage required |  |
| A105 | `L/🧪️tests/⚡️production-cache-input-boundary` | 0/1 | REPO-RUN | error: Original bounded Nx namespace required |  |
| A106 | `L/🧪️tests/⚡️workspace-negation-closure` | 1/2 | REPO-RUN | error: Command failed: /Users/ueli/.bun/bin/bun -e const { createProjectGraphAsync } = awa |  |
| A107 | `L/🧪️tests/🔄️transaction-v2` | 0/62 | STALE-LAW / DEFECT | error: Invalid transaction fixture run identity | stale law: fixture run identity |
| A108 | `L/🧪️tests/🔧️build-surfaces` | 20/1 | REPO-RUN | error: Original bounded Nx namespace required |  |
| A109 | `L/🧪️tests/🧱️locale-law-ownership` | 0/2 | STALE-LAW / DEFECT | error: Explicit Vitest policy required |  |
| A110 | `RM/🖥️server/🎛️coordinator/🏗️builder/🎟️admission/🧪️tests` | 3/0 | GREEN |  | FIXED: real defect in `🖥️server/…/🎟️admission/🟦️.ts` (built a structural control for `acquireCompilerCapacityLeaseV1`; now `createRequestedIssuedResourceControlV1(original,0,group.signal)`) + test created sibling dirs next to the artifact dir, poisoning the retention inventory (`mkdtemp(dirname(artifacts))` -> `artifacts`) |
| A111 | `RM/🖥️server/🎛️coordinator/🏗️builder/💾️storage/🧪️tests/🔌️nx` | 0/2 | NATIVE-IMAGE | error: Exact original parent child allocation required |  |
| A112 | `RM/🖥️server/🎛️coordinator/🏗️builder/💾️storage/🧪️tests` | 1/4 | NATIVE-IMAGE | error: Exact original parent child allocation required |  |
| A113 | `RM/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🔌️nx/🧪️tests` | 1/3 | TOOLING | error: Original SDK required |  |
| A114 | `RM/🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🧪️tests` | 0/3 | STALE-LAW / DEFECT |  | DEFECT (deps): configured Source `@semio-tech/framework/transient` undeclared in coordinator package.json (package.json owner) |
| A115 | `RM/🧪️test/🧪️tests/⚖️comparison-profiles` | 0/1 | STALE-LAW / DEFECT | error: Original finite test artifact callback required |  |
| A116 | `RM/🧪️test/🧪️tests/🔮️oracle-governance` | 0/1 | STALE-LAW / DEFECT | error: Original finite test artifact callback required |  |
| A117 | `RM/🧪️test/🧪️tests/🚷️discovery-boundaries` | 2/4 | NATIVE-IMAGE | error: /usr/bin/hdiutil attach -nobrowse -noautoopen -owners on -mountpoint /Users/ueli/Do |  |
| A118 | `RM/🧪️test/🧪️tests/🧾️artifact-root-source` | 7/2 | STALE-LAW / DEFECT | error: expect(received).toEqual(expected) |  |

### tst3c HARNESS-LAUNCHER list (48)
| id | suite | p/f | class | first error | note |
|---|---|---|---|---|---|
| B001 | `🌎️hub/🧪️tests/🌳️unit-census` | 4/1 | NEEDS-CARGO | error: cargo metadata --locked --offline --format-version 1 --manifest-path /Users/ueli/Do |  |
| B002 | `🌎️hub/🧪️tests/🧱️foundation-source` | 14/2 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| B003 | `FM/🌱️value/🧪️tests/🧩️neutral-owner` | 16/1 | NEEDS-CARGO | error: process didn't exit successfully: `bun '/Users/ueli/Documents/semio/🧰️framework/🛍️p |  |
| B004 | `FM/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests` | 0/1 | DEPS | error: Cannot find package 'cross-spawn' from '/Users/ueli/Documents/semio/🧰️framework/🔨️m | DEPS: `cross-spawn` not declared in the process package (package.json owner) |
| B005 | `FM/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input` | 0/1 | DEPS | error: Cannot find package 'cross-spawn' from '/Users/ueli/Documents/semio/🧰️framework/🔨️m | DEPS: `cross-spawn` (same) |
| B006 | `FM/🏃️process/🏗️build/🎟️capacity/🛟️custodian/🧪️tests` | 0/1 | STALE-LAW / DEFECT |  |  |
| B007 | `FM/🏃️process/🏗️build/🎟️capacity/🧪️tests` | 0/5 | STALE-LAW structural control/invocation | error: Live Original resource control issuance required |  |
| B008 | `FM/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope` | 1/4 | STALE-LAW / DEFECT | SyntaxError: JSON Parse error: Unexpected EOF |  |
| B009 | `FM/🏃️process/🏗️build/🗃️state/🧪️tests` | 1/1 | STALE-LAW / DEFECT | error: copied-control |  |
| B010 | `FM/🏃️process/🏗️build/🧪️tests/🏗️native` | 5/4 | NATIVE-IMAGE | error: Live Original resource control issuance required |  |
| B011 | `FM/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests` | 9/1 | STALE-LAW structural control/invocation | error: Live Original resource control issuance required | FIXED partial (direct): history-store test uses issued controls (9/10); last test needs `schedulerBinaries` (cargo build via `bun repo cargo`, structural control inside) |
| B012 | `FM/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🧪️tests` | 3/2 | STALE-LAW / DEFECT |  |  |
| B013 | `FM/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🧪️tests` | 4/2 | STALE-LAW structural control/invocation | error: Live Original resource control issuance required |  |
| B014 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests` | 11/1 | STALE-LAW / DEFECT |  |  |
| B015 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests` | 11/35 | LOAD/TIMING | error: Original resource loan deadline exhausted |  |
| B016 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧪️tests/📉️rebudget` | 4/0 | GREEN |  | GREEN |
| B017 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧪️tests` | 10/0 | GREEN |  |  |
| B018 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️paths` | 1/0 | GREEN |  |  |
| B019 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️receipt` | 2/0 | GREEN |  |  |
| B020 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🧪️tests` | 4/0 | GREEN |  |  |
| B021 | `FM/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests` | 2/8 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) |  |
| B022 | `FM/🏃️process/📦️artifacts/🏗️native-build/📈️telemetry/🧪️tests` | 3/4 | STALE-LAW structural control/invocation | error: Live Original resource control issuance required |  |
| B023 | `FM/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🎯️target/🧪️tests/📋️callers` | 0/1 | STALE-LAW / DEFECT | error: /Users/ueli/.bun/bin/bun /Users/ueli/Documents/semio/.cache/b/d/m/j/ecafa95add9056a |  |
| B024 | `FM/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🏎️overlap` | 1/1 | NEEDS-CARGO | error: process didn't exit successfully: `bun '/Users/ueli/Documents/semio/🧰️framework/🛍️p |  |
| B025 | `FM/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/📥️inputs` | 0/3 | STALE-LAW / DEFECT | error: Component input cancelled | STAGED(direct) partial: extracted-function bindings added (`withRequestedIssuedResourceControlV1`, `cpuControl`, `scriptResourceProcessEnvironmentV1`) and `toBe(phase)` -> `requireIssuedResourceControlRefinementV1` (library derives a child control); cancel row still rejects the envelope (needs isolated test) |
| B026 | `FM/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests` | 9/20 | STALE-LAW / DEFECT | error: expect(received).toBe(expected) | same family as B025 |
| B027 | `FM/🏃️process/📦️artifacts/🏗️native-build/🧪️tests` | 18/9 | NATIVE-IMAGE | error: Host storage cannot admit fixed image envelope |  |
| B028 | `FM/🏃️process/📦️artifacts/🐹️go-build/🧪️tests` | 2/0 | GREEN |  | FIXED (direct): `fixtureAdmissionV1` issues control from `withBuildTestInvocationV1` |
| B029 | `FM/🏃️process/📦️artifacts/💾️cache/🧪️tests` | 28/16 | STALE-LAW / DEFECT | error: expect(received).toThrow(expected) |  |
| B030 | `FM/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests` | 57/0 | GREEN |  |  |
| B031 | `FM/🏃️process/📦️artifacts/📤️publication/📥️sources/🧪️tests` | 21/0 | GREEN |  |  |
| B032 | `FM/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory` | 3/4 | LOAD/TIMING | error: Build test law body deadline exhausted |  |
| B033 | `FM/🏃️process/🔒️leases/♻️retirement/🧭️continuation/🧪️tests/📦️child/📜️script.ts` | 0/1 | STALE-LAW / DEFECT | error: Exact owned continuation worker selection required |  |
| B034 | `FM/🏃️process/🔒️leases/🧪️tests/🎟️writer` | 0/5 | STALE-LAW / DEFECT | error: Authored finite writer fixture output required | authored fixture output env (`SEMIO_*` writer fixture) has no producer |
| B035 | `FM/🏃️process/🧪️testing/📥️invocation/🧪️tests` | 1/14 | STALE-LAW / DEFECT | error: Authored receiver fixture artifacts required | authored receiver fixture artifacts env has no producer |
| B036 | `FM/🏃️process/🧪️testing/🟨️bun/📁️files/🧪️tests` | 2/0 | GREEN |  |  |
| B037 | `FM/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image` | 0/1 | NATIVE-IMAGE | error: /usr/bin/hdiutil attach -nobrowse -noautoopen -owners on -mountpoint /Users/ueli/Do |  |
| B038 | `FM/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests` | 23/5 | STALE-LAW / DEFECT | ReferenceError: originalInvocation is not defined | stale: test uses undefined `originalInvocation()` and untyped `ScriptInvocation`/`ExactCargoLawPort` (unfinished edit by a peer) |
| B039 | `FM/🏃️process/🧪️testing/🦀️cargo/🔒️lease/🧪️tests` | 1/2 | NATIVE-IMAGE | error: Host storage cannot admit fixed image envelope |  |
| B040 | `FM/🏃️process/🧪️testing/🦀️cargo/🧪️tests` | 3/5 | NATIVE-IMAGE | error: Host storage cannot admit fixed image envelope |  |
| B041 | `FM/🏃️process/🧭️routing/🎛️command/🧪️tests/❄️cold` | 0/1 | STALE-LAW / DEFECT |  |  |
| B042 | `FM/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary` | 1/2 | STALE-LAW / DEFECT | error: expect(received).toEqual(expected) |  |
| B043 | `FM/🏃️process/🧭️routing/🎛️command/🧪️tests` | 2/3 | STALE-LAW structural control/invocation | error: Live Original process issuance required |  |
| B044 | `FM/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests` | 1/0 | GREEN |  |  |
| B045 | `FM/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📊️profile` | 1/0 | GREEN |  |  |
| B046 | `FM/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📤️fractional` | 0/2 | STALE-LAW structural control/invocation | error: Live Original process issuance required |  |
| B047 | `FM/📁️filesystem/🛡️durability/🧪️tests` | 0/2 | STALE-LAW / DEFECT |  | Go split: root package has no .go files after the per-member split (unix/windows in subdirs) -> import path/layout (go-split executor) |
| B048 | `FM/🧬️schema/✅️validator/🧪️tests/🧩️neutral-owner` | 0/1 | NEEDS-CARGO | error: process didn't exit successfully: `bun '/Users/ueli/Documents/semio/🧰️framework/🛍️p |  |

## Open items for the coordinator
1. After PAUSE LIFTED and the sync: `bun repo tools setup nx` (unblocks the 14 TOOLING suites), then `show projects`; the REPO-RUN suites then need the real `bun repo run` per project target.
2. `apply.sh apply` for R.json/T.json (idempotent; verified via overlay on the suites above); re-run A007, A013, A024, A068/A069 afterwards (children/self-spawning tests were not overlay-verifiable).
3. Decision: `.cargo/config.toml` still defines `[profile.prototype]`/`[profile.release]` while A087/A067 laws (and cargo itself, via the temp manifests) require inline profile selection.
4. `phaseCompilerAdmissionControlV1` (structural control) has no production caller; delete it once the last tests (A021, A064, A065, A100/A102, B007, B010, B026, B013, B022, B043, B046) are migrated to issued controls (greenfield: no legacy API).
5. Dependency declarations (package.json): `cross-spawn` (B004/B005), `@semio-tech/framework/transient` in the coordinator Next app (A114).
6. Go durability layout (B047) and `tsc --ignoreConfig` (A099) are real defects in other owners' areas.
7. Hygiene: nothing left in the repo except `.cache/ep2/*` (artifact dirs of wrap2 runs, gitignored) and the diagnostics store `.cache/b/o/m/tests/…` run dirs created by `wrap3` (retention-managed; an invalid sibling dir I created once was removed immediately). Delete `.cache/ep2` after PAUSE LIFTED. Raw logs under `C/🗑️generated/env-pass2-*` can be deleted; digests are in this file and `env-pass2-tools/final-{a,b}.json`.

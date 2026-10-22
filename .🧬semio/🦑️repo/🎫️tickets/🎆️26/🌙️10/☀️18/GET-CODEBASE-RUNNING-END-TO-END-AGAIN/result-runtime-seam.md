# result-runtime-seam (executor runtime-seam, wave 1021a, round 2)

READY2 and audit blockers cleared. Staged and verified on scratch trees, NOT applied live (apply waits for chain17 green + coordinator GO). `C` = this ticket folder, tools `C/runtime-seam-tools/`, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `P` = `🧰️framework/🔨️modules/🏃️process`.

Nothing was written under the repository outside this ticket folder (no live edit, no `bun repo`, no cargo/bazel). Scratch trees: `/private/tmp/runtime-seam/` (git-visible `🧰️framework` clones, 29k files, git only inside the scratch tree, commit signing disabled there).

## 1. Outcome

| item | round 1 | round 2 |
|---|---|---|
| Bun-specific findings in production code | 138 in a 76-row tolerated-exception ledger | **0, and no ledger exists any more** (`⚖️law/📒️.json` is gone; `checkRuntimeLawV1` has no exception list) |
| Law result over the git-visible tree | 1,235 findings migrated, 138 ledgered | 0 violations over every git-visible source file of the transformed 29k-file clone (the module test runs it) and over the 775 files the live dry run touches |
| Node adapter | glob, TOML, JSONC, which, resolve, spawn, serve, sqlite, semver, stdin, file | + YAML, + transpile/scan (own scanner), + bundle (esbuild behind our interface), + FFI (native-binding provider port), + loader plugin (`module.registerHooks`), + test API and runner |
| Test runner | `bun test` only | one thin test API (`test`, `describe`, `skipIf`, hooks with timeouts, completion callbacks, `expect`) behind the runtime interface: `bun test` and `node --test` run the same file |
| English-only runtime errors | serve body limit and law listing | none: every runtime error comes from the EN/DE catalog (`🧬️schema/🗣️messages.json`, 57 ids, no default language: the language from the environment, else both) |
| Facade load | about 8 ms | import 0.6 to 1.7 ms median for every family entry point, adapter loads on first member use (see 4) |

## 2. How the 138 findings went to 0

| class (n) | now |
|---|---|
| launcher (59): `["bun","repo",...]`, runtime alias in policy, `process.execPath` stand-ins | `runtimeCommandNameV1()`, `runtimeProgramV1(word)` (the host's registry name becomes the running executable, every other word stays), `isHostRuntimeCommandV1`, `isRuntimeExecutableV1` |
| test-runner-command (19), test-lane (12): `bun test`, preload, `bun:test` lanes | `runtimeTestArgumentsV1`, adapter member `testPreload`; the sandbox guard picks `./<adapter dir>/🔌️preload` from the registry, so a Node preload is one new directory; `atTestEnd` uses `runtimeTestApiV1().afterAll` |
| embedded-payload (12) | Bun-only child programs moved into `🍞️bun` directories next to their owner (volume-event program, closure loader, Bazel hermetic runner, guard preload and SQLite shim, ambient `bun:sqlite` types); owners re-export or import them; payload checks use `process.versions[<detect key from the registry>]` |
| nx-adapter (21) | `target:"runtime"`, `runtime?:RuntimeIdV1`, `runtimeExecutableForV1(id)`; receipt schema takes `name` from `runtimeIdentityV1().product` and `target` from the id (Bun output byte-identical: still `Bun`/`bun`) |
| package-manager (6) | `runtimePackageCommandV1("add"|"exec", items)` from the registry row (`bun add --no-save` / `npm install --no-save`), store label renamed `node-modules` |
| instrumentation / config-data / ambient-types / condition-vocabulary / toolchain-pin (3/2/2/1/1) | spawn census through `runtimeObserveSpawnV1` (the owned spawn family reports duration), builtin examples `node:test`/`node:sqlite`, ambient shim moved, export conditions from `runtimeIdsV1()`, pinned provisioner reads its runtime name from its own manifest (`runtime` field added to `🟦️bun/🧬️schema`) |
| found by the stricter law (not in the old ledger) | `"Bun" in globalThis` blob/data-URL switch -> `runtimeSourceUrlV1` (registry `sourceUrl`), `versions?.bun` alias in `⏳️async/🪃️continuation`, rustup TOML fallback that provisioned a pinned Bun (removed: portable TOML), static analyzers that recognised the `Bun.file` spelling now recognise `runtimeFileV1` |

Law (`⚖️law/🟦️.ts`): no `Bun` global, no string `bun`, `bun:*` or `Bun` (program words, specifiers, `"Bun" in globalThis`), no `versions.bun` / `versions?.bun`, no `import.meta.dir|path|main`, no embedded template payload doing any of that, in production code outside any directory named `🍞️bun`; seam files may contain registry names. Test files are exempt (and are not codemodded; only the ones that had to follow an API change carry manual edits).

New facade names this round: `runtimeProgramV1`, `isHostRuntimeCommandV1`, `runtimeCommandNameV1/NamesV1`, `runtimeIdsV1`, `runtimeAdapterNameV1`, `runtimeObserveSpawnV1`, `runtimeTestPreloadV1`, `runtimeSourceUrlV1`; registry rows gained `product` and `sourceUrl`; identity answers from registry + process without loading any adapter (`revision` reads it on demand).

## 3. Verification (scratch twins `p3` before / `w3` after, nice 10, load guard < 30, one heavy process at a time)

- Apply on a pristine clone: 772 files, 553 changed, 1,098 codemod edits, 174 anchored manual edits, 47 files created (44 module files + 3 extracted), 3 moves, 0 refused; second run 0 changes (idempotent). Dry run against the **current live tree**: 781 files, 555 changed, 0 refused, law 0 violations.
- Module tests `bun test ⚙️runtime/🧪️tests`: 33 pass, 0 fail, 1,555 asserts; the same file under `node --test` (Node 24.15): 33 pass, 0 fail. Oracles: picomatch, Bun.Glob, `@iarna/toml`, `jsonc-parser`, `semver`, `yaml`, Bun.Transpiler, esbuild; new tests cover command words, preload syntax per runner, source URLs, spawn observer, lazy adapter loading (subprocess), thin-API variants (completion callback, `skipIf`, non-void returns, hook timeouts) and the stricter law rules. Codemod tests 6/6.
- Type check (noResolve, node+bun types, negative control proven earlier): 10,609 errors before, 10,601 after, 27 gone, **0 genuinely new**; the 19 listed as new are 17 jsx path-text lines and 2 imports of moved/created files that exist but are outside the no-resolve file list.
- 50 suites run before/after (16 from round 1 + 34 next to every round-2 site: witness/events, conditional retirement, cache descriptors, sandbox, `🟨️bun` files, routing, authorizers, headroom, physical bundle, profile, bazel, schema-reads, runtime discovery, closure, load, and others): **pass/fail identical in all 50** (357 pass / 297 fail on both sides; 18 green on both; the red ones are red before, unrelated to this work). A real regression found and fixed this way: the profile test counted native `Bun.spawn`; it now goes through the owned spawn family.
- Bugs the verification caught in my own round-2 work, fixed: receipt schema pinned `Bun`/`bun`; AST-extracting witness tests needed the extracted payload file and bindings; loader payload interpolated a module-level `policy` (now declared in the extracted file); `Bun.build` output `arrayBuffer()` is not part of our output type (now `bytes()`); Node nested-test environment leaked `NODE_TEST_CONTEXT` into a probe.

## 4. Facade load (Bun 1.3.14, cold process, node builtins warmed, `process.cpuUsage` around `import()`, 40 to 60 runs, nice 10)

| entry | median ms | p25 |
|---|---|---|
| identity (`runtimeIdentityV1`, command words) | 1.67 | 1.55 |
| process (`which`, `spawn`) | 1.58 | 1.44 |
| source | 1.46 | 1.31 |
| parse / glob / serve / sqlite / semver / native | 1.25 to 1.43 | 1.15 to 1.33 |
| selection alone / entry-point test | 1.07 / 0.61 | |
| control (one empty emoji-path module) | 0.23 | |

Every family entry point imports in < 2 ms (round 1: about 8 ms). The adapter is loaded by `createRequire` on the first call that needs a member and costs about 0.9 ms once, so `import` + first member (`runtimeSleepV1`) measures 2.3 to 2.7 ms median; identity-only users never pay it. Per-file fixed cost in Bun with emoji paths is about 0.5 ms, so further cuts need fewer files, not less code.

## 5. Open points and decisions

- Residual text that is not runtime API and stays for the owners of those layers: 272 shebangs `#!/usr/bin/env bun`, CLI usage/help strings (`bun ./📜️script.ts ...`, `bun repo run ...`), lockfile/workspace logic (`bun.lock`, `🗂️workspaces/🟦️bun`, `bunx`), 22 JSON execution/policy files with `"program":"bun"` (data; code resolves the word through `runtimeProgramV1`), schema enums/pins naming `bun`. pm-seam and repo-launcher own them; the law deliberately scans code, not data.
- Node has no sandbox-guard preload yet (the guard looks for `🟢️node/🔌️preload`; absent means unchanged args). FFI on Node needs a provider addon (`runtimeNativeSystemV1` takes it from the provider port); without one the family reports its capability error in EN/DE.
- `RuntimeAdapterV1.identity()` stays as the source of `revision`; `product` is duplicated in registry and adapters on purpose, a test keeps them equal.
- Cache identity: unchanged on Bun by construction (`runtimeKeyV1()` = `bun:<version>`, digests use `runtimeIdentityV1().version`, receipt keeps `Bun`/`bun`). The Nx authorizer compile uses the same target mapping as before.
- Pre-existing, not mine: `⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts` calls `compileNxAuthorizerSnapshotV1` without the `entryRole` argument.
- Peers edited `🧾️closures/🟦️.ts` during the round (renamed the capture function); regenerate with `RUNTIME_SEAM_ROOT=<tree> python3 manual-gen.py` and rerun the dry run if the apply refuses. Commit signing hangs in scratch repos on this machine unless `-c commit.gpgsign=false` is passed (verification scripts do).
- Apply: `cd C/runtime-seam-tools && python3 manual-gen.py && python3 place.py && nice -n 10 bun apply.ts` (dry run); `--write` only after chain17 green and GO. `undo.py` restores from `C/🗑️generated/runtime-seam/backup`.

## 6. Files

- Tools: `C/runtime-seam-tools/{apply.ts,codemod.ts,codemod.test.ts,manual-gen.py,manual-gen2.py,manual-edits.json,messages.py,place.py,undo.py,clone.py,suite-list.py,suites.py,suites.json,mk-tsconfig.py,verify-all.sh,r2-check.sh,iterate.sh,guarded.sh}`; ASCII-named sources `src/` (44 files); staged module `staged/new/🧰️framework/🔨️modules/⚙️runtime/` (44 files: contract, selection, messages, 12 family entry points, Bun adapter + tooling, Node adapter + esbuild/plugin/native/test, portable glob/TOML/JSONC/YAML/source/expect/which/resolve/spawn/serve/sqlite/semver/stdin/file, law, tests + fixtures).
- Audit input answered: `C/audit-staged-1021a.md` blocker 2.
- Scratch (removed at the end of the round except the overlay): `/private/tmp/runtime-seam/`.

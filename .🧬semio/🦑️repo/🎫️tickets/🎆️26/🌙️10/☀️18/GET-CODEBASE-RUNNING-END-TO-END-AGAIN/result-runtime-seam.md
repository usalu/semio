# result-runtime-seam (executor runtime-seam, wave 1021a)

READY (staged, verified on scratch trees, NOT applied live; apply waits for chain17 green + coordinator GO). `C` = this ticket folder, tools `C/runtime-seam-tools/`, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `P` = `🧰️framework/🔨️modules/🏃️process`.

Nothing was written under the repository (no live edit, no `bun repo`, no cargo/bazel). Scratch trees lived under `/private/tmp/runtime-seam/` (clones of the git-visible `🧰️framework` source, 29k files, with git init only inside the scratch tree).

## 1. What now exists

A new leaf module `🧰️framework/🔨️modules/⚙️runtime/` (no package.json, so no lock/sync cycle; relative imports like the other framework modules). Neutral code imports only its facade `⚙️runtime/🟦️.ts`; every type in it is ours.

| part | path | role |
|---|---|---|
| contract | `📋️contract/🟦️.ts` | own types: families, adapter, glob, spawn, serve, file, sqlite, native binding, transpiler, bundler, identity; `RuntimeCapabilityMissingV1` |
| schema (SSOT) | `🧬️schema/🔣️.json` | closed registry: runtime ids `bun`, `node`; detection key, executable, minimum version, adapter dir, builtin names/prefixes, TypeScript `--types` environment, and a per-family support level `native|standard|portable|missing` |
| selection | `🧭️selection/🟦️.ts` | detects the host from the registry, memoizes one adapter, `runtimeSupportV1(family)`, refusal for missing families |
| Bun adapter | `🍞️bun/🟦️.ts` | the only file that names the `Bun` global or `bun:` modules (global reached loosely, so it type-checks without Bun types) |
| Node adapter | `🟢️node/🟦️.ts` | standard APIs + our portable code only; absent families are `missing` in the registry |
| portable code | `🪶️portable/*` | our own: glob matcher+scanner, TOML 1.0 parser, JSONC, which, ESM resolver, spawn (node:child_process, web streams), HTTP serve (node:http, web Request/Response), SQLite wrapper (node:sqlite, bun-shaped API, `null` for a missing row), semver ranges, stdin, file, entry-point test |
| law | `⚖️law/🟦️.ts` + `📒️.json` | static scan (comment/string/regex-aware tokenizer) + ledger check |
| tests | `🧪️tests/` | 20 tests, language-agnostic JSON corpus `🧫️fixtures/🔣️.json`, plus a script that runs the whole stack under real Node |

Facade names (all `V1`): `runtimeIdentityV1`, `runtimeKeyV1` (`bun:1.3.14`, the exact string old cache keys used), `runtimeSupportsV1`, `runtimeParseTomlV1/JsoncV1/YamlV1`, `runtimeWhichV1`, `runtimeResolveV1`, `runtimeSleepV1`, `runtimeSpawnV1/SpawnSyncV1`, `runtimeStdinTextV1/StreamV1`, `runtimeFileV1`, `runtimeServeV1`, `GlobV1`, `TranspilerV1`, `SqliteDatabaseV1` (class, bun-shaped), `runtimeNativeSystemV1` (dlopen/FFIType/read/ptr/toArrayBuffer/JSCallback as our types) and `runtimeNativeOpenV1/ReadV1/PointerV1/CallbackV1`, `runtimeBundleV1`, `runtimeSemver*V1`, `isMainModuleV1`, `isRuntimeBuiltinSpecifierV1`, `isRuntimeExecutableV1`, `runtimeExecutableV1`, `runtimeTypeEnvironmentV1`, `runtimeCollectGarbageV1`.

FFI (leases) is a native-binding interface: `P/🔒️leases/📁️physical/🟦️.ts` now calls `runtimeNativeSystemV1()` and guards with `runtimeSupportsV1("ffi")` (flock, errno read, descriptors unchanged). Same for the directory exchange, Windows host, jobserver semaphore, conditional retirement, descriptor, observe, tools kernel32.

## 2. Census (scoped `nice -n 10 git grep`, production code under `🧰️framework`, tests/tickets excluded)

First regex census: 814 hits in 478 files. The AST/law scanner (which also sees plain `"bun"` command strings) found, before the change, **1,235 findings in 538 of 759 candidate files**: `import.meta.dir` 481, `import.meta.main` 124, `import.meta.path` 5, `Bun` global 234, `"bun"` literal 270, `bun:sqlite` 37, `bun:ffi` 23, `process.versions.bun` 28, `bun:test` 7, embedded 17.

| family | Bun API (occurrences before) | now behind | disposition |
|---|---|---|---|
| glob | `Bun.Glob` 28 | `GlobV1` | own matcher + scanner; Bun adapter keeps native |
| config parse | `Bun.TOML` 66, `Bun.JSONC` 9, `Bun.YAML` 1 | `runtimeParse*V1` | own TOML/JSONC (portable), Bun native on Bun, YAML Bun-only |
| spawn | `Bun.spawn` 10, `spawnSync` 9 | `runtimeSpawn*V1` | node:child_process under both runtimes (web streams) |
| file / stdin / write | `Bun.file` 3, `stdin` 3, `Bun.write` 1 | `runtimeFileV1`, `runtimeStdin*V1`, node:fs | |
| sqlite | `bun:sqlite` 37 | `SqliteDatabaseV1` | one class; Bun native or node:sqlite; three duplicate runtime switches (leases, baseline, ledger, catalog) collapsed |
| ffi | `bun:ffi` 23 | `runtimeNativeSystemV1` | Bun adapter only; Node `missing` |
| serve | `Bun.serve` 2 | `runtimeServeV1` (async) | node:http adapter + Bun native; remote cache and storybook now `await` it |
| hash | `Bun.CryptoHasher` 1 | node:crypto | standard |
| resolve / which / sleep | `resolveSync` 20, `which` 9, `sleep` 8 | `runtimeResolveV1`, `runtimeWhichV1`, `runtimeSleepV1` | own ESM resolver (realpath like Bun), own which |
| transpile / bundle | `Bun.Transpiler` 19, `Bun.build` 16 | `TranspilerV1`, `runtimeBundleV1` | Bun adapter only; Node `missing` (honest) |
| semver | `Bun.semver` 2 | `runtimeSemver*V1` | own npm-range implementation |
| version / identity | `Bun.version` 6, `process.versions.bun` 28 | `runtimeIdentityV1`, `runtimeKeyV1` | digests and cache keys byte-identical on Bun, so no cache invalidation |
| meta | `import.meta.dir/path/main` 610 | `import.meta.dirname/filename`, `isMainModuleV1(import.meta.url)` | standard / our test (real-path compare, symlink-safe) |
| gc | `Bun.gc` 1 | `runtimeCollectGarbageV1` | |
| builtin vocabulary | `startsWith("bun:")`, `=== "bun:sqlite"` | `isRuntimeBuiltinSpecifierV1` | registry-driven (every registry row, since it is a fact about source text) |
| tsc types | `"--types","bun"` | `runtimeTypeEnvironmentV1()` | registry field |
| command name | `runCmd("bun",[script…])` | `runtimeExecutableV1()` | only when the argument is a script path |

## 3. Migration (staged)

`C/runtime-seam-tools/apply.ts` = AST codemod (`codemod.ts`, TypeScript compiler API, comments/strings untouched) + 66 anchored hand edits (`manual-edits.json`, generated read-only by `manual-gen.py`) + creation of the 23 module files from `staged/new/`. On a pristine clone it changes **502 files (995 codemod edits + 66 manual edits), creates 23 module files, refuses nothing**; a second run changes 0 files (idempotent). All-or-nothing: if any anchor is refused nothing is written. Backups to `C/🗑️generated/runtime-seam/backup/`; `undo.py` restores them.

Rules for the live apply: dry run (default) `cd C/runtime-seam-tools && nice -n 10 bun apply.ts`; live `... bun apply.ts --write` only after chain17 green and GO (it edits `L` and `P`, the frozen trees). Current live dry run: 760 files, 502 changed, 0 refused, law 0 violations / 0 stale.

## 4. Law and ledger

Law: no `Bun` global, no `bun`/`bun:*` specifier, no `process.versions.bun`, no `import.meta.dir|path|main` in production code outside `⚙️runtime/🍞️bun/`; inside the seam only the registry's runtime names may appear as strings. The `⚖️law/🟦️.ts` scanner + test (`no production source of the framework names a specific runtime outside the adapter and the ledger`) lists git-visible files, so it also catches files no grep pattern sees. Result on the transformed tree: **1,235 findings -> 138 findings in 66 files, every one ledgered, 0 violations, 0 stale ledger rows** (stale rows fail the test, so the ledger can only shrink).

Ledger `⚖️law/📒️.json` (76 rows, 138 findings):

| class | n | owner | why it stays |
|---|---|---|---|
| launcher | 59 | repo-launcher | `["bun","repo",...]`, runtime alias in policy; replaced when `bun repo` is replaced |
| test-runner-command | 19 | test-runner | `bun test` subcommand |
| test-lane | 12 | test-runner | files bound to `bun:test` (store one-item lane, sandbox guard, preload) |
| embedded-payload | 12 | runtime-seam | script text run by a child runtime (volume-event preparation, closure loader `Bun.plugin`/`Transpiler`); needs a real module importing the facade |
| nx-adapter | 21 | backend-contract | Nx authorizers/vite/bundle code (excluded from the codemod on purpose: `target:"bun"` is in its public signature and tests) |
| package-manager | 6 | pm-seam | `bun add`, `bun x`, dependency-store names |
| instrumentation / config-data / ambient-types / condition-vocabulary / toolchain-pin | 3/2/2/1/1 | various | spawn-census wrapper, builtin-name list in a `.cjs`, `bun:sqlite` module augmentation `.d.ts`, package export condition names, pinned toolchain kind |

## 5. Verification (all on scratch trees, nice 10, load guard < 30, one heavy process at a time)

- Module tests `bun test ⚙️runtime/🧪️tests`: 20 pass, 0 fail, 1,332 asserts, on the freshly transformed tree. Content: registry validates with Ajv and every adapter provides exactly what the registry declares; glob match corpus on both adapters plus picomatch (40 of 44 rows are picomatch-checked) and Bun.Glob; glob scan across 6 option sets x 15 patterns (dot, symlinks, broken symlink, async and sync) identical on both adapters; TOML corpus vs `@iarna/toml`, and every `*.toml`/`Cargo.lock` of the repo (first 400) vs the oracle; JSONC vs `jsonc-parser`; semver (88 version/range pairs and 64 ordering pairs) vs `semver`; which/resolve/spawn/serve/sqlite agree between adapters; entry-point test via symlink; flock/getpid through the native binding; Transpiler scan + dead-code removal + bundle; the stack under real Node 24.15 (`node 🧪️tests/🟢️node/🟦️.ts`, node adapter reported, 0 failures); law corpus; live law over the git-visible tree.
- Differential fuzz glob vs Bun.Glob (not committed): 317,614 pattern/path pairs, 26 differences, all exotic (a `**` glued to other characters or to a brace group, `****`, paths ending in `/` against `**/*`).
- TOML oracle on 4,597 repository files: 4,593 identical to `@iarna/toml` and to Bun.TOML (4 files invalid in all three). Bun.TOML itself is wrong for `\t` escapes, multiline strings, nested mixed arrays and datetimes; the corpus marks those rows `nativeDefect`.
- Type check: the whole seam compiles with `types:["node"]` only (no Bun types), so no Bun API can be used outside the adapter's loose global access. 620 touched files compiled before/after (noResolve, node+bun types as roots, negative control proven sensitive): **0 new errors**, 22 fewer. (The 17 `TS6142` jsx path-text differences are tree-path noise.)
- 16 touched suites run before/after in twin scratch trees (lease descriptors, directory storage, remote cache serve+file, cache-gc sqlite, cargo workspaces + toml + command + preparation storage, zero-touch, selection, ledger, descriptor publication, registry import language, providers): pass/fail counts **identical in all 16** (several are red in both: ledger 3/13, zero-touch 32/2, providers 3/3, descriptor-publication 2/5, cargo-command 1/3, cargo-workspaces 0/1, lease-descriptors 1/1; pre-existing/environment, unchanged).
- Codemod: `codemod.test.ts` 6/6.
- Cost: the facade adds about 8 ms of module load per process on Bun (16 ms vs 8 ms for the Bun adapter alone, because the Node adapter and portable stack are imported statically so Vite's config bundler can follow them). Lazy adapter loading would save that but breaks bundlers; left as is.

## 6. Coordination and open points

- repo-launcher: its registry (`🚀️launcher/🗂️registry/🔣️.json`, `runtimes` keyed by id with `executable.unix`) provisions runtimes; ours says what a hosted runtime can do. Both are keyed by runtime id; a test in `🧪️tests` asserts every launcher runtime id exists here with the same executable name (skips until the launcher is live). All `bun repo` and `bun test` command strings are ledgered for the launcher/test owners, not touched.
- storage-seam / pm-seam / backend-contract edit neighbouring files (directory exchange, Windows host, dependency storage, authorizers). The apply is anchored and refuses on drift; their edits to the same lines make it refuse the whole batch, in which case regenerate with `python3 manual-gen.py` and rerun the dry run.
- tool-iface: `runtimeSpawnV1` is the runtime primitive, not a budgeted runner; sites that used `Bun.spawn` raw keep their (absent) budgets until tool-iface routes them.
- Behaviour differences to know: node adapter serves files without a content type; `serve` is async; `get()` returns `null` for no row on both adapters (what Bun already did); `resolve` returns real paths (what Bun did).
- Not done: Node adapter for YAML, FFI, transpile, bundle (declared `missing`); the 12 embedded payloads; test files are exempt from the law (they still use `bun:test`/`Bun.*`).
- Generated cache keys: unchanged on Bun by construction (`runtimeKeyV1()` = `bun:<version>`, digests use `runtimeIdentityV1().version`), except the Nx authorizer compiler record (`target:"runtime"`) which rebuilds once.

## 7. Files

- Tools: `C/runtime-seam-tools/{apply.ts,codemod.ts,manual-edits.json,manual-gen.py,ledger.py,place.py,undo.py,clone.py,suites.py,mk-tsconfig.py,verify-all.sh,guarded.sh,codemod.test.ts}`; sources `src/`; staged module `staged/new/🧰️framework/🔨️modules/⚙️runtime/` (23 files).
- Overlay (kept): `/private/tmp/runtime-seam/overlay/`.

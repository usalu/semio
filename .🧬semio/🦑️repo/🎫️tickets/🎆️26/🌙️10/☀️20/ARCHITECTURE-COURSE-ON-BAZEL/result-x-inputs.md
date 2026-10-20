# x-inputs result (running record)

Status 16:50: staged and verified in a byte-copy mirror; waiting for the tree to be quiet before apply. The hard-link mirror is deleted and its tool removed (my error, sorry); `find 🧰️framework 🎓️teaching -type f -links +1` printed nothing at 16:20. Since the quiet-tree request I made NO live edit (the six test files below were edited before it).

## 1. What the inference already adds (command source closure)

`🧭️script-inputs` (`targetScriptSources`) adds to a cached `repo:script` target: the relative and workspace-package import closure of the command script (router), literal `new URL(.., import.meta.url)` targets, and literal MODULE inputs the target declares (not globs, not named inputs such as `default`). JSON imports are followed as resources. It does not follow test files, tsconfig include sets, `.d.ts` siblings, type-only imports, or files read at runtime.

Real inferred closure (live inference, run directly with `real-inferred.ts`): router closure = 1,466 files for `test` and `typecheck`, 0 generated dependencies (bare cache has no generator hook).

## 2. Real gap before (default ∪ inferred closure), measured

Method: `x-inputs-tools/gap-trace.ts` expands `namedInputs.default` + target inputs with `Bun.Glob` over `git ls-files` (tracked + untracked), unions the live inferred closure, and subtracts it from a read set.

| target | read-set oracle | read files | gap |
|---|---|---|---|
| test | fs trace of a real vitest run (`trace-fs.cjs` via NODE_OPTIONS, workers included; 630/631 tests passed, the failure is a missing generated ui-axes module) | 1,099 | 245 |
| typecheck | TypeScript program (`tsc-files.ts`, same tsconfig, createProgram) | 1,474 | 135 |

Test gap composition (245): 125 workspace `package.json` (alias generation `workspaceTestAliasesV1`, scope 🎓️teaching), 4 `tsconfig.json` (vite/esbuild nearest config), `bun.lock` x2, `🎓️teaching/Cargo.toml` and 80 crate `Cargo.toml` (proctor manifest closure read by proctor-build/local-stack tests), 5 energy quiz JSON, ~15 `@semio-tech/assets` sources + fixtures, the vitest `workspace-exports` helper + fixture + schema, `F/🛍️products/📚️course/🟦️.ts`, proctor fixtures `🧪️tests/🧫️fixtures/{📚️catalog,🏠️homes,⚡️power}`, `🔨️modules/📚️course/🧰️fixtures/🦀️.rs`, `🧪️tests/🌐️end-to-end/🧰️harness/🦀️.rs`, `🔨️modules/🗄️storage/🦀️.rs`.
Typecheck gap composition (135): energy quiz (5), assets (~20), workspace-exports, type-only/`.d.mts` siblings (~20 `.d.mts`), `paths`-mapped and type-only imports, kernel/actor test directories reached through types, generated modules (covered by the six generator dependencies).

Raw data: `🗑️generated/x-inputs/{gap-trace-test.txt,gap-tsc.txt,trace-files.txt,tsc-files.txt}`.

## 3. Real gap, before and after

The law (section 6) recomputes the gap = files read − (declared ∪ inferred) with independent oracles. Same method before and after; "before" = live manifest and live inference, "after" = staged state in a byte-copy mirror.

| target | read set (oracle) | files read | gap before | gap after |
|---|---|---|---|---|
| test | esbuild metafile of the vitest config + its 13 `include` tests, workspace manifests, nearest tsconfig, Cargo manifests of the proctor crate closure, 29 recorded runtime reads (fs trace) | 1,388 | 254 | 0 |
| typecheck | the TypeScript compiler's program for `tsconfig.json` | 1,493 | 132 | 0 |
| check | esbuild closure of the router + 1,067 `.rs`/`Cargo.toml` of the proctor crate roots (`proctorSourceDirectories`) | 2,008 | 1,018 | 0 |
| Python implementation (own glob, import scanner, Cargo parser) | | 1,223 / 1,070 / 924 | 226 shared with oracle (test) | 0 |

Earlier raw measurements agree: fs trace of a real vitest run 245 of 1,099; tsc program 135 of 1,474; Rust 1,018 of 1,067. The audits' "837 of 1,249 undeclared" were gaps against `default` only. The inferred router closure already covered 1,466 files; most of the 837 were never a gap.

Closure size (files hashed through the command closure): test 1,466 → 1,691 (tests and their imports), typecheck 1,466 → 1,5xx (program), check unchanged for TS, Rust sources now declared. Needless misses removed: `default` loses `🖱️ui/**/*` (2,342 tracked files), because the derived closures cover every ui file the targets read (the law still passes without it). Root `package.json` and `Cargo.toml` stay: both are read (workspace ownership; proctor manifest closure), so the audit's "over-broad" finding does not hold for them.

## 4. Design

Derived (generic) mechanism, opt-in per target through `metadata.semio.closure` (schema `🏗️build/🧬️schema/📋️project/🔣️.json`, closed object `{modules?, typescript?}`):
- `modules`: `{projectRoot}`/`{workspaceRoot}` globs of module files (`!` excludes). The inference expands them through the observed directory reader and adds the files as closure ENTRIES, so their whole import closure (workspace packages, JSON imports, `new URL` targets) joins the target. `test` declares `📚️course/🧪️tests/**/🟦️.ts` (vitest does not reach tests from the router).
- `typescript`: literal tsconfig paths. The inference runs the compiler's own program (include set, `paths`, `.d.ts`/`.d.mts` siblings, type-only imports, JSON, `extends`), joins every workspace file to the closure, records generator targets whose outputs the program reads and turns the installed packages it resolved (`@types/*`, `typescript`, `vite/client`) into `externalDependencies`. `typecheck` declares `{projectRoot}/tsconfig.json`.
- Opt-in on purpose: 154 existing `inputs` strings in 42 projects are module globs used for hashing only; reading them as entries would change every one of those targets and slow graph loading.
- Generic reference scanner improvement (`⚡️caching/📥️inference`, `commandUrlReferences`): `new URL("x", <expression containing import.meta.url>)` (the `physicalSourceOriginV1(import.meta.url, …)` form the build library itself uses) and `join`/`resolve(import.meta.dir | import.meta.dirname | dirname(fileURLToPath(import.meta.url)), "lit", …)` with literal segments now join as files. This closed the schema JSON reads of `🏗️build/📤️arguments`, `🧾️store-receipt` and `🏗️native-build`. The cache key tag changes (`urls-nested`) so persisted facts are recomputed.

Hand-written, minimal (named inputs in the course manifest; each is a runtime read no scanner derives from imports):
- `workspaceManifests`: `bun.lock` (root, 🎓️teaching), `🎓️teaching/package.json`, `🧰️framework/**/package.json`, `🧰️framework/**/tsconfig.json` (134 tracked package.json vs 125 read; the vitest alias helper reads every scope member manifest, vite/esbuild the nearest tsconfig).
- `proctorTestReads`: proctor `🧪️tests/🧫️fixtures/**`, `🔨️modules/📚️course/🧰️fixtures/**`, `🧪️tests/🌐️end-to-end/🧰️harness/**`, `🔨️modules/🗄️storage/🦀️.rs`.
- `proctorCargoManifests` (test, check): root/teaching/framework Cargo.toml and Cargo.lock, proctor `Cargo.toml`, `Cargo.toml` of the 20 framework crate roots of the proctor build (data `staged/rust-roots.json`, generated from the oracle `proctorSourceDirectories` by `rust-closure.ts`).
- `proctorRustSources` (check): `**/*.rs` of those 20 roots.
- `test`: `dependsOn` the six generator targets `typecheck` uses (the resolution test reads the generated `ui-axes` module; without it the test fails on a clean checkout), `dependentTasksOutputFiles`, `SEMIO_TEST_LEVEL`, `SEMIO_TEST_BUDGET_MS`, `{"runtime":"node --version"}` (vitest runs under node; `bun --version` is in sharedGlobals).

## 5. Test hermeticity (P1-3), live, done
No course test reads `SEMIO_TICKET_DIR` any more (`🏗️proctor-build` x2 threw without it). `🏗️proctor-build`, `🐳️image`, `🧪️deploy`, `🧪️deploy/🧰️deploy-fixtures` and `🧱️local-stack` create scratch with `mkdtempSync(join(realpathSync(tmpdir()), "course-…"))` and remove it (`rmSync` / the `scratch` list). The vitest `cacheDir` is an OS temp dir. Verified: the course vitest run without `SEMIO_TICKET_DIR` passes 680 tests; the 1 failure (`📦️resolution`, missing generated `ui-axes` module) is the missing generator dependency fixed by the staged `test.dependsOn`. Note `bun`/`node`/vite versions are keyed as above; loopback sockets remain (ports are chosen by the tests, no host state is read).

## 6. The law (P1-2 acceptance)
Staged at `🏗️build/🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/` (bun test; `🔣️.json` fixture, `🧬️schema/🔣️.json` closed schema validated with Ajv, `🟦️.ts`, independent `🐍️.py`):
- TS side: real `targetScriptSources` on the real manifest; declared inputs expanded with `Bun.Glob`; reads from esbuild (with `import.meta.vitest` defined away), `ts.createProgram`, `proctorSourceDirectories`, workspace `workspaces` list, nearest tsconfig, recorded fs-trace residual.
- Python side: own glob translator, comment-aware import scanner (no TypeScript, no Bun), Cargo `path` dependency parser. Asserts: identical declared selection (Bun.Glob vs Python), and no file both implementations read is uncovered.
- Sensitivity control: removing `proctorTestReads` / `workspaceManifests` / the closure metadata makes the gap contain the proctor catalog fixture, `🎓️teaching/package.json` and the energy quiz JSON.
- Mechanics law `🧪️tests/📐️declared-inputs/`: `modules` globs against esbuild and Bun's bundler, a declared tsconfig against `tsc --listFilesOnly`, mutations (new module under a glob moves closure and digest; excluded test and non-module file do not), nested `new URL` and `join(dirname(fileURLToPath(import.meta.url)),…)` references, and "no metadata = unchanged".

Final clean run (`check.sh mirror`: fresh byte copy of the current live tree, apply twice, second apply prints nothing but `unchanged`): mechanics+closure laws 11 pass 0 fail, course law 7 pass 0 fail, providers+covered-inputs 6 pass 0 fail. Earlier incremental runs: 17 pass, 1 skip (existing Nx-invocation test), 0 fail across `📐️declared-inputs`, `🔗️workspace-closure`, `🎓️course-inputs`; `🔌️providers`/`🧮️covered-inputs` 6 pass. `💾️cache` has one failure that is identical in the live tree (tmpdir under `/var` symlink), not caused by this work. The law before the fix fails 4 of 6 (gaps 254/132/1,018, python 226).

## 7. Files
Live (done before the quiet-tree request): `🎓️teaching/🏛️architecture/📚️course/🧪️tests/{🎚️config,🏗️proctor-build,🐳️image,🧪️deploy,🧪️deploy/🧰️deploy-fixtures,🧱️local-stack}/🟦️.ts`.
Staged in `x-inputs-tools/staged/` (`apply.py [--root <ws>] [--dry]`, idempotent, atomic writes, loud on a moved anchor; `check.sh mirror <dir>|live`):
- `🧭️script-inputs/🟨️.mjs` (declared globs, TypeScript projects, packages), `⚡️caching/📥️inference/🟨️.mjs` (reference scanner), project schema, course manifest, `🦑️repo/README.md`
- new tests: `🧪️tests/📐️declared-inputs/*`, `🧪️tests/🎓️course-inputs/*`
Tools (`x-inputs-tools/`): `mkmirror-copy.py` (byte copy only), `real-inferred.ts`, `gap-trace.ts`, `gap-files.ts`, `lib-inputs.ts`, `tsc-files.ts`, `rust-closure.ts`, `trace-fs.cjs`, `trace-files.py`, `try-globs.ts`.

## 8. Apply order (when the tree is quiet and the freeze is lifted)
1. `python3 x-inputs-tools/staged/apply.py --dry` (anchors), then without `--dry`. One run applies manifest, inference, schema, scanner, README and the two test directories together; the manifest needs the inference change (the `closure` metadata is inert without it) and the schema needs both.
2. `bash x-inputs-tools/staged/check.sh live` (needs a booting tree: the law imports the inference).
3. Real run: `bun repo --tool=bazel run @teaching/architecture-course:test` twice (second must be a cache hit), edit an energy quiz JSON, run again (must miss), edit nothing (hit); same for `typecheck`.
4. Graph load will recompute facts once (new scanner key).
Open: peers' `check` metadata (cache class) can sit next to `metadata.semio.closure`; apply.py refuses to add `closure` to a target that already has `metadata` (test, typecheck have none today) and says so.

## 9. Not verified
Real `bun repo` graph load and Bazel runs (tree frozen/quiet); `check` was not run (cargo); Windows path behavior of the new code (it normalizes `\` in the TypeScript-project path test only).

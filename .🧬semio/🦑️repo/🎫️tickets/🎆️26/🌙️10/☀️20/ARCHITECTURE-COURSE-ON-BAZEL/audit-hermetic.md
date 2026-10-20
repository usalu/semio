# 🔬️ Hermetic Audit: `@teaching/architecture-course`

Auditor: `aud-hermetic` (read-only). Date: 2026-10-20. No source file was edited, no build, test, cargo, docker or bazel command was run. Scratch files are under `🗑️generated/aud-hermetic/`, the import-closure helper is `aud-hermetic-tools/closure.mjs`.

Path legend (repo-relative): `C` = `🎓️teaching/🏛️architecture/📚️course`, `P` = `C/📦️packages/🟦️typescript` (the project root), `F` = `🧰️framework`, `R` = `F/🛍️products/🦑️repo/🔨️modules/📚️library`, `PR` = `🎓️teaching/🛂️proctor`, `PB` = `PR/🏗️bootstrap`. Line numbers are from the snapshot read at audit time. `P/📋️project.json` changed during the audit (cache-class metadata added to `preflight`, `ci-browser-prepare`, `build`, `test-e2e`, `docker-*`, `docker-stack-bundle`); the lines cited below are the current ones.

Coverage caveat: fully read are the router, `P/📋️project.json`, `C/🟦️.ts`, `C/🏗️builder/🌐️vite/🟦️.ts`, `C/🧱️stack/🟦️.ts`, `C/🎭️e2e/🟦️.ts`, `C/🚀️deploy/🟦️.ts`, `🌐️site`, `🐳️docker-image`, `📦️image`, `🎁️stack-bundle`, `⚖️readiness`, `PB/🟦️.ts`, `PR/📦️packages/🦀️rust/📜️script.ts`, and the framework entry points they call (routing, vite caching, cache store policy, vitest runner, preflight, typecheck, cargo native-build relay). Only grep-level reads were done of `🩺️docker-image-check`, `🧱️docker-stack`, `🎭️e2e/🚶️learner` (889 lines), most of `🧪️tests/**`, and `🧫️fixtures`.

---

## 1. Verdict

Not Bazel-ready. Four classes of defect block every course target:

1. The router cannot locate its workspace inside a Bazel execroot (symlinked inputs), and the course reads far more of the framework than `P/📋️project.json` declares.
2. The build product is written into the framework's bounded native cache store (`.cache/b/p/…`, a mounted image), not into a declared output. `build` declares no outputs at all.
3. Every stateful verb requires caller authority that only the native launcher can issue (`SEMIO_SCRIPT_PROCESS_INVOCATION` with a wall-clock deadline, plus `readNativeCallerAuthorityV1`).
4. Several targets are inherently live, host-mutating or network-bound (dev stack, Playwright, Docker, apt, `cargo fetch`, `git` via `sourceRevision`), so they cannot be cacheable actions; they need `local`/`no-cache`/`requires-network` treatment.

Two correctness bugs exist even without Bazel: the course material imports energy quiz JSON from outside its declared inputs (§5.2), and `check` is marked `cache: true` while its Rust closure is under-declared (§3.10).

---

## 2. Blocking findings, in order

### 2.1 Workspace-root discovery breaks on symlinked markers (verified)

- `F/🔨️modules/🏃️process/🧭️routing/🟦️.ts:105–120` (`findWorkspaceRoot`): requires `lstatSync(join(directory, f)).isFile()` for both `📋️project.json` and `package.json` at each ancestor. An execroot entry is a symlink, so `isFile()` is false and the walk skips it.
- Probe (`🗑️generated/aud-hermetic/ws-symlink-probe/probe.ts`, run with `bun`): with real files, `findWorkspaceRoot` returns the root. With the same markers as symlinks (bounded with `stopAt`, as an execroot would be), it throws `workspace root with paired build and package markers was not found`.
- Fallbacks: `start || SEMIO_WORKSPACE_ROOT || REPO_ROOT || process.cwd()` (line 105). Every `new ScriptRouter(import.meta.dir)` (`P/📜️script.ts:131`), `repositoryVitestPolicyV1(cwd)` (`R/🏃️process/📋️context/🔣️policy/🟦️.ts:23`), `repositoryProcessOwnerContextV1` (line 20) and `getWorkspaceRoot()` (`R/🏗️build/🟦️.ts:13`) depend on it.
- Fix: the Bazel backend passes the workspace root explicitly and the marker check uses `statSync` (follows links), or the root is taken from the issued invocation only.

### 2.2 Declared inputs do not cover the real closure

Method: `aud-hermetic-tools/closure.mjs` follows static `import`/`export`/`import()`/`require` edges from the router, the Vite config, the stack, e2e, deploy verbs and `PB`, resolves `@semio-tech/*` through the root `node_modules` symlinks, and checks each reached file against the positive/negative globs of `P/📋️project.json` `namedInputs.default`. Output: `🗑️generated/aud-hermetic/closure-import-only.json`.

- Import-only closure: **1,249 files reached, 837 not matched** by `default`. By group: `R` 378, `F/🔨️modules/🏃️process` 176, `F/🔨️modules/🎭️actor` 59, `F/🔨️modules/🎠️kernel` 48, `F/🛍️products/💻️os` 12, `C/…/⚡️energy` 5 (see §5.2). File types: 567 `.ts`, 227 `.json`, 39 `.mjs`.
- Why so large: barrels. `R/📦️packages/🟦️typescript/🎮️playground/🟦️.ts` re-exports the whole dev/build library; `@semio-tech/repo-lib/cargo` (used by `P` docker verbs) pulls `R/🏗️build/🕸️graph/…`, which reaches the OS product's plugin publication (`F/🛍️products/💻️os/…/📣️publication/🏗️build/🟦️.ts`). The chain is `C/🚀️deploy/🐳️docker-image/🟦️.ts` → `R/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts` → `R/🏗️build/🕸️graph/🟦️.ts` → … → os. Also reached through `R/🚀️bootstrap/🛠️tools/🟦️.ts` (`require.resolve("nx/bin/nx.js")`) and `🛠️tools/🔌️sdk/🟦️.ts` (`locate("nx/package.json")`), so the Nx SDK is inside the course's runtime closure.
- Adding `{projectRoot}` tests does not change the count (import-only with tests: 1,321 / 846).
- The literal-scan variant (quoted `🧰️framework/…` strings) is an upper bound only (3,283 / 2,482) and over-includes; it is not used for conclusions.
- Rust: the proctor's path-dependency closure (`🗑️generated/aud-hermetic/rust-crates.json`) reaches **109 crate dirs, 96 of them undeclared** (upper bound: all `workspace.dependencies` path deps are included). The undeclared ones are `F/🔨️modules/⏳️async`, `🌱️value` (≈30), `🎒️pack` (≈30), `📡️replication` (≈30), `🗣️dsl`, `🚪️io`, `🧬️schema`, `🔀️dispatch`, `🔏️hash`, `🗜️deflate`, `🧵️job`, `🧪️test/🧲️fixture-reader`. `proctorSourceDirectories` (`PB/🟦️.ts:408`, via `PB/🕸️manifest-closure/🟦️.ts`) computes this exactly from manifests; the course never runs it into a declared input set.
- `R/…/🏗️build/…` loads gitignored generated modules (8 reached, e.g. `F/🔨️modules/🖱️ui/🎨️styling/🤖️generated/🔤️tokens/🟦️.ts`, `F/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🖼️icons/🟦️.ts`, `F/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts`). `.gitignore:97` ignores `**/🤖️generated/`. The producers are `@semio-tech/assets:build` (`F/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json`, outputs list in-tree) and `@semio-tech/ui-styling-tokens:generate` (`F/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json`, `outputs: null`, its script prints "wrote generated CSS/TS/C#/Rust/Python styling artifacts" into the source tree). The actor generated module has no producer among the course's `dependsOn`; producer not identified in this audit.
- Fix: generate the input set from the import graph (a tool task, not hand-maintained globs), narrow the barrels the course imports (`repo-lib/dev`, `repo-lib/cargo`), and add the Rust closure. The Bazel package-name rule in the brief (no emoji in package names) also applies to every `C`/`P` path: a mapping layer is needed (decide per backend).

### 2.3 Environment-dependent node_modules and caches

- `P/node_modules` → `…/.cache/b/d/m/j/f812…/w/134/node_modules` (symlink). Root `node_modules` → `/Users/ueli/Documents/semio/.cache/b/d/m/j/81b8…/node_modules` (absolute symlink into the dependency partition mount). Absolute, machine-specific links: not portable, not an action input.
- Vitest: `C/🧪️tests/🎚️config/🟦️.ts:16` sets `cacheDir` only when `SEMIO_TICKET_DIR` is set. Otherwise vitest writes into the course's own `node_modules/.vite/vitest`, which exists in the checkout (`C/node_modules/.vite/vitest`, observed). That write is outside the declared `outputs`.

### 2.4 Vite build writes to the framework store, not to a declared output

- `C/🏗️builder/🌐️vite/🟦️.ts:139`: `cacheDir: viteCacheDirectory(repoRoot, { …process.env, SEMIO_BUILD_MODE })`, evaluated at config load. `R/⚡️caching/🌐️vite/📂️paths/🟦️.ts:8` throws unless `SEMIO_VITE_CACHE_DIRECTORY` is set and sits under the products partition (≤256 chars). So `vite build` or `vite` config loading outside the framework's own runner fails.
- `R/⚡️caching/🌐️vite/🏗️build/🟦️.ts:52–74`: `buildViteArtifact` opens `withRepositoryCacheStoreV1(…,"dependencies")` and `withViteStorageV1(…,"products")`, runs `process.execPath vite/bin/vite.js build … --outDir <temp>` (line 66), then stages into `layout.output` (line 69). The partition policy `R/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` maps `products` to `.cache/b/p` (capacity 24 GiB) and `dependencies` to `.cache/b/d`, both mounted images (`.cache/b/d/fixed.dmg`).
- `P/📋️project.json:192–216` (`build`): `outputs: []`, `cache: false`. The only product is in the store. Nothing is declared for Bazel to produce or restore.
- `P/📋️project.json:217–229` (`publish`): declares `dist/release/pages/course`; reads the store artifact (`C/🚀️deploy/🌐️site/🟦️.ts:121`, `withViteArtifactV1`) and stages it (`:129–141`).
- `C/🏗️builder/🌐️vite/🟦️.ts:112–117` (`closeBundle`): writes `robots.txt` and `manifest.webmanifest` into the Vite outDir. Under `publish` that is the temporary store directory, so it is fine; it becomes a problem if the Bazel build uses `dist/` directly.
- `C/🏗️builder/🌐️vite/🟦️.ts:141` `canonicalWasmBuildVitePlugin(repoRoot, mode)`: for any `@semio-tech/*` import whose target is `pkg/` or `🕸️bindings/` (or whose manifest ships `.wasm`), it throws unless a canonical snapshot exists at `<workspace>/<owner>/dist/wasm/<mode>/…` (`R/⚡️caching/🌐️vite/🕸️wasm/🟦️.ts:27,33–34`). None of the 38 `@semio-tech` packages the closure imports declare `semioWasmOwner`, so it is inert today; it becomes a hidden cross-project dependency if that changes.

### 2.5 Caller authority and the process envelope

- Every stateful verb requires an issued invocation: `P/📜️script.ts:133` runs `receiveScriptProcessInvocation(process.env, …)`, which throws `Original parent Script process invocation required` without `SEMIO_SCRIPT_PROCESS_INVOCATION` (`F/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:142`).
- The envelope carries an absolute epoch deadline (`🟦️.ts:96–99`, `deadlineEpochMilliseconds: now + …`) and the capabilities (absolute `repositoryRoot`, `artifactDirectory`, transport limits, `cargoExecutionBackend`). `scriptInvocationEnvironment` (`🟦️.ts:193–196`) injects it into every child env the course spawns: vite, tsc, cargo, docker, playwright, proctor, the dev-site re-entry (`C/🧱️stack/🟦️.ts:415`, `C/🎭️e2e/🟦️.ts:264`).
- If the Bazel backend puts this envelope in an action's env, every action key is unique (deadline changes per run) and cache hits are impossible. The envelope must be a stable, non-hashed channel, or the deadline must be omitted for cacheable actions.
- Caller authority required by build code: `readNativeCallerAuthorityV1` in `R/⚡️caching/…/📥️invocation/🟦️.ts` (`tool?: "nx"|"bazel"` already in its type, line 32). `courseOriginalBuildModeV1` (`C/🏗️builder/🎚️mode/🟦️.ts:12–13`) refuses a publish/rehearsal unless the native caller's `original.buildMode` is `release`. `buildProctor` (`PB/🟦️.ts:272`) and `repositoryCargoNetworkV1` need the same authority. Bazel must issue this (it is a real backend task, not a path fix).

### 2.6 Cargo ingress and raw cargo

- Root `.cargo/config.toml:4–10`: `rustc` is the bun ingress script (`F/…/🏗️compiler/🚪️ingress/📜️script.ts`), `rustc-wrapper = "bun"`, and `target-dir` / `build-dir` are under `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/shared/`. Any Bazel cargo action that sees this file is routed through the bun ingress; without it, cargo runs unconfigured. Bazel needs its own cargo configuration.
- The native build path spawns the raw `cargo` binary from `PATH` (`F/…/📦️artifacts/🏗️native-build/🟦️.ts:258`, `🌊️relay/🟦️.ts:31`) and captures `cargo metadata`/unit-graph (`:285`). `cargo fetch --locked` (`R/…/📥️dependencies/🟦️.ts:44`) downloads crates unless the caller's offline grant is set.
- `PB/🟦️.ts:259–267` (`proctorBuildLaunch`) is the only place that spells `bun …/📜️script.ts repo cargo …` (the recursive `bun repo` form). At runtime it is dead code; only `C/🧪️tests/🧱️local-stack/🟦️.ts:357–366` calls it.
- The cargo inputs are incomplete: `P/📋️project.json` `check.inputs` (lines 305–316) and `PR/📦️packages/🦀️rust/📋️project.json` `cargoSources` (lines 22–31) omit `🎓️teaching/Cargo.toml` and `🎓️teaching/Cargo.lock` (the teaching workspace manifest and lock, both tracked). `cargoSources` also omits the framework crates in §2.2.

### 2.7 Non-hermetic clocks, git, and PATH

- `C/🚀️deploy/🟦️.ts:216–224` (`sourceRevision`): `git rev-parse HEAD`, `git status --porcelain --untracked-files=no` (appends `-dirty`), `git log -1 --format=%cI`. These feed OCI labels (`C/🚀️deploy/🐳️docker-image/🟦️.ts:38–39`). Outputs depend on the working-tree state; they must come from Bazel stamping (`workspace_status`) in hermetic mode.
- `C/🎭️e2e/🟦️.ts:336`: run directory `new Date()` + pid. `C/🧱️stack/🟦️.ts:85`, `🎭️e2e/🟦️.ts:346,376` use the clock for progress only. `C/🚀️deploy/🧱️docker-stack/🟦️.ts:58` names the compose project `Date.now()`, `:145` uses `randomBytes`.
- `C/🚀️deploy/🐳️docker-image/🟦️.ts:32`: image production requires `linux` + `x64`; the verb throws elsewhere. The cross-platform requirement of the brief does not hold for `docker-image-build`, `docker-image-publish`, `docker-stack-bundle` (they need `docker`, §3).
- `R/⚡️caching/…/🌿️environment/🟦️.ts:20–29` (`devToolingEnv`) copies the entire `process.env` and deletes only `NODE_OPTIONS` / `VSCODE_INSPECTOR_OPTIONS`. `🎭️e2e/🟦️.ts:179` (`proctorEnvironment`) inherits all non-`PROCTOR_*` env. `PB/🟦️.ts:59–66` (`proctorDevelopmentEnvironment`) starts from `env = process.env`. None of these is hashed by Bazel unless it reaches an action env.

---

## 3. Per-target matrix

Columns: cache flag and declared outputs from `P/📋️project.json`; "reads outside inputs" refers to §2.2; "writes outside outputs" lists undeclared writes.

| Target | cache / outputs (project.json) | Reads outside declared inputs | Writes outside declared outputs | Spawns | Network / ports | Bazel verdict |
|---|---|---|---|---|---|---|
| `preflight` | `false`, `[]` (81–94) | `PATH`, `PATHEXT` (`F/…/🩺️preflight/🟦️.ts:39–40`) | probe directories under `.cache` (`:41–44`, exchange and case probes) | none | none | Not an action. `local`, `no-cache`, run by the backend before a build. Tagged "diagnostic" already (`:91`). |
| `ci-browser-prepare` | `false`, `[]` (95–108) | system package DB | apt state, `node_modules/playwright` (`P/📜️script.ts:37–43`) | `node playwright/cli.js install-deps chromium` (apt, sudo) | apt network | Side effect on the host. Keep out of the graph. Tagged "side-effecting" (`:105`). |
| `dev` | `false`, continuous (109–160) | `TEACHING_ARCHITECTURE_COURSE_PORT`, `PROCTOR_PORT`, `PROCTOR_DATA`, `PROCTOR_COURSE`, `TEACHING_ARCHITECTURE_COURSE_WATCH` (dashboard parameters) | `.🧬semio/🎓️teaching/proctor-dev/` (`PB/🟦️.ts:55`), `proctor-bin/` copies (`:48, 423–434`, used by `launchProctor` `:473–476`), `proctor-beside`, the store | `process.execPath script.ts dev-site` (`C/🧱️stack/🟦️.ts:415`), `buildProctor` (`:266`), cargo, proctor | ports 6061 (site), 8791 (proctor), 6063 (beside); `fs.watch` recursive over Rust source dirs (`:310–333`) | Live. `local` + `no-cache`; never a Bazel action. |
| `dev-site` | `false`, continuous (162–190) | `PROCTOR_PORT`, `DEVCONTAINER` (`R/🌐️service/🟦️.ts:34`) | the vite store (`C/🏗️builder/🌐️vite/🟦️.ts:139`) | `vite` dev server | 6061; probes existing listener, calls `process.exit(1)` (`R/🌐️service/🟦️.ts:47–50`) | Live. `local` + `no-cache`. |
| `build` | `false`, `[]`, inputs `default` + `env PROCTOR_URL` (192–216) | all of §2.2 (837 files); `@semio-tech` WASM snapshots `dist/wasm/<mode>` (`🕸️wasm/🟦️.ts:33`) | framework store `.cache/b/p/…`, `.cache/b/d/…` (`🌐️vite/🏗️build/🟦️.ts:57,69`) | `process.execPath vite/bin/vite.js build` (`:66`) | none | Needs declared Vite output tree in `dist/<mode>`. Currently produces nothing observable. |
| `publish` | `false`, outputs `dist/release/pages/course` (217–229) | `PROCTOR_URL` (`C/🚀️deploy/🌐️site/🟦️.ts:20–21`) not declared; store artifact (`:121`) | `dist/release/pages/course` (declared); store | none beyond vite | none | Pure function of the Vite tree plus `PROCTOR_URL`; should be a Bazel action with declared outputs and `PROCTOR_URL` as env input. Deterministic (see §4). |
| `test` | `true`, inputs `default`, outputs `[]` (233–245) | `workspaceTestAliasesV1(repoRoot)` reads every workspace manifest (`C/🧪️tests/🎚️config/🟦️.ts:13`, `R/🧪️testing/📦️workspace-exports/🟦️.ts:41–60`); `SEMIO_TEST_LEVEL`, `SEMIO_TEST_BUDGET_MS` (not declared on `test`, declared on `check`) | `C/node_modules/.vite/` (vitest default cacheDir, `config:16`); `.🧬semio/🦑️repo/⚡️cache/vite` (`policy.ts:23`); `.🧬semio/🦑️repo/📊️metrics/coverage/js/` created at config time (`policy.ts:14`); `SEMIO_TICKET_DIR`/tmp dirs (`🧪️tests/🧱️local-stack/🟦️.ts:121–123`, `🧪️deploy:28–30`, `🐳️image:34`) | `node` via vitest; tests spawn bun/node/vite (`🧱️local-stack:112, 382`) | TCP listeners on 127.0.0.1 (`🧱️local-stack:53–86, 565–655`; `🪢️presence:23`) | Cacheable only after splitting: a hermetic unit set with declared manifests and sources, and a `local` set (`local-stack`, `proctor-build`, `presence`, `resolution`). `🏗️proctor-build:37` calls `buildProctor(resolve("."))`, which is cwd-dependent. |
| `test-e2e` | `false`, `[]` (246–270); live | `TEACHING_ARCHITECTURE_COURSE_E2E_*` (set by the gate), `PLAYWRIGHT_BASE_URL`, full env minus `PROCTOR_*` (`🎭️e2e/🟦️.ts:179`) | `.🧬semio/🎓️teaching/architecture-course-e2e/<time>-<pid>/` (`:63–65, 336`), ticket dir if `SEMIO_TICKET_DIR` set; browsers into `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` (`R/🌿️environment/🟦️.ts:32–37`); proctor build in store | `node` for port probes (`:145`) and Playwright (`:307`); Playwright install (`:300`); `buildProctor` (`:354`); bun re-entry (`:264`) | ephemeral ports (`:169–173`), control endpoint (`:186`), static server (`:278`); **network download of Chromium on first run** (`:293–301`); cargo | Live. `local`, `no-cache`, `requires-network` (first run). Not cacheable; the browser is an external repo. |
| `typecheck` | `true`, inputs `default` + transitive outputs of `@semio-tech/*:generate` (287–295), outputs `[]` | path-mapped framework sources (`P/tsconfig.json` `paths`), `@types` from node_modules, all of §2.2 | `state.tsbuildinfo` in the store (`F/…/🏗️build/🟦️typescript/🟦️.ts`: `tsBuildInfoFile` override). The `tsconfig` `node_modules/.cache/architecture-course-typecheck.tsbuildinfo` (`P/tsconfig.json:5`) is dead config under the framework. | `tsc` through `typecheckProject` | none | Good Bazel `ts_project` candidate once inputs are complete. Missing: `C/⚡️energy` quiz JSON (§5.2), the generated modules (§2.2), `tsconfig` root. |
| `check` | `true`, outputs `[]`, inputs `default`, teaching proctor, root `Cargo.toml`/`Cargo.lock`/`.cargo/config.toml`/`rust-toolchain.toml`, env `SEMIO_TEST_*`, `PROCTOR_COURSE` (297–317) | Rust closure (§2.6); framework crates outside `cargoSources`; `🎓️teaching/Cargo.toml`/`Cargo.lock` not declared | `.cache/b/…` store, `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/cargo/shared/{target,build}` (`.cargo/config.toml:8–9`) | `buildProctor` (cargo), `proctor check <course>` (`C/🚀️deploy/🟦️.ts:226–236`) | `cargo fetch` network unless offline grant | Cacheable **only** after the Rust closure is declared (today `cache: true` is unsound). Output should be a report file. |
| `docker-image-build` | `false`, no outputs (318–326) | `git` (`sourceRevision`), root `Cargo.toml` version (`docker-image:39`) | docker daemon image; no repo output | cargo (release proctor), `docker build` (`:48`) | docker daemon, base-image pulls | Linux x64 only (`:32`). `local`, `no-cache`. Reproducible image needs `oci_image` rules and stamping, not `git` calls. Context tar has zero mtimes (`📦️image:30`), good. |
| `docker-image-check` | `false`, `[]` (328–341; grep only) | `mkdtemp(tmpdir())` (`🩺️docker-image-check:204, 295`) | temp dirs, docker containers | `docker run`, container probes | host ports | `local`, `no-cache`, `requires-network` (base images). |
| `docker-image-publish` | `false` (dashboard `execution-mode`) | `git` revision (`docker-image:70`), docker login state | pushes to registry (`:80`), tags locally (`:79`) | `docker push` | `ghcr.io` network | Manual release action. Never cached or run by Bazel CI without explicit approval. |
| `docker-stack-check` | `false` (352–367), live | `Date.now()`, `randomBytes` (`🧱️docker-stack:58, 145`) | compose project, volumes, certificates in `tmp`; removed at end | `docker compose` | host ports 18080/18443 defaults (`:53–54`), `localhost` TLS | `local`, `no-cache`, `requires-network` (Caddy image). |
| `docker-stack-bundle` | `false`, outputs `dist/proctor` (368–384), live | docker daemon images; `--out` resolved from cwd (`🎁️stack-bundle:103`) | `dist/proctor/…` (declared); `docker tag` (`:114`) mutates daemon state; `docker save` tar (`:121`) is not guaranteed byte-identical across runs | `docker pull --platform linux/amd64` (`:113`), `docker save` | `pull` network | `local`. Declared outputs are produced from daemon state; treat as release packaging. |
| `deploy-check` | `false` (385–398) | `📋️project.json` read at runtime (`⚖️readiness:36`), drift files (`deploy/🟦️.ts:270–298`) | everything the steps write | all steps below | all of the above | Aggregate. Model as a `test_suite` of the individual targets, not one action. |

Proctor-side consumers of `buildProctor` (in `check`, `test-e2e`, `docker-image-build`, `deploy-check`): cargo, a Bazel-issued native caller, and the framework store (`PB/🟦️.ts:270–369`). Unused in these targets but linked into the course tree: `proctorBuildLaunch` (`PB:259–267`, `bun repo cargo` recursion, test-only).

---

## 4. Environment variables

Declared in `P/📋️project.json` (namedInputs and per-target `inputs`): `PROCTOR_URL` (build), `SEMIO_TEST_LEVEL`, `SEMIO_TEST_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`, `PROCTOR_COURSE` (check). `SEMIO_BUILD_MODE` comes from `nx.json` `sharedGlobals` (the repo-level named input; no Bazel analogue). `NODE_OPTIONS=""` and `VSCODE_INSPECTOR_OPTIONS=""` are set on dev targets only.

| Variable | Read at | Declared as input? | Effect |
|---|---|---|---|
| `SEMIO_SCRIPT_PROCESS_INVOCATION` | routing `🟦️.ts:142`; spawns `scriptInvocationEnvironment` | no | Envelope with deadline (§2.5). Must not enter action keys. |
| `SEMIO_BUILD_MODE` | `C/🏗️builder/🌐️vite/🟦️.ts:139`, `readBuildMode`, `C/🧱️stack/🟦️.ts:415`, `🎭️e2e/🟦️.ts:69` | yes, via `sharedGlobals` | Selects `prototype`/`release`; conflicts with caller `original.buildMode` (§2.5). |
| `SEMIO_VITE_CACHE_DIRECTORY` | `R/⚡️caching/🌐️vite/📂️paths/🟦️.ts:8` | framework-minted, not user | Required by Vite config; must be under the products partition. |
| `SEMIO_WORKSPACE_ROOT`, `REPO_ROOT`, `process.cwd()` | routing `🟦️.ts:105` | no | Workspace discovery fallback (§2.1). |
| `SEMIO_TICKET_DIR` | vitest cacheDir `C/🧪️tests/🎚️config/🟦️.ts:16`; e2e work dir `🎭️e2e/🟦️.ts:63–64`; tests `🧪️deploy`, `🐳️image`, `🧱️local-stack` | no | Redirects scratch to a ticket folder; changes outputs. |
| `PROCTOR_URL` | `C/🏗️builder/🌐️vite/🟦️.ts:41`; `C/🚀️deploy/🌐️site/🟦️.ts:21`; `🎭️e2e/🟦️.ts:69` (rehearsal) | yes on `build` only; **no** on `publish`, `test-e2e`, `deploy-check` | Baked into the bundle (`VITE_PROCTOR_URL`, `vite:153`). Changing it changes the release artifact. |
| `VITE_PROCTOR_URL` | `C/🏗️builder/🌐️vite/🟦️.ts:153` (define); read by `C/🟦️.ts:bakedProctorOrigin` | set by define | Baked at build. |
| `PROCTOR_PORT` | vite `:22`; `PB/🟦️.ts:45, 62, 448`; stack `:386`, `🎭️e2e:180` | no | Dev proxy target; default 8791. |
| `PROCTOR_DATA`, `PROCTOR_COURSE`, `PROCTOR_MODE`, `PROCTOR_ALLOWED_ORIGINS` | `PB/🟦️.ts:59–66`; `🎭️e2e/🟦️.ts:180, 237` | partly (PROCTOR_COURSE on check) | Data directory and course for the proctor. Mutable data outside outputs. |
| `TEACHING_ARCHITECTURE_COURSE_PORT` | `C/🧱️stack/🟦️.ts:386`; `P/📋️project.json` dev metadata (port 6061) | no | Site port. |
| `TEACHING_ARCHITECTURE_COURSE_WATCH` | vite `:157`; stack `:410`; e2e `:264` | no (set by dev parameter) | Disables HMR and file watching. |
| `TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY/PROCTOR/CONTROL` | `C/🎭️e2e/🚶️learner/🟦️.ts:23–28`, set at `🎭️e2e/🟦️.ts:306` | no | Spec-to-stack contract. |
| `PLAYWRIGHT_BASE_URL`, `PLAYWRIGHT_BROWSERS_PATH` | `🎭️e2e/🎚️config/🟦️.ts:22`; `🎭️e2e/🟦️.ts:300, 306`; `R/🌿️environment/🟦️.ts:35` | no | Browser location; default under `.🧬semio/…/ms-playwright`. |
| `DEVCONTAINER`, `WATCHPACK_POLLING`, `CHOKIDAR_USEPOLLING` | `R/🌐️service/🟦️.ts:34`; `R/🌿️environment/🟦️.ts:41` | no | Host binding (`0.0.0.0`) and polling. |
| `SEMIO_BUILD_PARALLEL`, `SEMIO_BUILD_TERMINAL`, `SEMIO_BUILD_DYNAMIC_OUTPUT`, `SEMIO_BUILD_VERBOSE` | `R/🌿️environment/🟦️.ts:24–27` | no | Output/parallelism tuning; changes logs, not artifacts. |
| `SEMIO_VITEST_POLICY`, `SEMIO_COVERAGE`, `SEMIO_TEST_LEVEL` | vitest runner `🧪️vitest/🟦️.ts:17, 39`; `🧪️tests/📦️resolution:76` (mutates env) | no | Selects test budget and coverage. |
| `CARGO_NET_OFFLINE` | `R/…/📥️dependencies/🟦️.ts:51` (set by framework) | no | Network grant. |
| `PATH`, `PATHEXT` | `F/…/🩺️preflight/🟦️.ts:39–40` | no | Tool discovery. |
| `DOCKER_*` (implicit), full env | `C/🚀️deploy/🟦️.ts:174` (`docker` spawn merges `process.env`) | no | Docker client config and daemon. |
| `BUN_CONFIG_MAX_HTTP_REQUESTS` | `PR/📜️script.ts` capacity command | no | Load-test client only. |
| `FORCE_COLOR`, `NO_COLOR` | `C/🧪️tests/🧱️local-stack/🟦️.ts:56` | no | Output only. |
| `SEMIO_BUILD_TOOL` | framework nx env (`R/…/🚀️bootstrap/…/📤️arguments/🟨️.mjs`) | no | Must be `bazel` under the backend. |

---

## 5. Other hermeticity facts

### 5.1 cwd and root assumptions

- `P/📜️script.ts:131`: `new ScriptRouter(import.meta.dir)`. `import.meta.dir` is the path Bun loaded the file from; a Bazel symlink changes it (§2.1).
- `PB/🟦️.ts:215–217` (`proctorCourse`): `resolve(named ?? env.PROCTOR_COURSE ?? …)`. A relative argument resolves against `process.cwd()`, which Bazel sets to the execroot.
- `C/🚀️deploy/🎁️stack-bundle/🟦️.ts:103`: `resolve(flag("--out") ?? join(bundleRoot, "dist", "proctor"))`. A relative `--out` escapes the declared output.
- `C/🧱️stack/🟦️.ts:318` and `PB/🟦️.ts:44–66`: `resolve(env.PROCTOR_COURSE)` and `resolve(env.PROCTOR_DATA)` use process cwd when the env value is relative.
- `C/🏗️builder/🌐️vite/🟦️.ts:19–21`: `siteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..")`, `repoRoot = siteRoot/../../..`. Hard-coded depth; it holds only if the sandbox keeps the exact tree depth (it does if the whole tree is mirrored).
- `C/🧪️tests/🏗️proctor-build/🟦️.ts:37`: `buildProctor(resolve("."), …)`: depends on the test's cwd.
- `C/🧪️tests/🧱️local-stack/🟦️.ts:106, 246, 355`: `import.meta.dirname`-relative roots; they assume the real checkout (the Cargo workspace `🎓️teaching/Cargo.toml`, `node_modules/vite`).
- `C/🎭️e2e/🟦️.ts:295`: `createRequire(realpathSync(join(repoRoot, "node_modules/playwright/cli.js")))` follows the node_modules symlink into `.cache`; a Bazel sandbox has no such target.

### 5.2 Energy quiz inputs are undeclared (correctness, not only Bazel)

`C/📚️course/🟦️.ts:12–16` imports `physics`, `ventilation`, `heating`, `cooling`, `demand` from `C/../⚡️energy/…/❓️quiz/🔣️.json`. `P/📋️project.json` `namedInputs.default` contains `{workspaceRoot}/…/🎓️teaching/🏛️architecture/**/📚️course/🔣️.json` and the course's own `📚️course/**/*`, but not `⚡️energy/**`. Editing an energy quiz leaves `build`, `typecheck` and `check` cache entries valid while the site and proctor data change. Fix: add `{workspaceRoot}/🎓️teaching/🏛️architecture/⚡️energy/**/❓️quiz/🔣️.json` (or the whole `⚡️energy` tree) to `default`.

### 5.3 Subprocess and network inventory

- Child processes: `vite` (build, dev-site, rehearsal), `tsc` (typecheck), `vitest` on node (test), `bun` re-entry of `script.ts dev-site` (`C/🧱️stack:415`, `🎭️e2e:264`), `cargo` (raw binary via `PATH`, `F/…/🌊️relay/🟦️.ts:31`), the proctor (built and spawned from the store or from `.🧬semio/🎓️teaching/proctor-bin`), `node` (`🎭️e2e:145, 307`, `🧱️local-stack:112`), `playwright cli.js` (`🎭️e2e:300`; `P/📜️script.ts:43`), `docker` (`C/🚀️deploy/🟦️.ts:174`), `git` (`C/🚀️deploy/🟦️.ts:218`).
- No `bun repo` recursion exists at runtime. The router re-enters itself (`bun script.ts dev-site`) only through `process.execPath`. `bun repo cargo` appears only in `PB:259–267`, which is test-only.
- Network: Chromium download (`🎭️e2e:293–301`), `cargo fetch` (`R/…/📥️dependencies/🟦️.ts:44`), `docker pull` (`🎁️stack-bundle:113`, the compose stack pulls `caddy`), `docker push` (`🐳️docker-image:80`), `apt` (`P/📜️script.ts:37–43`), `ghcr.io` (registry).
- Listening: 6061 (site), 8791 (proctor), 6063 (beside), OS-chosen ports (e2e), 18080/18443 (docker stack check), 127.0.0.1 listeners in tests (`🧱️local-stack`, `🪢️presence`).

### 5.4 Determinism of published outputs

- Site artifact (`publish`): no clock or random source in `C/🏗️builder/🌐️vite/🟦️.ts`, `C/🚀️deploy/🌐️site/🟦️.ts`, `R/⚡️caching/🌐️vite/…`; `robots.txt` and `manifest.webmanifest` are constants (`:115–116`); `_headers` is a constant (`deploy/🟦️.ts:100–110`). Output depends on `PROCTOR_URL` (not declared on `publish`) and on the source closure. No absolute paths were found in the checked artifact lists (`siteArtifactProblems`, `:74–101`, rejects loopback names and dev leftovers).
- Docker context tar (`📦️image/🟦️.ts:20–33, 36–60`): mtime 0, fixed modes, sorted explicit roster: deterministic given sources. The Dockerfile and labels are not: `REVISION` (`-dirty` suffix), `VERSION`, `CREATED` from `git log`.
- `docker save` (`🎁️stack-bundle:121`) and `docker build` layer timestamps are not guaranteed byte-identical; treat `proctor-image.tar` as non-cacheable.
- Test output: vitest report and coverage paths include absolute repo paths (`policy.ts:14`), and the e2e logs embed the run directory.

---

## 6. Prioritized fixes

### P0: blocks any Bazel run

1. **Explicit workspace root.** Change `findWorkspaceRoot` (`F/…/🧭️routing/🟦️.ts:105–120`) to `statSync` (follow links) and take the root from the Bazel backend's issued invocation first. Pass the root into `ScriptRouter` instead of `import.meta.dir`. Remove the `process.cwd()` fallback for Bazel.
2. **Generate the input closure.** Replace hand-written `namedInputs.default` in `P/📋️project.json` with the import-graph result, including the process module, the repo library, `@semio-tech/process`/`repo-lib` package sources, schema validator, the energy quiz JSON, the teaching Cargo manifest and lock, root `Cargo.lock`, `rust-toolchain.toml`, and the Rust crate closure from `PB/🕸️manifest-closure`. Keep the list in a checked-in file that a test compares to the real closure.
3. **Stop writing the product to the framework store under Bazel.** Use a plain `vite build --outDir <declared>` path that does not require `SEMIO_VITE_CACHE_DIRECTORY` (`C/🏗️builder/🌐️vite/🟦️.ts:139`; `R/…/📂️paths/🟦️.ts:6–10`). Declare `dist/<mode>` as the `build` output. The native backend keeps its store path.
4. **Caller authority without a wall-clock deadline.** Define how the Bazel backend issues `SEMIO_SCRIPT_PROCESS_INVOCATION` (`F/…/🧭️routing/📥️invocation/🏃️process/🟦️.ts:96–99, 188–196`) and `readNativeCallerAuthorityV1` (`R/…/📥️invocation/🟦️.ts`, `tool: "bazel"`). Keep the envelope out of action keys (pass it through a non-hashed channel or omit the deadline for cacheable actions).
5. **Declare `PROCTOR_URL` on `publish`, `test-e2e` (rehearsal), `deploy-check`.** It is baked into the release bundle (`C/🏗️builder/🌐️vite/🟦️.ts:41`, `C/🚀️deploy/🌐️site/🟦️.ts:20–21`).
6. **Declare the energy quiz inputs** (§5.2). This is a correctness bug in every cached course target today.
7. **Complete the Rust closure before caching `check`** (§2.6). Declare `🎓️teaching/Cargo.toml`, `🎓️teaching/Cargo.lock`, and the framework crates of §2.2 in `cargoSources` and `check.inputs`. Until then `check` should be `cache: false`.

### P1: makes the cacheable targets correct

8. **Vitest.** Set `cacheDir` and coverage under the declared output (`C/🧪️tests/🎚️config/🟦️.ts:16`; `R/…/🏃️process/📋️context/🔣️policy/🟦️.ts:14, 23`). Add `SEMIO_TEST_LEVEL` and `SEMIO_TEST_BUDGET_MS` to `test.inputs`. Replace `workspaceTestAliasesV1(repoRoot)` (config:13) with a declared manifest list so it no longer reads the whole workspace. Split the test set into hermetic (cacheable) and `local` (`🧱️local-stack`, `🏗️proctor-build`, `🪢️presence`, `📦️resolution`, `🧪️deploy`, `🐳️image`).
9. **Generated, gitignored sources.** Make `@semio-tech/assets:build` and `@semio-tech/ui-styling-tokens:generate` declare their outputs (today `ui-styling-tokens:generate` has `outputs: null`, `F/…/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json`). Emit them into a generated root Bazel can consume. Identify the producer of `F/…/🎭️actor/🤖️generated/🎭️actor/🟦️.ts` and add it to `dependsOn`.
10. **Typecheck.** Remove the dead `tsBuildInfoFile` from `P/tsconfig.json:5` or make it the declared output. Add `@types` externals, the root `tsconfig.json`, path-mapped framework sources, and the §5.2 quiz JSON to inputs.
11. **Cargo config.** Give Bazel its own cargo configuration. Stop reading the root `.cargo/config.toml` (the bun ingress `rustc`, `:4–10`) in Bazel actions. Keep the ingress for the native backend only.
12. **`sourceRevision`.** Replace `git` calls (`C/🚀️deploy/🟦️.ts:216–224`) with Bazel workspace status. Remove the `-dirty` suffix in hermetic mode.
13. **Relative paths.** Resolve `--out`, `PROCTOR_COURSE`, `PROCTOR_DATA`, and `check <course>` against the declared root, not `process.cwd()` (`C/🚀️deploy/🎁️stack-bundle/🟦️.ts:103`; `PB/🟦️.ts:215–217`). Replace `buildProctor(resolve("."))` in `C/🧪️tests/🏗️proctor-build/🟦️.ts:37`.

### P2: live and host-bound targets, tagging, cleanup

14. Tag as `local` + `no-cache` (never cached, never remote): `dev`, `dev-site`, `test-e2e`, `preflight`, `ci-browser-prepare`, all `docker-*`, `docker-stack-bundle`. Tag `requires-network` where applicable (`test-e2e` first run, `docker-stack-check`, `docker-stack-bundle`, `check` cargo fetch unless vendored). `deploy-check` becomes a test suite, not an action.
15. Replace `devToolingEnv`'s full `process.env` copy (`R/🌿️environment/🟦️.ts:20–29`) and `proctorEnvironment` (`C/🎭️e2e/🟦️.ts:179`) with an explicit allowlist in Bazel mode.
16. Narrow the barrels the course imports (`repo-lib/dev` through `R/…/🎮️playground/🟦️.ts`, `repo-lib/cargo`). This removes the OS product and the Nx SDK from the course's runtime closure (§2.2) and shrinks invalidation.
17. Delete or move `proctorBuildLaunch` (`PB/🟦️.ts:259–267`) and its test (`C/🧪️tests/🧱️local-stack/🟦️.ts:357–366`); it is the only `bun repo` recursion in this closure.
18. `docker-image-build` is Linux x64 only (`C/🚀️deploy/🐳️docker-image/🟦️.ts:32`). Express that as a Bazel `target_compatible_with` constraint instead of a runtime throw, and state it in the brief's cross-platform matrix.
19. `preflight`: keep as a backend check (environment probes write under `.cache`, `F/…/🩺️preflight/🟦️.ts:41–44`), not an action.

---

## 7. Files produced by this audit

- `T/audit-hermetic.md` (this report).
- `T/aud-hermetic-tools/closure.mjs` (read-only static import-closure helper; run with `IMPORT_ONLY=1` for the import-only result, `WITH_TESTS=1` to add tests).
- `T/🗑️generated/aud-hermetic/closure-import-only.json`, `closure-import-only-with-tests.json`, `closure.json` (literal-scan upper bound), `rust-crates.json`, `ws-symlink-probe/` (probe files for §2.1).

No repository source, `AGENTS.md`, `project.json` or framework file was modified.

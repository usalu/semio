# Lane S4 — OS Modules Shared-Cache Migration (2026-09-11)

Scope: `🧰️framework/🛍️products/💻️os/**` (except `🔨️modules/🧑‍💻dev/**`, owned by S3), plus
`🧰️framework/📦️packages/**` and `🧰️framework/🔨️modules/**`. Builds on Wave A (`.cargo/config.toml`
shared `target-dir`/`build-dir` + fine-grain-locking, sccache removed, `cargoTargetDirectory` /
`cargoBuildDirectory` / `repoCacheDirectory` resolvers) and consumes S1's `repoToolCacheEnv`
(`GOCACHE` + `PLAYWRIGHT_BROWSERS_PATH` → shared cache) once it landed mid-session.

## Changes

### Task 1 — plugin describe / typescript / host

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
  - `produceFreshComponentV1`: dropped the dead `RUSTC_WRAPPER: "", SCCACHE_DISABLE: "1"` keys from
    the `devToolingEnv({...})` call (sccache no longer exists anywhere in `.cargo/config.toml`) and
    the three `--config 'build.rustc-wrapper=""'` cargo CLI overrides that existed only to defeat it.
    Kept `CARGO_TARGET_DIR: targetRoot` and `CARGO_INCREMENTAL: "0"` — this is a genuinely isolated,
    caller-supplied, **empty**-directory verification build (`freshRoot` asserts the dir is empty)
    proving the component was freshly compiled, not the kind of unbounded private-target-dir problem
    the ticket targets; the shared `build-dir` is still used underneath since only `--target-dir`
    (deliverables) is scoped, matching the probe finding that a private target-dir only diverts the
    small uplifted deliverable, not the shared intermediates.
  - Updated the now-stale `pluginWasmArtifactPath` docstring (said "the same `CARGO_TARGET_DIR`
    override" — `ensureBuiltBin` no longer overrides anything, both resolve through the same
    `cargoTargetRoot`/`cargoTargetDirectory`).
  - `cargoTargetRoot()` was already delegating to `cargoTargetDirectory` (Wave A) — no change needed.
- `🔌️plugin/📦️packages/🟦️typescript/📜️script.ts` (support/materialize/activate/prepare) — no cargo
  target-dir/wrapper logic present; nothing to change.
- `🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` — already routes every native
  law through `runExactCargoLaws`, which already resolves `CARGO_TARGET_DIR` via `cargoTargetDirectory`
  (S1's fix, confirmed by reading `🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2051`).
  The on-disk `🖥️host/📦️packages/🗑️generated/cargo-plugin-host` directory is a stale leftover from
  before that fix — left untouched per instructions (never delete generated/cache dirs).
  - `🖥️host/🧵️shard/🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs`: the `process_shard_kill_is_detected…`
    doc-comment reproduction recipe told a developer to set a private
    `CARGO_TARGET_DIR=<ticket>/🎯️target-p1`. Rewrote it to build with the shared config (no override)
    and read `SEMIO_SCALE_FIXTURE_WASM` from `.🧬semio/🦑️repo/⚡️cache/cargo/target/…`. Comment-only;
    verified with `cargo check -p semio-framework-plugin-host` (clean, 1m33s).

### Task 2 — MCP module

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts`
  - `resolveMcpTargetDirectory` now delegates to `cargoTargetDirectory(repoRoot, env)` instead of
    `pathApi(platform).resolve(repoRoot, env.CARGO_TARGET_DIR ?? "target")` — honors the shared
    `.cargo/config.toml` `target-dir` when no env override is present, same env-precedence semantics
    as before for the override case.
- `🌉️mcp/📦️packages/🦀️rust/📜️script.ts`
  - `buildMcpBinary` no longer passes an explicit `--target-dir` to `cargo build` — cargo already
    resolves the identical directory from `.cargo/config.toml`/env, so the explicit flag was pure
    duplication of the resolver's own logic. Removed the now-unused `resolveMcpTargetDirectory` import.
- `🌉️mcp/🧪️tests/🧪️resolvemcpbinarypath/🟦️.ts`
  - Fixed a **pre-existing broken fixture path**: `new URL("./🧫️fixtures/🧱️binary-gate.json", source.url)`
    pointed at a directory (`🌉️mcp/🧫️fixtures/`) that has never contained `🧱️binary-gate.json` — the
    real file lives at `🌉️mcp/🎚️config/🧱️binary-gate.json` (confirmed via the package's own
    `📋️project.json` `namedInputs`). This was unrelated to caching but blocked verifying my own
    resolver change, so fixed the reference. All 8 tests in this suite now pass (previously the file
    could not even load).
  - `🌉️mcp/🎚️config/🧱️binary-gate.json`'s two `"CARGO_TARGET_DIR": "scratch/cargo"` pathCases were
    left as-is — they are fixture data proving `resolveMcpBinaryPath` (the staged-artifact consumer)
    stays independent of `CARGO_TARGET_DIR`, not a real directory anything writes to.

Verified: `bunx vitest run --config vitest.config.ts -t resolveMcpBinaryPath` → 8/8 passed, both
before and after adding `cacheDir` (see Task 5).

### Task 3 — WGPU renderer

- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` (lines ~466, ~516): dropped
  `RUSTC_WRAPPER: ""` from both `runExactCargoLaws` env overrides (dead sccache-era knob). Kept
  `CARGO_BUILD_JOBS: "1"` — this repo has a consistent, documented pattern of pairing
  `CARGO_BUILD_JOBS: "1"` with large `RUST_MIN_STACK` native-law test runs specifically because this
  renderer's debug builds overrun the default stack (see `.cargo/config.toml`'s own comment about the
  wgpu renderer's 16 MiB debug shadow stack and `PLUGIN_WASM_STACK_BYTES`); throttling to one job
  avoids multiplying that peak memory across parallel compiler jobs.
- `.🧬semio/🦑️repo/⚡️cache/📺️renderer-modules/…` (Trunk.toml, script.ts `outDir`, vite alias) — already
  inside the cache root; no change needed.

### Task 4 — standalone workspace fixtures/targets

Investigated every location named in the brief:

- `💻️os/🧫️fixtures/⏳️asyncprobe/🖥️host/target` — directory contains **only** `target/`, no
  `Cargo.toml`/source at all; fully orphaned, nothing currently creates it. Left untouched.
- `💻️os/🪧testkit/🧩️jcoprobe/👽️guest` — directory no longer exists on disk.
- `💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/target` — no longer exists on disk (already cleaned up
  or never regenerated since Wave A).
- `🧰️framework/📦️packages/🦀️rust/target-root-framework-schema` and
  `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/target-root-ui-contract-{check,wasm,native}`
  — `target-root-*` does not appear anywhere in the current source tree; these are stale artifacts
  from a prior mechanism. Both crates' `PreviewGeneratedScript`s use a self-cleaning
  `mkdtempSync(tmpdir())` + `CARGO_TARGET_DIR: join(temp, "target")` + `finally { rmSync(temp, …) }`
  — a genuinely ephemeral, bounded, outside-the-workspace typegen preview build (documented intent:
  "Runs the exact schema exporter outside the workspace"), not an accumulating private-target-dir
  problem; left unchanged. The same pattern (self-cleaning, `tmpdir()`-rooted, "typegen" preview)
  recurs in `🧰️framework/🔨️modules/⏳️async`, `🎭️actor`, and `💻️os/🔨️modules/🖥️shell` — all reviewed,
  all fine.
- `🔨️modules/✍️editor`, `🧬️schema`, `🔏️hash` — all are ordinary root-workspace members
  (`rust-version.workspace = true`, listed in root `Cargo.toml` `members`), so a plain `cargo
  check`/`test` from inside them already resolves the shared root `.cargo/config.toml`. `editor`'s wasm
  build goes through `runWasmPackWebBuild` → `wasmBuildEnvironment`, which S1 already changed to
  `return { ...env }` (no private default, doc: "Cargo's own config governs where it writes"). The
  on-disk `target/` dirs under these three are stale, pre-Wave-A leftovers. Not deleted.

Did not delete any existing directory per instructions.

### Task 5 — repo-wide sweep of my files

Ran `rg` (narrow, actionable patterns only — the bare `"target"` pattern was too noisy with
nx-target/tsconfig-target/rustup-target/etc. false positives to use directly) across every directory
in scope:

- Fixed the genuine hit: `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts`
  (`ExportLogoScript`) hardcoded `process.env.PLAYWRIGHT_BROWSERS_PATH ??=
  join(this.repoRoot, "node_modules/.cache/ms-playwright")`. Replaced with
  `repoToolCacheEnv(this.repoRoot).PLAYWRIGHT_BROWSERS_PATH` (S1's helper, already used by
  `runPlaywrightTest`/`🧪️test`'s Go runner) so Playwright's browser downloads land in
  `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/` instead of an unshared `node_modules/.cache`.
- Added `cacheDir: repoCacheDirectory(repoRoot, "vite", "<project>")` to every `vitest.config.ts` in
  scope that lacked one (Vite/Vitest default to `node_modules/.vite`, matching the
  `node_modules/\.vite` pattern in the brief), mirroring the existing
  `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts` pattern:
  - `💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/vitest.config.ts` → `vite/os-mcp`
  - `💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/vitest.config.ts` → `vite/os-shell`
  - `💻️os/🔨️modules/🔌️plugin/📇️registry/vitest.config.ts` → `vite/plugin-registry`
  - `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts`
    → `vite/renderer-react`
  - `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts`
    → `vite/renderer-wgpu`
  - `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/vitest.config.ts` → `vite/framework-kernel`
  - `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/vitest.config.ts` → `vite/framework-machine`
  - `🧰️framework/🔨️modules/🧬️schema/vitest.config.ts` → `vite/framework-schema`
  - `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/vitest.config.ts` → `vite/framework-3d`
  - `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/vitest.config.ts` → `vite/framework-replication`
  - `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` → `vite/ui-react`
- `🖱️ui/🎨️styling/🟦️.ts`'s `/node_modules/.vite/deps/` string match is a **URL-scheme** check against
  Vite's client-side dev-server path convention (always `/node_modules/.vite/deps/…` in the browser
  regardless of the on-disk `cacheDir`), not a cache-location config — left unchanged.
- Everything else the sweep found (fresh-component isolation builds, `runExactCargoLaws`
  callers, the actor-import fixture's mandatory ticket-scoped `CARGO_TARGET_DIR`, the mcp binary-gate
  fixture's env-independence test data) was already reviewed under Tasks 1–4 and needed no change.

## Verification

| command | result |
| --- | --- |
| `bun build --target=bun --no-bundle <file>` for all 19 edited `.ts`/`.rs`-adjacent TS files | all OK |
| `cd 🌉️mcp/📦️packages/🟦️typescript && bunx vitest run --config vitest.config.ts -t resolveMcpBinaryPath` | 8/8 passed (both before and after the fixture-path + cacheDir fixes) |
| `cd 🧰️framework/🔨️modules/🧬️schema && bunx vitest run --config vitest.config.ts` | 3 files / 27 tests passed; `.🧬semio/🦑️repo/⚡️cache/vite/framework-schema` created |
| `cd 🖥️shell/📦️packages/🟦️typescript && bunx vitest run --config vitest.config.ts` | 2 files / 7 tests passed; `vite/os-shell` cache dir created |
| `cd 🔌️plugin/📇️registry && bun ./📜️script.ts test` | 1 unrelated failure (crate `semio-s-plugin-lowpoly`→`semio-s-artifact-lowpoly-lowpoly` rename in flight elsewhere) + suite killed by a 15s budget under fleet load; NOT caused by cacheDir — `vite/plugin-registry` cache dir was created and other assertions ran normally |
| `cd 🎠️kernel/📦️packages/🟦️typescript && bunx vitest run --config vitest.config.ts` | 1 unrelated failure: `$ref` to `value/schema.json#/$defs/NonZeroU64` cannot resolve (pre-existing schema gap, confirmed via clean `git status` on `🌱️value`/`🎭️actor` — not caused by this lane); flagged via `spawn_task` (`task_0845a137`) rather than fixed in-lane (out of scope for caching) |
| `cargo check -p semio-framework-plugin-host` (comment-only `.rs` edit) | clean, 1m33s |
| `cd 🌉️mcp/📦️packages/🦀️rust && bun ./📜️script.ts build` and `cargo check -p semio-framework-os-mcp` | both fail with **pre-existing, unrelated** Rust errors: `ProbeSnapshot: ArtifactCompositionFields` trait bound unsatisfied in `🌉️mcp/🏠️workspace/🦀️.rs` (`ArtifactStore::undo/redo`) and an `expires_at_ms`→`expires_at` field rename mismatch — both in code I never touched, `git status` clean on those files, and the failure reproduces identically with `cargo check` (no `CARGO_TARGET_DIR`/wrapper flags at all), so it is not a target-dir/resolver regression. Left for the owning lane; not re-flagged since it looks like an in-flight rename by another concurrent session. |

## Left undone / notes for other lanes

- The `NonZeroU64` schema gap and the `semio-framework-os-mcp` / `ArtifactStore` trait-bound break are
  real but unrelated to shared-cache work; the former is now a spawned follow-up task
  (`task_0845a137`), the latter looked like active concurrent work by another session and was left
  alone per "ignore unrelated recent changes."
- Several stale `target`/`target-root-*` directories (listed under Task 4) were intentionally left on
  disk — not deleted per instructions; they will fall out of use now that nothing creates them, and
  can be swept by the eventual `cache-prune` job (S7) or a manual cleanup once the fleet is idle.
- Did not touch anything under `🔨️modules/🧑‍💻dev/**` (S3) or `📚️library/**`/`policy.json` (S1/S7),
  including `devToolingEnv`, `wasmBuildEnvironment`, and `repoToolCacheEnv` themselves — only consumed
  them.

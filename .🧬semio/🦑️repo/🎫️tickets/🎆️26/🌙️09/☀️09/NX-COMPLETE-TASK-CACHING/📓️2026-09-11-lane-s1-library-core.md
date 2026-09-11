# Lane S1 — Library Core (2026-09-11)

Scope: `📚️library/📦️packages/**`, `📚️library/🏃️process/**`, `📚️library/🧪️tests/**` (not `⚡️caching/🧪️tests`, except
the two named assertions), and the whole `🧪️test` module.

## Files changed

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- `wasmBuildEnvironment` (was ~line 2905): no longer defaults `CARGO_TARGET_DIR` to
  `.🧬semio/🦑️repo/⚡️cache/cargo/browser`; now returns `{ ...env }` unchanged — Cargo's own
  `.cargo/config.toml` (`target-dir` = `.🧬semio/🦑️repo/⚡️cache/cargo/target`) governs unless a caller
  explicitly sets `CARGO_TARGET_DIR`.
- `runWasmPackWebBuild`'s threaded branch: `buildEnv.CARGO_TARGET_DIR!` (would now be `undefined`) replaced
  with `cargoTargetDirectory(repoRoot, buildEnv)` from the central resolver.
- `runExtensionComponentPackage`: `resolve(repoRoot, process.env.CARGO_TARGET_DIR ?? "target")` replaced with
  `cargoTargetDirectory(repoRoot)`.
- `runExactCargoLaws` (~line 2049): the private fallback `join(getWorkspaceRoot(), "target")` (which ignored
  `.cargo/config.toml` entirely) replaced with `cargoTargetDirectory(getWorkspaceRoot(), configuredEnv)`. This
  is the one call site that was actually forking a private/stale target dir; every other call already only
  echoes the resolved value into the child env for its own source/target overlap safety check.
- `runPolicyScript`: `join(getRepoMetaDir(repoRoot), "⚡️cache", "breaches")` replaced with
  `repoCacheDirectory(repoRoot, "breaches")` (same bytes, uses the central helper).
- `runPlaywright`: now wraps its env in `repoToolCacheEnv(findRepoRoot(bundleRoot), playPollingEnv())` so
  Playwright's browser downloads land in `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` instead of
  `~/Library/Caches/ms-playwright`.
- Added imports: `cargoTargetDirectory` from `⚡️caching/🦀️cargo/🟦️.ts`, `repoCacheDirectory` from
  `⚡️caching/🟦️.ts`, re-exported `repoToolCacheEnv` alongside `devToolingEnv`.
- Left untouched (verified not cache-location paths): the `--target-dir`/`--target-dir-remap` string literals
  in `partitionNextestExecutionFilters`'s option sets (~1571/1597/2103) — these only recognize cargo/nextest
  flag *names* for argv routing, they never construct a directory; and the `node_modules/.vite` cache-clear
  helper (~2830), which is Storybook/dev-server plumbing owned by lane S3 (`🧑‍💻dev`).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts`
- `devToolingEnv`: removed the `env.RUSTC_WRAPPER ??= ""` default (sccache is gone; no wrapper needs
  defeating).
- Added new exported `repoToolCacheEnv(repoRoot, extra)`: sets `GOCACHE` → `repoCacheDirectory(repoRoot,
  "go")` and `PLAYWRIGHT_BROWSERS_PATH` → `repoCacheDirectory(repoRoot, "tools", "ms-playwright")`, both via
  `??=` so an explicit caller value always wins. `devToolingEnv` itself has no repo-root parameter (checked
  every caller in-scope — none pass one), so per the brief this is a sibling function wired in at the two
  places in my files where a repo root is actually known: `runPlaywright` (via `findRepoRoot(bundleRoot)`,
  in the typescript packages file) and the Go test-host spawn in the `🧪️test` module (below).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts`
- `materializeRustHost`'s env (~line 498/499): `CARGO_TARGET_DIR: process.env.CARGO_TARGET_DIR ??
  join(agentCacheRoot(repoRoot), "cargo-test-hosts")` replaced with `CARGO_TARGET_DIR:
  cargoTargetDirectory(repoRoot)` — cargo test hosts now build into the one shared cargo target dir instead
  of a per-agent `cargo-test-hosts` fork.
- `materializeGoHost`'s env (~line 517): wrapped in `repoToolCacheEnv(repoRoot, { ...process.env, GOFLAGS:
  "-mod=mod", GOWORK: "off" })` so `go run` for the generated Go test host uses the shared `GOCACHE`.
- Added imports: `cargoTargetDirectory` from `../📚️library/⚡️caching/🦀️cargo/🟦️.ts`, `repoToolCacheEnv` from
  the existing `../📚️library/📦️packages/🟦️typescript/🟦️.ts` import list.
- `agentCacheRoot` import kept — still used for `PYTHONPYCACHEPREFIX` (non-cargo per-agent scratch).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
- `testCacheRoot` (~1169): `join(getRepoMetaDir(repoRoot), "⚡️cache", testTaxonomy(repoRoot).testOutputCacheDirName)`
  → `repoCacheDirectory(repoRoot, testTaxonomy(repoRoot).testOutputCacheDirName)`.
- `agentCacheRoot` (~1186): body switched to `repoCacheDirectory(repoRoot, "agents", agentId…)`; docstring
  corrected from "so concurrent sessions never contend on one target directory" (no longer true — Cargo
  output no longer lives here at all) to "scratch root for non-Cargo per-agent state (e.g. Python's
  pycache); Cargo output lives in the one shared cargo cache instead."
- `cleanTestOutputs`'s `protectedPaths` (~2404): `join(getRepoMetaDir(repoRoot), "⚡️cache")` →
  `repoCacheDirectory(repoRoot)`.
- Added import of `repoCacheDirectory` from `../../../📚️library/⚡️caching/🟦️.ts`. `getRepoMetaDir` import
  kept — still used for non-cache paths (`relative(repoRoot, getRepoMetaDir(repoRoot))` in several places).

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/📜️script.ts`
- `nativeState`: `join(root, ".🧬semio/🦑️repo/⚡️cache/dotnet/repo-test")` → `repoCacheDirectory(root,
  "dotnet", "repo-test")`. Added the `repoCacheDirectory` import. Same resolved path, routed through the
  central helper instead of a hand-spelled literal.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🟦️.ts` + its fixture/schema
- No code change in the test itself — its existing assertion `expect(options.env.CARGO_TARGET_DIR).toBe(
  resolve(getWorkspaceRoot(), fixture.compilerStorage.defaultDirectory))` already reads the resolver's
  contract correctly. What changed underneath it is `runExactCargoLaws`'s implementation (above), so the
  fixture's stale expected value needed updating to match:
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦀️exact-cargo-laws/🔣️.json`:
    `compilerStorage.defaultDirectory` `"target"` → `".🧬semio/🦑️repo/⚡️cache/cargo/target"`.
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🦀️exact-cargo-laws/🔣️.json`: the matching
    `const` in the schema updated the same way.
  These two JSON files sit under `📚️library/🧫️fixtures/**` / `📚️library/🧬️schema/**`, outside the literal file
  list in my brief, but they are a private fixture/schema pair that exists only to back this one test — no
  other lane references them — and leaving them stale would have left `bun ./📜️script.ts test
  exact-cargo-laws` broken by my own implementation change, so I updated them as necessary collateral.

### `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts`,
`…/🧲️rust-physical-reference-context/🟦️.ts`, `…/↪️rust-divergence-callback/🟦️.ts`
- Removed all `RUSTC_WRAPPER: ""` overrides (11 call sites across the three files) — sccache is gone, so
  defeating it is a no-op now; left as `env: { ...process.env }` (or `{ ...process.env, CARGO_MANIFEST_DIR:
  … }` where that was already present).
- **Deliberately did NOT remove the `--target-dir` values** in these three files (`🎯️target`, `🧪️build`,
  `🏗️target`/`target`). I read all three files end-to-end before deciding: these are not the "private
  target-dir" anti-pattern Wave A eliminated (concurrent forks of the *shared* crate-graph build). Each one
  is an isolated, single-file, throwaway oracle/probe crate written fresh into a ticket-scoped fixture root
  and built exactly once:
  - `🍃️artifact-support-leaf-authority`: `retireRustOracleOutputs`/`disposableOutputs` is a documented,
    fixture-JSON-backed retention CONTRACT (`Cargo.lock` + `🎯️target` are named, schema-validated
    "disposableOutputs"; the suite also unit-tests retain-on-failure debugging behaviour by hand-creating a
    fake `🎯️target/🧪️.bin`). Rerouting the real compile into the shared cache would silently break that
    contract (nothing to retire) without a corresponding, deliberate rewrite of the JSON contract that this
    plugin's own domain (energy, owned by lane S5) may also depend on.
  - `🧲️rust-physical-reference-context`: same `retainRun(row.directory, passed)` colocated-on-failure
    pattern.
  - `↪️rust-divergence-callback`: the `🏗️target` build is explicitly asserted `existsSync(target) === false`
    beforehand and used to measure a **cold** compile under a 120s budget (`evidence(owner, "Cold Compiler
    Authority", { targetInitiallyAbsent: true, … })`) — routing it through the shared, already-warm cache
    would invalidate the very thing the test measures.
  I only removed `CARGO_TARGET_DIR: join(f.directory, "🧾️cargo-target")` from one place —
  `🥤️rust-finite-target-consumption/🟦️.ts`'s `cargo metadata` call — because `cargo metadata` never invokes
  rustc and never touches a target dir at all, so the override was pure dead weight.

## Verification run

All commands run with `cd /Users/ueli/Documents/semio &&` from the repo root; no `CARGO_TARGET_DIR` /
`RUSTC_WRAPPER` set by me for any cargo invocation; `cargo check --workspace` never run; no backgrounded
builds.

- `bun build --target=bun --no-bundle <file>` — **all exit 0, no errors** — for every edited TS file:
  - `📚️library/🏃️process/🌿️environment/🟦️.ts`
  - `📚️library/📦️packages/🟦️typescript/🟦️.ts`
  - `📚️library/🧪️tests/↪️rust-divergence-callback/🟦️.ts`
  - `📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts`
  - `📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts`
  - `📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts`
  - `📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` (the two assertion lines I own)
  - `🧪️test/📜️script.ts`
  - `🧪️test/📦️packages/🔷️dotnet/📜️script.ts`
  - `🧪️test/📦️packages/🟦️typescript/🟦️.ts`

- `bun 📚️library/📦️packages/🟦️typescript/📜️script.ts test exact-cargo-laws`
  (`SEMIO_TEST_ARTIFACT_DIR` pointed at my ticket `🗑️generated/s1` scratch dir):
  **26 pass / 0 fail, 596 expect() calls** — including the `options.env.CARGO_TARGET_DIR` assertion against
  the fixture I updated, confirming `runExactCargoLaws` now resolves through `cargoTargetDirectory` and
  matches `.cargo/config.toml`'s real `target-dir`.

- `bun 📚️library/📦️packages/🟦️typescript/📜️script.ts test artifact-support long -t "retains authored oracle
  inputs and retires only exact completed compiler outputs"`: **1 pass** — the retention-contract unit test
  I chose not to touch is unaffected.

- `bun 📚️library/📦️packages/🟦️typescript/📜️script.ts test artifact-support long -t "plans the complete real
  Energy owner…"` (the one test that actually exercises the real `cargo run` at line 168 with
  `RUSTC_WRAPPER` removed): failed, but at the very *first* assertion (`entries.toHaveLength(329)`, received
  11338) — before any cargo code runs at all. Confirmed via `find .../🔋️model -type f | wc -l` → 5002 files
  on disk, i.e. the real repo directory this test walks has drifted far from the fixture's expected physical
  file count. This is unrelated to caching (`✏️s/🔌️plugins/🔋️energy/**` is owned by lane S5, actively being
  edited concurrently per the observed git status) — could not get this one specific assertion path to run
  clean in this environment, but the code it would have exercised (`env: { ...process.env }` with no
  `RUSTC_WRAPPER`) is byte-identical in shape to the passing `exact-cargo-laws`/`rust-finite-target-consumption`
  cargo/rustc spawns below, so I'm not flagging it as a regression risk.

- `bun 📚️library/📦️packages/🟦️typescript/📜️script.ts test rust-finite-target-consumption -t "Cargo metadata"`:
  **1 pass** — the one call site where I actually removed a `CARGO_TARGET_DIR` value.

- `bun 📚️library/📦️packages/🟦️typescript/📜️script.ts test rust-finite-target-consumption` (full file):
  **82 pass / 4 fail**. All 4 failures are `rustc`/native-compile tests unrelated to my one-line diff in this
  file (confirmed via `git diff` — only the `cargo metadata` line changed) — they fail with `error: couldn't
  create a temp dir: No such file or directory` inside
  `.../🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/.../🧾️runs/🧫️fixtures/…`, i.e. a retained-ticket-evidence
  directory this August-dated ticket expects is simply absent from this working tree. Same missing-directory
  pattern (not present anywhere under that ticket's `🧾️runs/`) also broke `rust-divergence-callback-native`
  (0 pass / 3 fail, same `ENOENT … ↪️rust-divergence-callback/🦀️.rs`) — confirmed pre-existing/environmental
  by diffing my 3-line RUSTC_WRAPPER-only change against this file.

- `bun test ./🧪️test/🧪️tests/🧪️test-platform/🟦️.ts -t "clean"`: **3 pass / 1 fail**. The one failure
  (`existsSync(join(repoRoot, "compose"))` expected `true`, got `false`) is unrelated — there is genuinely no
  `compose/` directory at the repo root right now. The assertion that *does* exercise my `testCacheRoot` /
  `repoCacheDirectory` refactor (`row.path.startsWith(".🧬semio/🦑️repo/⚡️cache/tests/")`) passed.

## Left undone / blocked

- Could not get a fully clean run of `artifact-support`'s "plans the complete real Energy owner" test or the
  full `rust-divergence-callback` / `rust-finite-target-consumption` suites, due to pre-existing,
  out-of-scope drift (a plugin directory owned by another lane, and missing retained-ticket-evidence
  directories under an unrelated, months-old ticket). None of the failures touch code I changed; each was
  isolated to a line/assertion outside my diff before concluding this.
- Did not touch `--target-dir` in the three oracle/probe test files listed in the brief
  (`🧲️rust-physical-reference-context`, `↪️rust-divergence-callback`, `🍃️artifact-support-leaf-authority`) —
  see the reasoning above. If the coordinator wants those private per-fixture target dirs eliminated too
  (routing even these one-off oracle compiles through the shared cache), that requires a coordinated rewrite
  of the `🍃️artifact-support-leaf-authority` retention-contract fixture JSON (shared with lane S5's energy
  plugin domain) and would invalidate `↪️rust-divergence-callback`'s cold-compile timing assertion — flagging
  for an explicit decision rather than making it unilaterally.
- `.cargo/config.toml`, `⚡️caching/**`, and other lanes' files were left untouched, as instructed.

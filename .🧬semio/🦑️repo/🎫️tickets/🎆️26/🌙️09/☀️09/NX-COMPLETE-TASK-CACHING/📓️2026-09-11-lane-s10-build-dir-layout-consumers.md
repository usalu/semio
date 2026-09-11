# Lane S10 — Build-Dir Layout Consumers (2026-09-11)

Scope: Task 1 (redesign the hub's headless Stdio metadata-capture contract to be layout-independent)
and Task 2 (repo-wide audit of every site that derives paths inside a cargo target/build dir, other
than uplifted deliverables).

## Task 1 — headless Stdio metadata capture redesign

### Root cause

`🌎️hub/📦️packages/🦀️rust/📜️script.ts`'s `captureHeadlessStdioMetadata` assumed the pre-Wave-A cargo
layout: one dependency directory at `<cargoTargetDir>/debug/deps` holding every compiled crate's
`.rlib`/`.rmeta` flat. Since `.cargo/config.toml` now sets `build.build-dir` separately from
`build.target-dir` with `build-dir-new-layout`, intermediates (including `semio_s_plugin_stdio`'s own
`.rmeta` and every crate it depends on) live in the shared build-dir under
`<build-dir>/<profile>/build/<package>/<unit-hash>/out/…` — `<target-dir>/debug/deps` no longer exists
at all. `receipt.cargoTargetDir` (from lane S1's `runExactCargoLaws` fix) still resolves through
`cargoTargetDirectory`, but that only ever governed the *uplifted* deliverable, never the dependency
closure — so the old `dependencyRoot = join(cargoTargetDir, "debug", "deps")` was reading a directory
that no longer exists.

### Redesign

Every original artifact path — the Stdio crate's own `.rmeta` and its full dependency closure
(`.rmeta`/`.rlib`/proc-macro dylibs) — is now taken exclusively from the `compiler-artifact` JSON
records already captured in `receipt.artifactDir/build.stdout` (never from a directory-layout guess).
`captureHeadlessStdioMetadata` copies every one of those files (stable-read, bounded, byte-verified,
race-refused — same safety primitives as before) into **one** private `deps/` directory under the
command's own `runRoot`. `headlessStdioImportArguments` passes that single directory to rustc via
`-L dependency=<dir>` (still exactly one directory — the "exactly one private dependency root owned by
the command" invariant is preserved, now enforced on the *capture's own output* rather than on a
`<target>/debug/deps` convention). `headlessStdioOwnedCargoTarget`/`headlessStdioCommandRoots`/
`proveHeadlessStdioLaunchIsolation` were **not** touched — they validate that the command's own
`CARGO_TARGET_DIR` env value is a private, ticket-owned child of its artifact root, an isolation
concern orthogonal to where cargo's own intermediates happen to live.

### Files changed

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
  - `basename` added to the `node:path` import.
  - `HeadlessStdioMetadataCaptureV1` (schema bumped to `/v2`): `originalPath`/`dependencyRoot` replaced
    with `capturedPath` (the Stdio crate's own copy, now living inside `depsDirectory`), `depsDirectory`,
    and `artifacts: readonly HeadlessStdioMetadataArtifactV1[]` (one entry per captured file, each
    `{originalPath, capturedPath, byteLength, sha256}`).
  - `captureHeadlessStdioMetadata`: now takes `stdioOriginalPath` + `dependencyOriginalPaths` (both
    JSON-sourced) instead of a single `originalPath` computed from a directory convention; copies the
    whole closure into one `deps/` directory under `runRoot`, rejecting colliding basenames up front
    (two different original paths that would land on the same captured filename).
  - `headlessStdioImportArguments`: now validates against `capture.depsDirectory` instead of
    `capture.dependencyRoot`; otherwise unchanged (still exactly one `-L` flag, still refuses a second
    dependency root).
  - `proveHeadlessStdioMetadataCaptureContract` (self-test): rewritten to build each simulated command's
    original artifacts across **several distinct directories** (`build-dir/debug/build/<pkg>-<hash>/out/`
    per crate) instead of one fake `cargo-target/debug/deps`, proving the capture never assumes that
    convention. Added two new law assertions: dependency-closure completeness (all 3 artifacts land in
    one private directory) and colliding-artifact-name rejection. Fixed the pre-existing race-test bug
    the redesign surfaced (below).
  - `proveHeadlessStdioImports`: now parses **every** `compiler-artifact` record in `build.stdout` (not
    only the Stdio-named one), collecting every `.rmeta`/`.rlib`/proc-macro-dylib filename into the
    dependency closure passed to `captureHeadlessStdioMetadata`; downstream `-L`/`--extern` wiring
    unchanged.

### A real bug the redesign surfaced (and fixed) in the self-test itself

The original race-test triggered the mid-read file-swap on the **2nd** `check()` call, which — once the
capture reads multiple files in sequence — actually lands during the **first** file's read window, not
the file being swapped. `readStableBuildFile` only detects a swap that happens strictly after it has
taken its own `before` fstat snapshot and strictly before it finishes reading; a swap that completes
*before* the target file's own read even starts is invisible to it (self-consistent by the time it's
opened). Fixed by making the first file (`raceStdio`) empty (contributes exactly one `check()`, no
in-loop chunk checks) and sizing `raceDep` at 128 KiB (two chunks) so the trigger (`checks === 3`) lands
inside `raceDep`'s own read loop, after its `before` snapshot. Verified with a standalone harness (see
below) before and after: the *un-fixed* trigger count reproducibly failed to refuse the race; the fixed
one reproducibly refuses it.

## Task 1 — verification

All commands run with `cd /Users/ueli/Documents/semio &&` from the repo root.

1. **`bun build --target=bun --no-bundle` on every edited file — all exit 0**:
   - `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
   - `🌎️hub/📦️packages/🟦️typescript/🟦️.ts`
   - `🌎️hub/🧪️tests/🤝️integration/🟦️.ts`
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts`
   - `📜️script.ts` (root)
   - `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`

2. **Standalone verification harness** (`🗑️generated/s10/headless-capture-verify.ts` — the redesigned
   `headlessStdioOwnedCargoTarget`/`captureHeadlessStdioMetadata`/`headlessStdioImportArguments`/
   `proveHeadlessStdioMetadataCaptureContract` copied verbatim from the hub script, since they are
   module-private and not importable; the real, exported `readStableBuildFile` was imported, not
   copied). Chosen because the actual self-test can only be reached through the hub CLI, which — see
   below — is currently blocked by an unrelated, concurrent, in-progress edit. Real run output:
   ```
   headless-stdio-metadata-capture-oracle: private-commands=2 closure-complete=1 replacement-stable=1
   replacement-during-capture-denied=1 second-dependency-root-denied=1 outside-target-denied=1
   colliding-artifact-name-denied=1
   [DEBUG] headless-capture-verify: all assertions passed
   ```
   (First run, before the race-test size/trigger fix, reproducibly failed at
   `replacement-during-capture-denied` — real evidence the bug above is real, not theoretical.)

3. **Real CLI run** — `SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/s10/native-openable-provider-exact
   CARGO_TARGET_DIR=<same>/cargo-target CARGO_BUILD_JOBS=1 bun nx run
   os-hub:native-openable-catalog-provider-check --skip-nx-cache -- --oracle-only`: **fails**, but at
   `proveHeadlessStdioLaunchIsolation` (line 5162, **unchanged code**, not part of this redesign) —
   `.vscode/🧩️launch.seed.jsonc` is currently mid-edit by another concurrent session/lane (`git status`
   shows it `M`, uncommitted, last real commit 2026-09-10) and its three headless-Stdio launch entries
   (`⚖️gate📇️native-openable-catalog-provider🌎️hub`, `⚖️gate📇️headless-native-catalog🗄️stdio`,
   `⚖️gate📇️native-catalog-selection🌎️hub`) currently carry **no** `CARGO_TARGET_DIR` key at all (grep
   confirmed zero `CARGO_TARGET_DIR` occurrences in the whole file). `proveHeadlessStdioLaunchIsolation`
   requires each entry's `CARGO_TARGET_DIR` to literally equal `${artifactRoot}/cargo-target`, so it
   throws `headless Stdio launch is not privately isolated: ⚖️gate📇️native-openable-catalog-provider🌎️hub`
   before ever reaching `proveHeadlessStdioMetadataCaptureContract`. This confirms lane S5's flagged
   cross-lane follow-up ("`.vscode/🧩️launch.seed.jsonc` still supplies these `CARGO_TARGET_DIR`s at the
   moment; if S6 removes them, `headlessStdioCommandRoots()`/`proveHeadlessStdioLaunchIsolation()` will
   start throwing") has now actually happened, live, mid-session. **Not fixed here** — `.vscode/**` is
   lane S6's file, it is mid-edit by someone else right now, and guessing at its still-in-flight shape
   would risk clobbering their work. This is a real, currently-reproducing failure independent of my
   redesign (my redesign's own code was never reached).

4. **What was and was not executed**: the self-contained capture/import-argument/self-test logic was
   executed for real (via the standalone harness, item 2) and passed. The full multi-hour real
   `cargo test --no-run` + `rustc` law run (`proveHeadlessStdioImports` against the actual
   `semio-s-plugin-stdio` crate) was **not** executed — it is unreachable right now due to the
   `.vscode` blocker in item 3, and even without that blocker a `full-artifact-catalog` Stdio build is
   the multi-hour build the brief anticipated. Once `.vscode/🧩️launch.seed.jsonc` is settled (either by
   S6 restoring a private `CARGO_TARGET_DIR` per entry, or by `proveHeadlessStdioLaunchIsolation` being
   deliberately redesigned to not require one), rerun
   `bun nx run os-hub:native-openable-catalog-provider-check -- --stdio-only` end to end.

## Task 2 — repo-wide audit

`rg` sweep (excluding `node_modules`, `.git`, `.🧬semio/🦑️repo/⚡️cache`, `.🧬semio/🦑️repo/🎫️tickets`)
for: `"deps"`/`/deps/`, `.fingerprint`, `debug/build`/`release/build`, `incremental`, `out_dir`/`OUT_DIR`
guesses, `-L dependency=`, `*.rlib`/`*.rmeta`/`*.d` globbing, `cargo-target`, `requireEmptyFreshRoot`,
plus a follow-up sweep for hardcoded `"target"`/`"target", "debug"`/`"target", "release"` literals once
the pattern above surfaced real hits of that shape.

| site | reads | verdict | change |
| --- | --- | --- | --- |
| `🌎️hub/…/📜️script.ts` `headlessStdioOwnedCargoTarget`/`captureHeadlessStdioMetadata`/`headlessStdioImportArguments`/self-test/`proveHeadlessStdioImports` (~5131-5416) | dependency closure `.rmeta`/`.rlib` (intermediates) | was broken (`<target>/debug/deps` no longer exists) | **fixed** — Task 1, JSON-message-driven |
| `🌎️hub/…/📜️script.ts` `hubBinaryPath` (~862), `nativeWgpuExecutable` (~1183) | uplifted `os-hub`/wgpu bin | fine | already fixed by lane S5 (`cargoTargetDirectory`), verified unchanged and correct |
| `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` `resolveHubBinaryPath` | uplifted `os-hub` bin | was hardcoded `join(repoRoot,"target","debug",name)` — a **different file** from the already-fixed Rust script, missed by lane S5 | **fixed** — now `join(cargoTargetDirectory(repoRoot),"debug",name)`; confirmed resolves to `.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/os-hub` on the real repo |
| `🌎️hub/🧪️tests/🤝️integration/🟦️.ts:795` test expectation | test fixture | stale literal `join("/repo","target","debug",…)` | **fixed** to `join(cargoTargetDirectory("/repo"),"debug",…)`; test run: 1 passed |
| `🔌️plugin/📇️registry/📜️script.ts` `emitRustArtifacts` → generated `🤖️generated/🗿️artifacts.rs`'s `pub const PLUGIN_WASM_TARGET_DIR` | uplifted plugin `.wasm` (consumed by `🏃️run/🚀️bin.rs` via `include!`) | hardcoded `"target/wasm32-wasip2"`, baked into checked-generated Rust | **fixed** — codegen now interpolates `relative(repoRoot, join(cargoTargetDirectory(repoRoot),"wasm32-wasip2"))` at generation time (a compile-time `&str` const can't call a resolver at Rust runtime, so the *codegen* now does the resolving, same architecture as before) |
| `🔌️plugin/📇️registry/📜️script.ts` `WASM_TARGET_DIR`/`publicationWasmPath` (~1989/2000) | uplifted `.wasm` for publication identity | hardcoded `["target","wasm32-wasip2"]` | **fixed** — `publicationWasmPath` now computes via `cargoTargetDirectory(repoRoot)` directly |
| `🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts:92-131` two tests | test fixtures for the two sites above | one already synthetic (`freshTarget`, passed explicitly — fine, untouched); the other (`publicationWasmPath` non-substitution test) hardcoded `join(root,"target",…)` against the **real** `getWorkspaceRoot()` | **fixed** the second one to `join(cargoTargetDirectory(root),"wasm32-wasip2",…)`; test run: 1 passed |
| `📜️script.ts` (root) `PLUGIN_WASM_TARGET_DIR`/`pluginWasmArtifactExists` (~17009) | uplifted plugin `.wasm` (preflight for `os run`) | hardcoded `"target/wasm32-wasip2"` | **fixed** — now resolves `join(cargoTargetDirectory(repoRoot),"wasm32-wasip2")` per call |
| `🌉️mcp/🏠️workspace/🦀️.rs:82` `PLUGIN_WASM_TARGET_DIR` (hand-duplicated, not generated) | uplifted plugin `.wasm` at MCP-gateway runtime | hardcoded `"target/wasm32-wasip2"` — this file's own doc comment (lines 60-69) already documents this as a known, separately-tracked duplication (a filed "lease" asking for a shared `PluginPaths` extraction) | **fixed** — hand-synced to the resolved value `.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2`, with a comment explaining it must be hand-kept in sync with the generated copy until the lease lands (no Rust-side dynamic resolver added — no `toml` crate dependency exists anywhere in this workspace's Rust code, and introducing one for a single well-known static key would cut against AGENTS.md's "no new runtime dependencies" rule) |
| `🏃️run/🚀️bin.rs:88-98` `resolve_plugin_paths` | uplifted plugin `.wasm` at run-CLI runtime | consumes `PLUGIN_WASM_TARGET_DIR` via `include!()` of the generated file above | **fixed transitively** by the codegen fix; doc comment corrected (no longer says literally `target/wasm32-wasip2`) |
| `✏️s/🔌️plugins/🗄️stdio/…/📜️script.ts` `requireEmptyFreshRoot`'s `ambientTarget` (~899) | safety check refusing an ambient/shared root as a "fresh" build root | hardcoded `resolve(repoRoot,"target")` — now checks the **wrong, dead** path; the *real* shared cargo state (target-dir **and** build-dir) would silently pass as "fresh" | **fixed** — now `cargoTargetDirectory(repoRoot)` **and** `cargoBuildDirectory(repoRoot)` (build-dir added since it didn't exist as a separate concept before Wave A) |
| `🔌️plugin/📇️registry/📜️script.ts` `createFreshCatalogBuildVerifier`'s `sharedTarget` (~2792) | same class of safety check | same bug, same file family | **fixed** — same pattern; verified the one existing test (`🧪️tests/✅️catalog-complete`, "refuses ambient roots…") is unaffected since it uses a synthetic `mkdtempSync(tmpdir())` root with no real `.cargo/config.toml`, so `cargoTargetDirectory`/`cargoBuildDirectory` fall back to the exact same `<root>/target` value the old literal used; ran it for real: 1 passed |
| `✏️s/🔌️plugins/🗄️stdio/…/📜️script.ts` `CatalogRootScript`'s `cargoTarget = join(buildRoot, ".stdio-cargo-target-${pid}")` (~927) | uplifted `.wasm` at `<cargoTarget>/wasm32-wasip2/<profile>/<out>` | fine — private per-process target-dir, still governed by the shared build-dir underneath; reads only the final deliverable | left as-is (matches lane S5's own conclusion, re-verified) |
| `🔌️plugin/🖨️describe/…/📜️script.ts` `cargoTargetRoot`/`ensureBuiltBin`/`pluginWasmArtifactPath`/`produceFreshComponentV1` | uplifted bin/wasm | fine — `cargoTargetRoot` already delegates to `cargoTargetDirectory`; all reads use the standard `<target>/<triple?>/<profile>/<name>` convention, unaffected by the build-dir split | left as-is (matches lane S4's conclusion, re-verified) |
| `🌎️hub/…/📜️script.ts` native-gate `--features test-support` / `produceFreshComponentV1`'s isolation builds (~10233/10823/12948-12973) | uplifted deliverables, deliberately isolated feature/freshness variants | fine — isolation is orthogonal to layout; each build still resolves the shared build-dir underneath, only the *deliverable* landing spot is private | left as-is (matches lane S5's conclusion) |
| `📚️library/🧪️tests/🍃️artifact-support-leaf-authority`, `🧲️rust-physical-reference-context`, `↪️rust-divergence-callback` (`--target-dir` kept) | uplifted deliverables of one-off, single-compile oracle/probe crates | fine — documented retention contracts / cold-compile timing assertions that a shared warm cache would invalidate | left as-is (lane S1's explicit, documented decision; re-verified reasoning holds) |
| `📚️library/⚡️caching/🦀️cargo/📜️script.ts` `buildCargoArtifacts` (~87-154) | Cargo's declared deliverables **and** their dependency closure | fine — the gold-standard pattern: streams `--message-format=json-render-diagnostics`, copies every file from `message.filenames` verbatim, never guesses a directory | left as-is; this is the pattern Task 1's redesign now matches |
| `📚️library/⚡️caching/🧹️pruning/🟦️.ts` + `⚡️caching/🧪️tests/🧹️cache-prune/🟦️.ts` + `⚡️caching/🧫️fixtures/cache-prune/🔣️.json` | walks the *entire* real target/build tree generically for LRU eviction | fine — a pruner must walk the real tree by nature; it doesn't guess any *specific* artifact's path, so it's immune to the layout change | left as-is (S7's module) |
| `📚️library/⚡️caching/🔣️policy.json` `"deps"` entries (×2) | task-name enum (`nx` target name), unrelated to any directory | fine (false positive on the raw grep) | none |
| `🖱️ui/🎨️styling/🟦️.ts:80` + test `/node_modules/.vite/deps/` | Vite dev-server URL-scheme check (browser-side path convention, not an on-disk `cacheDir`) | fine (already reviewed by lane S4) | none |
| `📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` self-managed `🧪️rustc`/`compiler` scratch dirs (~955-1000, ~7095) | self-authored rustc invocations with explicit `-o`/`--emit=…=<path>` | fine — fully self-contained, never reads any cargo target/build-dir | none |
| `💻️os/🔌️plugin/📇️registry/📜️script.ts:3064` direct `rustc --emit=metadata=fixture.rmeta,dep-info=fixture.d` | explicit output path | fine — same pattern | none |
| `✏️s/🔌️plugins/🖍️draw` & `➗️mathematical` probes `📜️script.ts:96` `"🦀️oracle-probe","target","gate-inputs"` | the probe's own fixture-output directory (name collision with the word "target", unrelated to cargo) | fine (false positive); the same file's own comment (lines 100-101) confirms its actual `cargo run` already relies on `.cargo/config.toml` with no override | none |
| `💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:348-352` `PLUGIN_WASM_TARGET` + `cargoTargetRoot` | uplifted `.wasm` at dev-server boot | fine — already `cargoTargetDirectory(repoRoot)`-based (lane S3's territory, already correct) | none |
| `.vscode/🧩️launch.seed.jsonc` three headless-Stdio launch entries | `CARGO_TARGET_DIR`/`SEMIO_TEST_ARTIFACT_DIR` env values | **currently broken** — mid-edit by a concurrent session, all three `CARGO_TARGET_DIR` keys are gone, so `proveHeadlessStdioLaunchIsolation` throws (see Task 1 §3) | **not fixed** — lane S6's file, actively being edited right now; flagging only |

### Notable non-finding

`📚️library/⚡️caching/🦀️cargo/📜️script.ts`'s `buildCargoArtifacts` — the one place in the whole repo
that already builds Cargo *and* locates every dependency artifact — has always read paths exclusively
from `compiler-artifact` JSON `filenames`, never from a directory convention. It was never broken by the
build-dir layout change. This is strong independent confirmation that the JSON-message-driven pattern
Task 1's redesign now follows is the established, correct approach in this codebase, not a new invention.

## Verification commands (Task 2 fixes)

- `bun build --target=bun --no-bundle` on every file in the audit table marked "fixed" — all exit 0
  (same files listed under Task 1 §1, plus `🧰️framework/…/🔌️plugin/📇️registry/📜️script.ts` and its
  `🚀️launch` test, `📜️script.ts`, `✏️s/🔌️plugins/🗄️stdio/…/📜️script.ts`).
- `cargo check -p semio-framework-os-mcp --message-format=short`: 4 errors, all **pre-existing** and
  **unrelated** (`expires_at_ms` field rename in `🔗️remote/🦀️.rs:315/338`, `ArtifactStore::undo`/`redo`
  trait-bound gaps in `🏠️workspace/🦀️.rs:1481/1493` — byte-identical to lane S4's already-documented
  finding, confirmed still an in-flight rename by another session). My edit (a string-literal constant +
  two doc comments, lines ~82-142) introduced **zero** new errors — the crate reached and fully
  type-checked past my edited lines before hitting the unrelated ones.
- `cd 🌎️hub/📦️packages/🟦️typescript && bunx vitest run --config vitest.config.ts -t "retains the platform debug-binary contract"`: **1 passed**.
- `cd 🧰️framework/…/🔌️plugin/📇️registry && SEMIO_TEST_LEVEL=quick bunx vitest run --config vitest.config.ts -t "refuses ambient roots and detects the exact artifact max+1 boundary"`: **1 passed**.
- `bun -e '…resolveHubBinaryPath(process.cwd())…'` on the real repo: printed
  `.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/os-hub` (previously would have been the stale, nonexistent
  `<repoRoot>/target/debug/os-hub`).
- `bun nx run @semio-tech/plugin-registry:generate --skip-nx-cache`: ran to completion in the foreground
  (11m 14s — a full repo-wide discovery walk, matches the documented "Dev Boot Repo Walks" cost elsewhere
  in this repo): `plugin registry catalog refreshed (59 plugin crates, 61 playgrounds, 47 framework
  packages)`. Read the regenerated `🤖️generated/🗿️artifacts.rs` directly afterward — it now reads
  `pub const PLUGIN_WASM_TARGET_DIR: &str = ".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2";`,
  confirming the codegen fix is correct and `🏃️run/🚀️bin.rs`'s `include!` now picks up the right value.
  `🤖️generated/**` is gitignored (no `git status` change); the run also regenerated `.vscode/launch.json`
  and `.claude/launch.json` from the current (still mid-edit, see "Left undone" #1) `🧩️launch.seed.jsonc`
  — an expected side effect of the same nx target any dev would run, not something this fix introduced.

## Left undone

1. `.vscode/🧩️launch.seed.jsonc`'s three headless-Stdio launch entries are missing `CARGO_TARGET_DIR` —
   currently breaks `proveHeadlessStdioLaunchIsolation` (and, downstream, `headlessStdioCommandRoots`)
   for both `native-openable-catalog-provider-check` and `native-catalog-selection-check`, at every
   `--oracle-only` level and above. This is lane S6's file and it is being actively edited right now
   (`git status` shows it modified, uncommitted). Needs either: S6 restores a private, unique
   `CARGO_TARGET_DIR` per entry (`${artifactRoot}/cargo-target`, as before), or a coordinated decision to
   redesign the isolation proof around something other than a per-launch private target-dir now that
   Wave A intentionally collapsed most private target-dirs into the one shared cache.
2. `proveHeadlessStdioImports`'s full real-build path (multi-hour `cargo test --no-run` against
   `semio-s-plugin-stdio` with `full-artifact-catalog`, then per-fixture-case `rustc` imports) was not
   executed end-to-end — blocked by item 1, and independently a multi-hour build as the brief
   anticipated. The logic it depends on (capture + import-argument construction) was verified via the
   standalone harness instead (Task 1 §2).
3. `🌉️mcp/🏠️workspace/🦀️.rs`'s hand-duplicated `PLUGIN_WASM_TARGET_DIR` (and the file's own filed
   "lease" to consolidate 3+ copies of this convention into one shared `PluginPaths` API) was hand-synced
   to the current resolved value, not consolidated — that consolidation is explicitly a separately-owned,
   larger piece of work per that file's own doc comment, out of scope for a cache-layout migration.
4. Did not add a Rust-side dynamic `.cargo/config.toml` reader — no `toml`-parsing crate exists anywhere
   in this repo's Rust code today, and both native consumers (`🌉️mcp`, `🏃️run/🚀️bin.rs`) can resolve the
   correct path at TypeScript codegen time instead (the values are static once `.cargo/config.toml` is
   fixed, matching this codebase's existing "handcraft/regenerate, no dynamic indirection" architecture)
   — introducing one now would be both a new runtime dependency AGENTS.md discourages and unnecessary
   given the codegen-time alternative.

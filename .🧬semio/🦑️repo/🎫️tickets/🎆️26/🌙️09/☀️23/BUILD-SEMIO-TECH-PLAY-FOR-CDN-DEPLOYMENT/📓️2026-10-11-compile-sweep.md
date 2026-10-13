# 2026-10-11 Pre-Release Compile Sweep

Executor: sweep (Sonnet). Scope: compile every caller of today's framework API changes (operation wire in owner catalogs, `ArtifactCanonicalJsonTree` bounds, `bounded_artifact_store_owners*` `Send + Sync + 'static`, completion facet defaults, 3-argument `plugin_exports!`, pack-json paged tree walk). All cargo ran in the foreground (the harness moved the 10-13 minute runs to the background) through `🚦️cargo-slot.sh` with a private `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` under `.🧬semio/🦑️repo/⚡️cache/play-fleet/sweep/` (both deleted, 22 GB). Logs and tsv results stay in that folder. No git-modifying command, no ticket operation, no sub-agent.

## Result: the release path compiles; nothing needed a Rust fix

| # | Command | Result |
| --- | --- | --- |
| 1a | `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --target wasm32-wasip2 --keep-going --message-format short` (first run, nothing excluded) | 4 errors, all host-only: `tokio` (feature set unsupported on wasm), `io-lifetimes`, `wasmtime-internal-*`, `libsqlite3-sys` (no wasi C sysroot), and `semio-framework-os-kernel-db` (69 errors). Every 35 hub composition crate compiled |
| 1b | same, `--exclude semio-hub --exclude semio-hub-auth-client` | `Finished`, exit 0 (10 m 40 s), 0 errors |
| 1c | same as 1b with `RUSTFLAGS="-Z threads=8 -C link-arg=--max-memory=536870912 -C link-arg=-zstack-size=8388608 --emit=mir"` (forces the MIR-only `arithmetic_overflow` lint that plain `check` hides, see 📓️2026-10-10-os-wasm.md section 2) | `Finished`, exit 0 (12 m 53 s), 0 overflow lints, 0 errors |
| 1d | `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --lib --keep-going` (native) | exit 0 |
| 2 | `cargo check --manifest-path <m> --lib [--features component-app-assembly] --keep-going` for all 132 `📦️packages/🦀️rust/Cargo.toml` under `✏️s/🔌️plugins` (artifacts, extensions, engines, lsp; list in `plugin-manifests.txt`, results in `native.tsv`) | 131 OK, 1 NOLIB (`🔱️trinity/🗿️artifacts/🔌️jack/🐚️shell`, bin crate; re-run as `cargo check` without `--lib`: OK). 58 of the 132 declare `component-app-assembly` and were checked with it. 0 failures |
| 2b | `cargo check --manifest-path ✏️s/Cargo.toml --workspace --lib --keep-going` | exit 0 |
| 2c | `cargo check --manifest-path 🧰️framework/Cargo.toml --workspace --lib --keep-going` | exit 0 (1 m 52 s) |
| 2d | `cargo check` of `semio-framework-plugin-describe` (play project `os-plugin-describe-rs`, not a member of the framework workspace) | exit 0 |
| 3 | `cargo check --lib --target wasm32-unknown-unknown` of the 22 wasm-bindgen crates (table below) | 21 OK, 1 FAIL (`semio-framework-os-renderer-wgpu`, not in the play release graph, see below) |
| 4 | TypeScript, see below | play closure unchanged at the 4 benign baseline errors; 4 `actorIdentityGrant` errors fixed |

## Excluded crates and the proof they are not in the play release graph

- `semio-hub` (`🌎️hub/📦️packages/🦀️rust`, nx project `os-hub`) is the only crate that depends on `semio-framework-os-kernel-db`; `semio-hub-auth-client` (nx `@semio-tech/hub-auth-client-rs`) is the only crate that depends on `semio-framework-os-mcp` (`cargo tree --workspace --target wasm32-wasip2 -i <crate> -e normal,build`). `kernel-db` and `mcp` are not workspace members, so excluding their two dependents removes them from the check.
- Neither project name appears among the 84 `@semio-tech/*` projects that printed in `coordinator/fresh-6.log` (the 375-task `semio-tech-play:build` graph), no `package.json`/`project.json` in the repo depends on either, and the play project's targets (`prepare-release`, `catalog-release`, `build`) reach only `@semio-tech/framework-os-dev:*` tasks. `nx run ... --graph` could not prove it directly (`Invalid string length`: the task graph exceeds a JS string).
- `semio-framework-os-run` was NOT compiled by this sweep: `semio-hub-note` / `semio-hub-bim` depend on it only as a native `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]`, which `check --lib` does not build. The lib API drift noted in 📓️2026-10-11-wire-source.md section 7.5 is therefore unverified (not in the play release graph: no play project depends on it).

## Step 3: wasm32-unknown-unknown targets

All default features unless noted; manifests under `🧰️framework`, `✏️s`, `🌎️hub`.

| Crate | Status |
| --- | --- |
| os-flow (`🌊️flow`), surface (`--no-default-features --features session-bindgen`), os-node-graph, os host (`semio-framework-os`), editor, actor | OK |
| spatial-kernel session (`--features browser-publication`), dev-cad, trinity jack-lsp, `semio-hub-puzzle` (`--no-default-features`) | OK |
| os-infinite, raster, machine, canvas, replication, async, ui, ui-host, ui-backend-webgpu, plugin SDK, os-kernel | OK |
| `semio-framework-os-renderer-wgpu` | FAIL, 95 errors in `🧊️renderer`, `📐️surface-lane`, `🖼️IconRenderHost` export scene, EngineCanvas, Interpreter, Shell: old job APIs (`StepOutcome` vs `Result<Option<JobOutcomeBorrow>>`, `InteractiveJobCloseStep::Pending` fields, `WorkerJobSession::try_new`, `take_checked_out_outcome`, `retire_step`, `Mesh3dOwnershipStep` bool, `draft_changes_json`). NOT fixed: this is the legacy wgpu playground renderer. Only `✏️s/🧑‍💻dev/💡️services` depends on the crate and only the `bench-*`/native-scale targets of the dev project reference `@semio-tech/framework-renderer-wgpu`; its `wasm` target is not a dependency of any play target (the play host is the React renderer). Treat as the wgpu quarantine debt tracked in the fleet notes |

## Step 4: TypeScript

The play project has no `ts-check` target and no tsconfig (the 2026-10-10 throwaway under `ts-check/` was cleared with the cache), so `.../sweep/ts/tsconfig.play.json` was regenerated (root tsconfig + OS `types`, the Vite alias table as `paths`, the 31 play `.ts/.tsx` entry files; `bun node_modules/typescript/bin/tsc -p`).

- Play closure: 4 errors, identical to the 2026-10-10 baseline and benign: `🏗️builder/🌐️vite/🟦️.ts:32` Vite `defineConfig` overload, `🧪️playpages` `whatwg-url` declaration, 2x `WasmSessionLoader` `@semio-tech/framework-os-node-graph-rs` (wasm-pack `pkg/` output, absent until the build).
- Wider OS gate `bun nx run-many -t typecheck -p @semio-tech/framework-os @semio-tech/framework-renderer-react --skip-nx-cache` (`NX_FORCE_REUSE_CACHED_GRAPH=true`): 291 errors before, 287 after (counted per project, duplicates included), ~215 in `🧪️tests`/oracles, ~76 in tooling. Fixed, because they are today's API drift (open request now requires the retained-clone `actorIdentityGrant`):
  - `🔌️plugin/⚡️effect-backbone/🟦️.ts`: `BackboneWorkerTransportOptions.actorIdentityGrant` (required, no default) forwarded in the `open` request; the only callers are the two tests below.
  - `🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts`: both `createBackboneWorkerTransport` calls pass the grant.
  - `🏛️ShellHost/👥️presence-scope/🌐️browser/🟦️.tsx` (`browserRequest`) and `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (`openWire`): `actorIdentityGrant` literal, same values as ShellHost (`1024 / 65536 / 65536 / 65536 / depth 64`).
- Left (pre-existing at HEAD, not from today's changes, none reachable from the play closure): `frameworkOsPlaygroundDefaultPort` imported from the library barrel in 25 files but exported nowhere (only `🧑‍💻dev/♻️activation/🌐️serve` calls it, in the wgpu/Trunk playground serve path: a bun link-time failure if that serve script runs; all other importers do not use it and bun drops unused imports), `textEditorSyncPackV1` (2, TextEditor echo-pack test), `exactCargoGeneratedOutputHasLiveLease` (root `📜️script.ts:4`), `InteractivityToolRunSource` / `DirectoryHomeControllerSurfaceV1` / `BrowserPluginRuntimeSurfaceV1` (hub/root scripts), `pack/🌱️value/📜️script.ts` TS2775 assertion-name errors (10), `plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts` `law` unknown (10), `DedicatedWorkerGlobalScope` missing lib (2 workers), the rest `🧪️tests` fixture typing.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️effect-backbone/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/👥️presence-scope/🌐️browser/🟦️.tsx`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- this report and the corrections entry 66.

No Rust source was edited; no generated file touched. Not run: `vitest` for the effect-backbone tests (only type-checked), any cargo `--tests` target, any `--features` combination other than `component-app-assembly` / the listed wasm-bindgen features, `wasm-pack` / component linking (coordinator-owned).

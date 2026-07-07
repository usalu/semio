---
name: Speed Up Rust Compile Times
overview: Configure the Rust workspace with industry-standard compiler caching (sccache), a faster Linux linker (mold), nightly parallel-frontend flags, leaner dev-profile debug info, correct rust-analyzer wiring, unified workspace membership, Nx build caching for cargo/wasm targets, and CI caching — plus reclaim the 110GB of stale `target/` bloat that has accumulated.
todos:
  - id: cargo-config
    content: Add root .cargo/config.toml (sccache, nightly parallel frontend, mold for Linux, wasm getrandom cfg)
    status: completed
  - id: cargo-profiles
    content: Add [profile.dev]/[profile.dev.build-override]/[profile.release] tuning to root Cargo.toml
    status: completed
  - id: tooling-install
    content: Add zero-touch sccache install to script.ts SetupScript and mold to devcontainer Dockerfile + native bootstrap script.sh; remove home-dir cargo config hack
    status: completed
  - id: rust-analyzer-fix
    content: Fix rust-analyzer.linkedProjects in .vscode/settings.json and devcontainer.json to point at root Cargo.toml
    status: completed
  - id: unify-os-hub
    content: Merge framework/product/os/hub/rs into the root workspace members, drop its standalone [workspace] table
    status: completed
  - id: nx-cache
    content: Add cache/inputs for wasm and native-build Nx target defaults using the existing cargo named-input bucket
    status: completed
  - id: ci-cache
    content: Add Swatinem/rust-cache to play-sites.yml and gh-pages.yml workflows
    status: completed
  - id: clean-target
    content: Run cargo clean to reclaim the 110GB stale target/ directory, then a warm rebuild to validate the new setup
    status: completed
isProject: false
---

# Speed Up Rust Compile Times

## Diagnosis

- **No `.cargo/config.toml` exists anywhere** (repo or `~/.cargo`, aside from a wasm `getrandom` flag the setup script writes into the *user's home dir* at [script.ts:164-170](script.ts)). No `sccache`, no linker tuning, no nightly parallel-frontend flags.
- **`target/` is 110GB** (`du -sh target` confirmed) despite `resolver = "2"` and a single shared workspace — years of accumulated incremental artifacts across dev/release/wasm32/native-bin profile permutations, never pruned.
- **No `[profile]` section in root [Cargo.toml](Cargo.toml)** — dev builds emit full debug info (default `debug = true`), which is expensive to generate *and* link, especially for the heavy graphics stack (`wgpu`, `vello`, `resvg`, `usvg`, `tiny-skia`, `image`, `winit` — see [framework/renderer/wgpu/rs/Cargo.toml](framework/renderer/wgpu/rs/Cargo.toml)).
- **`rust-toolchain.toml` pins `nightly`** repo-wide but nothing takes advantage of nightly-only speed flags (`-Z threads`, `no-embed-metadata`).
- **957 locked packages**, 11 of them `git` sources — a large, cold dependency graph; nothing caches compiled artifacts across branches/CI runs.
- **`framework/product/os/hub/rs/Cargo.toml`** declares its own `[workspace]`, silently excluding it from the root workspace and giving it a second, separate `target/` directory.
- **`rust-analyzer.linkedProjects` points at `compose/rs/Cargo.toml`**, a file that does not exist, in both [.vscode/settings.json](.vscode/settings.json) and [.devcontainer/devcontainer.json](.devcontainer/devcontainer.json) — rust-analyzer is not correctly wired to the real workspace manifest.
- **Nx target defaults** ([nx.json](nx.json)) already define a `cargo` named-input bucket but it's never referenced — the `wasm`/`native-build`/`test` targets in the 17 rust `project.json` files declare no `inputs`/`cache`, so Nx never skips redundant cargo/trunk invocations.
- **CI workflows** ([.github/workflows/play-sites.yml](.github/workflows/play-sites.yml), [.github/workflows/gh-pages.yml](.github/workflows/gh-pages.yml)) build wasm bundles (puzzle-2d/3d/5d, sketchpad) but only cache the Nx cache dir — every run does a fully cold Rust/wasm compile.
- Confirmed on this machine: `sccache`/`mold`/`lld` are **not installed**; macOS's default `ld64` linker is already the fastest option here (per Bevy engine's own build-speed guide) — no linker override needed on macOS, only on Linux.

## Changes

### 1. Root `.cargo/config.toml` (new)
- `[build] rustc-wrapper = "sccache"` — compiler cache shared across crates, branches, and clean checkouts.
- `[build] rustflags = ["-Z", "threads=8"]` — nightly parallel front-end (safe since the whole repo is pinned to nightly).
- `[unstable] no-embed-metadata = true` — nightly flag that shrinks intermediate artifacts and speeds incremental rebuilds.
- `[target.x86_64-unknown-linux-gnu]` / `[target.aarch64-unknown-linux-gnu]`: `rustflags = ["-C", "link-arg=-fuse-ld=mold"]` (Linux/devcontainer/CI only; skip macOS per the ld64 finding above).
- `[target.wasm32-unknown-unknown] rustflags = ["--cfg", "getrandom_backend=wasm_js"]` — move this out of the per-developer `~/.cargo/config.toml` hack in [script.ts](script.ts) into the versioned repo config so it's zero-touch and consistent for everyone (then delete the home-dir-writing code in `SetupScript.runFull()`).

### 2. Root `Cargo.toml` profile tuning
```toml
[profile.dev]
debug = "line-tables-only"
split-debuginfo = "unpacked"
incremental = true

[profile.dev.build-override]
opt-level = 3

[profile.release]
strip = "debuginfo"
```
`build-override` optimizes build scripts/proc-macros (heavily used via `serde`/`serde_json` derives across the workspace) once, instead of interpreting them unoptimized on every rebuild.

### 3. Zero-touch tool installation (cross-platform, per `AGENTS.md` rules)
- Extend `SetupScript.runFull()` in [script.ts](script.ts) to download/install `sccache` (prebuilt GitHub release binary, mac/linux/windows) so `rustc-wrapper` never fails on a fresh machine.
- Add `mold` to `.devcontainer/Dockerfile`'s apt package list and to the Linux branch of `repo/native/bootstrap/script.sh`'s toolchain setup.

### 4. Fix `rust-analyzer.linkedProjects`
Change `["compose/rs/Cargo.toml"]` → `["Cargo.toml"]` in both [.vscode/settings.json](.vscode/settings.json) and [.devcontainer/devcontainer.json](.devcontainer/devcontainer.json) so rust-analyzer actually loads the real workspace instead of a nonexistent path.

### 5. Unify the `os-hub` workspace
Remove the stray `[workspace]` table from [framework/product/os/hub/rs/Cargo.toml](framework/product/os/hub/rs/Cargo.toml) and add `"framework/product/os/hub/rs"` to the root `[workspace] members` list, eliminating its separate `target/` directory and lockfile resolution.

### 6. Wire up Nx caching for cargo/wasm targets
In [nx.json](nx.json) `targetDefaults`, add cached entries reusing the existing (currently unused) `cargo` named-input bucket:
```jsonc
"wasm": { "inputs": ["cargo", "^cargo"], "cache": true },
"native-build": { "inputs": ["cargo", "^cargo"], "cache": true }
```
so unrelated changes no longer trigger redundant `trunk`/`cargo` invocations across the 17+ rust project.json files.

### 7. CI caching
Add `Swatinem/rust-cache@v2` (keyed on `Cargo.lock`) to [play-sites.yml](.github/workflows/play-sites.yml) and [gh-pages.yml](.github/workflows/gh-pages.yml) so wasm/rust builds in CI stop being fully cold every run.

### 8. Reclaim the 110GB `target/` bloat
Run `cargo clean` once locally to drop the stale, oversized incremental cache (already confirmed `.gitignore`'d, safe local build artifact), then do one warm build to repopulate it under the new, leaner profile + sccache setup.

## Out of scope / explicitly not doing
- Not forcing an alternate linker on macOS (Apple's default `ld64` already outperforms `lld`/`zld` per current guidance).
- Not adding `opt-level` overrides for dependency packages in dev (that trades compile time for runtime speed — a separate concern from "compiling takes long").

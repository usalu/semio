# Compile Pass 1 Result (2026-10-15)

Status: **partial, stopped by the physical-headroom gate.** The first native library pass over the framework workspace ran for real: 129 of 137 requested compilable crates compiled (125 checked, 4 proc-macro or build crates compiled) (including all pets, quiz, repo-*, server-throttle, geometry, value-*, canvas-camera, ui-scene-*, ui-render-* members that were requested), 2 crates failed with 2 errors (both fixed, **not yet re-verified**), 6 crates were not reached behind those failures. No wasm check, no `test` run, no root-workspace run and no `--all-targets` run completed. Reason: after the first library pass, the rust image sat 3,457,024 bytes below the 4,294,967,296 byte initial-headroom gate and stayed there for more than 70 minutes of retries (`sleep 300`, 14 attempts, then another manual attempt).

Logs: `🪓️monolith-split/🗑️generated/pass1-*.log` (`pass1-fw-lib1.log` is the main compile; `pass1-fw-lib3.log` is the last refused retry). Census: `pass1-census.log`, `census-summary.json`, `crate-census.csv`; topological order of the 204 compilable crates: `pass1-compilable.txt`; per-crate gate blockers for the 353 refused crates: `pass1-blocked.tsv`.

## 1. Findings that change how the pass has to be run

1. **Two workspaces.** `🧰️framework/Cargo.toml` is its own workspace (284 members, own `Cargo.lock`). The root workspace has 420 members. Of the 204 compilable crates, 137 are framework-workspace members, 66 are root-workspace members, 1 is neither (`semio-framework-plugin-mounted-owner`).
   - A framework crate checked from the root (`-p`) is only a path dependency there: its `[dev-dependencies]` are not resolved. Proof: `-p semio-framework-value-control-contract --all-targets` from the root failed with 10 × E0433 (`serde_json`, `encoding_rs` unresolved) although both are declared in its manifest; `cargo tree -e normal,dev` listed only `serde`.
   - Correct invocation for framework crates: add `--manifest-path 🧰️framework/Cargo.toml` after `--`. With it the same crate passes `check --all-targets` (26 `unused_qualifications` warnings, 0 errors).
2. **Framework lock was stale** (`cargo fetch --locked` refused). Ran `-- update --workspace --offline --manifest-path 🧰️framework/Cargo.toml` once: 8 packages added (mesh-component-references, ui-render-device, ui-render-prepared-shaders, ui-wgpu-device, ui-wgpu-draw-execution, ui-wgpu-mesh, ui-wgpu-pipelines, ui-wgpu-raster). Backup before: `pass1-framework-Cargo.lock.before`. The root lock update was not needed yet because no root-workspace run started.
3. **`--all-targets` over the whole set is refused by the source gate**: `Rust crate Source refused before compiler: semio-framework-replication (boundary exceeded)`. The census only counts normal and build dependencies; some requested crate has `replication` (57 files, 1,039,653 bytes, 16,464 lines) as a dev-dependency. Library-only checks (`--lib`) do not hit it. Tests for such crates are blocked until `replication` is carved (see section 4). I did not find which crate it is (no cargo budget left to run `cargo tree -e dev`).
4. Compile cost: one launcher call is about 80 s of overhead plus compile; batching all 137 crates in a single `check ... --lib --keep-going` took about 10 min wall time at load 99.

## 2. Native check (library, framework workspace, `--parallelism=4`)

Command: `SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <137 crates in dependency order> --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`. Result: `exit=101`, 2 failed crates, 49 crates emitted warnings (all `unused_qualifications` or small `unused` lints, no errors).

| Crate group (requested) | native `--lib` check | wasm | test | Errors fixed | Peer-caused |
| --- | --- | --- | --- | --- | --- |
| value-* (28 crates incl. value, value-derive, retirement, ordered, intrinsic, paged-*, retained-*) | pass | not run | not run | none | none |
| io-base64, io-schema, io-sqlite-snapshot, io-binary-source, io-control, hash, deflate, schema-registry, schema-state, schema-composition, schema-derive, composition-admission | pass | not run | not run | none | none |
| 2d, 2d-* (engine, compute, retire, path-flatten, path-stroke, booleans, trace, regions, text) | pass | not run | not run | none | none |
| actor, actor-* (pack, retained-turn, retained-context, vocabulary, job-publication), job, job-*, async, trace | pass | not run | not run | none | none |
| dsl, dsl-*, dsl-record, dsl-record-derive, diagnostic, pack-json, pack-json-retained-schema, pack-error, pack-source, mutation-diff, replication-error, replication-crypto, artifact-reference, artifact-reference-borrow | pass | not run | not run | none | none |
| geometry, geometry-* (primitives, vector, bulge, solids, skeleton), pixels, ui-styling, ui-locale, ui-viewport, ui-contract-budget/text/identity, intrinsic-size, icon-contract, canvas-camera | pass | not run | not run | none | none |
| ui-scene-algebra, ui-scene-mesh, ui-scene-camera, ui-scene-pack, ui-scene-pass, ui-scene-picking, ui-raster-owner, ui-prepared-staging, ui-render-shader-contract, ui-render-packet, ui-render-prepared-shaders, ui-render-text, ui-render-device | pass | not run | not run | none | none |
| surface-terrain, surface-tiled-map-mvt, machine, machine-derive, dispatch-macros, compiler, manifest-contribution, manifest-media, kernel-history, mesh-component-references, plugin-mounted-owner, source-projection, abi, 15 font crates | pass | not run | not run | none | none |
| `semio-framework-canvas-assets` | **fail, 1 error, fixed, not re-verified** | not run | not run | see below | none |
| `semio-framework-ui-wgpu-device` | **fail, 1 error, fixed, not re-verified** | not run | not run | see below | none |
| canvas-icons, canvas-scene, canvas, surface-paint, surface-tiled-map-model, surface-tiled-map | **not reached** (behind the two failures) | not run | not run | - | - |

`semio-framework-value-control-contract` additionally passed `check --all-targets` through the framework manifest (command in section 1), exit 0.

The 66 root-workspace crates (pets-*, quiz, quiz-schema, repo-* including repo-cli*, repo-graphql*, repo-mcp, repo-dashboard-*, server-throttle, os-* small ones, plugin-interpreter, plugin-component-receipt, semio-repo-test-host, os-services, os-pack-tools) were **not run**: no check result exists for them. Their argument list is in `pass1-rt-args.txt`. The root lock will need `-- update --workspace --offline` first.

## 3. Errors fixed (all produced by the fleet's carves; none re-verified, see status)

1. `🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🦀️.rs` (`semio-framework-canvas-assets`), former line 205: E0599 `CompilerFonts::estimate_only` not found. That function exists only on the `wasm32` + `p2` arm of `compiler`; the new guest-reader install path (RESULT-W4-CANVAS addendum) called it on every target.
   - Fix: `Installed.fonts` is now `Option<CompilerFonts<'static>>`; the guest install stores `None`, `install_runtime_assets` stores `Some(fonts)`, and the native `compiler_fonts()` returns `installed().and_then(|installed| installed.fonts.as_ref())`. The wasip2 `compiler_fonts()` arm is unchanged (static estimate-only value).
   - Test updated to match: `🧪️tests/📥️guest-delivery/🦀️.rs` asserts `compiler_fonts().is_some()` only on wasip2 and `is_none()` on a glyph-painting target (a guest delivery carries no math, serif or mono face). Risk to verify: on wasip2 the `fonts` field is never read, so a `dead_code` warning is expected there; the wasip2 check was not run.
2. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🔌️device/🦀️.rs:125,126,130` (`semio-framework-ui-wgpu-device`): E0107 `struct takes 0 lifetime arguments`. wgpu 29.0.4 `BufferView` has no lifetime parameter (`BufferSlice<'a>` does).
   - Fix: `pub struct BufferView{raw:wgpu::BufferView}`, `impl Deref for BufferView`, and `get_mapped_range(&self)->BufferView`. `git diff --stat`: 3 insertions, 3 deletions.

Known existing breaks from the checklist (dag vcs `use ::graph::manifest`, puzzle2d snapshot E0603 x9, `PUZZLE2D_NODE_KIND_OPTIONS_MAX` import): **not reached**, those crates are in the root workspace or behind the gate.

## 4. Crates still blocked by the source gate

Census (`bun census.ts`, 557 crates): 204 `closureCompilable: true`, 353 refused, 10 oversized crates that cause every refusal:

| Oversized crate | files / bytes / active lines | Crates refused through it (transitive) |
| --- | --- | --- |
| `semio-framework-replication` | 57 / 1,039,653 / 16,464 | 332 |
| `semio-framework-ui-contract` | 37 / 756,785 / 14,919 | 203 |
| `semio-framework-os-kernel` | 180 / 3,676,487 / 54,677 | 166 |
| `semio-framework-plugin-host` | 32 / 1,026,642 / 14,532 | 96 |
| `semio-framework-os-infinite` | 53 / 1,265,782 / 25,189 | 85 |
| `semio-framework-plugin` | 185 / 6,010,310 / 83,203 | 64 |
| `semio-framework-os-store-sync` | 2 / 318,079 / 4,736 | 5 |
| `semio-framework-os-renderer-wgpu` | 402 / 5,962,082 / 101,947 | 2 |
| `semio-framework-os-kernel-db` | 29 / 2,614,222 / 51,618 | 1 |
| `semio-framework-plugin-host-test-component` | 41 / 52,228 / 626 | 1 |

Direct dependants of an oversized crate: os-kernel 197, replication 109, ui-contract 73, os-infinite 64, plugin-host 48, plugin 9, os-store-sync 4 (full list in `pass1-blocked.tsv`).

Our family status (compilable / total crates matching the family name): pets 8/8, quiz 2/2, repo-graphql 3/3, repo-cli 6/6, canvas 5/5, ui-render 5/7, ui-scene 6/14, surface 5/7, pack 5/19, server 1/6, mcp 1/23, renderer-wgpu 5/89, trace 1/1, flow 0/25, workflow 0/6, ui-runtime 0/6, print 0/1 (blocked by `plugin`), os-host 0/6. Not compilable at all until the named oversized dependency is carved:
- flow*, workflow*, os-dag, os-neural, os-host: `os-kernel` (and `replication`, `os-infinite` for flow).
- ui-runtime*: `ui-contract` (also the remaining ui-scene crates and ui-render siblings).
- pack-* beyond the 5 leaves, server-* beyond `server-throttle`, mcp-*: `replication` and `os-kernel`/`plugin-host`.
- renderer-wgpu skeletons beyond the 5 WP0 ones: `ui-contract`, `plugin-host`, `os-infinite`, `os-kernel`.
- Carving `replication` and `ui-contract` first unblocks the most crates (332 and 203).
- `value-dictionary` does not exist as a crate in the census.

## 5. Blocker for the next step

- Gate: `Insufficient physical compiler initial headroom`. Available 4,291,510,272 B, required 4,294,967,296 B, short by 3,457,024 B. The recovery plan finds 6 inactive units (37,274,038 B) but plans 0 B: they are the protected stale `-working` incremental sessions described in RESULT-INFRA-UNBLOCK.md section 5. Five peer rustc analysis oracles (writing to `/dev/null`) and one 18 h resource-lease holder (pid 44313) were running; none of them writes to the image.
- Sanctioned fix (Root's authority, not taken by me): `⚖️cargo-retirement-historical-preview` then `retire` for prototype, or a policy change that reclaims provably dead `-working` sessions. I did not delete anything on the image and did not bypass or weaken the gate.
- Once headroom is admitted, the remaining sequence is: (a) re-run the framework `--lib --keep-going` pass to verify the two fixes and reach the 6 blocked crates; (b) `update --workspace --offline` for the root lock, then the root 66 crates (`pass1-rt-args.txt`); (c) `--target wasm32-wasip2` for `canvas-assets` (dead_code check) and the plugin-facing crates, `wasm32-unknown-unknown` for the ui-wgpu crates; (d) `test -p <crate>` per crate through `--manifest-path` (batch with `--no-fail-fast`), excluding the crate that has `replication` as a dev-dependency; (e) the known existing breaks.

## Files changed

- `🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🦀️.rs`
- `🧰️framework/🔨️modules/🖼️canvas/🔌️runtime-assets/🧪️tests/📥️guest-delivery/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🔌️device/🦀️.rs`
- `🧰️framework/Cargo.lock` (offline workspace update, +8 packages)

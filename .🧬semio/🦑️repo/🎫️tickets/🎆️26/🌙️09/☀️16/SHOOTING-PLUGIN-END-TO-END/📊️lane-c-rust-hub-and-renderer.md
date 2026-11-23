# Lane C — Rust Hub and Renderer

## Fixes

- `IconRenderHost` frame measurement launches Chromium with `--font-render-hinting=none`. Host hinting was rounding Share Tech Mono advances (19×5.4px became 19×5px), so the landscape badge was `[291,229,103,15]` instead of the unhinted `[283.390625,229,110.609375,15]`.
- The `large-shot` fixture is `16384×8192`, the export-schema maximum, with unhinted badge bounds `[107.1875,129,126.8125,15]`. `262144×131072` was rejected by `renderIconRequest` (`width`/`height` maximum `16384`), so the host rendered the failure state and never produced the image the layout test measures.
- `🦀️native-shooting-cargo.sh` keeps a caller-supplied `CARGO_TARGET_DIR` / `CARGO_BUILD_BUILD_DIR` so the build directory can stay on the overlay while the target directory is separate.

## Commands

| Command | Exit | Result |
| --- | --- | --- |
| `TMPFS_SIZE=6000m` `cargo test -p semio-hub-shooting --no-fail-fast --offline` (jobs 2) | 101 | `rustc` SIGKILL while compiling `semio-s-artifact-stdio-txt-tests`. No source diagnostic. Package tests never started. |
| same, `--jobs 1` | 137 | `cargo` killed (OOM) around `semio-ui-declaration`. |
| `--jobs 1`, opt-level 0, codegen-units 1, `TMPFS_SIZE=4000m` | 101 | ENOSPC writing `object` rlib. |
| same, `TMPFS_SIZE=6000m` | killed | OOM killer: `rustc` for `semio-s-artifact-stdio-pdf-model` at about 7GB RSS. |
| persist dir `/dev/shm/semio-shooting-persist` (usrquota) | 101 | Disk quota exceeded writing a `syn` fingerprint. |
| private tmpfs target+build, opt-level 0, 6GB | 101 | ENOSPC at `semio-os-store-space-history` after 254 crates. |
| overlay build dir, opt-level 0, 6GB | 101 | Same ENOSPC. |
| overlay build dir, opt-level 1, codegen-units 16, log on `/tmp` | 137 | `cargo` killed after 120 crates while other compilers were resident. |
| `cargo test -p semio-renderer-projected-linework --jobs 1 --offline` | 0 | 3 passed, 0 failed. Doc-tests 0. |
| `cargo check -p semio-hub-shooting-component --target wasm32-wasip2` | not run | Same hub graph; the host never stayed up through `semio-hub-shooting`. |
| `cargo test -p semio-renderer-wgpu-icon-host` | not run | Icon-host tests live in that crate (`IconRenderHost` wgpu modules). Not started while the machine was already short of RAM. |
| React vitest, no `SEMIO_VITEST_POLICY` | 1 | `Explicit Vitest policy required`. |
| WGPU vitest `browser-frame-transport` | 0 | 1 file, 64 tests. |
| Ticket `browser-icon-svg-vitest.config.ts` (icon SVG preview + frame transport) | 0 | 2 files, 67 tests. |
| `bun test` line-visibility + icon export | 0 | 21 passed, 0 failed, 6301 expects. |
| React vitest, `SEMIO_TEST_LEVEL=long`, explicit policy, icon + frame files | 0 | 11 files, 88 tests, after the two fixes above. Before the fixes: 87 passed, 1 failed. |

React files: `icon-svg-lighting`, `browser-frame-transport`, and the `IconRenderHost` suites `frame-presentation`, `status`, `request`, `download`, `png-export`, `svg-export`, `gpu-export`, `export-batch`, `svg-mask`.

## Not green

`semio-hub-shooting` and `semio-hub-shooting-component` (`wasm32-wasip2`) did not compile on this host. Every attempt died in dependency compilation (OOM, tmpfs ENOSPC, or the shm user quota), before `cargo` reached the shooting package or a Rust error in it. `semio-renderer-wgpu-icon-host` was not compiled for the same reason.

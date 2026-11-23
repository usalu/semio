# Wave 1 Cargo Pressure

Date: 2026-11-23. Ticket: `🎆️26/🌙️09/☀️16/SHOOTING-PLUGIN-END-TO-END`.

## What is green

Lane A finished. Source closures for five targets are inside the 32-file / 4000-line / 256 KiB budgets. Drawing, world-transport, and SVG retry probes passed. Snapshot SQLite source typecheck and its 43 tests passed. English and German labels on the scanned shooting surfaces are complete. See `📊️lane-a-standalone-probes-and-schema-audit.md`.

The TypeScript suite is 84/84 and the Python mutation oracle is 68/68. Playground session `shooting` staged one plugin. Browser support published. The generated catalog row for variant `shooting` points at the hub component, app `s.shooting.shooting@1/*#editor`, and ports 6019 / 6119.

## Why the Rust lanes stopped

`cargo test` for the shooting workspace and for `semio-hub-shooting` did not report a source error. `rustc` was SIGKILLed (one compiler reached about 7 GiB RSS with `oom_score_adj` 200) and a later hub attempt died with `No space left on device` inside the 4500 MiB private tmpfs. Every retry mounted a fresh tmpfs and set `CARGO_INCREMENTAL=0`, so the finished crates were discarded.

The runner `🦀️native-shooting-cargo.sh` now keeps one shared target at `/dev/shm/semio-shooting-persist`, uses incremental compilation, one job, opt-level 0, a 512 MiB scratch tmpfs, and waits on `exclusive.lock` plus the three old slot locks. A holder currently owns `exclusive.lock` so a test retry cannot start a second compiler while the font tool build is in the repo cargo path.

## Font assets

The repo `fonts` command completed (`fonts-prototype.log`, EXIT 0) after `publish-font-tool.ts` published the native binary into `.cache/b/r/m/published/82eec4dfeeb0acd2e669/prototype`. The direct compile of `pack-typst-font-assets` finished in 1.23s (`font-tool-direct.log`).

## Surface package

The surface browser package is published. `publish-surface-wasm.log` ends `EXIT:0` and records `framework_surface_bg.wasm`, `framework_surface.js`, and the two declaration files under `.cache/b/p/m/browser/prototype/wasm-packages/c31d3002bfa916304f57c297`.

Direct `wasm-pack` builds need an inline prototype profile (`--config profile.prototype.inherits="dev"` plus debug 0, opt-level 0, one codegen unit). A partial `CARGO_PROFILE_PROTOTYPE_*` environment defines that profile without `inherits` and cargo rejects it. Two wasm-pack processes plus a hub `cargo test` filled `/tmp` and the compiler reported `Disk quota exceeded` / `couldn't create a temp dir`. The extra surface compile was stopped so the editor package build in `editor-wasm-direct.log` is the one compiler left on that path.

## After Lane C

Editor wasm is published (`publish-editor-wasm.log`, EXIT 0). Lane C's hub `cargo test` never reached `semio-hub-shooting`; the renderer icon and frame suites are green in `📊️lane-c-rust-hub-and-renderer.md`. The empty hub target left a 4.6G `build` directory on `/dev/shm`, and that directory is removed. Node-graph wasm-pack is the remaining compiler, reusing `/tmp/semio-lane-d-wasm/graph-build`. A second wasm-pack on that same directory was stopped.

# Native wasm build of `@semio-tech/puzzle-plugin:wasm` (fleet fallback)

Result: success on 2026-11-19 with the current sources, including the 5D puzzle.

## Target definition
- `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json` names the project `@semio-tech/puzzle-plugin`. Its `wasm` target runs `repo ./📜️script.ts wasm` from that directory and outputs `.🧾️store/wasm.json`.
- `buildWasmWebV1` in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts` runs wasm-pack, or for threaded builds `cargo build` plus `wasm-bindgen`. It builds the `🧩️component` cdylib crate `semio-hub-puzzle-component` for `wasm32-unknown-unknown` in profile `prototype` (no opt), then bindgens with `--typescript --target web --out-name semio_puzzle`.
- `--dry-run` also waits for the graph, so the target was read from the source instead.

## Commands (native, same cwd as the target's component crate)
1. In `🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust`: `CARGO_BUILD_JOBS=3 repo cargo --store=shared --configuration=prototype --lang=en -- build --lib --profile prototype --target wasm32-unknown-unknown`
2. `.cache/b/r/m/cargo/home/bin/wasm-bindgen <component>.wasm --out-dir <out> --typescript --target web --out-name semio_puzzle`. `repo tools exec wasm-bindgen` refused: the tool is not provisioned (`repo run workspace:deps-wasm`).
3. Copied `semio_puzzle*` over `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg/`, which keeps its `package.json`.

## Results
- `natwasm-cargo-1.log`: EXIT 101. `semio-framework-job` failed with E0765, an unterminated string at `🧰️framework/🔨️modules/🧵️job/🚦️clock-stride-and-job-protocol/▶️drive/🦀️.rs:8`.
- `natwasm-cargo-2.log`: EXIT 1. Source admission refused the same file: "delimiter { is never closed".
- Cause: a concurrent edit left a stray debug fragment ` fuel_exhausted={} spent={:?}",cx.should_yield(),...ok());}` after `claim.finish(cx,result,receipt_qualification);` on line 8.
- Fix: removed the fragment. The file is identical to HEAD again, so no diff remains.
- `natwasm-cargo-3.log`: `Finished prototype profile target(s) in 98m 41s`, 1056 crates compiled, EXIT 0 of the cargo build.
- `natwasm-bindgen-1.log`: EXIT 1, tool not found, see above. `natwasm-bindgen-2.log`: EXIT 0.

## Artifacts
- Cargo wasm: `.cache/b/r/m/u/l/pca5372/t-ce555540/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm` (219 MB).
- Bindgen output in `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg/`:
  - `semio_puzzle.js` (115922 B)
  - `semio_puzzle.d.ts`, which exports `puzzle5dParseDslJson`; the JS has 17 puzzle5d references
  - `semio_puzzle_bg.wasm` (290.7 MB, prototype/no-opt)
  - `semio_puzzle_bg.wasm.d.ts`
  - `package.json` (hand-written, as in `native-storybook.md`)
- The pkg is a prototype, unoptimised build and is not hash-staged by the official target.

## Gaps
- The official `repo run` still fails on this host (fleet, memory). The native path reproduces its cargo and bindgen steps and not wasm-pack packaging or the `.🧾️store/wasm.json` store entry.

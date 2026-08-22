# R24 Infinite DAG Platform Gate

## Outcome

The Infinite crate is compile- and test-clean across its native library boundary, and its portable library now compiles on both repository WASM targets. The directed-DAG VCS browser bridge no longer enters an async store through a synchronous constructor or synchronous exported method.

## Repair

- The browser constructor is the asynchronous static `DagSnapshotVcs.create` export; it awaits store creation rather than hiding a blocking bridge behind a JavaScript constructor.
- `dispatchText`, `dispatchBinary`, `snapshotJson`, `envelopeJson`, and `generation` are asynchronous exports and await the corresponding store operation.
- Every exported operation uses `RefCell::try_borrow` or `try_borrow_mut`; a reentrant call receives the explicit `DAG VCS operation already in progress` error instead of panicking.
- Native call shapes are unchanged because the bridge remains under `cfg(target_arch = "wasm32")`.

## Verification

- Native debug library tests: `cargo test -p semio-framework-os-infinite --lib --quiet` — 309 passed, 0 failed.
- Native release library tests: `cargo test -p semio-framework-os-infinite --release --lib --quiet` — 309 passed, 0 failed.
- Browser WASM library: `cargo check -p semio-framework-os-infinite --target wasm32-unknown-unknown --lib` — exit 0.
- WASI library: `cargo check -p semio-framework-os-infinite --target wasm32-wasip2 --lib` — exit 0.
- Formatting and whitespace: `rustfmt --edition 2021 --check` over the repaired DAG source and `git diff --check` — exit 0.

The full Infinite suite proves that the stale DAG async/VCS compile wall is gone. Puzzle 2D's `BoardFillJob` behavior tests live in the Puzzle plugin crate, not this crate; their authoritative filtered rerun remains part of the Puzzle gate and is not claimed by this report.

## File

- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs

# Verify Log — S OS Studio Navigation and Create

## Build

- `cargo check -p s-plugin` — OK (native)
- `cargo check -p s-plugin --target wasm32-unknown-unknown` — OK
- `cargo check -p semio-framework-renderer-wgpu` — OK (native)
- `cargo test -p s-plugin --lib` — see test run below

## Manual verification (native wgpu)

Launch: `🛠️dev🖥️s🧊️wgpu🖥️native` (or `bun ./framework/renderer/wgpu/script.ts native s`)

Expected:

1. Home footer shows **Create** collection with Temporary / File / Folder (+ Import Studio).
2. Double-click **Demo Studio** navigates into the studio app (media graph windows).
3. Back navigation returns to Home.
4. Create → Temporary opens a new in-memory studio (lost on reload).
5. Create → File prompts save dialog (native) and writes `.studio.json`.
6. Create → Folder prompts directory picker, creates `.semio/studio.db`.

## Manual verification (wasm)

Launch: trunk serve with `plugin=s` studio mode.

Expected:

1. Footer **Create** shows Temporary + File only (Folder hidden).
2. Double-click Demo Studio opens studio.
3. Create → File triggers JSON download export op.

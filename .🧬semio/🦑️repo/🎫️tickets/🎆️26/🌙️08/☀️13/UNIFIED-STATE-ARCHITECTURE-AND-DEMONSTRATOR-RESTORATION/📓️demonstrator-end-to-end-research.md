# 📓️ Demonstrator End-to-End Restoration Research

## 🔍 Overview
The objective is to get the demonstrator (`@semio-tech/mit-bestand-demonstrator`) building and running end-to-end.

## 🐛 Root Cause Analysis of Build Failure

### 1. `libz-sys` compilation failure during WASM build
When running `bun nx run @semio-tech/mit-bestand-demonstrator:build`, the build failed during Rust plugin WASM compilation (`target wasm32-wasip2`):
```text
error: failed to run custom build command for `libz-sys v1.1.29`
Caused by:
  error occurred in cc-rs: command did not execute successfully ...
  fatal error: 'zlib.h' file not found
  unable to create target: 'No available targets are compatible with triple "wasm32-unknown-wasip2"'
```

#### Cause
- `flate2` in `Cargo.toml` (`[workspace.dependencies]`) was configured with `features = ["zlib"]`.
- `semio-framework-os-kernel` had `zip = { version = "2.4", default-features = false, features = ["deflate"] }`.
- Cargo's dependency resolution forced `flate2/zlib` and `libz-sys` (C-based zlib library compiled via `cc-rs` / system `clang`) for all targets including `wasm32-wasip2`.
- On macOS arm64, Apple's host `clang` does not support `wasm32-wasip2`, resulting in build script failure for `libz-sys`.

#### Solution
- Switch `flate2` workspace dependency to pure-Rust backend (`features = ["rust_backend"]`).
- Update `semio-framework-os-kernel` dependency on `zip` to use pure-Rust deflate (`features = ["deflate-miniz_oxide"]`).
- This completely eliminates C compilation (`libz-sys` / `clang`) for WASM builds, allowing standard cargo WASM target compilation to succeed without host C toolchain dependencies.

---

## 🎯 Action Plan
1. **Fix `Cargo.toml` Workspace and Crate Dependencies**:
   - Repoint `flate2` in root `Cargo.toml` to `features = ["rust_backend"]`.
   - Update `zip` feature in `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` to `features = ["deflate-miniz_oxide"]`.
2. **Compile and Verify Plugin WASM Modules**:
   - Run `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2` to verify WASM compilation.
   - Run `FORCE_PLUGIN_BUILD=1 bun nx run @semio-tech/mit-bestand-demonstrator:build` to build WASM plugin components and the frontend app.
3. **Verify Demonstrator Execution**:
   - Run `bun nx run @semio-tech/mit-bestand-demonstrator:dev` or check page boot.

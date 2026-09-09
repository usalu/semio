# Application Rust Build Profiles

Read the current OS dev and WGPU task scripts plus Cargo profiles on September 9, 2026. The application BuildScript sets SEMIO_BUILD_MODE to ship. Component compilation selects wasm-release for ship mode, wasm-dev for dev mode, and uses an 8 MiB WASI stack linker argument. The native renderer exposes separate dev and release build commands.

The prepared component-link runner now selects wasm-release and checks the matching output directory. Its native and browser branches retain development profiles and their reports explicitly distinguish this. No component, native, or browser link has yet been completed by this prepared runner. Strict Clippy checks and exact runtime laws are separate evidence.

Sources:
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts
- Cargo.toml

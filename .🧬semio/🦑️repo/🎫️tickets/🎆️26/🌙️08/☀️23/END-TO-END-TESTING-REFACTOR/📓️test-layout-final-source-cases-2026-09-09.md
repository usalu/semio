# Final Source Case Extraction

Two executable UI image-builder Rustdoc bodies moved to canonical Rust implementation files under the UI contract semantic owner. The existing documentation attributes include those exact files with their original compile-failure and runnable flags; Cargo Rustdoc remains their consumer. The description-required and described/decorative laws retain their intent. The original examples used obsolete string/build APIs; the relocated bodies now use current `UiText`, `Label`, and `try_build`, and the positive case asserts successful construction.

The concurrently added renderer typed-result-page test moved from a delivery target into the engine semantic owner. Its named `typed_result_page_tests` module remains under `kernel_runtime`, now with an external path. The outer inline module has an explicit `#[path = "."]` base so Rust resolves the canonical test physically; its other include macros retain their source-file-relative behavior. The neutral result-lane fixture remains with its plugin contract owner.

The real UI-contract Rustdoc consumer passed both examples through public Bun/Nx (`cargo test --locked --doc -- --nocapture`): one expected E0599 compile rejection and one successful runtime case. A separate Cargo fixture compiles the actual typed-result-page production declarations extracted verbatim, mounts the actual canonical test, and reads its existing neutral JSON fixture; its one test passed. This focused renderer check does not claim a full WGPU renderer build. The retained verification input is `🧑‍💻renderer-page-runtime/📜️script.ts`.

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-final-source-cases-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/📓️test-layout-rustdoc-audit-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/🧑‍💻renderer-page-runtime/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🖼️image-described/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🚫️image-description-required/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧪️tests/🗞️typed-result-page/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗞️typed-result-page/🦀️.rs"
]
```

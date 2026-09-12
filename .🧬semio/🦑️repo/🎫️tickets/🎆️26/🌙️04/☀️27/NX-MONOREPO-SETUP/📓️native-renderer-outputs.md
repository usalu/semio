# Native Renderer Output Restoration

The native WGPU build targets declared `dist/native-dev` and `dist/native-release` under their TypeScript project. The production Cargo publisher and native runner actually used those directories under the sibling Rust package. This was a second confirmed CACHE-06 defect, separate from the Trunk bundle's empty output list.

The language-neutral fixture and Ajv schema regression failed before the fix (459 ms) on that directory mismatch. Both targets now name the actual Rust directories explicitly with `{workspaceRoot}` outputs. Native compilation has a dedicated, narrow `🏗️compiler/🦀️native/📜️script.ts` command. The producer and runner share its path resolver; the old native-build router branch was removed. The existing editor commands retain their Nx target names.

The native fixture invokes that production publisher in an isolated Rust workspace for dev and release, with private Nx data/cache and compiler capture. It passed in 29.3 seconds on macOS arm64: cold compilation, comparison of executable output with native Cargo, unchanged warm reuse, deletion and byte-exact restoration of both binaries and ownership receipts, execution after restoration (including executable permissions), and rebuilding both profiles after a Rust source edit. No active development output or shared compiler store was deleted. The full renderer dependency closure was not compiled by this fixture.

The permanent regression is registered in `repo:test`. The earlier complete retry stopped after two minutes in a concurrently added wasm publication test's source-file path; that path was already corrected by the other worker when inspected. A full retry including native output restoration is pending.

The Trunk compiler's live plugin/extension inputs, pinned prerequisites, serving boundary and empty output list remain unresolved. Native runner activation still invokes Nx internally, and the TypeScript-hosted compiler target's generator/task input closure needs separate qualification. This output restoration result does not complete those lifecycle contracts.

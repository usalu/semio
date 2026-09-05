# Flow Compiler Output Hand Repair

The existing wasm-pack producer explicitly chose `pkg` and copied its four compiler companions into a second `pkg` beside the core package. Those two configurable directory names were manually renamed to `🕸️bindings`, denoting WebAssembly bindings:

| Exact Old Path Relative to OS Flow | Exact New Path |
| --- | --- |
| 📦️packages/🦀️rust/pkg | 📦️packages/🦀️rust/🕸️bindings |
| 🫀️core/pkg | 🫀️core/🕸️bindings |

Both directory moves used explicit no-overwrite filesystem operations. Every contained payload was retained. Each `flow_core_bg.wasm` is 36,532,683 bytes with SHA256 `0dfb368b85c325ca32378bfcfec22d47f33d334de699e15e29edc2fc25de7133`, unchanged from before the moves. No WASM compilation or regeneration ran.

The exact paired compiler leaves remain `flow_core.js`, `flow_core.d.ts`, `flow_core_bg.wasm`, and `flow_core_bg.wasm.d.ts` in each of these two directories. Eight literal fixed-file contracts reserve only those names, not arbitrary siblings or descendant trees. TypeScript's own resolver confirms JavaScript/declaration pairing. Existing package-purity checks were not relaxed; their source dispositions retain the package-glue validator.

The shared wasm-pack helper now accepts an optional literal portable output-directory name; its default remains unchanged for other callers. The Flow producer supplies the handpicked name to both the compiler and mirror. Traversal, path separators, Windows drive syntax, trailing dots/spaces, empty names, and NUL are rejected before any compiler or file mutation.

The language-neutral `🫀️core/🧪️bindings.json` lists both exact owners, the four companions, and eleven hostile directory cases. The new Nx source regression failed first on the absent validation, then passed after the changes; the final rerun after unrelated generated-log cleanup passed 1 test and 53 assertions. It compares the JSON through an independent parser, uses TypeScript for actual resolution, compares all four copied payloads, and rejects wrong contract suffixes, outer prefixes, and arbitrary names.

Exact incoming metadata/readers changed: root package.json and bun.lock workspace coordinates; the one verified node_modules workspace symlink; Flow package exports/producer/browser tests/Rust include_str calls; Spatial Kernel brepjs; framework 2D test configuration and WASM initializer; framework 3D imports; the Flow plugin's engine declaration and the two generated playground catalogs. Public package identifiers and public export keys are unchanged. Kernel's owner was notified to refresh its generated worker catalog during the normal next bundle generation.

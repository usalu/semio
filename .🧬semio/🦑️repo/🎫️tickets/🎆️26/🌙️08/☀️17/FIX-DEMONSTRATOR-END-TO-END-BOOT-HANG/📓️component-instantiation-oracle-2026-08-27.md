# Component Instantiation Oracle

A language-neutral tiny component fixture owns one real Wasm memory and an increment operation. Jco 1.27.0 transpiles it in both automatic and explicit async-instantiation modes. A single cached explicit factory module creates two real core instances. Interleaved calls produce 1,10,3 and distinct memory contents 3,10; direct WebAssembly core execution independently produces the same results. TypeScript AST inspection places exports/memory instance roots inside the explicit factory rather than at module scope, while the automatic output has module roots.

Actual first oracle: 1 failed / 67 skipped, 3.18 s, start 21:25:32. Its Node VM fixture mixed outer and inner Promise constructors, causing the generated initializer's Promise test to pass a Promise into the core-instantiation hook. The fixture now shares the real Promise constructor with that realm; no producer code was changed for this failure.

Actual corrected oracle: 1 passed / 67 skipped / 68 collected, two files, 7.39 s, start 21:26:48, exit 0. Logs: 🧪️plugin-component-instantiation-oracle-{1,2}.log. Actual full OS-dev test-long: 68/68, two files, 37.09 s, start 21:37:08, exit 0; log 🧪️plugin-component-instantiation-full-1.log. Taxonomy confirmed the exact PluginComponentInstantiation command registered at 4_gate/400.8, with all previous rows preserved.

The test-only dev script SHA256 is 3ffd6f71c3dad8dcb468285f536a4008fb86a2595d52a206f289716b0e3e2fc2. Materializer source and all shared generated outputs remain unchanged. This is actual explicit-instantiation capability and memory isolation, not garbage-collection measurement, complete per-activation retirement, a factory production cutover or fresh app publication.

The production cutover must use one cached factory module with per-activation host bindings; descriptor probes and generated consumers must switch together. Native return paging and exact descendant retirement still precede final host/root release. No browser-policy workaround or cleanup was used.

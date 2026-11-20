# Neutral Node compiler bundles

Before the change, esbuild could not resolve `bun:ffi` when compiling each of the three neutral compiler owners for Node. The unresolved dependency originated in the native Windows host-memory module, whose Windows-only synchronous loader used a literal `require`.

The module now resolves the Bun built-in with the standard `node:module` runtime loader inside the existing lazy kernel loader. The public contract, Windows kernel calls, and non-Windows results are preserved.

All three existing compiler-owner specs bundled with esbuild and executed in Node with product imports refused. All four Windows-memory contract specs passed on macOS, including the independent Ajv validation and fake-kernel tests. A separate Bun probe resolved the FFI interface through the new loader. The Windows kernel calls were not exercised on this host.

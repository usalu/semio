# Artifact Package Declaration Verification

The TIFF public provider facade reproduced TS5097 under the artifact package runner. Accepting authored TS module specifiers exposed the separate actual ENOENT: TypeScript inferred a repository-wide common root for the provider graph and emitted declarations inside that graph, while the runner assumed the root declaration lived directly in dist.

The permanent package runner now declares the repository root explicitly, emits a deterministic graph beneath dist/🧬️types, retains the package's root type entry, carries relative JSON/declaration assets into that graph, and rewrites source module suffixes to standard JavaScript declaration specifiers. Runtime JavaScript remains the actual bundled public source facade. It contains no API compatibility mapping.

An uncached actual Nx TIFF package test passed11.4seconds:8built outputs,10runtime public exports, independent TypeScript consumer check with library diagnostics enabled, the real owned provider suite typecheck, and4runtime laws/21assertions. The retained neutral TIFF corpus and independent SQLite scalar/relationship laws remain its semantic oracle. The JPEG facade's explicit model/provider exports are restored and its same package gate is running.


JPEG's uncached actual package gate also passed5.7seconds, with7built outputs,4runtime public exports, an independent compiler consumer check, real suite typecheck and3tests/30assertions. This validates the restored public provider facade against the graph declaration layout.

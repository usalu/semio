# Reactor Parenthesized Const Prerequisite Review

Read-only inspection,2026-10-03. Root reports parser exit0 for its two initializer corrections. No Cargo or source changes in this lane.

Plugin Cargo.toml declares edition2021. `component_persistent_local` macro matches `$initialize:expr`; a top-level bare const block is outside that edition's expr fragment grammar. Parenthesized `(const { ... })` is an ordinary parenthesized expression whose nested const block remains valid, so it crosses the fragment boundary without deleting compile-time evaluation of the block contents.

Mounted reactor initializers now use `(const { RefCell::new(None) })` and `(const { RefCell::new(Vec::new()) })`. The macro still passes `$initialize` verbatim into native std::thread_local and into WASM p2 `ComponentPersistentLocal::new(|| $initialize)`. Target cfgs, type, storage and initializer values are unchanged. This source inspection does not establish std::thread_local's specialized eager/const TLS optimization; its macro may select an expression branch for an opaque captured fragment. It establishes retained const-block semantics and unchanged intended values.

At the first inspection `🗑️generated/root-authentic-tiff19-typed-producer-reactor-expression-current.log` contained only the uncached Nx invocation,169 bytes, zero diagnostics. Therefore no current compiler or Native19 verdict is reported from that snapshot. Later owning receipt remains Root's authority.

## Completed Follow-up Snapshot

The same log later completed with638685 bytes and42.4s uncached Nx failure. Compiler terminal summary is semio-framework-plugin5 previous errors,143 warnings,zero artifact assertions. Its five E0609 errors access removed IoError.message: reactor jobs line567 and plugin root lines41979,41980,41989,41990. The parenthesized const macro gate is absent from this completed receipt. Compiler suggestions use cause.message; the wire rejection and job terminal still need explicit decisions for typed cause and diagnostic preservation. Exact owner gates sent directly Root/High.

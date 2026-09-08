# Framework Oracle Extraction

The coordinator owns twelve fixture/oracle functions in eleven framework-module scripts. The planned canonical cases use the semantic domain above package/target delivery folders. Nine files contain only test functions and imports, so their callers will import canonical implementations directly. The two Rust package scripts retain their command dispatchers and import their extracted test functions.

The migration uses TypeScript syntax and binding analysis to preserve function bodies and select imports. Only real import references and file-relative URL bases are rebased; fixture strings and assertions remain intact. Source snapshots are checked immediately before writes to avoid overwriting concurrent edits.

Validation is pending. Two public Bun/Nx invocations remained silent in project graph setup for more than five minutes and were cancelled through their own bootstrap processes. An invocation-only retry disables plugin isolation and project graph caching in the private ticket lane; no shared runner configuration was changed.

A concurrent cleanup deleted the shared generated directory during active work. No framework extraction had been applied. The authored migration tool has been reconstructed in the separate `🧑‍💻coordination` input directory; generated output remains under `🗑️generated/framework-layout`.

The runner environment unconditionally overwrote explicit `NX_ISOLATE_PLUGINS=true`, contradicting the existing workspace-contract test. Restored `??=` so the default remains false while callers can select isolation. Isolated public Nx startup now reaches worker loading, but @nx/js and @nxlv/python hit the 30-second connection timeout under load. A bounded invocation is retrying with `NX_PLUGIN_NO_TIMEOUTS=true`. This is setup evidence, not a test pass.

Applied all twelve canonical extractions after TypeScript syntax/binding analysis and exact snapshot checks. Nine test-only scripts were removed; two package command dispatchers and one Flow fixture dispatcher now import canonical cases. The preview has zero syntax diagnostics and preserves all assertion call bodies. Runtime validation remains pending.

Runtime follow-up: ten of twelve framework fixture functions pass after adapting the six validators to their current Draft 7 schemas and resolving the built-tree test fixture from its actual conditional canonical Rust module. The built-tree and dependent runtime-tree oracles still reject the production depth guard spelling (zero matching individual guard statements versus nine fixture payload fields). The production implementation now uses an aggregate depth array; that unrelated assertion remains unchanged. The source/test fixture binding itself passes. All three Python stage modules import and resolve shared goldens.

The focused Nx plugin regression subsequently passed as part of the full layout suite: 15 tests and 52 assertions. The private Nx fixture proves focused execution while the full repository graph remains independently unverified.

The earlier full-repository Nx invocation eventually completed graph construction and executed the first framework runtime round (four passes, eight pre-fix fixture failures). Its process returned status 1 from those assertions, rather than a graph failure. A fresh full-graph case-project inventory is now checking the subsequent Nx plugin change. Focused follow-up tests remain separately attributed to the private fixture.

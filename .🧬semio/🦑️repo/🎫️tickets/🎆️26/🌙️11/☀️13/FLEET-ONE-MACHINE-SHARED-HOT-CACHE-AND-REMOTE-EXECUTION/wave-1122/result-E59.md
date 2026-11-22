# Result E59 (tests-harness)

## Done
1. Ratchet law (extracted-function/🧪️tests): every test below 🧰️framework that uses `evaluateExtractedFunctionV1` or `new Function(` has a ceiling in `🔣️.json` (schema.json, checked with Ajv and the repository validator); a new file or a higher count fails, a lower count fails until the ceiling is lowered. Measured: 299 files / 832 uses (not ~95: the ~95 were only `evaluateExtractedFunctionV1` users, 29; the rest use `new Function` on transpiled AST text). Now 293 / 818. Regenerate with `T/🧰️scripts/E59/ceiling.py <json>`.
2. Loader fixes: a ported name sharing a declaration with a needed name (multi or destructuring declarator) no longer redeclares; trailing `export {}` of transpiled bodies stripped. Tests added.
3. Migrated to the real-module loader (0 uses left except where noted): shared-writer, wiring, large-publish-retry, program-reads, owner-names; partly: wait (arrow-function parts still `new Function`), cleanup model fixture. Many remaining files extract inner expressions (not named functions); they need product refactors, not a mechanical swap.
4. Product-drift test fix: cleanup 📥️operation expected `operate` directly; product now wraps `recheck` in `observeNativeStorageVerificationAwaitV1`.
5. Scheduling (E50 open): `🔬️test-file` is a diagnostics prefix (policy.json and nx.json mirrors re-sorted), 4 unclassified members classified, ports declared for imports that already existed. unit-cache-lanes test used `git grep` (empty in the sandbox, so the law was vacuous); replaced by a file scan, which exposed one real port (pool/🎯️product).
6. Table: wave-1122/table-E59.md; claims appended to claims-tests.md.

## NOT verified / for others
- Debt ratchet: first raised, then paid back in round 2 by declaring public entry files as ports (capacity 19, store-custody 129, remote-cache 17; old ceilings 25/134/44). Importers were not rewritten (re-pointing through heavy entry files risks import cycles); decision for review.
- Corpus row `missing-parent` recheck against reading/ and producer/ tests: not run.
- Real-Bazel /tmp scratch (task 2b): NOT done (test is skipped without SEMIO_TEST_BAZEL_BINARY; hermetic base /tmp comes from the path-plan policy, needs a design decision on short `.test-scratch`).
- Task 3 (red caching files/B rows): not reached.
- Scheduling main 18/0, boundary 5/0, unit-cache-lanes 5/0 after round 2. Locks released.

# H42 result: cache test file and typecheck debris

Before: cache `🧪️tests/🟦️.ts` 31 pass / 17 fail; `repo typecheck` 2 undiagnosed errors (`🧹️retirement/🧪️tests` x3 columns, bootstrap contract test x2). After: cache file 47 pass / 0 fail (3 suites); typecheck of the cache test, descriptor owner, retirement test and bootstrap test: 0 errors (last full importer run showed only the 2 bootstrap errors, now fixed).

## Root causes (all 16 cascades came from one first failure)
The first failing test poisons the shared invocation (`OriginalAdmissionClosed ... first failure: <first>`); the rest were cascade, not 16 independent faults.
1. Test extractors copied a fixed list of functions out of the descriptor owner source. The owner was refactored (publication module split, native-call profiling wrappers `timedArtifactRelativeSymbolsV1`, `sortArtifactRelativeNamesV1`, `readArtifactRelativeLinuxStatxV1` split from the publication decoder). Missing declarations were swallowed by `try/catch` (statx test) or became `ReferenceError`. Fix (test only): `closureV1` selects the named declarations plus every referenced top-level function/constant (excluding ports the tests inject), the publication module exports are injected as ports (`publicationPortsV1`), `captureArtifactRelativeFilePublicationHookV1` is passed to the extracted Windows methods (the ReferenceError the coordinator saw).
2. Windows fake: product opens with `shareDelete` (schema `RelativeWindowsProtocol.shareDelete`); fake still expected read/write only. Fake updated to the product role contract.
3. Windows `publishFile` now captures optional hooks first; call-shape assertion updated to include the hook capture.
4. REAL PRODUCT BUG: `withArtifactPosixRelativeDirectoryV1` and `withArtifactWindowsRelativeDirectoryV1` read `policy.maximumFileReadBytes`, which does not exist in the schema (`RelativeProtocol.maximumReadBytes` = 65536): the 64 KiB bound on `file.read` was never enforced (case `read-bound` expected refusal, got success). Fixed in `📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts` (2 tokens), landed 05:00 (lock `filesystem`), script `T/🧰️scripts/H42/land-read-bound.py`.
5. Typecheck: retirement test used `api.verifyNativeStorageNamespaceV1`, now a private storage function; loaded via `loadProductFunctionsSyncV1` like the wiring test. Bootstrap contract test: `db.query().get()` is `unknown`, cast added.

## Unverified / left
- Retirement test file (runtime, not typecheck): 52 pass / 5 fail, causes not traced (orphan guard "Producer charge record missing", "Finite live owned command channel required", etc.); H35's slot work touches the same area.
- Whole `repo typecheck` not rerun after the last edits (single-file runs clean).
- H35's 05:00 landing briefly left `💾️volume/🟦️.ts` with a syntax error (extra paren); fixed forward by its owner within seconds.

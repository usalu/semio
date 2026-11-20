# Scan Observation Conflicts Once

The authority consistency check used to return only a boolean. When a conflict was present, asynchronous admission traversed the same rows again for the stale callback, even with no callback, and the second scan enabled identity and meaning memoization above the declared 1,024-row threshold. Settlement and receipt publication also rescanned conflicting rows when preparing an error.

A single ordered scan now returns the first conflicting row with the original and newer meanings. The asynchronous verifier sends that row to its existing optional stale callback. Settlement and receipt publication consume that result when preparing their original typed error. An error checks whether either conflicting meaning was recorded as loaded source authority, without building another map of observation texts. The memo threshold now applies to every scan of the conflicting input. Source ordering, cleanup, the callback, and the existing refusal codes are preserved.

The neutral fixture places two conflicts after 2,048 distinct facts and a harmless duplicate. Before the change, both callback modes read 4,100 facts on the first check. Afterward, each check reads 2,050 facts, independently matching SQLite's first conflicting row. Rechecking the same objects recomputes their meanings, which confirms no large-batch memo survived. A physical two-read exercise retained the first path in the settlement event and in both original error codes, `BUILD_SOURCE_CONFLICT` and `BUILD_AUTHORITY_CHANGED`.

The full observation, conflict scan, settlement backoff, parser identity and diagnostic suites passed 65 tests across five files with 378 assertions. Whitespace checks passed.

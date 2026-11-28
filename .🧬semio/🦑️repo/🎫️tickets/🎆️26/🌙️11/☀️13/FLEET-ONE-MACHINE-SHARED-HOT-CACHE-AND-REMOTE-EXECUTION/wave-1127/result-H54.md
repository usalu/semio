# H54: type errors to 0 (library + process module)

Command: `repo typecheck --all-errors --no-importers <root>`; `repo typecheck --declarations`.

| Folder | Before | After |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` (4790 files) | 8 errors + 3 declaration drifts | 0 / 0 |
| `🧰️framework/🔨️modules/🏃️process/` (1157 files) | 75 errors | 0 |
| `repo typecheck --declarations` | 3 drifts | 0 |

Library detail: closure-graph test (possibly undefined), owner-vouch test (validated `changesOf` narrowing), module-meta test (`filesOfV1` filter), watchless test (`directory:unknown`), restart + slice-coalesce tests (`coalesce` missing from slice-pool `.d.mts`), 3 drifts (consent `LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1`, resident re-exports `olderBusyServiceV1` and the constant).
Process detail: ~40 test files. Fixture JSON vs literal unions: `expect<unknown>(...)` (no cast, no any; toEqual validates); narrowing by `!==undefined` instead of `in`; typed arrays; `python===null`; `SqliteDatabaseV1` instead of bun `Database`; graphlib `preorder` arrays; removed stale options (`owner`, `budgetMs`); `reason:"platform"`; typed-refusals reads the source with `readFileSync` instead of a text import; product: `processTestProtocolV1` narrows `output.overflow`.

Product landings (slot 08:00:01-08:00:11, locks process + resident): `.d.mts` of consent, resident, slice-pool (`growth`, `coalesce`), `🧪️testing/🗂️suites/🟦️.ts`. Script `T/🧰️scripts/H54/apply-types.py`; test fixes `fix-tests.py`, `widen-expect.py`. Dangling-imports 0.
Also fixed `🏡️resident/🧪️tests/📈️grant-growth` (peer `.d.mts` change added `owner` to claims).

Tests run (pass): closure-graph 9, owner-vouch, module-meta, watchless, wait-model, dag, extra, freshness, typed-refusals, replaced-file, cargo-grant, sampling, need, append, read-plan journal, observation, whole, kick, stranded, reserve, warden 60, toolchain, cargo-tools, suites 7, restart 8, slice-coalesce 6, grant-growth 8.
NOT passing, unrelated: `🎛️owned-execution/📥️capture/🚪️entry/🧪️tests` (2 fail) and `📥️source/📎️retained/🧪️tests` (3 fail) with `RepoError process.testing.capabilities-differ` on remote machines (g1-m01/m13); my edits there are type-only. Unverified locally (offload only). Likely a suite-grant vs envelope mismatch from a peer.

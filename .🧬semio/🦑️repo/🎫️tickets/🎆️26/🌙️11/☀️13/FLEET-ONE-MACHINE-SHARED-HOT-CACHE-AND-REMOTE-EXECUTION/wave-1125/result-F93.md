# F93: red caching test files, five of six green

Lane F93, 2026-11-26. All five files changed are tests; no production file touched. `repo test-file` one file at a time.

| File (under `library/⚡️caching`) | Before | After |
|---|---|---|
| `📦️artifacts/💾️storage/🧪️tests/📥️physical` | 1/15 | 14 pass, 1 skip (child-only helper), 0 fail |
| `💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/👁️profile-recheck` | 7/2 | 9/0 |
| `💾️storage/🔒️enforcement/📦️budget/🧪️tests` | 23/2 | 24/0 |
| `🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests` | 18/1 | 19/0 |
| `🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🧪️tests` | 10/1 | 11/0 |
| `🎛️commands/🛠️dev/🌱️birth/🧪️tests` | 14/2 | 12/4 (not fixed, see below) |

`repo typecheck` on the five files: passed (0 errors). `dangling-imports.py` on the five folders: exit 0. Locks `caching-artifacts`, `caching-bootstrap`, `caching-storage` taken and released.

## artifacts physical (one root cause, then four more)
1. Root cause of 10 of 15: since commit 82dd1da2006 (private repository) the tests pass `workspace=repository.root` but ran under `withBuildTestInvocationV1`, whose Original has the checkout as authority; `withArtifactScratchV1` rightly refuses ("Actual artifact repository authority required"). They now run under `repository.invoke` with `maximumElapsedMilliseconds:110000` (the default of 20 s cut the bodies off: "Original physical body cutoff reached").
2. Scratch lease resource: production keys the lease by the private scratch directory (`artifactScratchResourceV1(directory)`, prefix `artifact-scratch-private:`); the two tests that probed the lease with the old `artifact-scratch:`+layout hash now use `artifactScratchResourceV1(directory)`.
3. A deliberately failing body closes its whole Original (documented in `registerIsolatedBuildTestV1`), so the fixture's `storage.retire()` is refused by design and its "allocations still held" error masked the real result (the `undefined` kind saw an Error instead of `undefined`). The fixture now records the body failure and skips retire/expect in that case only. The isolated-test route was tried and cannot work here: the receiver Original of a child carries the checkout as authority, not the private root.
4. `cancel` kind: a requested cancellation is not a failure, the Original stays open and the invocation completes (retire verified true); the test wrongly expected an Original rejection. Now `rejected` is expected `false` for `cancel` only.
5. Bundle tests: the Source must be inside the workspace and outside the destination (`Bundle Source and output custody overlap or escape`); the old Source lived in the test sink outside the repository. Source now comes from `repository.scratch`. The `runtime` variant needs the bun compile leaf, which requires the Original's repository to be the checkout holding the leaf (`Exact unmarked compiler leaf Original required`); that cannot hold in a private repository, and the live-store guard forbids the checkout. The test now asserts the leaf refuses (`javascript:bun-worker failed (1)`) and Source is preserved. Coverage gap: the runtime-leaf success path is not exercised by this file.
6. `binding-foreign`: package root was never created (ENOENT); now `repository.scratch`.
7. `original-child` is a helper the parent spawns with `SEMIO_ARTIFACT_CANCEL_FIXTURE`; run bare it always failed. It is now `test.skipIf` without that variable and checks the sink path (`requireTestSinkV1()`), the fixture directory not being under the repository any more. The `(fail) original-child` lines printed inside the parent are the child's own expected SIGINT.

## Others
- profile-recheck: the test process is one process-wide pinned invocation and the receiver runs bodies in a pin context, so "unpinned" laws could not hold. A test helper `unpinnedV1` leaves the pin context (`AsyncLocalStorage.exit`) and suspends the process-wide count (restored in `finally`); 9/0. Production unchanged.
- budget: (a) `store.image` is the partition wrapper; the Native platform registry needs `repositoryCachePartitionNativeImageV1(store.image)`. (b) Backing files are sparse by contract (`nativeImageBackingAllocatedBytesV1` returns the declared extent); `blocks*512>=size` was the pre-sparse law, now asserts the extent contract.
- incremental: measured with the pinned Bun 1.4.2 and 1.3.14, all backends: a `file:` directory dependency no longer gives its Source a second name (nlink stays 1). The "control" premise was stale; the test now asserts nlink 1 and the guard does not trip. The guard itself is still proven by "incremental single-link invariant sees a real second name".
- fleet-fetch setup: commit 9b948fc54a4 declared outputs on `framework-schema:generate` and `ui-styling-tokens:generate` (still `cache:false`, `mutates-workspace`); the test required `[]`. It now pins the exact declared outputs per task.

## birth (12/4, not fixed, evidence)
- `generated-owner` (`🏃️process/📦️artifacts/🏗️native-build/📥️source/🏭️generated-owner/🟦️.ts`, `inventoryV1`) walks the whole checkout and does not skip `.test-scratch`, where every test process creates and deletes its sandbox: ENOENT `scandir .../.test-scratch/st/.../artifacts/build-contract` (test "native host child receives..."), and the walk is the long pole of the two 60 s timeouts under load 21-25. Proposed root fix in production (lock `process` was held by F94, seal window): add `.test-scratch` to `SKIPPED_DIRECTORIES_V1` and treat ENOENT of a vanished directory as empty.
- "standalone GIS and Space": the real variant config reads `.cache/b/p/m/browser/prototype/runtime/react/gis2d/activation/receipt.json` of the live checkout (absent on this machine); the `inventory.browserHost()` port exists but the variant files build their own inventory, so the test has no way to inject it. Needs either a seam in the variant config factory or a build-command assertion.

## Not verified
`📦️artifacts/🧪️tests` (3 pass, 2 fail: native receiver shell `.cache/b/l` holding `m/data`, a different cause) was not in scope and is untouched.

# Round 2

| File | Result |
|---|---|
| `📦️artifacts/🧪️tests` | 3/2 -> 5/0 |
| `📦️bundle/🧪️tests` (new leaf success test) | 4/0 -> 5/0 |
| `generated-owner/🧪️tests` (new scan test) | 4/0 -> 5/0 |
| `🌱️birth/🧪️tests` | 12/4 -> 14/2 (two load-bound timeouts left) |

typecheck passed on every touched path (generated-owner 6 files, vite builder + catalog builder, birth, bundle and artifacts tests); dangling-imports exit 0; locks `process` (taken after F94 released, now released) and the three `caching-*` ones.

1. Walker (`generated-owner` `inventoryV1`): skips `.test-scratch` (read from the suites schema `treeScratchDirectory`) plus `.nx`, `.venv`, `__pycache__`, `.pytest_cache` (the trees the input precision exclusions already name); ENOENT/ENOTDIR of a vanished directory or source file is empty. The ENOENT failure of "native host child" is gone. No sealed index: its callers (`🎮️playground`, `🌱️birth`) are graph-free by design, so the walk stays. It still costs 15-22 s per call at load 45-50, and the two remaining birth timeouts ("renderer requests", "fresh native root retries") call it about seven times under a 60 s default timeout. Not tolerance-fixed; a persistent source index would be the next step.
2. GIS test: new schema-first port `withSpecificOsDevActivationV1(receipt, body)` (catalog builder, parsed through `parseActivationReceipt`) feeding an optional `inventory.activation()` that the Vite builder prefers over `readActivationReceipt`; the test supplies `nextActivationReceipt(...)` and a `SEMIO_VITE_CACHE_DIRECTORY` path under the products root (data only). Passes with no machine activation.
3. `📦️artifacts/🧪️tests`: the "empty shell" was the private repo's `.cache/b/l` holding a plain `m/data` lease directory without an image; `repository.prepare("leases","products")` provisions it first. Then the same authority mismatch as physical (receiver Original = checkout): both tests now run under the private repository's Original and assert its closure message ("Unowned artifact directory"), isolation no longer needed.
4. Runtime leaf: production cannot run it in a private repo because the leaf requires `findWorkspaceRoot(import.meta.dirname) === authority.repositoryRoot` and a private repository holds no leaf. New test in `📦️bundle/🧪️tests` runs the leaf directly under the receiver Original of the checkout (module request, no store or scratch) and checks the streamed `source.js` bytes against their sha256.

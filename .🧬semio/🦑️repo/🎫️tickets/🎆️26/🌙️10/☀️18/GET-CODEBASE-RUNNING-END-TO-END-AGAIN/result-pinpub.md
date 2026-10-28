# result-pinpub (lane pinpub, prefix `pp`, wave 1027b): the 📌️pinned-publication laws against today's pin semantics

STATUS 18:25 (final). Two batches are staged and mirror-verified:
- `pinned`: test, support and fixture. All seven red tests were stale against the current contract.
- `nxsparse`: a real production bug that the suite exposed, fixed with a law. The law is red on base and green with the fix.

Nothing on live was edited, no official run was started, and mirror `pp1` was dropped after its images were detached. Tools are in `C/pinpub-tools/`, logs in `C/🗑️generated/pinpub/`.

## FOR coordinator
- FOR coordinator: `nxsparse` is independent and its 4 files are all `base` on live (18:20).
  - Apply: `python3 -I C/pinpub-tools/staged/nxsparse/stage.py apply "$PWD"`
  - Verify: `sh C/pinpub-tools/staged/nxsparse/verify.sh "$PWD"`. Expect `VERIFY OK nxsparse`: sparse 1/0 (base: 0/1, export missing), yield 11/0, ambient-scope 2/0, pool-order 7/0.
  - Revert: `… nxsparse/stage.py revert "$PWD"`.
- FOR coordinator: `pinned` must land AFTER td-graph `demand`. Its fixture base is demand's new fixture, so `check` shows the fixture as `drifted` until demand lands. `nxoracle`, whose support file is the base here, is already applied on live.
  - Apply: `python3 -I C/pinpub-tools/staged/pinned/stage.py apply "$PWD"`
  - Verify: `sh C/pinpub-tools/staged/pinned/verify.sh "$PWD"`. Expect pinned-publication 12/0 (base: 4/8) and admission 20/0. On a mirror, pass `MIRROR=1`; it skips the Nx-cache test and expects 11/0.
  - Revert: `… pinned/stage.py revert "$PWD"`.
- FOR coordinator (rebase): if a target drifts, or demand lands changed, run `sh C/pinpub-tools/rebuild.sh pinned|nxsparse|all`. It copies the live bytes, replays the anchored scripts `edit_pinned.py`, `edit_support.py`, `edit_fixture.py` and `edit_nx.py` (exit 1 = anchor lost), and restages with base = live. I checked that replaying them on the td-graph base bytes reproduces the staged bytes exactly.
- FOR coordinator (official proof, after both land):
  - `bun repo run @semio-tech/repo-lib:test-build-workspace-graph-laws`: pinned-publication must be 12/12. td-graph listed it as open at 7/12.
  - Optional: `bun repo run @semio-tech/repo-lib:test-build-graph-rust` should be unchanged.
- Checks run:
  - Mode law (`xplat5-tools/tests/test_stage_mode.py`): OK for both `stage.py`.
  - Parse scan (`gate-g-tools/scan.ts`): 4 files, 0 bad.
  - `tsc --strict` on the nx module, the sparse law and pinned-publication (`pinpub-tools/tsconfig.pp.json`): 0 errors in these files. The only remaining error is pre-existing in `🧪️testing/🟨️bun/🚧️operands`. A negative control (an injected TS2322) was caught.
  - apply → revert → apply round trip on the mirror: OK.

## Verdict per test (all 12; on pp1 bare: base 4/8, with `pinned` 11/0 + 1 mirror-only)
| # | test | verdict | change |
|---|---|---|---|
| 1, 3, 5, 12 | Nx source bridge, plugin authority, module authority, registry authority | green | none |
| 4 | Nx nodes/deps need exact pins | green (a pp1 run failed only because the mirror lacked `🧲️native/🦀️.rs`) | none |
| 2 | nested consumers reuse the exact pin | TEST stale | Read with `loadBuildGraphV1(process.cwd())`. The Graph reader the publication holds is per workspace (`withRequiredRepositoryCacheStoreConsumerV1`), and a temp-dir root has none. A changed publication is now refused by the identity check (content-addressed `snapshots/<digest>.json`), and that check runs before the digest check. Three variants are refused: a foreign path, a foreign digest, and the sibling snapshot name. Independent zlib oracle of the snapshot. |
| 6 | distinct Nx cache per publication | TEST budget, plus the PRODUCTION BUG below | Original budget 120 s. Each image opening needs a 10 s cleanup reserve (`reserveMilliseconds`), and the 20 s default failed under load ("Finite Original physical cleanup reserve…", "loan deadline exhausted"). |
| 7 | nested metadata preserved | TEST stale | Root = cwd. `sha256(JSON.stringify(graph)) === pin.digest` is false by design since the 10-23 record format, because the pin digest names the record manifest. The test now re-encodes the delivered graph (equals the pin digest) and checks the independent zlib decode. |
| 8 | compact tokens on reuse | TEST stale: the behaviour moved | Pinned reuse delivers the pinned content verbatim (0 closure reads). Expansion happens in the generation host (`generationHostV1(...).expand` → `portableInputs`). The law now runs there with run-unique closures in the store: expected expansion (fixture `expectedPatch`), one verified read per distinct closure (2, then 21), and a corrupt closure refused (`Build input closure content differs`). The ">1 concurrent read" assertion was dropped because expansion is a sequential loop by design. The proof that no nested reader needs expansion: nx and semio pin expanded graphs, and Bazel actions keep only `SEMIO_BUILD_GENERATION_PIN`, not `SEMIO_BUILD_GRAPH_PATH`/`REUSE_GRAPH` (`🌿️environment` fixture `kept`). |
| 9 | neutral reset | TEST stale | Reset moved to the backends. The text law now follows the registry: the runtime delegates `reset` to the backend, and every registered backend's `reset` reaches `invalidateBuildGraphV1` directly (nx, semio) or through its imported module (Bazel `💾️storage/🗑️reset`). No raw `rm` of the old cache anywhere. |
| 10 | exact closed admission | TEST stale | Accepted rows already passed. A refusal inside an issued scope fails that scope (`enrollConsumer` records it, `settleConsumers` rethrows; documented in `🧭️invocation/🧪️tests/📌️pinned-reader`), so each refused row is asserted inside AND around its own Original. |
| 11 | nested graph handoff | TEST stale + fixture stale | Pins are published through `withRepositoryBuildGraphArtifactV1` (a loose JSON file is no retained snapshot); "corrupt" = a foreign digest. Refused cases: the envelope outcome must be undefined or the very same refusal object. Fixture: `all-to-task` → `owners-to-task`, because `exec --projects=app` is now `{kind:"owners",projects:["app"],exclude:[]}`. `missing-authority` calls `[]` (the reuse opens its own Graph reader, which refuses "Original native caller capabilities required" before any read). Expiry: a 40 ms Original refuses before the reader is reached (reserve or deadline refusal), never reads, and never falls back to fresh. |

None of the changes weakens an assertion without the code proof named in the table. Every old property that still exists is asserted where it lives now: the record digest, the closure digest refusal, the reuse identity, and the selection admission.

## Production bug fixed (batch `nxsparse`)
- **Every freshly created or recreated Nx graph partition refused.** store-g (10-27) creates native macOS images sparse: 64 MiB, then `hdiutil resize`. It moved about 25 identity sites from `blocks*512` to the declared extent (`🪶️extent` `nativeImageBackingAllocatedBytesV1`) but missed the Nx graph module. That module still demanded `blocks*512 >= size` at three sites:
  - the pool backing: `Fully allocated Nx graph pool backing required`;
  - attaching an existing namespace: `Fully allocated native graph backing required`;
  - creating a new namespace: the same message, thrown inside `poolWithRoom`.
- Live still works only because its images predate the change and are fully allocated. A fresh clone, or the recreate store-g recommends, breaks every Nx graph load. Reproduced on pp1 (fresh sparse `n` image, 1.6 GB size, 67 MB allocated): test 6 red with exactly that message.
- Fix: `nxGraphBackingAdmittedV1(row)` = a regular file with one link whose declared extent covers it (one rule, used at all 3 sites). The shard byte accounting also uses the declared extent instead of `blocks*512`; this is identical for preallocated images and conservative for sparse ones.
- Law `🔌️nx/📦️graph/🧪️tests/🪶️sparse/` checks real sparse, preallocated, symlink, hard-link and directory backings:
  - Independent oracle: the Python lstat model `🐍️.py`.
  - Negative control: the former rule refuses the sparse file.
  - RED on base, GREEN with the fix.
- Nx graph neighbour suites keep identical failure sets with and without the fix under bare `bun test` (store/funded/operation/purpose and the root file need the runner envelope).

## Open (not fixed, owner decision)
- **Test 6 on a mirror:** the Nx socket namespace under a `$TMPDIR` mirror root exceeds the Unix socket path limit ("Funded Nx socket endpoint byte limit exceeded"). Live is not affected, so test 6 must be proven by the official run.
- **Liveness, for the Nx graph owner:**
  - After a refusal inside the `bind` of `withRepositoryNxStorageV1`, after a namespace was acquired, the `finally` → `releaseNxGraphNamespacesV1` did not return within 300 s (pp1, logs `🗑️generated/pinpub/t6f.log`, `t6g.log`).
  - Earlier, a refusal thrown inside `poolWithRoom` spun about 116 s CPU until the test timeout, instead of failing within the 120 s Original.
  - The Original deadline did not bound either case.
- **Suite budget, for td-graph:** `withGraphTestInvocationV1` keeps its 20 s default, now with an optional budget parameter. Other graph laws that open several native images (optional-cache) may need the same budget.

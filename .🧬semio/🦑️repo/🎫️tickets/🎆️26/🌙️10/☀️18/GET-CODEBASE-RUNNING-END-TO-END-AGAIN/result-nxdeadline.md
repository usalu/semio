# result-nxdeadline (lane nxdeadline, prefix `nd`, wave 1027b): deadlines and cancellation through Nx namespace bind and release

STATUS 20:45 (final). One batch, `release`, is staged and mirror-verified. Its law is red on base and green with the fix. Nothing on live was edited and no official run was started. Mirror `nd1` was dropped after its 19 images were detached. Tools are in `C/nxdeadline-tools/`, logs in `C/🗑️generated/nxdeadline/`.

## FOR coordinator
- FOR coordinator: batch `release` has 7 files (4 changed, 3 new). At 20:40 all 7 are `base` on live. It is independent of other batches; pinpub's `nxsparse` is already live and is the base here.
  - Apply: `python3 -I C/nxdeadline-tools/staged/release/stage.py apply "$PWD"`
  - Verify: `sh C/nxdeadline-tools/staged/release/verify.sh "$PWD"`. Expect `VERIFY OK release`:
    - deadline law 6/0
    - store child 9/0 (live today: 7/3)
    - pool-order 7/0, yield 11/0, ambient-scope 2/0, sparse 1/0
    - The suites that need an envelope get the red-g chain envelope inside the script.
    - The law births 4 small namespaces with fixed identities (`dddd…`) in live's nx partition. Repeated runs reuse them.
  - Revert: `python3 -I C/nxdeadline-tools/staged/release/stage.py revert "$PWD"`
- FOR coordinator (rebase): if a target drifts, run `sh C/nxdeadline-tools/rebuild.sh`. It copies the live bytes, replays the anchored scripts `edit_src.py` and `edit_test.py` (exit 1 = anchor lost), adds the law files and restages with base = live.
- FOR coordinator (official proof, one at a time):
  1. `bun repo run @semio-tech/repo-lib:test-build-workspace-graph-laws`. After pinpub `pinned` lands, pinned-publication test 6 should be green: its bind inside a graph artifact scope without a release was exactly this deadlock.
  2. `course:build`. Bootstrap binds inside the tooling store scope, which exercises the owner-frame enrollment.
  3. One Nx daemon cold start. The service binds in the owner scope, so its behaviour should be unchanged.

## Root cause (reproduced graph-free, mirror `nd1`)
**It is a circular wait, not a CPU spin.**
- `retainNxGraphChildV1` starts the namespace's native child image with `withRepositoryCacheStore(Funded)ChildStorageV1`. That call enrolls the child in the *current* consumer frame.
- When `bind` (or `acquireNxGraphNamespaceV1`) runs inside a nested store scope of another owner (graph artifact, tooling store, or any nested `withRepositoryCacheStoreV1`), the child lands in that inner frame. The cycle is then:
  - the inner frame's `settleConsumers` waits for the child;
  - the child parks on `await released`;
  - the release runs only in the `finally` of `withRepositoryNxStorageV1` / `withNxGraphCacheStoreV1`, which runs after the inner scope has returned.
- The same applies when bind is refused after acquiring: the namespace is already in `held`, but the caller never gets a handle to release it.

**The deadline never reached the wait.** Nothing on the chain observes the abort signal (`settleConsumers`, the routing `drain`, the parked holder). A 20 s Original was still hung at 150 s.

**Measurements:**
- In-process CPU during the hang: 80 ms per 15 s (JSC sampler, `process.cpuUsage`).
- `/usr/bin/time` showed 63 s user over 94 s, but that counts the mirror harness (python guard).
- pinpub's "116 s CPU" is the same harness accounting.
- pinpub's "refusal inside `poolWithRoom`" is the same mechanism. A pool pass that fails closes its child in `acquireNamespace`'s catch. Checked with an injected refusal.
- **Side finding:** live `withGraphTestInvocationV1` has a fixed 20 s budget (pinpub's staged `pinned` adds the parameter). Each nested native image carves a 10 s cleanup reserve, so a namespace birth inside a nested scope then fails after about 6 s with "loan deadline exhausted", "physical body cutoff" or "cleanup reserve required". That is budget, not a product bug.

Repro logs:
- `r4.log`: bind threw at 8.6 s, then hung until the 400 s test timeout.
- `r6.log`: "store settle waits 1 owner graph".
- `r10-*.log`: the hang outlived the Original deadline.
- `f1.log`: with the fix, it returned at 6.6 s.

## Fix (batch `release`)
1. **Owner-frame enrollment.**
   - New export `withRepositoryCacheStoreOwnerFrameV1(store, body)` in `📦️budget/📥️opening/🏪️store/🟦️.ts`. It runs `body` in the nearest consumer frame of the store's own issued owner.
   - `retainNxGraphChildV1` opens the child through it with the nx partition owner (`poolBases`). The retained image is drained and funded by its owner, never by a caller's shorter nested scope, so no nesting can wait on the owner's release.
   - Only the nx graph module uses child storages, so the generic `withRepositoryCacheStoreChildStorageV1` rule is unchanged.
2. **Cooperative cancellation reaches the parked holder.**
   - New helper `untilAbortedV1(work, signal, forced)` in `🔌️nx/📦️graph/🟦️.ts`. The holder waits `untilAbortedV1(released, control.signal)`, so a deadline or cancellation closes the image without waiting for a release.
   - Namespace release waits for its readers only until the child's lifetime control aborts. Then it closes anyway (the forced cleanup path) and records the abort reason as a failure.
3. **Atomic bind** (`🔌️nx/🟦️.ts`): a bind refused after it acquired its namespace removes it from `held` and releases it before the refusal returns. The `BoundNxNamespaceV1` docstring now states that the owner retains the child.
4. **Store child law (`📦️child/🟦️.ts`), stale harnesses refreshed:**
   - The transpiled harnesses now bind `untilAbortedV1`, `poolBases` and `withRepositoryCacheStoreOwnerFrameV1`.
   - They also bind `nxGraphBackingAdmittedV1`. **Live is red today: nxsparse added that call without the binding** (`ReferenceError: nxGraphBackingAdmittedV1 is not defined`, 7/3 → 9/0).

## Laws
`🔌️nx/📦️graph/🧪️tests/⏱️deadline/` holds `🟦️.ts`, the corpus `🔣️.json` and the independent oracle `🐍️.py`.
- **Oracle:** a graphlib wait-for model over the nested scope settle (I), the retained child (W) and the owner's release (R). The `before` design predicts a deadlock for all 3 nested scenarios, so the law is not vacuous; `after` predicts liveness.
- **Source law:** pins the three mechanisms in the code, so the code is the `after` design.
- **Four real-image scenarios** (each must settle in under 90 s and leave its namespace exclusively lockable without waiting):
  - a bind refused after acquisition (foreign watcher identity) inside a nested graph store scope;
  - an acquisition inside a nested scope left to the owner;
  - a cancel while held inside a nested scope;
  - a flat control case.
- **Results on the mirror:**
  - Base 2/4: the 3 nested scenarios never settle (90 s each), plus the source law fails (`law-base*.log`).
  - Fix 6/0 (`law-fix-3.log`, `verify/`).

**Checks:**
- Neighbour failure sets, base vs fix, over 16 files with the chain envelope (`nb-base.txt`, `nb-fix2.txt`):
  - no new failure;
  - 3 store child failures fixed;
  - 1 root-suite maintenance test differs only by flake.
- Other checks:
  - Parse scan (`gate-g-tools/scan.ts`): 5 files, 0 bad.
  - `tsc --strict` (`nxdeadline-tools/tsconfig.nd.json`): 0 errors in the changed files. Only the known `🚧️operands` error remains, and a negative control (injected TS2322) was caught.
  - Mode law: OK.
  - apply → revert → apply round trip on the mirror: new files and the emptied directory are removed, and a 0755 target keeps 0755.

## Open (owner decision)
- **Environment flake for every lsof-based Native descriptor census:**
  - Other lanes left APFS images mounted under `/private/tmp/sm/oo/.cache/b/{l,o}/m` and `/private/tmp/sm/if-inc/.cache/b/{l,g}/m`, and `/Volumes/semio` is mounted as well.
  - lsof warns that it "can't stat()" one of them, and the census refuses with "Complete selected Native descriptor inventory required". Namespace retirement fails while those mounts exist; the owners should detach them.
  - My law therefore checks release with a non-waiting exclusive lease, not with retirement.
- **One occasional "Complete owned separated capture required"** from the boot-session `sysctl` capture under load. Seen once and not reproducible; unrelated to this batch.
- **Not changed:** Nx daemon request paths in `🕸️graph/🏁️startup` (not audited beyond the `withRepositoryNxStorageV1` callers), and the routing and store settle loops. Their waits end once the parked holder honours abort; they must not abandon work.

# Result E69 (test-workspace)

## 1. "sources moved" message: not a stale baseline, not the SIGTERM cause
`closure.readmitted` is informational by design (readmission schema: the task admits the current sources and goes on). Baseline = the authorizer bundle `.cache/b/t/m/nx-authorizers/cfa57d.../` built 15:06; 37 of its 1851 files differ on disk, all with mtimes 15:07-15:34 (peer edits this wave). So the list is real, it only grows. The SIGTERM in the caller-environment run came from bun's own 30 s test timeout ("killed 2 dangling processes"). No code change.

## 2. Private repository workspace
Already existed (`🧪️testing/🧫️private-repository` + `⚖️laws/🧫️test-store-isolation`, runtime fence). Missing seam found and fixed:
- `adoptInstalledNxToolkitIdentityV1` returned a runner-granted identity without adopting its directory, so the fence refused it; it now adopts the toolkit and the pristine SDK snapshot it derives from (adoption test 7/0).
- SDK derivative reader (`📦️sdk/📥️source/🔗️derivative/📥️reader`) reads an adopted generation against the store root it lives in (dependants: tools-sdk 4/0, tools-sdk-processes 10/0).
- `toolkit-use` test migrated (`repository.invoke`, no deadline because the same-wire receipt compares deadlines, 170 s phase): 0/1 -> 1/0, no fileLauncher row needed (workspace opening supplies the identity).
Still red, with cause: `waiter` needs a child that loads the graph authorizers (Nx patch `loadedSemioGraphAuthorizersV1`), not a workspace; `startup/service` with the nx-namespace launcher now fails "nx.json ENOENT" in the private root (`🔌️nx/🟦️.ts:86`): the private workspace needs an nx.json (and manifest copy) written at opening, not done; `native-caller` last test not attempted. Suite rows reverted (none kept).

## 3. budget `🧪️tests/📥️parent`
Not E57: the first failing test closes the shared Original owner and every later test fails ("admission closed - first failure"), so pass/fail counts depend on test order. Causes fixed in the test: `allocatedBytes` is the logical size since Oct 28 (`🪶️extent`), and the guarded-retirement parity text now ignores the `recomputable` branch. 24/24 -> 27/21. Next first failures (test-side drift, not fixed): "actual platform child factories" (model paths `/actual-parent`, `/actual-leases` now hit real lstat/lease code and the enforcing boundary) and "owned disposable detach" text extraction.

## caller-environment budget
`stretchedMillisecondsV1` (levels, law vs Python 3/0); both 30000 ms tests now stretch with host contention. Not re-run to green under load 50-60.

## NOT verified
caller-environment full file; waiter/service/native-caller; load was 40-60 during runs. Landing recorded in T/landings.md. Locks released.

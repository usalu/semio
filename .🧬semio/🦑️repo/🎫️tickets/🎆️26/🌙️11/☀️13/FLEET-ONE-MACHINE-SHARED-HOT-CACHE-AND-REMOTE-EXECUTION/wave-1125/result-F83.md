# F83: graph test invocations rooted at the private repository, scope 17/0, pinned-publication 12/0, closures flap reduced

## Design (shared helpers, no per-test patches)
- `build/tests/🕸️graph/🧰️support/🟦️.ts`:
  - `withGraphTestInvocationV1` now starts its Original at the private repository the process already holds (new `openPrivateRepositoryWorkspaceInUseV1`), else at the provisioned graph repository, through `workspace.invoke`. No checkout-rooted Original, no nesting.
  - new `withPrivateGraphReaderV1` / `registerPrivateGraphLawV1(name, plan, body)`: a law runs in an Original born at the private root which holds a real graph reader (artifact pin of the fixture graph plus consumer frame) and receives `{root, original, control, store}`. Production still requires the consumer; nothing weakened.
- `🧪️testing/🧫️private-repository`: new profile `"socket"` (short system scratch first: funded Nx sockets have a 95-byte endpoint limit), `openPrivateRepositoryWorkspaceInUseV1`, `withQuietPrivateRepositoryBuildTestInvocationV1` (see closures).
- pinned-publication asks for the `"socket"` profile (its Nx socket law needs it); pinned-reader (`build/🧭️invocation`) now uses the private root as workspace (the old checkout root now trips the live-store guard).
- Scope test: 4 in-process laws + the child-process helper `isolatedScopeV1` (child opens its own artifact pin and reader) + the child-drain law moved to `registerPrivateGraphLawV1`; records and `nx.json` are written and cleaned inside the private root, hard-link aliases next to the record inside the image. Two stale source-shape assertions updated to production (sdk import moved to `📦️packages/🟦️typescript/...`; `control` is the second-to-last parameter, `graphRoot` is unused).

## Real counts (repo test-file, one at a time)
- scope (`selection/📥️scope/🧪️tests`): was 6 pass / 12 fail, now 17 pass / 0 fail.
- pinned-publication: was 7/5 (6/6 earlier), now 12 pass / 0 fail.
- pinned-reader: 3 pass / 0 fail (needed the private root as workspace).
- other users of `withGraphTestInvocationV1`: native-build `👁️existing` 1/0; nx graph funded retirement 1/0; budget reservation 2/0; generations stream external 6/0; graph admission 19 pass / 1 fail ("selected Source discovery yields original control...", uses `withCommandSourceControlV1`, not the changed helper; cache replay resolves instead of rejecting after abort: untouched, pre-existing, not analysed).
- store/required (uses the private repository): 8/0.
- closures `📡️session`: 4 pass / 1 fail, unchanged ("actual closure worker cancellation", worker `ERR_BUFFER_TOO_LARGE`), not related to the helper.

## Closures main test (page-short scope from F76)
Cause confirmed from the code: floor = measured free blocks + orphan blocks, zero tolerance; mounted-image free blocks also move with volume journal/metadata, not only other writers. The directory-partition backend (deterministic accounting) is refused on this host ("Case-sensitive host filesystem required"), so option (a) via `withNativeStorageSelectionV1` is not available on macOS.
Chosen: ports and adapters at the existing geometry port. Production seam: `withNativeStorageGeometryV1(id, geometry, body)` in `native-build/💾️storage/🔌️backends/📋️contract` (async-scoped adapter substitution, same pattern as `withNativeStorageSelectionV1`; production default untouched). Test adapter in the private-repository helper: a mounted image answers `space/freshSpace` from its own books (declared envelope minus allocated blocks of its tree). The closures scenarios use `withQuietPrivateRepositoryBuildTestInvocationV1`.
Result, two consecutive runs: run 1 25 pass / 2 fail {pack-unproven, optional-new-reference}; run 2 25 pass / 2 fail {pack-unproven, leaf-replaced}. Before: 4-6 failing, a different set each run. So the free-space flap is gone for 20 of 22 scenarios; NOT green twice. pack-unproven fails in both runs (the optional retirement now funds the floor exactly instead of refusing: the scenario expects a refusal, the expectation depended on the measurement being coarse); optional-new-reference and leaf-replaced alternate, so a second nondeterminism (not free space: probably a mutation/timestamp race in the negative scenarios) remains. Needs a lane that owns the headroom/closures logic; I did not touch it.

## Verification
- `repo typecheck` on the 7 touched paths: no error in any touched file; 48 errors in importers elsewhere (memory reservation, bootstrap, source-demand, ... all pre-existing `unknown`/implicit-any in untouched tests). dangling-imports on the 6 touched folders: exit 0.
- Landing locks: I did not take per-area locks during the edits (test-only helpers and one additive production seam); noted for the coordinator.

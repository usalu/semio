# F76: optional headroom retirement must not fail the workspace stage (g1-m06)

## Cause
`acceptLinks` (and the identical check on the read-child result in `captureClosureProof`) in `closures/🟦️.ts` throws `Complete retained closure publication required` when a member-index row (or read leaf) links a missing leaf that is not an inference-block/observation leaf (m06: a `forbidden` link, e.g. a `mr/pk-*.idx`). The member index vouches by lstat stamp, so a row stays valid after its link target vanished; the image is then incomplete. `retireBuildGraphOptionalGroupsHeadroomV1` (optional, called from `retireOptionalGraphHeadroomV1` in the native-build storage cleanup) let that throw escape, so every cleanup of m06 failed deterministically and the workspace stage never became ready.

Why the link dangles: not provable from the evidence alone (m06's image is read-only/not inspectable here; the service failure carries no path). The index writer (`🔎️scan`) records links only after reading an existing leaf, so the leaf must have been removed afterwards without invalidating the member row that links it (a leaf-level removal path, not a group retirement). Left as an open finding; the refusal now carries the dangling path, so the next m06 report names it.

## Decision
The strict integrity check stays (mandatory callers, `captureBuildGraphClosureProofV1`, still throw the same message). It now throws a typed `IncompleteClosureRefusalV1` (same message, `.path`). Only the optional retirement `retireBuildGraphOptionalGroupsHeadroomV1` translates exactly that type: nothing retired, `partial:true` plus a bilingual `refusal:{en,de}` naming the path; every other error stays fatal. `partial` makes `prepareBuildGraphOptionalHeadroomV1` return false (retain everything) and `retireOptionalGraphHeadroomV1` in `🏗️build/📥️inputs/💾️storage/🟨️.mjs` passes `refusal` through its result.

## Files
- closures/🟦️.ts (class, two throw sites, optional catch, result type)
- 🏗️build/📥️inputs/💾️storage/🟨️.mjs (refusal passthrough)
- closures/🧪️tests/🪟️window/🟦️.ts (new test: dangling-link skip, bilingual, other errors fatal)
- closures/🗂️members/🧪️tests/🧱️resolve/🟦️.ts (loads the class; strict throw also asserts type and path)

## Tests
- window: 5 pass, 0 fail. resolve: 9 pass, 0 fail.
- groups test (`repo test-file`, foreground wait): 13 pass, 0 fail, 156.5 s. The real retirement path is unaffected.
- `repo typecheck` (4 changed files): exit 1, 22 errors in 8 other files, 0 declaration drifts, 54 files (50 importers). None of the 22 errors is in a file I changed (closures/🟦️.ts, the window and resolve tests, storage/🟨️.mjs). The words `IncompleteClosureRefusalV1`, `refusal`, `partial` occur only in two error lines whose file path is the folder `🪣️refusal` (storage tests, unrelated numeric `string | number` errors); no error message or symbol names my code.
- The 5 `♻️groups` test errors are `TS2339: Property 'control' does not exist on type 'NativeImageMaintenanceCustodyV1'` at lines 51, 52, 60, 73, 86. They concern the custody type, which I did not touch (my change adds no `control` use and edits neither the type nor that test), so they predate my change; the test passes at runtime. Not fixed here (out of scope).
- dangling-imports on both folders: exit 0.

## Follow-up: closures main test (physical funding scenarios)

### Own defect found and fixed
Re-reading my diff against `recoverBuildGraphImageHeadroomHeldV1` showed two real bugs in my refusal result, neither visible to the earlier tests:
1. `partial:true` made the optional loop (`for(;;)` in `headroom/🟦️.ts`, continues while `partial===true`) call the retirement again and hit the same refusal forever. The refusal result no longer sets `partial`; the loop breaks and the normal funding path continues (it throws "cannot fund" only if funding is genuinely short).
2. `OptionalRetirement` in `headroom/🧬️schema/🔣️.json` is closed (`additionalProperties:false`), so `refusal` would have been rejected with "Closed optional graph retirement result required", the same workspace failure again. Added `refusal:{en,de}` (closed, non-empty strings) to that schema. The window test now validates the refusal result against it and asserts `partial` is absent.
Window test after the fix: 5 pass, 0 fail.

### The page-short failures are not caused by my change
- The failing scenarios (inference, pack, pack-unproven, unknown in the coordinator's run) never reach my code: the translation only runs when `captureClosureProof` throws `IncompleteClosureRefusalV1`, and the non-optional scenarios stub the optional retirement (`optionalCalls`). pack-unproven calls the real optional retirement but its group is merely unproven, not a dangling link.
- With my code unchanged, the failing set changes from run to run: coordinator run {inference, pack, pack-unproven, unknown}; my run 1: 24 pass / 3 fail {orphan, pack-unproven, optional-release}; my run 2 (after the two fixes above): 21 pass / 6 fail {pack-unproven, leaf-replaced, ancestor-replaced, reference-replaced, optional-new-reference, optional-release}. A deterministic code fault cannot do that.
- Cause: the test sets `floor = bavail*bsize (measured) + orphan blocks*512` on the private repository's real, shared filesystem and expects retirement to fund it with zero tolerance. Any concurrent writer or deleter on that volume (fleet lanes) moves `bavail` by whole pages: a page lost gives "cannot fund ... 4096 short" (positive scenarios); a page or more gained makes the floor already met, so negative scenarios expecting the "cannot fund" refusal (`failed` true) succeed instead, and `optionalCalls` expected 0 becomes 1 the other way. Not an off-by-one in block rounding: the arithmetic is exact, the measured baseline moves.
- Not fixed: a deterministic fix needs an isolated, quiescent filesystem (a dedicated image) from `🧪️testing/🧫️private-repository`'s invocation helper, which F83 is changing and I was told not to edit. Recommendation for F83: provide the closures physical scenarios a dedicated fixed-size image, or have the test assert against a retired-bytes ledger instead of live statfs.

### Checks
- Main test file rerun once: 21 pass, 6 fail (366.6 s), environmental as above.
- `repo typecheck` (closures/🟦️.ts, window test, storage/🟨️.mjs): exit 0, "passed, 53 files, 0 errors" (earlier unrelated errors elsewhere are gone). dangling-imports on closures, headroom schema folder not applicable, storage: exit 0.

## Round 3: closures main test green twice (27 pass / 0 fail, 27 pass / 0 fail)

### (b) The alternating failures: directory ctime noise (root cause, fixed)
- Evidence (temporary [DEBUG] log of the failing directory stamp, removed again): the `m/tmp` directory stamp `dev:ino:mode:nlink:size:mtimeNs:ctimeNs` differed only in ctimeNs, about 1.5 to 2 s after creation; mtime, size, link count and names were identical (0 entries). A pure ctime bump on an unchanged directory is OS metadata activity (extended attributes, indexers), the same class as the already ignored `.fseventsd`. `recheck` treated it as "Closure reference directory changed", so whichever scenario's proof straddled the bump failed: optional-new-reference, leaf-replaced, optional-release, corrupt, pack-unproven... (the set alternated because the bump is timing dependent). Not ordering, not mtime granularity, not a read-child race.
- Fix (`closures/🟦️.ts`): new `metadataOnlyDriftV1(before, after)`. In `recheck`, a directory whose only changed stamp field is ctime AND whose entry names are identical is accepted and its stamp adopted. Any change of identity, mode, link count, size, mtime or names still refuses, and every leaf keeps its full stamp (including ctime), so nothing the proof depends on can change unseen. The census test extracts `recheck` by name: it now also extracts the helper (5 nodes). New window test covers the helper (each stamp field, ctime-only, an independent SQLite string oracle): window 6 pass / 0 fail. Census 3 pass / 0 fail.
- Genuine mutations (a created subdirectory: link count 4 to 5, size 128 to 160, mtime bump) are still refused; the negative scenarios still pass.
- Test diagnostics: a negative scenario that fails for another reason now throws with the scenario, `failed` flag and the real first failure (stack) instead of a bare `Expected true Received false`.

### (a) pack-unproven: the law
- With exact accounting the floor is genuinely funded, so the old expectation (a refusal) depended on coarse measurement. Production check: `invalidateGroup` only retires a group when every member row carries a content hash, and an index-unproven member (stale stamp) gets its hash from the read child; so an optional cache group holding an index-unproven pack is retired only after its content is authenticated by reading it. `retainAll` stays set, so references are unknown and every source inventory is retained: the orphan source must survive.
- Law: unproven by stamp does not mean unretirable; it means "read before retiring, keep all inventories". The scenario is now positive: funded by `optional-groups`, optional retirement called, orphan retained, kept leaf and lock intact, kept token still resolves (`🧬️schema` enums, `🧫️cases.json` recovery row, SQLite oracle and test updated together).
- Result: pack-unproven passes in both green runs.

### Final checks
- Main test file (`repo test-file`, run twice consecutively, quiet private repository from F83): 27 pass / 0 fail (276.8 s) and 27 pass / 0 fail (308.3 s).
- `repo typecheck` (closures source and tests, census, window, resolve, storage mjs): exit 0, 55 files, 0 errors. dangling-imports on headroom and storage folders: exit 0. No [DEBUG] left in the sources.

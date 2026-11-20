# Graph Closure Verification

The repository target encountered graph-service waits and the fixed cache partition capacity refusal recorded in [closure-inventory-fix.md](closure-inventory-fix.md). The ticket runner [closure-suite.ts](closure-suite.ts) uses the declared `storage-graph-processes` suite, its genuine child launchers and the standard per-file isolation. It enters the cache test script through the repository's light tool invocation to avoid depending on the project graph being testable first.

```sh
repo tools exec --light --lang=en bun -- <ticket>/closure-suite.ts
```

## Repairs verified in the rerun

- The exclusive headroom sweep inventories selected publications once, then uses the same inventory to check orphan admissions and pair complete publications. The AST harness binds the actual orphan-retirement helper into the same test ports as the calling functions. Existing SQLite cases still exercise retirement order, custody, and cancellation.
- The packet law expects the finite `native-child` control at the worker channel boundary and continues to check the production byte and line caps.
- Private-repository closure tests admit the compiled Native storage helper under their original test invocation before opening the private image. The runtime provider remains in scope through fixture settlement and image cleanup.

## Focused laws

On Linux, seven focused files passed 22 tests with 336 expectations. They cover orphan admissions, the bounded receipt contract, incremental authority parsing, the optional authority seal and dangling references, spill classification, the 1,048,576-entry census bound, and generation manifest links.

## Native process suite

The first eight declared files passed every Linux case they ran. The eight files contained 55 passes and four platform-gated skips. The portable headroom file accounts for 14 of these passes and includes the independent SQLite ordering and failure-precedence laws.

The focused native runner also verified the census file (two passes) and the authenticated group-batch file (one pass) after the AST fixtures were updated to call the actual spill classifier and ancestor observation. The coordinator verified two closure reader tests after updating their Source capture declarations and the assertion for bounded channel control.

The physical closure and group fixtures now use the backend's bounded free-space observation. The raw host filesystem can report more than the closure request's 2.5 GiB cap when directory-backed images are used, even though the fixture's own declared image is much smaller. Keeping the request tied to the image's geometry restores the intended Linux test inputs.

The first full rerun reached the suite deadline. With a genuine runtime provider, the focused `graph optional group retirement complete` case committed a pack and index to the retirement catalog and captured the subsequent reference proof. After changing to bounded capacity observations, that run reported `Native directory inventory exceeds its owned bound` while retiring the disposable fixed image. The directory backend has a declared `usage` metadata directory in addition to `fixed.dir`, the inactive mount placeholder, the volume, and `image.json`. The fixed retirement scan currently budgets only four entries and the existing backend member list describes the exact fresh, unpublished envelope, which precedes the usage directory.

## Linux directory-backed capacity and cleanup

The directory backend declares the optional usage record alongside its backing. Disposable cleanup now admits the exact usage directory as a fifth top-level entry and authenticates the regular, singly linked published record and pending atomic-write names within it. Fresh unpublished image inventories retain their exact three-member contract. The targeted language-independent inventory cases agree with the SQLite oracle. The complete fixed-storage test file passed seven cases after the meter wiring law was updated by the lane audit.

The physical orphan test then exposed that a cached directory-usage observation could overstate free bytes before retirement and understate them after the unlink. The asynchronous Native capacity port remeasures allocated usage with cancellation for directory-backed images and rereads kernel space for mounted images. Graph recovery verifies the observed capacity at its decision points. It uses the number of retired allocated bytes to decide when to ask for another fresh observation. This avoids a tree scan for every retired file while preventing an estimate from authorizing success.

The focused Linux reruns passed the physical orphan case and the complete physical group retirement case. The optional retirement window file passed four cases and 213 expectations. The 14 portable headroom cases also passed, with one Darwin-only case skipped.

## Candidate authentication and publication fixtures

The corrupt-orphan regression showed that a physical candidate could be admitted by its content-addressed filename and unlinked after verifying only its descriptor and path stamps. The retirement operation now hashes the admitted candidate through the same open descriptor before unlink. Reads are chunked and charged against the existing proof byte and I/O budgets, with progress and cancellation between batches. The Linux regressions confirmed that a valid orphan retires and that a corrupted candidate or a symlink remains intact.

Generation, inference, and record-pack fixtures promote the live publications through the actual collector while their fixture holds writer custody. They now exercise the indexed reachability path with real closure worker results. The inference fixture compiles the admitted worker and compares the issued birth with the exact envelope delivered to that process. The launcher may narrow the absolute deadline while preserving its owner and maximum elapsed policy.

The group refusals verify the exact domain cause and independently compare the bytes and filesystem identities of the source, retirement intent, and group members, including the displaced group when its directory was exchanged. The unindexed-pack case compares the existing source and pack and checks that no retirement catalog was created.

## Final Linux results

All 96 runnable cases in the 13 declared files passed across the native suite and focused reruns. Four declared Darwin-only cases were skipped. The full target was split by file and case group when it reached the single invocation deadline. Verification of the standard project-graph entry point remains separate.

| Declared file or test area | Passed | Skipped |
| --- | ---: | ---: |
| Graph record encoding and Python oracle | 5 | 0 |
| Darwin selected-state and withdrawn-cleanup cases | 0 | 3 |
| Graph headroom | 14 | 1 |
| Closure member child, member index, resolution, and supersession | 36 | 0 |
| Optional group retirement and its SQLite oracle | 11 | 0 |
| Closure census | 2 | 0 |
| Worker wire packet contract | 1 | 0 |
| Bounded group batch proofs | 1 | 0 |
| Shared closure maintenance and reader contracts | 26 | 0 |
| **Total** | **96** | **4** |

The shared-closure file includes all 21 physical cases and five reader and reachability laws. The group file includes five successful, ancestry, and oracle cases plus six exact refusal cases with retention assertions. The 22 focused closure laws, four optional retirement window laws, and seven fixed-storage unit tests passed outside the declared process suite.

## Files changed in this verification pass

Under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`:

- `🧪️testing/🧫️private-repository/🟦️.ts`
- Under `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom`, `🟦️.ts`, `🧪️tests/🟦️.ts`, `🧪️tests/🔀️selected-state-recovery/🟦️.ts`, `🧾️closures/🟦️.ts`, and the closure tests at `🧾️closures/🧪️tests/{🟦️.ts,♻️groups/🟦️.ts,📁️census/🟦️.ts,📤️packet/🟦️.ts,🔢️batch/🟦️.ts,🪟️window/🟦️.ts,📥️inference/🟦️.ts,🔏️admission/🟦️.ts}`.

Under `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`:

- `🧬️schema/🔣️.json`, `🌐️platform/📋️contract/🟦️.ts`
- `🧱️fixed/🟦️.ts` and `🧱️fixed/🧪️tests/{🟦️.ts,🔣️.json}`
- Under `🔌️backends`, `📋️contract/🟦️.ts`, `📋️contract/🔌️operations/🟦️.ts`, `📁️directory-partition/📐️geometry/🟦️.ts`, and `🍎️mounted-image/📐️geometry/🟦️.ts`.

The ticket holds the reusable `closure-suite.ts` runner and this verification report.

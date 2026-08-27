# Transaction Source Authority Regressions

## Scope

This packet owns focused disposable-fixture tests and the transaction empty-source-parent verification boundary. No live CAD/Draw apply, production generator run, actual-repository Git mutation, or access to actual `compose/**` or `temp/compose/**` occurred. Fixture Git writes remain confined to fixture repositories created under this ticket.

## Source Admission And Retry Contract

The initial ten-case focused matrix produced **8 pass, 2 fail, 257 filtered, 284 assertions, 135.73 seconds**. CAD/Draw and the Nx-owned preview both failed on the second apply with `Plan source-tree digest cannot be rederived exactly from current schema-owned authority`. Cancellation/convergence passed in 3992.19 ms when given the explicit 120-second test budget.

Both failing tests planned with the original fixture ticket as an explicit inventory-admission root, then retried with a newly created sibling ticket. The latter is a different admission input, not an equivalent transaction attempt. Transaction-v2 already supports append-only attempt ordinals in one ticket. The fixtures now retry in that original ticket and assert ordinal `000002`. The cancellation fixture has an explicit 120-second budget; no production cancellation relaxation was needed. The earlier interrupted Git probe was downstream of the whole-test timeout, not a cancellation signal defect.

The new admission-negative fixture deliberately ignores its source workspace. Original inventory source rows are exactly:

```text
🧪️tests
🧪️tests/🧪️fixture
🧪️tests/🧪️fixture/🧪️subject
🧪️tests/🧪️fixture/🧪️subject/🦀️component.rs
```

Replacing the explicit admission with `🧪️tests/🧪️transaction-evidence` yields **zero rows**, a different source-tree digest, and rejection before any ticket/journal mutation. Separate negative checks change an unmoved physical Markdown source, then restore it and add a new ignored-but-explicitly-admitted in-scope Markdown source; both fail before journal mutation. A repeated full CAD/Draw inventory/plan compares exact source rows and source-tree digests.

An experimental source-admission fallback was removed completely after review: immutable scope equality cannot authorize arbitrary missing/extra source rows, and admission recovery would require reproducible resume authority. The exact source-tree, operation-set, and preimage rederivation remains fail-closed.

## Empty Source Hierarchy Boundary

With correct same-ticket retries, the expanded thirteen-case matrix produced **12 pass, 1 fail, 257 filtered, 303 assertions, 165.31 seconds**. Both digest-rederive errors disappeared; the real fixture Nx generation and verification succeeded, as did cancellation and all three new source-authority checks. The remaining CAD/Draw failure moved to post-state validation:

```text
Post-state does not converge to an empty plan: 0 operation(s), 270 finding(s); first projection-authority-invalid at 🧪️tests/🧪️fixture/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions
```

The focused CAD-only rerun reproduced that journal error: **0 pass, 1 fail, 269 filtered, 6 assertions, 59.86 seconds**. Post-plan verification runs before durable-commit source-parent pruning. Moreover, the old pruning predicate protected every descendant of the ticket root, not just the ticket root/ancestors. Explicit ticket admission therefore exposed empty old source-owner hierarchies before verification and would retain them after commit.

The implemented fix is an exact eventual-prune set:

- `emptySourceParents` considers only ancestors of frozen move sources. It walks those candidates bottom-up, checks all ancestors without following links, and accepts a directory only when every physical child is another already-proven candidate. Unplanned empty directories, ignored files, and symlinks therefore block pruning. The ticket root and all its ancestors are protected; descendants are not blanket-exempt.
- `artifactProjectionPostApplyViolations` rejects a still-present CAD/Draw old source owner outside that exact set as `projection-source-directory-stale`.
- `inventoryAfterSourceParentPruning` projects out only those exact directory rows, refuses referenced prunable directories, and recomputes retained directory/source/inventory digests. No source-prefix suppression is used.
- The set is recomputed for equality immediately before durable commit. Only afterward does `pruneEmptySourceParents` recheck each directory, remove it with nonrecursive `rmdir`, and sync its parent. No precommit directory mutation occurs, so rollback retains original directory modes and structure.
- The coordinator independently bound the exact set into required `sourceParentPrunePaths` journal evidence. Terminal recovery consumes only the recorded set, not newly emptied candidates from the current tree. The initial rederived-terminal-cleanup proposal was removed before release. The coordinator reports its focused journal packet as 5 pass / 0 fail / 65 assertions and the refreshed language-neutral journal golden audit as 1 pass / 0 fail / 804 assertions; those are coordinator-owned results, not reruns by this subtask.

The positive CAD/Draw fixture now checks rollback directory modes, an independent set-model oracle for every removed directory, committed old-root removal, retention of an unplanned empty sibling under an old source ancestor, and unchanged surviving directory modes. The new negative fixture inserts ignored file/directory/symlink residue during apply and requires fail-closed rollback without following the symlink. Its red run before the implementation returned the former 274-finding post-plan error instead of the new exact boundary error: **0 pass, 1 fail, 271 filtered, 4 assertions, 109.61 seconds**. The normalizer import then passed after implementation.

During negative-fixture setup, in-process synchronous Git commands (`rev-parse --show-toplevel`, ordinary `ls-files`, literal `ls-files`) returned status zero with empty captured stdout, including after a fixture-only forced index add. This reproduces the pipe-capture limitation already documented by `fixtureGitHead`; it is not proof of missing physical files or a new production pathspec defect. The finalized test uses explicit-ticket admission, asserts a nonempty move plan, and validates ignored status through `check-ignore --quiet` exit status. No tracked-only admission result from that setup is claimed as passing evidence. Temporary debug output and the forced-index setup were removed.

## Canonical Destination Authority

After exact pruning was implemented, the two-case CAD/residue packet produced **1 pass, 1 fail, 271 filtered, 372 assertions, 160.50 seconds**. The ignored/unplanned/symlink residue test passed in 23828.77 ms. The positive CAD/Draw apply advanced to a distinct second-plan defect: **211 canonical-destination findings**, first `directory-kind-unresolved` under `📚️examples/🪆️1-any/🏗️models/🏛️aec.building.structure`. Source projection authority existed, but canonical destinations had no equivalent authority pass.

The schema now registers CAD `profileVectors` explicitly as `{ artifactId: "📐️cad", standardVersion: "1", subsetId: "any" }`. Draw reuses its existing exact owner vectors. `artifactPathProjectionCatalogRoots` renders both source and destination roots forward from those tuples, validates grammar, and rejects folded destination collisions. Profiles are never reverse-split or inferred from a basename. Both schema readers validate the tuple registry.

`semanticPathProjectionAuthority` now supports `layout: "destination"` while retaining the same source/destination mappings and mapping digest. The common authority validates exact CAD model manifests/category rules and exact Draw descendants/configurable `lib.path` references. Canonical layout returns no reference edits. Normalization adopts a canonical directory/file only after that exact owner-root and whole catalog/bundle proof succeeds; unregistered profiles, malformed manifests, wrong leaf paths, symlinks, and unadmitted physical children remain errors. This does not add generic basename permissions.

A compact two-file CAD fixture reproduced the missing canonical authority in **1288.62 ms**; the initial canonical-golden/minimal-apply packet was **0 pass, 2 fail, 274 filtered, 6 assertions, 7.43 seconds**. After implementation, the compact packet passed: **3 pass, 0 fail, 275 filtered, 27 assertions, 9.53 seconds**. It covers:

- Both complete language-neutral CAD/Draw canonical mapping digests, with wrong-leaf and symlink negatives.
- Missing, duplicate, and unsafe CAD profile tuple rejection.
- Two-file CAD rollback, retry commit, empty second plan after physical old-root removal, malformed canonical manifest rejection, and unknown canonical profile rejection.

Production reference completeness remains a separate coordinator-owned blocker: its current read-only preapply report has fewer scoped external edits than the broad authority requires. No live CAD/Draw apply is justified by the fixture-only results here.

## Commands

All focused runs use:

```sh
bun test --timeout 120000 '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern='<selector>'
```

Initial ten-case selector:

```text
artifact-example-model-catalog-projection is schema-owned|artifact-example-model-catalog-projection fails closed|artifact-editor-command-projection|projects every registered golden bundle|plans the exact CAD and Draw authority mappings|rejects unowned artifact prose|rolls back and atomically applies CAD and Draw projections|normalization rejects malformed projection consumers|cancellation rolls back and a successful retry converges|plans, applies, verifies, and converges an exact Nx-owned preview
```

Expanded source-authority selector adds:

```text
rederives repeatable CAD and Draw source-tree authority|rejects a changed explicit source-admission ticket|rejects changed and newly admitted in-scope sources
```

The repeatability-only run passed: **1 pass, 0 fail, 267 filtered, 2 assertions, 16.82 seconds**. Temporary README/LICENSE schema edits briefly made a generator-only invocation and bundle compile fail before these fixtures could execute; the owner restored a coherent parser before the thirteen-case run. Those transient failures are not reported as passing checks.

Final seventeen-case command:

```sh
bun test --timeout 120000 '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern='artifact-example-model-catalog-projection is schema-owned|artifact-example-model-catalog-projection fails closed|artifact-editor-command-projection|projects every registered golden bundle|plans the exact CAD and Draw authority mappings|rejects unowned artifact prose|rolls back and atomically applies CAD and Draw projections|rederives repeatable CAD and Draw source-tree authority|rejects a changed explicit source-admission ticket|rejects changed and newly admitted in-scope sources|normalization rejects malformed projection consumers|cancellation rolls back and a successful retry converges|plans, applies, verifies, and converges an exact Nx-owned preview|rejects ignored and unplanned residual children in a projected source owner|canonical CAD and Draw authority retains|CAD profile vectors reject|a minimal CAD catalog commits'
```

The first run of this seventeen-case command completed **16 pass, 1 fail, 261 filtered, 691 assertions, 96.12 seconds**. Its only failure was the newly added retained empty fixture sibling `🧪️unplanned-empty`: under the assets owner, that name had ambiguous `test-case`/`test-fixture-member` classification before apply. A second run was **16 pass, 1 fail, 261 filtered, 691 assertions, 85.41 seconds** because the replacement `📚️library` is restricted to language-package parents, not assets. Inspection of the actual registry selected `🔤️fonts`, an exact `members-of-assets` member. The minimal CAD test now proves that sibling survives rollback/commit and passes **1 pass, 0 fail, 277 filtered, 12 assertions, 7.50 seconds**. No production classification relaxation was made for either fixture-name issue.

## Release Status

**Released.** The final seventeen-case command above completed successfully: **17 pass, 0 fail, 261 filtered, 753 assertions, 164.42 seconds**, exit status 0. All selected cases executed; none were skipped. The fixture process has finished and this subtask holds no test subprocess.

Notable final runtimes: full CAD/Draw rollback/commit/resume/convergence 87737.27 ms; ignored/unplanned/symlink residual rejection 21403.00 ms; exact source-tree repeatability 10743.48 ms; cancellation/convergence 1807.51 ms; Nx-owned preview generate/check/convergence 5125.38 ms. The full positive test also compares the resulting directory census with third-party `fast-glob` and independently models the exact set of removed source directories. `git diff --check` over the four shared implementation/test files exited 0. No temporary debug logging or source-admission fallback symbols remain in the normalizer/tests.

Files changed in this packet:

- [Normalization implementation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts): exact pruning boundary and canonical catalog adoption; the coordinator owns the additional durable journal binding in this shared file.
- [Discovery authority](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts): shared source/destination catalog validation and forward profile enumeration.
- [Taxonomy schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json): explicit CAD profile tuples.
- [Focused tests](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts): same-ticket retries, cancellation budget, source-authority negatives, exact pruning/rollback/retention, canonical corpus parity, and minimal idempotence regressions.

External production consumer completeness remains assigned to the coordinator/README agent. This fixture-only release neither performs nor authorizes a live CAD/Draw apply.

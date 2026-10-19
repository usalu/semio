# Workspace Recheck Cancellation

The native read-only workspace source API now advances every operation in its fresh membership recheck through the caller's original `BunWorkspaceControl`. The initial and fresh computations share one cumulative progress counter and original cancellation signal. Internal callers await the fresh verification before returning, publishing, or completing custody.

`drainControlledPlanV1` drives existing plans without recursively obtaining new ownership or issuing any authority. It checks cancellation before and after each awaited original progress callback and closes the generator in `finally`. The existing policy bytes, fresh selected membership, manifest bytes, bounded physical reads and final manifest fences remain in place. No repository source publication, repository installation, compiler or root manifest change was performed for this repair. An expanded regression selector also ran two existing scoped tests that execute native workspace-only installations inside their independently authored ticket fixtures; the final source-only regression excludes those cases.

The schema and portable fixture specify actual cancellation, selected manifest drift, topology drift, symlink substitution, a manifest edit after fresh membership verification, unrelated authored document changes, and unchanged rechecks. Tests exercise the public read-only API on independently authored temporary workspaces, validate vectors with existing Ajv, and compare package membership with existing fast-glob. Cancellation preserves the exact original reason and progress is strictly cumulative.

## TDD Evidence

The four initial fixture cases failed against the original synchronous fresh drain: the callback could not observe or cancel actual recheck selection operations (`workspace-fence-red.log`, four failures). After the controlled drain change, the same four cases passed (`workspace-fence-green.log`, four passes and 298 assertions).

The wider regression initially exposed stale historical tests: physical limits expected 131072/262144 while the current authored schema is 524288/1048576; a historical terminal-observation replay test expected behavior absent from current source; topology error assertions expected an old diagnostic. The current source deliberately rederives selected membership while permitting unrelated concurrent authored documents. The tests now verify that current contract, final manifest order and the original signal. No old observation replay or lower limits were restored.

The final source-only regression passed 40 tests with 947 assertions and zero failures in 7.92 seconds (`workspace-fence-final.log`). It includes all seven recheck vectors, existing custody lifetime/pending-work checks, physical byte limits, source projections, source-only scoped guards, original cancellation and the independent bounded-difference oracle. Generated evidence stays under this ticket's `🗑️generated` until the parent task finishes.

## Relevant Course Verification

The actual installed Node/Vitest diagnostic with native configuration loading passed all 44 launch, workspace-resolution and deployment tests across three files after this source repair (`course-fence-final.log`). This is a focused unit diagnostic, not a claim that the canonical course release or deployment command completed.

The bounded directory-difference diagnostics remain checked against fast-json-patch. Their integration assertions now distinguish rejected symlink substitution from admissible changes to directories containing no workspace package. Manifest peer-edit tests continue to assert exact peer bytes and no publication; they now expect the earlier controlled-verification diagnostic.

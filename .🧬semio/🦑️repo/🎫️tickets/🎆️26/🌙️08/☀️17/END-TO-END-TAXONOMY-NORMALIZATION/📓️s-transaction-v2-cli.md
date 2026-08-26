# Transaction v2 CLI Integration

## Result

The root `clean taxonomy` router consumes the frozen plan/journal v2 public surface without a v1 cast or compatibility parser.

- `apply` parses JSON through `parseTaxonomyPlan(value: unknown)` before digesting or mutating.
- `plan` summaries expose moves, embedded ticket-root relocations, symlink target edits, evidence removals, reference edits, regenerations, and unresolved findings.
- `apply` summaries expose exact applied counts for every operation class.
- The root planner no longer probes, reads, hashes, or otherwise accesses the intentionally deleted `compose` path; `excludedTreeDigests` is empty under the developer's explicit scope override.
- No live plan was applied and no Git state was mutated.

## Verification

The root router and its workspace imports bundle successfully while leaving third-party packages external:

```text
bun build ./📜️script.ts --target=bun --packages=external --outfile <ticket>/🧪️root-cli-v2-build.js
Bundled 11 modules
🧪️root-cli-v2-build.js 3.0 MB
```

The initial fully bundled probe reached an existing Playwright `chromium-bidi` resolution failure. Re-running with Bun's supported `--packages=external` mode isolated repository syntax and workspace export resolution and passed; no dependency was installed or changed.

Final transactional behavior remains owned by the normalization fixture suite and independent v2 audit. This report records only the root CLI integration boundary.

## Independent v2 checkpoints

The coordinator independently rebuilt the current root after the v2 public contract landed:

```text
bun build ./📜️script.ts --target=bun --packages=external --outfile <ticket>/🧪️root-transaction-v2-build.js
Bundled 11 modules in 44ms
🧪️root-transaction-v2-build.js 3.1 MB
```

The focused transaction checks passed `6/6`, with `48` assertions and no failures: strict v2 parsing, virtual platform sentinels, no-follow symlink hashing, non-empty symlink retarget rollback/apply/convergence, stale preimage refusal, and cancellation/retry convergence.

The first cross-feature regression run passed `8/11` and exposed one affected-state defect: a moved-and-reference-edited file used the final content hash with its pre-edit size. After binding the final result size to the move destination row, all three affected CAD/Draw regressions passed independently. The non-empty embedded disposition fixture then passed with three roots, four relocations, two evidence removals, injected rollback, successful apply, and an empty second disposition plan (`11` assertions).

The Cargo cache marker authority was also rerun independently: `3/3` tests, `16` assertions, no failures. It admits the six embedded blockers through the exact registered parent directory kind and rejects basename-only and counterfeit-parent matches.

These are intermediate correctness checkpoints. Partial resume, forged journal state, complete conflict-matrix, all disposition failure/cancellation stages, and final repository-wide convergence remain blocking.

## Append-only attempt and custom-plan boundary

Apply now returns an append-only canonical attempt journal rather than one reusable digest journal. The root CLI treats `result.journalPath` as the only resume identity and passes the exact lexically/no-follow-guarded `--plan` path to the engine as `planArtifactPath`. The engine can therefore bind and exclude the actual canonical plan bytes even when the user selected a valid non-default plan artifact; it no longer invents a second canonical path that can self-block reference or stale-source scans.

The updated root bundled successfully with 11 modules in 46 ms to retained evidence `🧪️root-transaction-attempt-build.js` (3.20 MB). The focused transaction suite is currently under its 15-second test budget and the expanded permanent matrix is not yet green; this is a CLI boundary checkpoint, not transaction sign-off.

## Atomic preparation schema checkpoint

The schema boundary now includes exact attempt, backup, restore-exchange, reference-edit, lease, and nested JSON-write preparation authorities. Restore exchange accepts only the frozen empty / backup-only / matching backup-plus-post / post-only state union. Lease and inner JSON-write candidate names bind a positive PID and UUID v4 to their exact parent kinds. The taxonomy remains strict v7 with zero validation problems.

Independent root verification:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cli-artifact-directory-kinds.test.ts'
4 pass
0 fail
124 expect() calls
74ms
```

An initial invocation omitted the leading `./`; Bun treated it as a name filter, searched the workspace, and ran no tests. That routing failure is retained separately from the successful path-qualified evidence above.

The first exchange-union rerun then exposed one transient edit-preparation negative-vector mismatch (`3` pass, `1` fail, `105` assertions). After the strict validator and golden update completed, the same command passed with the final 124-assertion state above. Edit preparation now accepts only exact hash-bound subsets of `.edit`/`.pre`; nested JSON-write preparation accepts only exact subsets of `🔣️.json`/`⏮️.json`. Duplicate, mismatched, and foreign leaves reject.

The owner-file projection boundary was also rerun against the same live taxonomy:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️ticket-important-owner-authority.test.ts'
4 pass
0 fail
30 expect() calls
279ms
```

These results verify schema/parser authority only. Atomic engine publication, crash recovery, and the full matrix still require independent audit sign-off.

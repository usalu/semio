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

## Frozen Transaction v2 integration checkpoint

The shared Transaction Plan/Journal v2 implementation is frozen for integration at these SHA-256 identities:

```text
86f90f2e954e8082e0a6f9b0f5432a1e0131f86137624312e945849a602dc76f  normalization/🟦️.ts
22099778a38e0107cdadae4762010ba4f001bd484efb924ca350ee6c51b0539c  transaction-v2/🟦️.test.ts
e3c9dbad890beda23b7ed8233cb027ccd9374dc77ed72beb077c55ba2fd4138d  transaction-dispositions/🔣️.json
e5e205edf9bf00643ed29bb05b5ba3f9a92363186f31f5b21f7bebfae92fd1f4  repo-lib/📜️script.ts
```

The engine checkpoint implements the global validate/action split before transaction mutation; strict append-only attempt allocation; exact attempt/preparation collision rejection; nested schema-owned backup and edit writers; journal canonical/previous JSON exchange; lease preparation, publication, stale quarantine, and restoration; immutable source copies without mutable-source hardlinks; strict reachable recovery tuples; transaction-aware resume authority; exact plan/source/preimage/reference/regeneration validation; rollback recovery; terminal backup-only recovery; and typed committed/rolled-back terminal closure. The reusable engine requires an explicit `expectedBaselineCommit`; it does not bind a frozen plan to mutable `HEAD`.

The root `clean taxonomy apply` command requires `--baseline <commit>` and passes it to the engine. Both `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` pass `${env:TAXONOMY_BASELINE}`. The focused command is registered as `🧹clean🧩️taxonomy🧪️transaction-v2` and routes through the Nx target `@semio-tech/repo-lib:test-transaction-v2`; no additional permanent script file was introduced.

The permanent focused aggregate registers 62 tests and 98 exact boundary outcomes. Its coverage includes:

- all eight injected apply failure stages;
- parent-issued process-tree termination across attempt, initial journal/lease, journal previous/canonical exchange, WAL, backup, edit, restore, lease, terminal cleanup, and mixed-generator boundaries;
- exact caught-callback rollback at allocation and journal previous exchange;
- deterministic cancellation, double-plan identity, append-only ordinal retry, stale second apply, and mixed generator `[rolled-back, committed]` history;
- live double contenders, loser cleanup, winner retry, stale lease quarantine, and terminal stale-lease backup closure;
- stale baseline, source digest, preimage, incoming reference, regeneration, forged resume, malformed sibling, skipped/future ordinal, and unreachable backup/edit tuple rejection with no mutation;
- exact workspace and transaction-tree evidence including directory modes, file modes and bytes, complete normalized JSON, and raw symlink targets.

The language-neutral disposition golden is closed-set and human-reviewable. An independent unchanged-source audit passed:

```text
1 pass
0 fail
804 expect() calls
98 boundaries = 43 killed + 43 recovered/terminal + 11 rolled-back + 1 committed mixed-generator
63 transaction ledgers + 9 workspace ledgers
```

That audit recomputed every ledger, file-byte, and symlink-target digest; rejected extra boundary keys and orphan ledgers; and confirmed complete JSON/bytes/raw-target evidence. Stable semantic digests remain byte-exact and are not normalized as runtime nondeterminism. The current normalization module also bundles successfully as 15 modules (0.97 MB).

## Outstanding timing acceptance

Transaction correctness and exact evidence are structurally green, but final performance sign-off is intentionally **not claimed**. The required evidence is three unchanged, uncached executions of:

```text
bun nx run @semio-tech/repo-lib:test-transaction-v2 --skip-nx-cache
```

with every run completing in less than 15 seconds. The latest complete attempts reached the coordinated 14-second internal deadline at approximately 14.69–15.21 seconds while unrelated repository lanes were concurrently running multiple Cargo, rustc, and nextest workloads, including `rustc -Z threads=8`. At the frozen checkpoint, seven unrelated Rust processes remained active. The coordinated runner terminated and awaited every owned process group on timeout, and post-run process censuses found no surviving Transaction v2, fixture-generator, or mixed-generator child.

The focused runner is already reduced to four balanced process groups, selects all 62 registered cases exactly once, enforces a hard outer budget, and cleans its run-scoped fixtures. Integration may proceed from the frozen hashes above, but the complete target and three uncached sub-15-second timings must be rerun on an unchanged checkpoint after the host becomes quiet. This report therefore records a timing blocker, not Transaction v2 acceptance.

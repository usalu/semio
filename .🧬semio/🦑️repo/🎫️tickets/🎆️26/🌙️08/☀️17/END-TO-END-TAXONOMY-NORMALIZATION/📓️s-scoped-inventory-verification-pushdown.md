# Scoped Inventory and Verification Pushdown

## Outcome

Scoped taxonomy work now pushes a conservative literal prefix into tracked and ordinary-untracked Git enumeration while retaining NFC `inScope` as final authority. Transaction verification no longer scans unrelated repository projections: it admits the plan scope, every affected path, exact regeneration inputs/outputs, and exact schema-declared external projection consumers.

This closes the reproduced ticket-important failure boundary without weakening CAD/Draw reference authority. An unrelated old projection token does not block an exact scoped transaction; an active declared external consumer remains parsed, edited when supported, and fail-closed when stale or unsupported.

No production move was applied by this packet. No Git state or actual `compose/**` / `temp/compose/**` path was read or changed.

## Implementation

- `taxonomyScopedGitPathspec` renders an argv-only `:(top,literal)` positive prefix and only intersecting opaque exclusions.
- The positive prefix stops before the first NFD-sensitive segment; the existing NFC scope predicate remains final.
- Proper symlink, non-directory, or exact indexed-leaf ancestors fall back to the unscoped Git candidate command.
- The unscoped command retains positive `.` and all opaque exclusions.
- Inventory emits the closed phases `setup`, `tracked-enumeration`, `untracked-enumeration`, `ignored-generator-admission`, `explicit-ticket-admission`, `directories`, `files`, `references`, and `complete`.
- The root CLI passes one throttled progress sink to inventory/plan/apply/verify and keeps progress on stderr.
- Scoped planning reads exact schema-declared external projection consumer identities separately, so inventory bytes/digests stay scoped while structured external edits retain their authority.
- Post-apply stale-token and lexical incoming-reference scans use scope candidates plus all plan-affected and declared consumer paths.
- Exact ticket-important mutation catalog admission is optional in isolated repositories with no governed source, but remains fail-closed when any of the three production-governed source sentries exists.

## Root Coherent Verification

The root coordinator repaired the first test-only retry error by retaining the same `ticketDir` across rollback and retry. The final exact-history regression proves an injected rollback restores the source, a retry commits the move byte-for-byte, and the second plan is empty.

```text
bun test --timeout 120000 './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern 'scoped Git pathspec|scoped inventory preserves|a scoped apply ignores|an exact ticket-important|exact CAD source scope|plans the exact CAD and Draw authority mappings|inventory bytes are deterministic'
7 pass, 0 fail, 130 expect() calls, 23.48 s
```

The seven cases prove:

- language-neutral pathspec golden parity;
- literal metacharacter and NFD/NFC scope membership;
- third-party physical census parity;
- exact closed phase ordering;
- unrelated stale-token isolation;
- exact scoped ticket history rollback, commit, byte preservation, and empty replan;
- 209 CAD plus 11 Draw mappings with structured cross-profile references;
- out-of-scope declared consumer edit planning and fail-closed stale-consumer preflight with zero workspace mutation;
- deterministic inventory bytes.

## Production Transaction Proof

The first production apply exposed one remaining deterministic widening bug: `lexicalTargetIncomingReferences` seeded the scoped plan candidates and then re-added every explicit ticket row. Retained active-ticket plan/report evidence therefore referenced the disposed source and blocked its own transaction. The function now adds unscoped explicit-ticket rows only when no authority plan exists; authority-plan calls retain the already scoped and affected-path-expanded candidate set.

The exact-history regression now adds unrelated ticket evidence that names the source and proves rollback, committed retry, byte preservation, and empty replan. After the coherent packet passed, production attempt `000003` committed the exact `CLI-HUMAN-READABLE-OUTPUT` history move. Source and destination SHA-256, mode, size, and inode agree; an identical original-source scope replans to zero operations and zero unresolved findings. Full journal evidence is in `📓️s-ticket-important-lifecycle-history-integration.md`.

## Acceptance Matrix Closure

Dedicated disposable-fixture tests now cover exact symlink-leaf scope, symlink-ancestor fallback, mode-`160000` gitlink-ancestor fallback, cancellation at every nonterminal phase boundary, standalone ignored-generator and explicit-ticket admission parity, the frozen unscoped Git pathspec plus canonical bytes, reversed-creation-order byte/event determinism, and CLI stdout/stderr purity. The focused matrix passes 9 tests with 33 assertions; exact evidence is recorded in `📓️h-scoped-inventory-acceptance-matrix-addendum.md`.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️scoped-inventory-pathspec/🔣️.json`

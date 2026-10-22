# Result: lease-proof (wave 1022a)

VERIFIED with the unit tests below; not re-run in a full live `bun repo` graph/course run.

## Root cause
`requireResourceLeaseWriterProofV1` compared the call site's control object with the control the lease was issued to (`issued.control !== control`), so a proof worked only for the exact holder object, never for a narrowed control of the same holder.

The failing caller is `requireRepositoryGraphStoreFormatV1` (`...opening/🏪️store/📇️format/🟦️.ts`), custody path. `repositoryCacheStoreMaintenanceOwnedV1` (`...opening/🏪️store/🟦️.ts`) opens the image with `withExclusiveMaintenance`, then runs `withRequestedIssuedResourceControlV1(custody.control, 0, control => requireRepositoryGraphStoreFormatV1(workspace, opened, control, custody))`. That helper always issues a NEW child control (`controlFor`), so `control` is never `custody.control`. The proof (captured against `custody.control`) was then checked with the child. The marker is only missing, and `initialize` only runs, on a fresh image. That explains the intermittency: it appears after `receipt stale|busy` forces the maintenance owner path onto a fresh graph image. The non-custody path (`initialize(held.control, held)`) always passed. Slice-demo's fix 2 (observationPort.check uses the held control) only fixed the inner check; the first check in `initialize` and the custody path still passed the child.

## Census of callers (all non-test sites of `requireResourceLeaseWriterProofV1`, about 45)
All pass the control used at `captureResourceLeaseWriterProofV1` (`custody.control`, `creatorControl`, `proofControl`, `phase`, `body`, own `control`) except the format caller above. No other site passes a phase/child control against a holder proof. Re-entrant/child admissions are covered by the generic fix below.

## Fix
1. `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`: new `requireIssuedResourceControlDescendantV1(holder, borrower)` (borrower is the holder or a live issued descendant via `parentControl`; siblings and ancestors refused; not re-exported from the package).
2. `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` `requireResourceLeaseWriterProofV1`: the proof identifies the holder (`issued.control`, live, `issued.signal === holder.signal`); the passed control must be the holder or an issued descendant, else the same error "Exact live original writer proof required". Removed course-rest's `[DEBUG] writerproof-stack` line (as asked).
3. Format `initialize`: both proof checks now use `held.control` (the holder), not the work control.

## Test (language-agnostic)
- Fixture `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/🎟️writer/🎟️interleaving.json` (schema embedded, Ajv strict): 4 sequences of acquire / busy contender (queued behind the holder) / child / sibling / require / release / settle, including `stale-busy-format` (holder, busy contender, phase child, format grandchild: all accepted; contender refused; after release all refused), `restricted-descendant` (60 s narrowed child), `sibling-refused`, `ancestor-refused`.
- Test `writer proof identifies the holder for descendants and refuses siblings and ancestors against a SQLite lineage oracle` appended to `.../🧪️tests/🎟️writer/🟦️.ts`: every require step is checked against the fixture AND an independent bun:sqlite recursive-CTE lineage oracle (descendant-or-self of the holder and lease alive).
- Negative control: with the descendant acceptance disabled the test fails (`stale-busy-format: phase Expected true, Received false`); restored.

## Commands and outcomes (no repo-runner target exists for this file; envelope built by `C/lease-proof-tools/run-test.ts`, which creates a Script process envelope and runs `bun test <file> -t <name>` with it; scratch dir under the session scratchpad because the ticket path exceeds the 256-char lease path limit)
- New test: 1 pass (29 expects). Existing tests of the same file each run alone: `actual shared lease public promotion`, `mapping cleanup descendant`, `current retirement catalog`: 1 pass each.
- Format tests `bun test ./…/📇️format/🧪️tests/🟦️.ts`: 4 pass.

## Open / observed, not mine
- `lease authority component schema preserves every owned contract` (same file) FAILS on the pinned hash `NamespaceTicket` (expected b1c5d358…, got a25f5fa3…): `🧬️schema/📜️ticket.json` was changed by the conc-leases fix8 (admission patience fields) without re-pinning `schemaOwnershipHashes` in `🧫️fixtures/📜️queue.json`. I did not touch either file. A later test in the same file then reports "Original process admission closed" (cascade of that failure when the whole file runs in one process).
- Not re-run: a live graph cold boot through `bun repo` (would need the dependency build lease queue); the next live run that hits `receipt stale|busy` on a fresh graph image should now pass the format step.

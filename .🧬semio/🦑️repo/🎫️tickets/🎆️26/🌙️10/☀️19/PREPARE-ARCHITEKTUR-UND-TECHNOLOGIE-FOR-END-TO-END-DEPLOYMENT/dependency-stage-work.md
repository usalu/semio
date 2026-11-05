# Dependency Staging Semantic Work

## Proven Cause

The terminal dependency retry passed the exact Source fence and failed with `Current Store group work capacity exceeded`. The stack places the increment inside Windows `statusCall -> facts -> fileGuard -> write`. The stage callback counted every cancellation/custody/descriptor checkpoint as a semantic work unit. Windows legitimately issues many facts and ACL queries per descriptor operation, so the callback count depends on implementation detail rather than admitted materialization size. Capture and replay used the same counting pattern.

The current unready Store observation contains six roots, 871 directories, 874 files, 3,328,255 bytes, depth three, and no ready receipts. This is far below the retained 4,194,304 semantic work cap. No binding was published and no Source transition was unresolved.

## Contract and Repair

The existing SourceOwnerStoreProtocol now declares `workUnit: semantic-entry-or-chunk-attempt` and a 64-unit progress cadence. The capacity remains 4,194,304. A bounded Store work tracker separates `check` (always executes the original cancellation/custody checkpoint) from `charge` (checks first, then admits one semantic unit). Stage, capture, and replay pass `check` to the native descriptor interface and charge explicit file/directory/roster entries and each actual I/O attempt. Short reads/writes therefore consume additional semantic work, while additional native facts/ACL guard calls do not. Every descriptor check, identity comparison, digest verification, no-progress rejection, and cancellation continuation remains in place.

The existing admitted group bound is 64 batches, 32768 files, 65536 directories, and 64 MiB. Ordinary full-size chunks require at most about 33792 attempts (per-file rounding plus total bytes/chunk). The largest replay accounting is at most twice the directory count, twice the file count, and the chunk attempts: about 230400 units. Short operations can use more attempts and still fail at the unchanged hard cap. The capacity is no longer spent by platform guard amplification.

## Test-First Evidence

RED: the portable regression first failed because the semantic work implementation was absent. GREEN: the focused pure verifier passed twelve authored cases and four invalid capacities across three tests. Coverage includes exact capacity, one-unit overrun, unsafe numeric overflow, cancellation during native guards, cancellation during semantic work, cancellation precedence, 4,194,305 guard callbacks, short writes, zero progress, and actual extracted stage/capture/replay bodies operating on a test-only memory descriptor. Independent SQLite arithmetic and recursive write-state oracles reproduce the expected results. The three-phase harness reports stable semantic counts `[9,12,8,14,14]` under callback amplification and `[13,12,8,14,14]` under short writes.

Verification command: `repo tools exec --light bun -- <ticket>/stage-work-check.ts D:/semio`, exit 0. This is a short pure query with memory descriptors; it does not create Store files or issue native authority. The integrated test uses Ajv; Ajv remains unavailable until dependency synchronization succeeds, and the pure verifier explicitly reports that deferred schema-library check.

## Files

- Dependency storage TypeScript implementation
- Dependency storage install schema
- Dependency storage test aggregator
- New dependency storage `🧪️tests/📏️work/🔣️.json`
- New dependency storage `🧪️tests/📏️work/🟦️.ts`
- Retained ticket verification input `stage-work-check.ts`
- This report

No dependency writer, endpoint mutation, unknown-input deletion, or ownership bypass was performed by this agent. Parent owns the next real synchronization.

## EOF, Representative Shape, and Progress Review

Review identified two omissions in the initial repair: terminal one-byte EOF probes were not charged, and directory/roster-only capture/replay work did not publish progress. Both are corrected. Every terminal read attempt now charges before reading, including empty files. Every charged semantic operation advances the schema-defined 64-unit progress cadence, retaining cancellation/custody checks before publication, after publication, and after yielding. The original descriptor, identity, digest, terminal-EOF, and zero-progress checks remain intact.

The updated replay bound includes terminal probes: twice the directory count, three times the file count, and data read attempts, about 263168 units at the admitted full-chunk group maximum (plus the shared namespace). The unchanged 4194304-unit cap still permits the bounded ordinary group while refusing excessive short-operation attempts.

Test-first RED: the updated EOF vector rejected the prior uncharged counts. Final GREEN: eighteen portable cases and four invalid capacities across three focused tests, with independent SQLite arithmetic/recursive I/O oracles. An actual extracted stage/capture/replay harness now accepts a synthetic group matching the observed six roots, 871 directories, 874 files, 3328255 bytes, and depth three. It checks the resulting receipt directory/file counts, total bytes, and path depth. Semantic counts are `[3493,5238,3494,5240,5240]`, independently reproduced from the file-size vector and directory count in SQLite.

Additional cases cover short reads, empty files at exact per-phase capacities, one extra terminal-probe overrun, and 874 empty files across 871 directories. The empty representative vector requires capture and replay progress. Its cancellation case interrupts capture at the first 64-unit continuation and verifies the original cancellation reason. Closed neutral fixture schema now describes the shape, capacity, and cadence vectors.

The final focused verifier exited 0. Ajv validation remains deferred as previously documented; the existing integrated test uses Ajv when dependencies are available. Root continues to own the held real retry. No endpoint mutation or writer was performed during this follow-up.

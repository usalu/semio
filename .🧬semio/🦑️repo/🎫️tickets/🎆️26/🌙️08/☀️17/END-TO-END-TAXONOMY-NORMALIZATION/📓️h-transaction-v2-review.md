# Transaction Plan/Journal v2 Independent Review

## Scope

Read-only audit of the current normalization engine, strict transaction artifact taxonomy, and permanent TypeScript gates. The intentionally deleted opaque tree and its temporary counterpart were not traversed or read. Isolated generated fixtures remain permitted by the ticket scope.

## Current decision

Signoff is withheld. The current engine compiles, but the permanent focused v2 gate is red and the apply control flow still mutates transaction evidence before all plan, baseline, workspace, artifact, and recovery evidence has been globally validated.

## P0 findings

### Mutation begins before validation completes

In the audited checkpoint, the first recovery mutations occur when dead unpublished attempt preparations are removed and terminal attempts are cleaned. These actions precede the later symlink authority checks, baseline commit validation, opaque-digest rejection, affected pre-state check, destination-ancestor checks, workspace/reference inventory, incoming-reference checks, and exact plan rederivation.

The required order is one global read-only pass over all external and transaction evidence, followed by lease acquisition, a second identical read-only pass, and only then recovery or cleanup mutations. This applies to terminal attempts and dead attempt preparations as well as the selected active attempt.

### Attempt allocation collisions are accepted

The current scan validates each unpublished preparation independently and then removes it. It does not reject two preparations that claim the same ordinal, nor a preparation whose ordinal collides with an already published canonical attempt. Both are ambiguous allocation evidence and must fail closed before any removal.

### Direct rollback can begin around unclassified partial writes

The normal exception path calls rollback directly. Rollback does not first run the same global root-name classification, validate-only backup/restore/edit/lease/WAL pass, and preparation recovery used by resumed attempts. An I/O exception inside backup, edit, or journal publication can therefore leave a reachable partial preparation that rollback begins mutating around. The fresh `WAL is occupied` failure is concrete evidence of this recovery-order class.

### Stale resume checks happen after irreversible stale-lease cleanup

Incoming-reference and other selected-resume drift checks currently run after lease acquisition. Acquiring over a stale canonical lease moves the old lease to quarantine and later removes it. If the second validation callback or a later resume drift check fails, acquisition cleanup removes the new owned lease but does not reconstruct the old stale lease exactly. A rejected stale resume can therefore change the transaction tree. All drift predicates must be part of both read-only validation passes, or lease acquisition must restore every quarantined byte and directory exactly on failure.

### Terminal cleanup must admit its own backup-only crash tuple

Both terminal cleanup functions remove the stage root before the backup root. A kill between those two durable removals therefore leaves `stage absent + backup present`. The current checkpoint's terminal validator treats any one-root-only state as incomplete. Backup-only must be validated against the exact typed journal backup set and then cleaned; otherwise terminal cleanup is not crash-convergent.

### Permanent crash/recovery proof is incomplete

The permanent v2 block currently proves its language-neutral golden, strict plan parsing, no-follow symlink hashing, one symlink failure/retry flow, one symlink child interruption, and one embedded-root flow. It does not yet prove:

- killed canonical journal JSON exchange, including `⏮️.json` tuples;
- killed lease JSON exchange, stale quarantine, prebuilt nonempty publication, and two contenders;
- killed binary backup publication and nested writer recovery;
- killed reference-edit publication across every reachable outer and nested writer tuple;
- killed restore exchange across empty, candidate-only, candidate-plus-postimage, and postimage-only tuples;
- a generator that writes a genuinely mixed output tree before termination, rolls ordinal 1 back exactly, and commits a fresh ordinal 2;
- all eight declared injected failure stages;
- byte-level before/after snapshots that include regular-file bytes and modes, directory modes, and raw symlink target text for the workspace and transaction tree;
- stale baseline, stale resume, and source-drift failures proving zero transaction-tree mutation;
- one combined focused gate below fifteen seconds.

## Fresh checkpoint evidence

The engine bundle completed successfully as two modules, producing a 0.69 MB ticket-local audit artifact.

The focused permanent selector completed in 11.14 seconds with `3 pass / 3 fail / 46 expectations`:

- symlink rollback failed because the journal WAL was occupied at `persistJournal`;
- the symlink interruption child exited `1` rather than the expected marker `73`;
- embedded cancellation failed exact current-authority rederivation because the affected pre-state digest differed.

This is a current-red implementation checkpoint, not transaction acceptance evidence.

## Closed mechanisms observed in source

- Strict journal schema v2 parsing and canonical JSON byte checks.
- Strict typed regular-file and symlink backup records.
- JSON exchange tuples with a previous-image leaf.
- Nested binary writer directories for backup and edit candidates.
- Validate-only passes for active backup, restore, edit, lease, and journal-WAL preparations.
- Append-only journal transition checks and exact operation membership checks.
- Recovery-root name classification for selected active attempts.
- Started-generator partial output is identified as rollback-only rather than forward-resumable.

These mechanisms remain provisional until the ordering defects are closed and the permanent behavioral matrix is green.

## Signoff requirements

1. No filesystem mutation before all external and all attempt/preparation evidence is validated globally.
2. Reject duplicate and colliding attempt ordinals and preparation identities.
3. Re-run the strict taxonomy/artifact union gates.
4. Run the complete permanent crash, failure, concurrency, stale-input, rollback, fresh-retry, terminal-cleanup, and empty-replan matrix in under fifteen seconds.
5. Confirm exact byte/mode/directory/raw-symlink snapshots before and after every zero-mutation and rollback case.

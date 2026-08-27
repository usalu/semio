# Transaction Plan/Journal v2 Independent Review

## Scope

Read-only audit of the current normalization engine, strict transaction artifact taxonomy, and permanent TypeScript gates. The intentionally deleted opaque tree and its temporary counterpart were not traversed or read. Isolated generated fixtures remain permitted by the ticket scope.

## Current decision

Signoff is withheld. A dedicated transaction-v2 aggregate has landed, but its current fixtures do not execute the full declared stage matrix, several durable exchange boundaries are absent, zero-mutation assertions are incomplete or incorrect, and no three-run sub-fifteen-second aggregate proof exists.

## P0 findings

### Fresh regeneration ordering finding closed in source

An intermediate checkpoint removed unpublished dead attempts and closed terminal attempts before explicit regeneration input/output preimage validation. The latest source moves regeneration input/output validation and a cancellation check before cleanup or lease/allocation mutation. A byte-identical transaction-tree assertion for stale fresh application must still permanently prove this ordering.

### Baseline and source-tree findings closed in source

The latest source requires a separately supplied `expectedBaselineCommit` and rejects format or equality mismatches before transaction-tree access or mutation; the root apply command now requires `--baseline` and passes it through. It must never compare against mutable branch `HEAD`. Fresh exact rederivation now also includes `sourceTreeDigest`. Fresh rejection must still prove the pinned baseline plus exact current-workspace source-tree and preimage authority; resume rejection must prove the pinned baseline plus transaction-aware source/reference/preimage authority, with byte-identical workspace and transaction trees.

### Transaction filtering regression closed in source

An intermediate checkpoint suppressed inventory rows whenever any segment matched the transaction-directory basename, which could hide counterfeit trees. The latest source restored exact `path === transactionRoot` or descendant-prefix filtering plus the exact plan artifact. Permanent counterfeit-segment negative proof remains required.

### Resume lease-before-cleanup ordering closed in source

The latest checkpoint validates plan, baseline, workspace/reference, selected-resume tuples, incoming references, recovery evidence, selected canonical lease liveness, and selected-state admissibility, then acquires the selected active attempt lease and repeats selected validation before deleting dead preparations or closing terminal history. It also rejects rolled-back resume selection before cleanup and restores an exact quarantined stale lease if acquisition fails. Permanent loser-side transaction-tree snapshot proof remains required.

For resume, the required order is one global read-only pass over all external and transaction evidence, immediate selected-attempt lease acquisition, a second identical global pass with owned-token allowance, and only then winner-side recovery or cleanup mutations. The current structural ordering matches this shape, subject to adopting the post-lock journal described below.

### Attempt allocation collision finding closed in source

The latest source rejects two unpublished preparations that claim the same ordinal and rejects a preparation whose ordinal collides with an already published canonical attempt. Permanent behavioral proof is still required.

### Direct rollback recovery-prologue finding closed in source

The normal exception path now re-reads the canonical journal, globally classifies recovery roots, validates lease/WAL/backup/restore/edit preparations without mutation, promotes the authoritative WAL, recovers all preparations and durable tuples, persists reconciled authority, and only then begins rollback. Permanent caught-failure and kill-boundary proof remains required.

### Stale-lease rollback finding closed in source

Incoming-reference and selected-resume drift checks now run before acquisition and again through the owned callback. Acquisition tracks a quarantined stale lease and durably restores it if acquisition fails. Permanent exact transaction-tree snapshot proof remains required.

An intermediate probe checkpoint placed canonical stale-lease quarantine and preparation creation before the acquisition catch scope. The latest source moves every mutation beginning with quarantine inside the ownership-restoring `try`. A thrown-failure exact transaction snapshot test remains required in addition to SIGKILL recovery.

### Terminal backup-only tuple and classifier finding closed in source

Both terminal cleanup functions remove the stage root before the backup root, so `stage absent + backup present` is reachable. The latest source admits this tuple and runs the shared recovery-root classifier even when stage is absent. Permanent kill/resume proof remains required.

The latest source now exposes separate committed and rolled-back probes immediately after stage removal and before backup removal, enabling deterministic parent-kill proof of the backup-only terminal tuple. The aggregate has not yet adopted them.

An intermediate committed recovery path cleaned terminal residue and then unconditionally required an active locked journal. The latest source handles selected committed resumes explicitly after cleanup, verifies the post-state digest and journal-only terminal tree, and returns committed. A committed-stage-removed parent kill must still prove this behavior permanently.

### Double-contender protocol improved; proof absent

The lease is published by atomic preparation-directory rename. The latest source lets the canonical winner wait for a live loser's self-cleanup instead of immediately tearing down its own winning lease. The permanent barrier-synchronized proof must still require exactly one returned owner, a fail-closed loser with zero workspace mutation, and eventual cleanup of foreign preparation evidence.

### Post-lock journal adoption finding closed in source

The owned-lease validation callback now returns the reconciled durable journal snapshot. Main recovery re-reads and promotes the canonical/WAL journal under the lease and exact-compares it with the locked snapshot before recovery. Permanent contender-handoff proof remains required.

### Dedicated aggregate stage matrix is not active

The dedicated aggregate's failure fixture contains only a move and reference edit. Consequently, the `after-embedded-root-staging`, `after-relocations`, `after-symlink-retargeting`, and `after-regenerations` injections occur after empty phases. Every failure-stage case must assert that its corresponding operation family is nonempty before accepting that case as coverage. The rollback fixture also lacks a symlink, so its exact snapshot cannot prove raw-target restoration.

### Dedicated kill matrix omits reachable boundaries

The aggregate covers lease preparation, WAL preparation, nested backup writer preparation, nested edit writer preparation, outer edit exchange, and restore exchange. It omits the production `transaction-backup-exchange` and `transaction-restore-prepared` probes. It also does not create or kill the canonical JSON previous-image exchange: `transaction-wal-prepared` currently fires after publication into an initially empty WAL root, and the new-attempt lease case has no prior canonical leaf. Parent-issued kills must prove the final-to-previous and candidate-to-final JSON sub-boundaries, rather than relying only on schema/golden enumeration.

The helper returns exact transaction-tree snapshots at kill boundaries, but the tests discard most of them. The permanent assertions must use those snapshots to prove each reachable tuple, then prove the exact terminal attempt tree and lease cleanup. They must also verify that the child actually died at the requested boundary.

### Canonical JSON exchange is newly reachable but not recoverable at both sub-boundaries

The latest checkpoint makes the journal publisher exchange the canonical attempt journal through a WAL-contained `⏮️.json` and exposes writer, candidate, previous-exchanged, and canonical-exchanged probes. Two recovery paths are currently invalid:

- after the previous exchange, the canonical attempt journal is absent, but initial attempt enumeration rejects the attempt for lacking its direct journal before it can inspect or reconcile the WAL;
- after the canonical exchange, the canonical journal is revision N+1 and the WAL previous image is revision N, but the reconcile validator treats every preparation leaf as a candidate that must be current revision plus one, so it rejects the valid predecessor.

There is also an empty-WAL regression: after the JSON exchange finishes but before the empty WAL directory is removed, recovery initializes its prospective result from the external canonical journal even though there is no preparation action. Reconcile then compares the canonical current journal with itself as though it were revision N+1 and rejects it.

Recovery must bootstrap current authority from the previous leaf when the canonical is absent, and distinguish exact predecessor evidence from a next candidate when canonical is present. Parent-issued kills must prove both sub-boundaries. The lease publisher still targets a newly created preparation and therefore has no real previous-image exchange; real lease exchange protection also remains required.

The next checkpoint closes empty-WAL handling and admits an exact predecessor during reconciliation, but two blockers remain. Initial enumeration can identify a prospective journal with the canonical absent, while selected-resume validation and main resume still read the absent canonical path before reconciliation. Conversely, with canonical N plus candidate N+1, enumeration adopts N+1 as current and the generic recovery helper never submits the external canonical N to transition validation; a forged canonical predecessor could be overwritten and removed. Canonical authority must remain current whenever present, and a previous image must be the bootstrap current only when canonical is absent.

### Mixed-generator process-tree termination finding closed

An intermediate aggregate blocked the mixed generator for only 100 ms and sent `SIGKILL` only to the main Bun process. The latest helper now uses a detached process group on POSIX, `taskkill /t /f` on Windows, blocks the generator for thirty seconds, records the generator pid, asserts abnormal parent termination, and waits until that descendant is gone. This structurally closes the orphan race. Exact mixed boundary transaction snapshots, terminal ordinal-1 and committed ordinal-2 journal evidence, and a full empty replan remain required.

### Stale-input zero-mutation assertion is invalid

The current stale test snapshots the workspace before deliberately changing the source, then expects the rejected resume workspace not to equal that old snapshot. That only observes the test's own drift. It must snapshot after introducing drift and assert exact equality after rejection. The aggregate must additionally prove stale-baseline fresh apply, stale source/reference/preimage resume, and forged or nonexistent resume selection with byte-identical workspace and transaction trees.

### Double-contender proof remains incomplete

The contender test manually removes the killed canonical lease, so it cannot prove stale-lease quarantine. Its barrier loop has no deadline or guaranteed child cleanup, and its eventual successful commit does not prove loser-side zero mutation before the winner. It needs bounded orchestration, exact pre/post workspace and transaction evidence, stale-lease coverage, exactly one owner, deterministic loser classification or retry as specified, and terminal cleanup including stale-lease backup evidence.

### Permanent crash/recovery proof remains incomplete

The dedicated aggregate now exercises a subset of the requested crash and failure paths, but it does not yet prove:

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

Its final-replan assertions currently check only `moves` in the failure matrix and only `regenerations` in the generator case. Acceptance requires every operation family and unresolved finding to be empty.

## Fresh checkpoint evidence

An independent run of the current seven-test dedicated aggregate completed in 41.86 seconds by Bun, 41.93 seconds wall clock, with `4 pass / 3 fail / 41 expectations`. The kill/recovery and stale-resume cases failed with `Taxonomy journal WAL identity or revision differs from its durable attempt`; the synchronized contender timed out at fifteen seconds. Individual durations included 5.31 seconds for the incomplete failure matrix and 12.57 seconds for the generator case. This checkpoint is neither functionally green nor close to the aggregate timing budget before the missing matrix is added.

After the empty-WAL and predecessor fixes, an independent filtered kill/recovery rerun progressed through WAL, nested backup, nested edit, and outer edit interruption cases, then failed the restore-exchange resume during validate-only rollback reconciliation with `rollback-state-drift: move`. The single incomplete filtered case took 15.62 seconds wall clock, already exceeding the complete aggregate budget. The restore-preparation-aware move tuple is therefore still incomplete.

The next source checkpoint admits the exact typed pending-restore tuple for an absent moved-and-edited destination and uses an independent copied restore candidate. An independent refreshed rerun of the currently covered kill/recovery subset is green at `1 pass / 0 fail / 19 expectations`; the test body took 6.23 seconds and Bun took 9.94 seconds. This closes that observed restore regression for the narrow subset, while leaving roughly five seconds for all omitted boundary cases under the final aggregate budget.

The latest engine bundle completed successfully as two modules, producing a 0.70 MB ticket-local audit artifact.

The latest focused permanent selector completed in 20.77 seconds with `6 pass / 0 fail / 86 expectations`; an immediately preceding green checkpoint took 16.59 seconds. This closes the earlier transient WAL, child marker, and embedded-rederivation failures, but confirms the monolithic filtered command cannot reliably meet the required sub-fifteen-second aggregate before the missing crash/concurrency cases have been added.

The previous red checkpoint completed in 11.14 seconds with `3 pass / 3 fail / 46 expectations`:

- symlink rollback failed because the journal WAL was occupied at `persistJournal`;
- the symlink interruption child exited `1` rather than the expected marker `73`;
- embedded cancellation failed exact current-authority rederivation because the affected pre-state digest differed.

The current checkpoint is functionally green for its narrow six cases, not transaction acceptance evidence. The permanent aggregate should be isolated from the 240-test monolith so the complete matrix can remain within budget.

The isolated aggregate now exists, but its per-test fifteen-second timeout is not the required aggregate performance gate. Acceptance requires three complete uncached executions of the full isolated file, each below fifteen seconds, with exact pass/fail/expectation counts recorded.

The strict CLI artifact authority gate is independently green at `4 pass / 0 fail / 177 expectations` in 108 ms. Its language-neutral golden currently covers eight attempt preparations, six preparation collections, eight edit tuples, five backup tuples, five edit-writer tuples, five backup-writer tuples, ten lease tuples, six restore tuples, and nine canonical JSON exchange tuples, with third-party candidate matching parity.

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

## Expanded Aggregate Checkpoint — 2026-08-27

The expanded dedicated aggregate is visible at source checksum `c82d56a07d9539d5ee10da91309eb35b8eda547a9672c139419940d9e9f7891`; the corresponding normalization source inspected here is `dfc4b7c28b52aa762f09ce24703ee72e105b6c992b9b08423e82d7ce2aec3ea7`. It now supplies distinct reference, embedded-root, symlink, and generator fixtures and asserts that each of the eight injected failure stages has nonempty operation authority. It also includes parent-kill loops for attempt creation, journal exchange, backup/edit writers, restore exchange, stale-lease acquisition, terminal cleanup, mixed generator recovery, synchronized contenders, thrown stale-lease restoration, and stale-input rejection.

Signoff remains withheld for the following P0 proof gaps:

- `expectCompleteSnapshot` checks only that a snapshot is nonempty and that its record strings are well formed. It does not compare the transaction tree with phase-specific expected paths, bytes, modes, or exact tuples. The parent-kill, failure, terminal, and stale-input cases therefore do not yet provide the required exact transaction-tree boundary evidence.
- The source emits reachable `transaction-initial-lease-json-canonical-exchanged` and `transaction-lease-json-canonical-exchanged` probes when publishing into an absent destination, but neither appears in the kill lists. The frozen matrix requires real previous-image exchange for WAL/journal publication; it does not require artificial mutable-lease or heartbeat semantics solely to make a lease JSON previous-image exchange reachable.
- The mixed generator assertion only checks the last journal state. It must prove exact ordered states for ordinal 1 and ordinal 2: `["rolled-back", "committed"]`. There are still no behavioral cases for duplicate attempt preparations, duplicate/colliding canonical ordinals, or append-only rejection with byte-identical trees.
- Global classification and restore/backup authority remain unproved against adversarial evidence. Missing permanent cases include malformed attempt/preparation siblings, forged backup leaves or modes, forged restore/preimage tuples, and distinct stale incoming-reference/preimage drift.
- The contender children have no bounded exit wait or guaranteed process-tree cleanup on a barrier/exit failure. The current assertion that the transaction tree merely differs after the winner commits does not isolate loser-side zero mutation or deterministic loser rejection.
- The rolled-back terminal test immediately starts a fresh committed attempt and then asserts the workspace differs from its pre-transaction snapshot. That conflates rollback-cleanup closure with the next attempt's mutation rather than proving the backup-only rolled-back tuple closes exactly.
- Kill cases end at rollback and do not establish a clean fresh retry for each relevant interruption family. The newly expanded aggregate has not yet supplied three complete uncached green runs below fifteen seconds.
- The normative transaction matrix also explicitly requires planning twice with byte-identical output, rejecting a stale second apply, and deterministic cancellation rollback or resume. These cases are absent from the dedicated aggregate and therefore absent from its focused Nx target even if related assertions exist elsewhere in the monolithic suite.

The pinned baseline authority is correctly independent of mutable Git HEAD: the source requires `expectedBaselineCommit`, checks its format and equality with the immutable plan baseline before transaction-tree access, and freshly rederives the source-tree digest before transaction mutation. Acceptance must preserve this plan-pinned contract rather than require equality with current branch HEAD.

### Caught boundary failures outside recovery ownership

The external and transaction validation order is structurally correct in the inspected source: fresh apply completes baseline, plan/artifact, global transaction-tree, workspace/preimage/reference/generator, and exact rederivation checks before cleanup or allocation; active resume validates WAL/journal, every recovery root, lease/backup/restore/edit preparations, durable tuples, and incoming references before lease quarantine or preparation mutation.

Two subsequent caught-failure holes remain:

- Initial attempt allocation does not place its first mutation under the cleanup `try`. The preparation directory is created, parent-synced, and the `transaction-attempt-preparation-mkdir` progress callback runs before the cleanup scope begins. A thrown callback leaves an empty preparation owned by the still-live caller PID, so another apply in that process rejects it as active. At the other end, a throw after preparation-to-canonical rename at `transaction-attempt-canonical-published` sees no preparation root to remove and leaves a live-PID canonical attempt. The entire allocation from first preparation mutation through canonical publication needs one ownership-aware recovery scope.
- During ordinary journal publication, a throw at `transaction-journal-previous-exchanged` occurs while the canonical attempt journal is absent and its exact predecessor is the WAL `⏮️.json`. The outer catch immediately reads the absent canonical path instead of bootstrapping from validated WAL evidence, so the caught failure cannot roll back. Parent-kill resume coverage does not prove this normal exception path.

The strict binary publication unions also accept unreachable duplicate evidence. Backup recovery accepts an outer candidate simultaneously with a nested writer candidate leaf. Edit recovery accepts the equivalent duplicate and can accept an exchanged preimage alongside a nested writer even though production removes the writer before exchanging the target. Exact tuple validation must reject these forged combinations before mutation.

### Expanded matrix checkpoint after ordinal and tuple fixes

The next source checkpoint requires every unpublished preparation to use the exact next ordinal and rejects outer-plus-nested backup/edit candidate duplication and writer coexistence with an edit target exchange tuple. The aggregate adds collision/malformed-sibling, forged backup/restore, double-plan, cancellation, and stale-second-apply cases, and includes the previously omitted reachable initial and normal lease JSON canonical-exchange probes.

The cancellation case is red: it waits for progress phase `staging-moves`, while production reports the phase as `staging`. An independent focused execution completed in 3.67 seconds with `0 pass / 1 fail / 3 expectations`, receiving `committed` instead of `rolled-back`. Moreover, the focused Nx router's default filters do not select the new ordinal/malformed-sibling, forged backup/restore, or double-plan/cancellation/second-apply tests, so the registered target is not yet the full permanent matrix.

The new `expectBoundaryTuple` helper improves tuple-specific assertions, but it still performs presence/count/shape checks rather than comparing a normalized phase snapshot with an exact expected transaction tree including every path, file byte payload, mode, directory, and symlink target. Exact boundary snapshot acceptance remains open.

### Runtime audit of expanded shards

Independent shard executions exposed the following checkpoint failures:

- Attempt/initial publication: `11 pass / 1 fail / 204 expectations`, 18.64 seconds. The assertion accidentally counted the preparation root and all three child directories as preparations; after that assertion fix, the registered 14.5-second bundled shard still timed out.
- Journal/WAL: `4 pass / 1 fail / 92 expectations`, 14.45 seconds. A kill at `transaction-journal-candidate-written` failed during lease acquisition because candidate validation required the backup root to contain only stored backup leaves after stale/preparing lease evidence had already been created. The next source checkpoint adds an already-classified-preparation mode for this revalidation; it still needs a refreshed run.
- Backup: green at `6 pass / 0 fail / 113 expectations`, but 17.15 seconds and therefore requires smaller shards or equivalent proven optimization.
- Edit: green at `6 pass / 0 fail / 113 expectations` in 13.20 seconds.
- Restore: green at `4 pass / 0 fail / 74 expectations` in 10.72 seconds.
- Lease: green at `7 pass / 0 fail / 148 expectations` in 14.02 seconds, leaving inadequate contention/build margin for a three-run acceptance gate.
- Four early injected-failure cases were green but took 20.32 seconds in one synchronous shard. The three later non-generator cases included a red symlink case because the fixture produced zero symlink-target edits. The generator failure case rejected its fresh retry with an affected-pre-state rederivation mismatch.
- Terminal cleanup plus contender was green but took 19.18 seconds in one shard.
- The mixed-generator case failed because its killed workspace snapshot did not contain the expected mixed output.

The generator and symlink failures share a fixture-template authority defect. The new template cache commits a template repository and copies it into each fixture clone. The generated fixture script embeds the template repository's absolute output path, so every clone writes into the shared template rather than its own workspace. The symlink template similarly stores an absolute raw target under the template root, which points outside every clone; its absent non-moving target also creates no retarget operation. Path-dependent fixture configuration must be runtime-relative or applied safely per clone.

The runtime-relative generator output fix closes the functional generator regressions: the mixed process-tree case is green at `1 pass / 0 fail / 20 expectations` in 7.58 seconds and explicitly proves journal states `["rolled-back", "committed"]`; the ordinary after-regenerations rollback case is also green. Removing the fixture check target brings the registered generator rollback shard to 10.35 seconds while retaining its Nx generator execution. The refreshed journal/WAL shard is green at `5 pass / 0 fail / 99 expectations` in 7.93 seconds, and backup is green at `6 pass / 0 fail / 113 expectations` in 10.30 seconds.

### Complete focused gate remains red and leaks sibling process groups

After registering all static adversarial cases and splitting initial and lease groups, an independent complete script-gate run exited `1` at 15.3 seconds. Nineteen concurrent Bun shards overloaded their fixture children: the terminal cleanup test timed out waiting ten seconds for its boundary marker, and another shard hit the 14.5-second budget. The first initial split is independently green but reports 14.47 seconds, leaving no contention margin; the registered bundled lease group independently times out and must remain split.

The failure path also exposed a harness ownership defect. Each `runTestBudgeted` child is a detached process group, but one failing promise calls `process.exit(1)` immediately. Sibling detached shard groups therefore outlive the router. Four exact test-owned child processes were observed reparented to PID 1 after the failed gate, including blocked WAL-boundary and mixed-generator fixtures. They exited on their own roughly thirty seconds later; a subsequent process census was empty. The aggregate runner must coordinate shard completion/failure and terminate every owned shard process group in a shared `finally` instead of exiting from one promise while siblings remain active.

### Coordinated harness and latest acceptance checkpoint

The router now uses one coordinated process owner: it starts every shard itself, records nested child pids in a ticket-local registry, kills every shard process group and registered nested group on the first failure or deadline, waits for every shard exit, and verifies that registered pids are no longer alive. An independent timed failure left no matching Transaction v2 process in the post-run census. This closes the previously observed orphan leak.

The redundant fresh commit and empty-replan work was removed from every parent-kill boundary. Fresh retry remains explicitly covered by the injected-failure flow, while the mixed generator still proves ordinal 1 `rolled-back`, ordinal 2 `committed`, and a fully empty replan. This matches the clarified acceptance scope: per-boundary cases need exact recovery/rollback evidence, not forty duplicate fresh commits.

The inspected source still completes fresh baseline, canonical plan/artifact, global transaction sibling, workspace/preimage/reference/generator, and exact source rederivation validation before cleanup or attempt allocation. Resume still completes selected WAL/journal, global recovery-root, lease/backup/restore/edit, durable tuple, and incoming-reference validation before lease acquisition mutates stale evidence. Allocation's first `mkdir` is now inside its ownership cleanup scope, and the ordinary journal exception path bootstraps an absent canonical journal from its validated WAL previous image.

Two P0s remain at this checkpoint:

- `expectBoundaryTuple` still validates a generic snapshot encoding, canonical JSON, and selected per-phase counts or presence. It does not compare the normalized complete transaction and workspace trees with phase-specific exact expected path, byte, mode, directory, and raw-symlink records. Exact recovery/rollback boundary snapshots therefore remain unproved.
- A complete direct script-gate diagnostic reached the coordinated fourteen-second deadline and exited `1`; `/usr/bin/time` reported `real 14.97`. The aggregate test source changed during the execution (`ac96992d...` to `867ad06a...`), so the run is invalid as acceptance evidence in any event. Coordinated cleanup left an empty process census. Three unchanged-source uncached Nx executions below fifteen seconds have not been produced.

The writer's next checkpoint began hashing a canonical ledger containing every normalized transaction and workspace path, node kind, mode, regular-file byte payload, canonical JSON value, and raw symlink target. Stable plan, transaction, and operation digests must remain byte-exact; only proven runtime nondeterminism such as the fixture-root prefix, process id, and UUID lease token may be normalized. Hash-only boundary values are still too opaque to serve as independently reviewable snapshot authority. The permanent language-neutral golden must retain the full normalized ledgers, deduplicate them by digest, and map every boundary key to its exact transaction and workspace ledger digests (or an equivalently human-reviewable structure).

Exact rollback coverage must include not only killed/resumed crash phases and the mixed generator, but also the final transaction and workspace trees for all eight injected failure stages, deterministic cancellation, and the caught canonical-attempt-publication and journal-previous-image callback failures. Those cases currently assert exact workspace restoration but only a terminal journal-leaf shape for the transaction tree.

For human review and exact byte authority, every regular-file ledger node must retain its bytes as base64 in addition to its SHA-256; every symlink node must retain its normalized raw target plus the target hash; every JSON node must retain the complete normalized canonical value (or its exact normalized canonical bytes) in addition to any selected semantic summary. A path/mode/size/per-file-hash list with only selected JSON fields remains opaque and is not the required full normalized ledger.

The next capture supplies `98` boundary keys: `43` killed, `43` recovered, one mixed-generator committed, and `11` explicit rollback finals. It deduplicates them through `61` transaction ledgers and `9` workspace ledgers, and each node now retains the required bytes or raw target plus its digest. Self-consistency inspection found every boundary digest resolvable and each ledger digest, file byte digest/size, symlink target digest, path ordering, and unique path set internally consistent.

That checkpoint is not green. A single-process diagnostic first exposed zero symlink-retarget authority because the relative link target did not move. The next fixture revision restores nonempty authority, but a focused rerun is red only against its now-stale rollback ledger. More importantly, that revision uses one fixed fixture path, ignores the case name, and recursively deletes the shared path at test start. Concurrent gates or developers can therefore destroy one another's fixture. The permanent fixture must retain a unique temporary root and use a deterministic relative logical target that is genuinely renamed, rather than obtain determinism through a shared mutable path.

The fixture returned to unique ticket-local temporary roots and relative logical targets, and production validation now resolves relative raw symlink targets against the link's source path. The golden integrity case also rejects extra boundary keys and orphan ledgers, recomputes every ledger, file-byte, and symlink-target digest, and restricts the canonical-JSON requirement to transaction ledgers; pretty workspace JSON remains admissible while its full normalized canonical value and bytes stay pinned. These changes have not yet received an unchanged-source complete green run.

A subsequent complete diagnostic again reached the coordinated fourteen-second deadline and exited `1` at `real 14.88`. Its source, test, and golden remained stable, but the runner changed during execution from checksum `1bbf70cb...` to `5e4e698c...`, invalidating it as acceptance evidence. Process cleanup again left an empty census. The latest runner schedules three shards followed by five shards under one global deadline and uses a cross-shard registry to require every exact golden boundary exactly once; it still needs a clean functional and timing run.

The human-reviewable golden integrity and relative-symlink rollback selector is independently green at `2 pass / 0 fail / 804 expectations` in `1.23` seconds of Bun time. Source, test, and golden were unchanged across that run, but the router itself changed during the selector, so this is content evidence rather than runner acceptance.

Performance work now caches one exact active reference fixture per shard by killing a reference transaction at the WAL boundary once, then making independent copy-on-write clones for later journal, backup, edit, restore, and lease interruption cases. This preserves independent mutable roots and substantially reduces repeated preparation work, but it changes later killed-boundary context from a fresh attempt to a resume after stale-lease admission. The publication code and required tuple are still exercised; its exact captured ledgers must remain the review authority for that stronger nested-recovery context.

One nine-shard one-wave diagnostic overloaded child startup: mixed-generator, restore, lease, terminal, and stale-input cases exceeded their ten-second marker waits, and the outer run exited `1` at `real 15.11`. The test source changed during that execution, invalidating it as acceptance evidence. Later four-shard schedules are still under active iteration.

The mixed-first fanout strategy introduces a runner race if it waits for the first shard's ready marker before registering that child's `exit` and `error` listeners. A fast shard can exit in that gap, lose its terminal event, and leave the router waiting until its global deadline. The runner must create each child-exit promise immediately at spawn time or explicitly resolve already-terminal children before listener attachment.

### Launch registration P0

The focused Nx target exists as `@semio-tech/repo-lib:test-transaction-v2`, but there is no focused Transaction v2 configuration in the developer launch registry. More critically, the existing production taxonomy-apply registration is now invalid: both `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json` invoke `workspace:clean-taxonomy-apply` with ticket and plan only, while the CLI now requires the immutable `--baseline` authority. Both ordered launch entries must pass `${env:TAXONOMY_BASELINE}` and remain aligned.

### Exact-ledger and launch closure; focused timing remains red

The current exact snapshot authority closes the earlier transparency P0. The golden contains all `98` expected boundary keys and maps each one to deduplicated complete transaction and workspace ledgers. Its file nodes retain `bytesBase64`, SHA-256, size, and mode; symlink nodes retain the normalized raw target and its hash; JSON nodes retain both exact normalized canonical values and byte payloads. The permanent integrity test rejects missing and extra boundary keys, orphan ledgers, invalid path order or duplicates, unresolved digest references, altered ledger digests, altered file bytes/size/digests, and altered symlink targets/digests. Exact rollback finals cover every injected failure stage, deterministic cancellation, caught attempt-canonical publication, and caught journal-previous-image exchange. No remaining structural source or exact-golden P0 was identified at source/test/golden checksums `86f90f2e...`, `59fdcfb5...`, and `6f272f15...`.

The launch registration P0 is also closed. Both `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` now pass `--baseline ${env:TAXONOMY_BASELINE}` to taxonomy apply and contain aligned ordered entries for `bun nx run @semio-tech/repo-lib:test-transaction-v2 --skip-nx-cache`.

The current acceptance P0 is functional/timing proof. With runner checksum `100364c6...`, an unchanged complete direct gate exited `1` at `real 14.89`: the internal fourteen-second deadline fired. The mixed-generator shard completed green in `10.34` seconds of Bun time, but the three shards spawned only after its ready marker were killed together about `9.45` seconds after their delayed starts and never completed. This is scheduling/resource contention, not the earlier post-shard event-listener hang. Coordinated cleanup left an empty process census.

An isolated early/static filter then reached twenty green cases before the same old deadline killed it at `real 14.65`. The runner changed during that diagnostic to checksum `62ec0acd...`, removing the mixed-ready barrier and spawning all four shards immediately, so the isolated run is not runner acceptance evidence. Source, test, and golden remained unchanged. Signoff stays withheld pending one complete functional green followed by three uncached Nx runs on identical source, test, golden, and runner bytes, each below fifteen seconds.

The cross-shard shared-template checkpoint made the frozen golden stale. A complete run at source `86f90f2e...`, test `204459b0...`, golden `6f272f15...`, and runner `81701316...` produced widespread exact transaction-ledger failures before again reaching the deadline at `real 14.69`; workspace ledger digests remained exact. An isolated capture proves the difference is a real stable plan digest and every dependent transaction path/JSON value: the current fixture produces `8b0d0610...`, while the frozen ledger still encodes `87a2fbce...`. No PID, UUID, or placeholder ordering differs. The writer separately reports byte-identical full normalized ledgers across two shared runs and the same `8b0d0610...` digest from a direct nonshared-template run, so this is deterministic stale-golden authority rather than shared-template path leakage. Stable plan/transaction/operation hashes must remain unnormalized. The golden must be recaptured only after fixture semantics and grouping stabilize, then independently rechecked for all `98` exact keys, no missing/orphan ledgers, complete bytes/raw targets, and digest self-consistency.

## Final Independent Disposition — 2026-08-27

The read-only review freezes and signs off the following stable checkpoint:

- normalization source: `86f90f2e954e8082e0a6f9b0f5432a1e0131f86137624312e945849a602dc76f`
- dedicated aggregate: `22099778a38e0107cdadae4762010ba4f001bd484efb924ca350ee6c51b0539c`
- exact golden: `e3c9dbad890beda23b7ed8233cb027ccd9374dc77ed72beb077c55ba2fd4138d`
- aggregate runner: `e5e205edf9bf00643ed29bb05b5ba3f9a92363186f31f5b21f7bebfae92fd1f4`

Structural transaction-v2 signoff is granted. The inspected fresh path finishes immutable-baseline, canonical plan/artifact, globally classified transaction siblings, workspace/preimage/reference/generator, and exact source rederivation validation before cleanup or allocation. The resume path finishes WAL/journal, global recovery-root, lease/backup/restore/edit, durable tuple, and incoming-reference validation before lease acquisition mutates anything. Allocation and ordinary journal previous-image callback failures are ownership-recovered. Strict sibling/tuple unions reject malformed, duplicate, colliding, forged, and unreachable evidence. Ordinals are append-only. Nested backup/edit and restore publication, real journal previous-image exchange, initial/normal lease exchange, stale quarantine, atomic nonempty publication, contender fencing, stale-input zero mutation, rollback, retry, terminal cleanup, and mixed-generator ordinal-one rollback to ordinal-two commit all have permanent behavioral authority. The pinned baseline remains the immutable configured `9f449b10659b95148c8bcb3f91ce583bf7446973`, never mutable branch HEAD.

Exact golden signoff is granted. The recapture is a closed set of `98` boundaries: `43` killed, `43` recovered, `11` rolled back, and `1` committed. It contains `63` digest-deduplicated transaction ledgers and `9` workspace ledgers, with no missing or orphan ledger. Every regular file retains bytes, SHA-256, size, and mode; every symlink retains normalized raw target and target hash; every transaction JSON leaf retains the complete normalized canonical value and bytes. Stable plan, transaction, and operation hashes are preserved. The independent permanent integrity selector passed on unchanged bytes with `1 pass / 0 fail / 804 expectations`, `real 0.53` seconds.

Cleanup and registration signoff is granted. Coordinated failure paths repeatedly left an empty matching process census, including timing kills; the final census is also empty. Parent cleanup is scoped to the run-owned ticket prefix. The Nx target is registered as `@semio-tech/repo-lib:test-transaction-v2`. Both ordered launch registries pass the mandatory `${env:TAXONOMY_BASELINE}` to taxonomy apply and expose the uncached focused target.

Only performance acceptance is withheld. The saturated shared host did not produce any qualifying complete uncached Nx run below fifteen seconds, so the acceptance count is `0/3`. The most recent complete diagnostic remained red at `real 14.72` because its non-generator groups reached the internal deadline while unrelated long-running Nx/dev workloads were active; isolated current groups and the generator groups were functionally green, and cleanup remained exact. After integration on a quiet host, run `bun nx run @semio-tech/repo-lib:test-transaction-v2 --skip-nx-cache` three times without changing the four frozen files and record each wall time below fifteen seconds. No other Transaction Plan/Journal v2 P0 remains in this review.

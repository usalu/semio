# H-Transaction Dispositions Design

## Decision

The normalization transaction needs an incompatible **plan schema v2 and journal schema v2**. Three dispositions must become digest-owned operations rather than exceptions or direct cleanup:

1. absolute repository-local symlink target edits;
2. evidence removals with explicit retained authority;
3. embedded ticket-root relocations with exact many-to-one handling.

The existing file-move, structured-edit, and generator operations remain, but they are insufficient for these cases. The implementation must add four plan arrays:

- `symlinkTargetEdits`;
- `evidenceRemovals`;
- `embeddedTicketRootRelocations`;
- `embeddedTicketRoots`, which binds leaf operations into an exact no-follow source-root closure.

It must also replace the current one-sided affected digest with exact affected **pre-state and post-state** digests. Staged removals and embedded root structures must remain recoverable until the journal has durably entered `committed`.

No production, taxonomy, test, physical-tree, Compose, temp/Compose, AGENTS, or Git state was modified. This report is the only write.

## Authority evidence

The design uses these authority reports verbatim:

- `📓️s-absolute-symlink-authority.md`, SHA-256 `9a1ef369ff506008e2d791cec37df92a216675edd0879ad99e3715a7783fb395`;
- `📓️s-platform-path-sentinels.md`, SHA-256 `25c4e55bd9021fd83d634a07281a930465ab42a10db78bc58f88a52726dd1ff8`;
- `📓️s-misplaced-ticket-roots.md`, SHA-256 `abc20aaaa95ec4dce3362690c511f271fe8de643370a6c3001ad3fdf76810501`.

The bounded current evidence is:

| Disposition | Evidence | Planned result |
| --- | --- | --- |
| Repository-local absolute symlink | 13 links: 11 print harness links and 2 stale-config probe links; all logical targets are repository-local and absent | 13 first-class target edits; preserve links and logical target paths, replace absolute target text with final-path-relative text |
| Trailing-space FEM log | 1 hostile one-space basename, 38,467 bytes, content hash `d5246ed036182a3505508fe7c264d1da42fac2d46b2ff391ca3c251cbbebf7fc`; 3 byte-identical named siblings | 1 evidence removal with all 4 preimages in one authority digest; retain all 3 named files |
| Embedded ticket roots | 6 `CACHEDIR.TAG` leaves, each hash `6d9d1d216e0f83abc5e5662ca62c92b4f23009466b54fa27321a69acdb778bb2`, below 3 nested `.🧬semio` roots | 4 installs at canonical ticket destinations plus 2 redundant-evidence removals; stage all 3 emptied nested roots |
| Windows-reserved sentinel | Previous retained execution fixture `CON.ts` | Preserve the case in a serialized, language-neutral golden and pure policy test; emit a physical removal only if an exact live preimage and serialized authority coexist at plan time |

The current exact active-ticket lookup found no physical `CON.ts`; the transaction must not invent a removal when the source is absent. The permanent test still must stop materializing reserved path nodes on native Windows.

The six embedded source leaves resolve by removing the production/package prefix before the nested metadata root:

```text
<package-prefix>/.🧬semio/🦑️repo/🎫️tickets/<ticket-relative-path>
→ .🧬semio/🦑️repo/🎫️tickets/<ticket-relative-path>
```

There are three sources for the one canonical `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-os-errors/CACHEDIR.TAG`. If the destination is absent, the source with the bytewise-smallest UTF-8 path is the deterministic installer; the other two are removals retained by that destination. The other three source leaves install one-to-one. If an exact destination is already occupied with identical bytes, mode, kind, and role, it becomes the retained authority and every source for that destination is a removal. Conflicting occupancy blocks.

## Current implementation boundary

Snapshot hashes while this design was produced:

- normalization engine `🧹️normalization/🟦️.ts`: `46c6853d63bc6c1e3a0fd5bbd43b69a097f5a03c2038c38d510fb714dc212135`;
- shared test `📦️packages/🟦️typescript/🧪️index.test.ts`: `c182260c63b2fb095193f60b57d49bc4a66e1a202511dc6128bf0ff3cb1fb656`.

Relevant production regions are:

| Region | Current behavior | Gap |
| --- | --- | --- |
| Contracts, lines 37–229 | Plan v1 contains moves, text edits, regenerations; journal v1 tracks move/edit/generator state | No target-edit, removal, embedded-root, typed-backup, or pre-state contract |
| Inventory candidates/content, lines 1376–1546 | No-follow symlink hash is SHA-256 of raw target text | Frozen public entries omit mode, size, and raw symlink target needed for exact authority |
| Path policy, lines 2388–2406 | Reserved and trailing segments always produce errors | No proof-bearing disposition can supersede a violation |
| Inventory leaf handling, lines 2973–3135 | Absolute symlinks always error; nested ticket roots enter ordinary production/package classification | No repository-local absolute target proof; no embedded-root classifier |
| Planning/digests, lines 3322–3402 | Every same-destination collision blocks; post digest covers destinations/edited paths/generator summaries | Cannot prove identical many-to-one evidence; ignores move sources, absence, removals, modes, symlink retargets, retained authorities |
| Journal internals, lines 3464–3835 | Backups are string-tagged; parent pruning is derived and unjournaled | No precise operation replay; recursive evidence cleanup would be unsafe |
| Apply, lines 3837–4020 | Move preflight occurs after journal creation; stage is removed before `committed` is persisted | A removal would become unrecoverable in the crash window; no new-operation resume/cancel semantics |
| Shared tests, lines 2620–3208 | Failure rollback/cancel/convergence exist for moves/edits; snapshot reads files | No resume test, no no-follow symlink snapshot, no removal/root disposition golden; physical `CON.ts` fixture is not Windows-safe |
| Root CLI, `📜️script.ts` lines 17580–17770 | Parses plan by unchecked cast and reports only move/edit/regeneration counts | Must strictly parse v2 and report all operation counts |

Useful reusable mechanisms are `contentOf` and `hashPath` for no-follow symlink identity, `backupPath`/`restoreBackup` for ordinary edited files, canonical operation IDs, two-phase rename staging, opaque-prefix checks, generator preimage validation, failure injection, and the existing empty-second-plan acceptance test.

## Exact public contracts

### Shared preimages and tree evidence

```ts
export type TaxonomyDispositionLeafKind = "file" | "symlink";

export interface TaxonomyLeafPreimage {
  readonly nodeKind: TaxonomyDispositionLeafKind;
  readonly contentHash: string;
  readonly mode: number;
  readonly size: number;
}

export interface TaxonomyPathPreimage {
  readonly state: "absent" | "directory" | "file" | "symlink";
  readonly contentHash?: string;
  readonly mode?: number;
  readonly size?: number;
}

export interface TaxonomyNoFollowTreeDigest {
  readonly algorithm: "sha256-no-follow-merkle-v1";
  readonly digest: string;
  readonly files: number;
  readonly directories: number;
  readonly symlinks: number;
  readonly others: number;
}
```

`contentHash` is SHA-256 of file bytes or raw symlink target UTF-8 bytes. `mode` is `lstat.mode & 0o7777`; equality is required within an evidence set. `TaxonomyPathPreimage` has no optional payload when `state === "absent"`, and requires all payload fields for a present leaf. A directory preimage is represented by a no-follow tree digest when recursive identity matters.

The new no-follow digest must share the safe traversal implementation with `opaqueTreeDigest`, but it is not an opaque exclusion and must not require a `pathExclusions` registration.

### Symlink target edit

```ts
export interface TaxonomySymlinkTargetEdit {
  readonly operationId: string;
  readonly sourcePath: string;
  readonly finalPath: string;
  readonly oldTarget: string;
  readonly newTarget: string;
  readonly oldTargetHash: string;
  readonly newTargetHash: string;
  readonly logicalTargetSourcePath: string;
  readonly logicalTargetFinalPath: string;
  readonly logicalTargetPreimage: TaxonomyPathPreimage;
  readonly windowsLinkType: "file" | "dir";
  readonly sourceTargetDigest: string;
  readonly rationaleRule: "repository-local-absolute-symlink-target-v1";
  readonly ownerId: string;
}
```

`sourceTargetDigest` is SHA-256 of canonical JSON containing `sourcePath`, `finalPath`, raw old/new targets, logical source/final target paths, and logical target preimage. `operationId` is the first 24 lowercase hex characters of SHA-256 over `"symlink-target-edit\0" + canonicalJson(all fields except operationId)`.

`newTarget` is always a non-absolute forward-slash path rendered with `posix.relative(dirname(finalPath), logicalTargetFinalPath)`. Its lexical resolution from `finalPath` must equal `logicalTargetFinalPath` exactly after NFC normalization. `windowsLinkType` is not guessed: an existing target supplies its proven kind; an absent target must have a schema-resolved file/directory contract. Ambiguous absent extensionless targets block. The 13 current targets resolve as files.

### Evidence removal

```ts
export interface TaxonomyEvidenceMember {
  readonly sourcePath: string;
  readonly finalPath: string;
  readonly disposition: "remove" | "retain" | "relocate";
  readonly preimage: TaxonomyLeafPreimage;
}

export type TaxonomyRemovalAuthority =
  | Readonly<{
      kind: "byte-and-mode-identical";
      evidenceSetDigest: string;
      retainedFinalPath: string;
      members: readonly TaxonomyEvidenceMember[];
    }>
  | Readonly<{
      kind: "serialized-path-sentinel";
      fixturePath: string;
      fixtureContentHash: string;
      caseId: string;
      serializedInputPath: string;
      expectedViolationCode: "windows-reserved-name" | "trailing-dot-or-space";
      authorityDigest: string;
    }>;

export interface TaxonomyEvidenceRemoval {
  readonly operationId: string;
  readonly sourcePath: string;
  readonly preimage: TaxonomyLeafPreimage;
  readonly authority: TaxonomyRemovalAuthority;
  readonly embeddedTicketRootId?: string;
  readonly rationaleRule: "redundant-ticket-evidence-v1" | "serialized-platform-sentinel-v1";
  readonly ownerId: string;
}
```

`evidenceSetDigest` is SHA-256 of canonical JSON containing the algorithm ID `sha256-byte-mode-evidence-set-v1`, the bytewise-path-sorted complete member set, and `retainedFinalPath`. Every `byte-and-mode-identical` member must have the same node kind, content hash, mode, and size. At least one member must be `retain` or `relocate`; the retained final path must equal that member's final path. The one-space FEM operation includes all four files, not merely one arbitrary sibling.

A serialized sentinel removal is permitted only when the permanent golden contains the exact hostile input path, expected violation code, and source bytes or content hash. A violation by itself never authorizes deletion. The operation is absent when no physical source exists.

The operation ID is the first 24 hex characters of SHA-256 over `"evidence-removal\0" + canonicalJson(all fields except operationId)`.

### Embedded ticket-root group and leaf relocation

```ts
export interface TaxonomyEmbeddedTicketRootDisposition {
  readonly operationId: string;
  readonly sourceMetadataRoot: string;
  readonly sourceTicketRoot: string;
  readonly canonicalTicketRoot: string;
  readonly ticketId: string;
  readonly sourceTreeDigest: TaxonomyNoFollowTreeDigest;
  readonly incomingReferenceDigest: string;
  readonly relocationOperationIds: readonly string[];
  readonly removalOperationIds: readonly string[];
  readonly rationaleRule: "embedded-ticket-root-relocation-v1";
}

export interface TaxonomyEmbeddedTicketRootRelocation {
  readonly operationId: string;
  readonly embeddedTicketRootId: string;
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly relativeEvidencePath: string;
  readonly preimage: TaxonomyLeafPreimage;
  readonly fixedContractId?: string;
  readonly ownerId: string;
  readonly rationaleRule: "embedded-ticket-root-relocation-v1";
}
```

`sourceMetadataRoot` is the exact nested `<package-prefix>/.🧬semio`; `sourceTicketRoot` ends at the validated ticket slug; `canonicalTicketRoot` is `.🧬semio/🦑️repo/🎫️tickets/<date>/<slug>`. `ticketId` uses the existing `YY/MM/DD/SLUG` identity. The group tree digest covers the entire nested metadata root without following symlinks. Every leaf in that digest must be represented exactly once by either a relocation or a referenced evidence removal; unexpected leaves block.

`incomingReferenceDigest` uses `sha256-taxonomy-reference-set-v1` over the sorted set of resolved textual and lexical symlink references entering `sourceMetadataRoot`. Planning requires the set to be empty, and apply recomputes it before mutation. This is stronger than trusting a stale `referencesIn: []` snapshot.

The group operation ID hashes all group fields except the ID. A leaf relocation ID hashes all leaf fields except its ID. The group's referenced IDs must exist, be unique globally, point back to the group, and exhaust the source tree's leaves.

### Plan v2

```ts
export interface TaxonomyPlan {
  readonly schemaVersion: 2;
  readonly taxonomySchemaVersion: 7;
  readonly baselineCommit: string;
  readonly scope?: string;
  readonly sourceTreeDigest: string;
  readonly excludedTreeDigests: readonly OpaqueTreeDigest[];
  readonly moves: readonly TaxonomyMove[];
  readonly embeddedTicketRoots: readonly TaxonomyEmbeddedTicketRootDisposition[];
  readonly embeddedTicketRootRelocations: readonly TaxonomyEmbeddedTicketRootRelocation[];
  readonly symlinkTargetEdits: readonly TaxonomySymlinkTargetEdit[];
  readonly evidenceRemovals: readonly TaxonomyEvidenceRemoval[];
  readonly edits: readonly ReferenceEdit[];
  readonly regenerations: readonly TaxonomyRegeneration[];
  readonly unresolved: readonly TaxonomyViolation[];
  readonly expectedAffectedPreStateDigest: string;
  readonly expectedPostStateDigest: string;
  readonly planDigest: string;
}
```

Missing arrays are invalid; they are never defaulted to empty. Old plan v1 parsing is removed. Add an exported strict `parseTaxonomyPlan(value: unknown): TaxonomyPlan`, and make both the root CLI and `applyTaxonomyPlan` call it before digesting. Unknown keys, aliases, wrong versions, noncanonical paths, invalid hashes, duplicate IDs, mismatched IDs, unsorted evidence members, and dangling references fail before mutation.

`TaxonomyApplyResult` must add exact `appliedEmbeddedTicketRootRelocations`, `appliedSymlinkTargetEdits`, `appliedEvidenceRemovals`, and `appliedRegenerations` counts. Root plan/apply summaries must expose them.

## Canonical detection and planning

### Absolute repository-local symlinks

Detection must operate on `lstat` plus raw `readlink`; it must never dereference the link.

1. Recognize POSIX-rooted, Windows drive-rooted, and UNC absolute syntax explicitly rather than relying only on the host flavor of `node:path.isAbsolute`.
2. Compare target and current repository root using the same path flavor. Containment must use a relative-path calculation or an exact separator boundary; `/repo-other` must not match `/repo`.
3. Reject another drive/share, any lexical escape, NUL, an empty/unprovable target, or a target under an opaque prefix.
4. Strip the exact repository root to obtain `logicalTargetSourcePath`; no filesystem existence is required.
5. Resolve `finalPath` from the link entry's complete move/relocation mapping. Resolve `logicalTargetFinalPath` through the unique longest source-directory mapping, even when the broken target itself has no inventory entry. Ambiguous mappings block.
6. Render and re-resolve `newTarget`; it must be relative, NFC, nonempty, and remain outside every opaque prefix.
7. Freeze target existence/kind as `logicalTargetPreimage`. A preimage change requires replan.
8. Supersede `symlink-absolute-target` only when the exact operation is present. External absolute targets and opaque targets remain errors.

Relative symlinks are not silently rewritten by this authority. A future relative-target move feature would need its own explicit contract.

### Redundant evidence

The planner may emit `byte-and-mode-identical` only when all of these are true:

- every member is a non-directory leaf with exact bytes, kind, mode, and size;
- semantic role/fixed contract and ticket owner agree;
- every remove source has zero incoming references;
- `retainedFinalPath` is unoccupied or occupied by an exact member;
- no member crosses an opaque prefix or transaction root;
- the evidence set is complete and has a deterministic retained member;
- removing the source clears a policy violation or a declared many-to-one relocation collision.

For a basename made solely of trailing spaces/dots, normalization must never trim or construct an empty destination. The current one-space file is eligible because its three named same-parent records are all retained and exactly identical. A single byte/mode mismatch blocks.

### Embedded ticket roots

Lexically detect the exact segment sequence `/.🧬semio/🦑️repo/🎫️tickets/` when `.🧬semio` is not the registered repository metadata root. Classification happens before package-role and semantic-name analysis, so package discovery does not treat ticket evidence as package implementation.

Eligibility requires:

1. the suffix resolves to exactly one ticket root accepted by `fixedDirectoryContracts.ticket-slug`;
2. the matching canonical ticket root and its exact ticket manifest authority exist;
3. the nested metadata root contains no symlink and no unexpected non-ticket subtree;
4. every leaf is exact ticket evidence or has a registered fixed contract such as `cargo-cache-tag`;
5. every destination is canonical, under the same ticket identity, at most 240 UTF-8 bytes, and outside opaque/transaction roots;
6. incoming textual and symlink reference set is empty;
7. destination occupancy is absent or exact byte/kind/mode/role identity;
8. every many-to-one destination is one complete evidence set with a deterministic installer or pre-existing retained destination.

Unknown content, invalid ticket identity, reference ingress, a nested symlink, conflicting occupancy, partial source-root coverage, or a noncanonical destination remains a blocking violation. There is no basename or package-prefix allowlist.

For the six current leaves, planning yields three root groups, four relocation operations, and two evidence removals. The trailing-space file adds one more evidence removal. The authority minimum is therefore 13 target edits, 4 relocations, 3 byte-identical removals, and 3 embedded-root group records; a serialized-sentinel removal is conditional on a current physical source.

## Conflict and preflight matrix

All checks complete before the journal or stage/backup directories are created.

- Strict-parse the plan and recompute every operation ID, evidence/tree/reference digest, affected pre-state digest, and `planDigest`.
- Verify the baseline commit exists and opaque digests are unchanged without traversing excluded trees.
- Require every source, destination, final link, logical target, retained authority, fixture authority, and embedded root outside opaque and transaction prefixes.
- Require global operation IDs unique across all arrays.
- A relocation/removal source cannot also be a normal move, text edit target, generator output, or another disposition source.
- A symlink edit may overlap one normal move only when `sourcePath` is that move's source and `finalPath` its exact destination. It cannot overlap a removal, embedded root, text edit, or generator output.
- Destination paths are unique unless exact identical occupancy is represented as retained evidence. No destination may be another staged removal source.
- Re-read every symlink target and compare raw text/hash; re-read every removal, relocation, retained member, serialized golden, and source-root tree preimage. No dereference.
- Recompute empty incoming-reference sets for embedded roots after applying the same lexical opaque filter used by inventory.
- Validate every destination parent can be created without a path-policy or collision violation.
- Check the cancel marker once before journal creation. Early cancellation leaves no transaction artifact.

Preflight drift returns an error and performs no mutation. It does not create a rolled-back journal for a transaction that never began.

## Affected-state digest v2

Replace content-hash-only destination rows with explicit path-state records:

```ts
type TaxonomyAffectedStateRow =
  | Readonly<{ path: string; state: "absent" }>
  | Readonly<{ path: string; state: "file"; contentHash: string; mode: number; size: number }>
  | Readonly<{ path: string; state: "symlink"; targetHash: string; targetSize: number }>
  | Readonly<{ path: string; state: "directory-tree"; tree: TaxonomyNoFollowTreeDigest }>
  | Readonly<{ path: string; state: "generator"; contentHash: string }>;
```

Rows are UTF-8-byte-path sorted, unique, canonical-JSON encoded, then SHA-256 hashed with algorithm identifier `sha256-affected-path-state-v2`.

The affected set includes:

- every normal move source as absent post-state and destination as present;
- every embedded relocation source as absent and destination as present;
- every evidence removal source as absent;
- every retained evidence/serialized authority path unchanged;
- every symlink edit source/final path with old/new raw-target hashes; if the link also moves, source is absent and final is the new symlink;
- every embedded source metadata root as a pre-state directory tree and post-state absence;
- every edited file's exact final bytes/mode/size;
- every generator output summary.

Conflicting duplicate rows are an error. `expectedAffectedPreStateDigest` is checked before journal creation and after rollback. `expectedPostStateDigest` is checked before commit. This closes current holes where a move source can remain, a mode can drift, or a removal can be skipped without affecting the digest.

## Journal v2

```ts
export type TaxonomyJournalState =
  | "prepared"
  | "staging"
  | "disposing"
  | "installing"
  | "retargeting"
  | "editing"
  | "regenerating"
  | "verifying"
  | "committed"
  | "rolling-back"
  | "rolled-back";

export type TaxonomyBackupRecord =
  | Readonly<{ kind: "absent" }>
  | Readonly<{ kind: "file"; backupPath: string; contentHash: string; mode: number; size: number }>
  | Readonly<{ kind: "symlink"; target: string; targetHash: string }>;

export interface TaxonomyJournalRecord {
  readonly schemaVersion: 2;
  readonly planDigest: string;
  readonly state: TaxonomyJournalState;
  readonly stagingRoot: string;
  readonly backupRoot: string;
  readonly preparedMoveIds: readonly string[];
  readonly stagedMoveIds: readonly string[];
  readonly installedMoveIds: readonly string[];
  readonly preparedEmbeddedRelocationIds: readonly string[];
  readonly stagedEmbeddedRelocationIds: readonly string[];
  readonly installedEmbeddedRelocationIds: readonly string[];
  readonly preparedEvidenceRemovalIds: readonly string[];
  readonly stagedEvidenceRemovalIds: readonly string[];
  readonly preparedEmbeddedRootIds: readonly string[];
  readonly stagedEmbeddedRootIds: readonly string[];
  readonly preparedSymlinkTargetEditIds: readonly string[];
  readonly stagedSymlinkTargetEditIds: readonly string[];
  readonly installedSymlinkTargetEditIds: readonly string[];
  readonly appliedEditPaths: readonly string[];
  readonly startedRegenerationIds: readonly string[];
  readonly completedRegenerationIds: readonly string[];
  readonly backups: Readonly<Record<string, TaxonomyBackupRecord>>;
  readonly error?: string;
}
```

Every `prepared*` marker is persisted before its first rename/replacement. Every `staged*`/`installed*` marker is persisted after the corresponding filesystem result is verified. Resume may therefore distinguish intent from completion and reconcile exactly one of the expected source/stage/destination states. String-tagged `@absent`/`@symlink:` backups are removed; typed records cannot alias raw target text.

## Apply order

1. **Preflight:** strict parsing, all conflicts/preimages/digests/references/cancellation; no writes.
2. **Prepare journal:** create exact transaction stage/backup roots and persist state `prepared`.
3. **Stage leaves:** rename normal move sources, embedded relocation sources, and evidence-removal sources into disjoint operation-ID stage paths. A removal is never unlinked.
4. **Dispose embedded roots:** after all represented leaves are staged, verify each source metadata root now contains only its frozen empty directory structure; rename the entire `.🧬semio` root into an embedded-root stage path. This preserves directory structure and modes for rollback without recursive deletion.
5. **Install:** rename normal moves and embedded relocation winners from stage to destinations. Retained identical destination occupancy has no installer operation.
6. **Retarget symlinks:** operate at each link's `finalPath`. Rename the old symlink itself into its symlink stage path, then create the new relative symlink and verify raw `readlink` text. This is no-follow and preserves the old link for rollback. Use the proven Windows link type.
7. **Edit:** apply grouped textual reference edits with typed backups.
8. **Regenerate:** preserve existing generator preview/preimage/check behavior under its own journal state.
9. **Verify:** exact projection checks, stale-token checks, affected post-state digest, embedded-root absence, retained evidence, policy sentinel golden, and opaque digests.
10. **Commit:** persist journal state `committed` **before** deleting stage/backup contents. Cleanup is idempotent and may be completed by a committed-journal resume.

The current `pruneEmptySourceParents` must not be responsible for embedded root disposal. It is unjournaled, runs after the affected digest, and does not see removal-only source parents. Ordinary empty-parent pruning can remain derived only if its absence is not contractual; every embedded `.🧬semio` root is instead staged explicitly.

## Rollback, resume, and cancellation

### Rollback

Rollback ignores cancellation and proceeds to completion:

1. persist `rolling-back`;
2. remove/recreate generator output roots and restore typed output backups;
3. restore textual edit backups;
4. remove each installed new symlink and rename its staged old symlink back to the link final path;
5. move installed normal/embedded destinations back to their leaf stages;
6. rename each staged embedded empty-root structure back to `sourceMetadataRoot`;
7. rename staged move, relocation, and removal leaves back to exact source paths;
8. verify `expectedAffectedPreStateDigest`;
9. clear active arrays, persist `rolled-back`, and retain the journal as evidence.

Any occupied rollback source/destination or pre-state digest mismatch stops with `rolling-back` retained and an exact error; it never overwrites concurrent data.

### Resume

Resume accepts only the canonical journal path and matching plan digest.

- `committed`: verify post-state, finish idempotent stage/backup cleanup, return committed counts.
- `rolled-back`: reject; a fresh transaction/journal is required.
- `rolling-back`: validate recoverable staged records, finish rollback, verify pre-state.
- active states: validate every operation's journal tuple against its only allowed source/stage/destination/current-target state before performing a write. Prepared-but-not-completed intent may be retried; completed state must match exact bytes/mode/target.
- Any intermediate drift is `resume-state-drift`; preserve journal and filesystem for explicit recovery rather than overwriting the drift.

Resume never reparses missing arrays as defaults and never reselects a different many-to-one winner.

### Cancellation

Check cancellation before journal creation, before and after each new operation, before/after each regeneration, before verification, and immediately before commit. Cancellation before the journal leaves no artifacts. Cancellation after it exists triggers full rollback and pre-state digest verification. No cancellation check interrupts rollback. A cancellation discovered after an individual atomic rename/retarget is safe because the completed or prepared marker plus stage location is sufficient to reverse it.

## Permanent fixture and test matrix

Add one language-neutral fixture at:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️transaction-dispositions/🔣️.json`

It should encode virtual preimage nodes, expected operations, expected affected pre/post rows, and negative cases. The reserved-name cases are virtual strings; tests must not create `CON`, `PRN`, `AUX`, `NUL`, `COM1`–`COM9`, `LPT1`–`LPT9`, or trailing-dot/space physical nodes.

Required cases:

### Symlink target edits

- broken repository-local absolute file target;
- link moves, target does not;
- target moves, link does not;
- both move;
- exact new relative target and second-plan absence;
- outside-root absolute, prefix-confusion `/repo-other`, another Windows drive/share, lexical escape, opaque target, ambiguous absent target kind, and target-text drift all block;
- no-follow proof: mutating a target's bytes never changes symlink preimage hash, while changing raw target text does;
- Windows file-type field is schema-derived, never inferred from a missing target at apply.

### Evidence removals

- one hostile blank basename plus three identical named retained records produces one four-member removal;
- one-byte, mode, size, kind, role, retained-path, or reference mismatch blocks;
- stage/failure/cancel restores the hostile source exactly;
- serialized `CON.ts` case produces the expected violation from the pure path-policy API without filesystem materialization;
- a physical sentinel removal is absent without both source preimage and serialized authority.

### Embedded ticket roots

- three nested roots/six markers produce exactly three group records, four relocations, and two removals;
- exact current duplicate destination selects the bytewise-smallest source installer;
- already occupied identical destination selects no installer and removes sources against the retained destination;
- conflicting destination, invalid ticket identity, unknown leaf, nested symlink, incoming reference, opaque crossing, over-budget destination, partial tree coverage, or non-ticket suffix blocks;
- package classification never reports embedded evidence as implementation/package glue;
- post-state has zero nested `.🧬semio` roots and exactly four canonical markers.

### Transaction protocol

- strict plan parser rejects v1, missing arrays, extra keys, duplicate/dangling IDs, wrong hashes, and mutated authority members;
- mutate each new plan array independently and prove `taxonomyPlanDigest` changes;
- stale preflight creates no journal and changes no workspace state;
- inject failures after leaf staging, embedded-root staging, relocation installation, symlink retargeting, editing, regeneration, and before verification; each restores the exact no-follow pre-state digest;
- cancellation from every new progress phase rolls back;
- child-process interruption at each active journal state, followed by resume, reaches the exact post-state; interruption during rollback resumes to exact pre-state; committed resume only cleans transaction artifacts;
- success yields exact applied counts, exact post digest, unchanged opaque digest, and an empty second plan with all seven operation arrays empty and no unresolved violations.

Replace `normalizationWorkspaceSnapshot` for these tests with a no-follow snapshot that records directories, file bytes/modes, and symlink raw targets. Its current `readFileSync`-based file snapshot can dereference a link and cannot prove symlink identity.

Use existing `fast-glob` as third-party parity for the ordinary fixture's no-follow pre/post leaf census, comparing it with the native walker. The virtual sentinel golden supplies language-agnostic policy coverage without creating checkout-hostile names. No runtime dependency is added.

## Verification and CLI behavior

`verifyTaxonomy` adds one required violation for every pending disposition:

- `symlink-target-edit-required`;
- `evidence-removal-required`;
- `embedded-ticket-root-relocation-required`.

It must not repeat the superseded raw `symlink-absolute-target`, trailing-space, reserved-name, collision, or path-budget violation when and only when an exact valid operation owns it. A malformed disposition leaves the original error and adds an authority error.

Root `clean taxonomy plan/apply` must:

- call `parseTaxonomyPlan`, not cast JSON;
- display and persist move, embedded relocation, symlink edit, evidence removal, text edit, and regeneration counts;
- include all counts in `TaxonomyApplyResult` and Markdown summaries;
- report the same canonical journal path and digest;
- require the post-apply verify/second plan to contain no operation of any class.

## Risks and mitigations

| Risk | Required mitigation |
| --- | --- |
| Broken symlink target cannot prove Windows file/directory type | Require schema-resolved `windowsLinkType`; block ambiguous absent targets. Current 13 targets are file-format paths. |
| Raw absolute target persists a user-specific checkout path in the plan | This is required preimage evidence; include it in plan/journal only, hash it in summaries, and never print it in ordinary progress output. |
| Crash after deleting staged evidence but before journal commit | Persist `committed` before cleanup; committed resume finishes cleanup. |
| Direct recursive deletion of embedded roots could consume unknown content | Freeze no-follow tree closure, stage every leaf, verify empty structure, then rename the exact root into transaction stage. Never `rm -rf` the live root. |
| Many-to-one grouping could conceal semantic conflicts | Require identical kind/bytes/mode/size/role/owner and complete evidence-set digest; otherwise block. |
| A new reference appears after planning | Recompute incoming reference set during preflight and include authority paths in affected pre/post digests. |
| Concurrent writer changes a retained evidence file | Exact retained-member preflight blocks before journal; resume drift never overwrites it. |
| Current post digest ignores absence and modes | Use affected path-state v2 rows and verify both pre and post. |
| Current journal string backup tags can alias target text and omit metadata | Replace them with strict typed backup records. |
| Native Windows cannot materialize reserved names | Test reserved/trailing policy through the serialized golden and pure policy function only. |
| Test snapshot follows symlinks | Use lstat/readlink no-follow snapshot plus fast-glob parity. |
| Empty-parent pruning is outside journal/digest | Stage embedded root structure explicitly; include source-root absence in post digest. |

## Disjoint implementation modules

After the CAD/Draw normalization writer reaches a stable boundary, this work can be divided without semantic overlap:

1. **Disposition contracts/detection:** public v2 types, strict parser, inventory mode/size/target evidence, absolute-target proof, evidence grouping, embedded-root classification.
2. **Transaction protocol:** affected state v2, preflight/conflict checks, staged removal/root/link primitives, journal v2, rollback/resume/cancel.
3. **Permanent tests/golden:** virtual sentinel golden, no-follow snapshots, planner negatives, failure/cancel/resume/convergence, fast-glob parity.
4. **CLI integration:** strict parse and operation counts in root `📜️script.ts` after the engine interfaces freeze.

The first two both touch `🧹️normalization/🟦️.ts` and must be serialized or use exclusive regions. Tests/golden and CLI can proceed in parallel after the public interface freezes.

## Acceptance boundary

- Plan and journal are strict schema v2 with no v1 fallback.
- Exactly 13 current absolute repository-local targets become relative without dereferencing or changing logical targets.
- External, escaping, ambiguous, and opaque absolute targets remain blocking.
- The one-space FEM file is absent; all three named 38,467-byte records remain with hash `d5246e…f7fc`; one four-member authority digest is journaled.
- All six nested cache markers are accounted for by exactly four installs and two redundant removals; three nested metadata roots are absent; four canonical destinations contain hash `6d9d1d…778bb2` with exact modes.
- No physical Windows-reserved/trailing-dot/space node remains, while serialized negative policy cases still reject every class.
- Every new operation participates in operation identity, plan digest, preflight, affected pre/post digests, progress, cancellation, rollback, resume, result counts, verification, and empty-second-plan checks.
- Failure and cancellation at every phase restore the no-follow pre-state digest; interrupted resume reaches either exact pre-state or exact post-state, never a mixed state.
- `committed` is durable before evidence cleanup; committed resume is idempotent.
- Compose and temp/Compose remain lexically excluded, untraversed, unread, and unchanged.

## Evidence commands

All commands were read-only. Searches used exact production/report paths or explicit `-g '!compose/**' -g '!temp/compose/**'` filters. The three authority ticket roots and three exact framework embedded roots contain no actual Compose prefix.

```text
sed -n <ranges> <three authority reports>
rg -n 'planTaxonomy|applyTaxonomyPlan|journal|rollback|resume|cancel|digest|symlink' <normalization component>
sed -n '37,235p;1350,1560p;2388,2430p;2950,3140p;3260,4030p' <normalization component>
rg -n 'applyTaxonomyPlan|rollback|cancel|resume|symlink|reserved|trailing' <shared test>
sed -n '2600,2735p;2860,3220p;3290,3350p' <shared test>
find <two exact ticket roots> -type l -print0 | readlink
find 🧰️framework -type f -name CACHEDIR.TAG -path '*/.🧬semio/🦑️repo/🎫️tickets/*' -print
find <exact FEM ticket> -type f -name ' ' -o <three exact retained names>
find <exact active normalization ticket> -type f -name CON.ts -print
shasum -a 256 <engine> <shared test> <three authority reports>
```

# Protected Parent Maintenance Bridge

Read-only Source design. No Core edit, protected filesystem/journal read, Native command, or recovery operation was performed. Source inputs are the existing nested owner research/Low review, Native platform/contract, persistent and Fixed maintenance-child owners, explicit publication owner, Repo partition bridges, Graph pool owner and its private fixture owner. The protected42743 kernel DATA remains unadopted: Graph12 descendants and Products5.

The current maintenance-child API authenticates its parent through `persistentMaintenanceParents`/the equivalent Fixed registry and requires an actual ready image, real maintenance borrower and exact kernel EX proof. Repo's wrapper additionally requires its private partition owner. Neither a direct parent SH nor kernel DATA can supply those objects. Ordinary Graph/Products opening still rejects the incomplete current receipts before it can create that parent. Existing fixture recovery therefore cannot bootstrap this cycle, and passing a structural image/port or copying a custody object would violate the real boundary.

## Proposed Small Source Batch

Add a distinct explicit protected-parent operation, imported only by a dedicated Repo maintenance leaf. Do not import it through ordinary opening or weaken current receipt readers. Provisional Core API: `withNativeImageProtectedMaintenanceV1(options, request, body(token, actualPhase))`. Its closed request contains purpose `mounted-parent-maintenance`, opaque old receipt SHA, current directory/backing/root CAS and actual physical envelope; it never parses an old receipt shape as current or invents previous underlying mount facts. Repo authenticates the exact issued Original/control, current setup-all purpose, declared Graph/Products options and Root before calling it.

The Core factory acquires the actual nonwaiting parent image EX, validates the received Native grant, prefunds existing cleanup, observes its exact mounted kernel association and retains backing/root/ancestry checks. Descendant mappings are returned as bounded DATA rather than automatically rejected, adopted or opened. The callback receives an opaque private token, not `NativeImageStorageV1`, a mutable scope, a caller-provided port, a reservation, or a charge. Its lifetime and pending work use the actual owner/borrower machinery and the same issued phase; clones, foreign tokens and post-callback use refuse.

For journal inspection, factor the existing real provider-bound storage port construction into the actual provider owner, under the newly accepted kernel-maintenance borrower. Ordinary receiver ingress retains all of its existing current-receipt checks. The protected factory's internally issued port checks the actual EX proof, physical identity, scope/control lifetime and metadata barrier on every operation. First-batch callback methods expose closed read-only journal projection through that real port; they do not expose a forgeable image or provide a default retirement callback. No allocation is acquired, copied, guessed, rebudgeted or retired by this batch. Mac and Fixed provider contracts must share the same honest custody semantics; a Fixed metadata operation cannot use a forged ready-image handle.

Explicit child enrollment must require the actual live parent token, exact descendant path derived by the domain owner, current child CAS, its own nonwaiting EX, and an actual descendant control. Acquire outer image before inner image and release inner before outer. No parent EX callback may call an ordinary child opener that silently borrows a fabricated parent. The first implementation should observe one real child and its journal only; mutation remains separately gated until concrete owner evidence is available.

## Domain Ownership and Remaining Barriers

Graph namespaces require the actual closed `.namespace.json` plus canonical `nxGraphNamespaceDirectoryV1` derivation from the complete workspace/tooling/graph/implementation identity. Twenty-hex path segments alone are insufficient. The pool's actual journal must confirm the exact unique committed allocation id/owner/kind/amount from current domain policy. Unknown, ambiguous, retiring or foreign rows stop; they are not converted into a new charge. Production optional-cache retirement must be authored in the production Graph owner, never borrowed from the fixture's arbitrary payload-removal callback.

The Products fixture's current recovery path recognizes a `gf-` name and owner text after opening Products. Its original `BuildTestFixtureV1` is process-private and is not restored by that name. A bridge cannot manufacture lost fixture provenance. Protected inspection must find genuine persisted producer/child-request/namespace ownership, if present; absence is a mechanical blocker. Do not create a marker now to claim historical ownership. State and other non-recomputable allocations must remain untouched. Merely preserving a whole nested tree for later republication is a different, larger coordinated operation; it is not silently substituted for fixture recovery here.

Existing direct image retirement can handle an authenticated zero-charge child with a current valid receipt and exact expected-image CAS, but current child receipt validity and usedBytes are not yet observed. An incomplete child receipt would require its own explicit canonical republication under the real protected parent scope before normal retirement. A charged child needs the actual domain reservation continuation recovered from its journal under the real enforcing port. Parent publication remains blocked until a fresh protected kernel census proves every mounted descendant has settled.

## Source and Private Proof Before Any Global Action

Closed portable outcome/state corpus and Ajv/SQLite oracles must cover genuine/forged/released controls and tokens, exact parent resource, wrong root/backing/receipt/kernel mapping, outside/symlink child, nested lock order, active readers, complete journal projection, foreign allocations, callback expiry, undefined/frozen primary and independent drain/close failures. Returning an outcome cannot erase an enrolled failure.

Private physical proof uses fresh canonical diagnostics-owned parent and child images with current receipts, unchanged image envelopes, genuine finite Original/actual phase and canonical cleanup. First hold real parent SH in another same-wire owned process: nonwaiting protected EX must refuse without a callback or mutation. After that reader closes, acquire real protected EX, enroll the real child, inspect an actual preexisting journal allocation created earlier through the normal image's real port, and verify identical id/owner/kind/amount/sequence. No new charge is issued by the bridge. A child guard/proof substitution and callback-after-close must refuse in isolated owned negative processes. Release every child before parent; ordinary reopening must remain current and payload/journal SHA must match independent Node/SQLite observations. Canonical original owners retire the private fixtures afterward. This proof does not qualify Graph/Products global recovery or prove missing historical fixture provenance.

No Native implementation or physical proof is released by this plan. The next reviewable unit is the token/state/schema plus read-only actual-provider bridge and its Source laws, followed by the private proof; domain mutation follows only concrete protected evidence.

## Refined First Batch: Actual Parent EX And Journal Observation

Planning only after the bounded witness Source hold. All shared Native Source remains frozen during Root qualification. Low's `protected-parent-provider-factor-low-review.md` identifies the exact common anchor: `acceptNativeImageMaintenanceV1` in `storage/📥️borrower/🟦️.ts` captures a real writer proof from the actual held EX lease. The new path must call that existing function; it must not manufacture a `PersistentReceiver`/`FixedReceiver`, set stage `ready`/`maintenance`, insert objects into ordinary receiver maps, register a fake Native image or construct a port from request DATA.

First batch observes the parent journal only. Mounted descendant mappings remain bounded returned DATA, not ownership. It does not yet open a descendant, recover a reservation, mutate a journal, retire an allocation or publish a parent. This is sufficient to establish the real provider entry boundary and learn whether existing parent rows contain exact child/namespace provenance. It avoids declaring child adoption before those facts exist.

### Exact Proposed Public Surface

Use existing files; no new Source leaf, facade or script is needed. The provisional names and signatures are:

```ts
// Existing storage/🌐️platform/📋️contract/🟦️.ts
type NativeProtectedParentV1 = Readonly<{ readonly [protectedParentBrand]: true }>;
type NativeProtectedParentJournalV1 = Readonly<{
 version: 1;
 backing: NativeStorageBackingV1;
 state: NativeStorageStateV1;
 journalSha256: string;
 journalBytes: number;
}>;

// Existing storage/🌐️platform/🟦️.ts
withNativePlatformProtectedParentV1<T>(
 options: NativeFixedPlatformOptionsV1,
 request: unknown,
 body: (parent: NativeProtectedParentV1, actual: ResourceLeaseControl) => Promise<T>
): Promise<NativeImageMaintenanceResultV1<T>>;

readNativePlatformProtectedParentJournalV1(
 parent: NativeProtectedParentV1,
 control: ResourceLeaseControl,
 policy: NativeStoragePolicyV1
): Promise<NativeProtectedParentJournalV1>;
```

The token has no mutable fields or methods, no port, no image, no release method and no kernel-proof getter. A separate private platform WeakMap binds it to the actual provider-issued token and original control. Each provider maintains its own private protected scope registry; this does not enroll anything in ordinary `platformImages`, `persistentMaintenanceParents` or Fixed ready/maintenance maps. Copied, foreign, closed and wrong-provider tokens fail before filesystem/journal access. The callback's actual control comes from the original borrower's accepted operation, with no new Original or unrelated root phase.

The closed request is owned by new named definitions in existing `storage/🧬️schema/🔣️.json`: version1, purpose `mounted-parent-read`, `wait:false`, opaque receipt SHA256 and current directory/backing/mounted-root physical CAS. Its physical fields reuse existing safe-integer/closed image constraints or explicitly bounded decimal DATA with the same preeffect representability checks; implementation must choose one coherent existing representation and update every fixture together. No old receipt shape is admitted. Receipt bytes are bounded CAS input only and are never parsed as a current receipt, a creator's mount claim, a capacity declaration or a child grant. Unknown pending publication metadata refuses in this first batch rather than being silently adopted.

The Repo wrapper belongs to the existing explicit `opening/📤️publication/🟦️.ts` leaf, not ordinary opening. Proposed `withRepositoryCacheProtectedParentV1(invocation, control, id, request, body)` reuses the exact issued Original identity check and `readIssuedScriptProcessEnvelopeV1`, computed `build-orchestration` role, current repository root and exact `tools setup all` purpose. Restrict this first Repo wrapper to the declared `graph` or `products` asset and current `repositoryCacheBudgetV1` options. It retains the already current leases infrastructure through the existing genuine owner, derives the actual lease data directory and supplies the current physical policy. Core never imports Repo budget types or validates a caller's hash as authority. Normal opening does not import the explicit publication/maintenance leaf.

### Provider-Private Factor

In Mac `storage/🧹️cleanup/🎛️command/🟦️.ts`, factor the physical read/operation construction currently inside `createPersistentImage` into a private helper accepting privately verified facts and a real `NativeImageBorrowerV1`. The ordinary constructor still performs its unchanged `inspectPersistentReceiver` current-receipt/pending checks before invoking that factor. A distinct `withPersistentNativeProtectedParentV1` independently derives fresh physical facts under direct actual EX and uses the same read-only factor. It never calls `createPersistentImage` with an invented receiver.

In Fixed `storage/🧱️fixed/🟦️.ts`, factor the actual physical observation/recheck and borrower operation portion of `createFixedReceiverImage`, separately from ready-image metadata/cleanup methods. A distinct `withFixedNativeProtectedParentV1` captures the existing platform/system and actual kernel mapping under real EX. Its read-only scope must not invoke Windows metadata through an absent `receiver.image`. Later mutation needs a separately authenticated actual barrier and remains outside this batch.

Both providers derive backing identity using their existing canonical identity formula from actual backing/root/mapping facts, never from an arbitrary token or request string. Their private `requireProtectedParent(scope, control)` authenticates the actual writer proof bound to `native-fixed-image:<exact directory>` and the exact `resourceLeasePathV1`, genuine control refinement, scope/borrower lifetime, outside ancestry, current receipt CAS, backing allocation envelope, current mounted root and bounded kernel association. Checks surround every await and operation acceptance. Physical identity fields remain distinct from content timestamps: a full fresh FD interval is stable, but historical mounted-container timestamps are not promoted into ownership.

### Actual EX And Borrower Lifetime

Snapshot all request primitives and authority references before the first await. Validate the received Native grant and unchanged actual control/Original before admission. Use the existing provider image-resource guard primitive with mode EX and timeout0, retaining its actual maximumLeaseDatabaseBytes and canonical lease directory; do not create a queued wait or sibling phase. The only `acquired:false` path is the canonical actual busy result before borrower/body admission. Arbitrary authorization, schema, IO, cancellation and expiry failures stay errors.

After acquiring EX, obtain its real writer proof and issue the real borrower through `acceptNativeImageMaintenanceV1({lease, leaseDirectory, resource, control, leaseControl})`. Authorize information-only kernel observation through the actual received Native child grant: no creator action, detach, attach, allocation, receipt write or cleanup mapping command. The exact new role's bounded information roster must be schema-owned and validated against the received child/time/output limits before dispatch; it cannot reset an existing creator's count. Capture actual before/after mounted associations and current root/backing/receipt CAS. Descendant mounts are reported as DATA and are neither traversed nor adopted.

Use the existing accepted cleanup enrollment before body admission solely to retain/drain/release this actual guard if the ordinary body expires. Capture its actual borrower cleanup loan while body authority is live. Close admission, join all accepted reads, then close the real borrower outside its active operation. Expired-body settlement uses `closeNativeImageBorrowerCleanupV1` with the actual captured loan/enrollment, never a fabricated cleanup control or an ordinary read with expired body authority. Any ambiguous descriptor/lease/child close retains the actual scope strongly and reports uncertainty. Combine independent body/drain/release errors as flat ordered AggregateError, body first, exact identity deduplicated; no accepted failure is cleared. These are auxiliary lease/process effects, not Native payload or journal mutation.

### Read-Only Journal Boundary

Existing `observeNativeStorageStateV1` reaches `storageCustodyV1`, whose transaction owns an actual exclusive journal lease and bounded `readCleanupJournal`. The callback currently observes rather than appends. Factor that read path in existing `storage/🟦️.ts` so the protected provider supplies only a repository-owned read capability:

```ts
type NativeStorageObservationPortV1 = Pick<NativeStoragePortV1,
 "withOperation" | "observe" | "recheck">;
```

This narrow port is private to the provider/platform bridge and never returned to Repo. It is backed by the real borrower and actual physical checks. It is not a structural substitute for a full enforcing write/retirement port. Refactor the shared actual journal lock/replay function so ordinary transactions keep their complete existing mutation/metadata behavior while read-only observation cannot receive append/checkpoint/retirement callbacks or require syncMetadata. Do not install no-op mutation methods to satisfy an interface.

The read path validates the current closed Native policy against the exact scope root/lease directory and actual maximum backing envelope, then acquires the same actual journal EX resource/order as existing storage. Replay the same complete bounded raw journal and checkpoint rules. Return detached deep-frozen state plus SHA256/byte count of those exact read bytes; do not invent empty state for a missing, malformed, shortened or unrecognized journal. The current private `readCleanupJournal` explicitly returns empty state on ENOENT for ordinary fresh storage, so merely reusing its current return value is insufficient here. Preserve that legitimate ordinary behavior and add internal observed-presence DATA to the shared private read result; the protected existing-journal entry requires actual presence. A file disappearing before its FD read or post-observation must refuse, not be confused with a present zero-byte journal. This adds no legacy format/parser and does not relax an ordinary guard. Cache reuse, if retained, must be keyed by actual identity and exact current raw bytes; fresh physical rechecks and journal lease remain mandatory. No checkpoint, append, recovery, retirement or metadata durability operation is exposed.

### First Source And Private Proof

Language-neutral closed schema/corpus with Ajv and SQLite independently covers: actual/structural/foreign/released token and control; exact Original/root/purpose; busy real reader before callback; wrong EX resource/path/directory; receipt byte replacement; backing/root/ancestor substitution; mounted-association ambiguity; unknown pending namespace; expired/cancelled body; complete and malformed/checkpointed journals; retained child/namespace fields; no append/checkpoint/syncMetadata dispatch; callback work accepted before close; post-callback use; child-before-parent drainage; undefined/frozen primary with distinct drain/release failures. Provider Source tests must call the real borrower with actual private fixture leases and genuine controls; explicit filesystem/kernel ports remain clearly labeled models. Those tests use Generated scratch only, with no canonical Store/Native operation.

Private qualification, separately released by Root, uses fresh canonical diagnostics-owned parent and child images with current receipts. An independent same-wire owned worker holds actual parent SH: timeout0 EX must return busy with zero callback and unchanged receipt/journal bytes. After it drains, close every ordinary parent/child SH, retain the child mapping, and admit actual protected parent EX. Observe the parent journal through the new real provider read capability while the nested image remains mounted; the nested mount is DATA, not an adopted child port. Compare sequence/allocation fields and exact raw SHA/bytes to independent Node/SQLite observations before/after. A known real allocation may be created earlier by the current private fixture owner; its exact existing id/owner/kind/bytes must be returned unchanged, never reissued by the bridge.

The original private fixture owner, not the protected bridge, must already have a concrete canonical child-first cleanup continuation for its known allocation/image before that physical test is released. If closing the ordinary parent invalidates an old reservation handle, cleanup must use the existing genuine recovery/retirement operation on a freshly reopened real image and the exact current fixture-owned allocation, not reuse the released port. Prove that cleanup sequence in Source before physical execution. Retain all child images/allocations honestly if it cannot be established. The protected bridge itself performs no charge recovery. Separate isolated negative OS workers qualify accepted failure/drain semantics without poisoning a positive parent's ledger. Normal parent reopening and independent unchanged payload/journal checks follow complete protected close; original canonical owners retire child before parent.

Only the above parent-token/journal batch is proposed for implementation after Root releases the Native Source hold. A future child-read enrollment function must bind the exact parent token, persisted full child declaration/namespace row, descendant control and child EX; it is deliberately not part of the first patch. The Graph/Products current census and lost private fixture provenance remain unqualified until actual protected facts and owner proofs exist. No current global recovery or cross-platform physical success follows from this plan.

Concrete existing test locations are `storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts`, `storage/🧪️tests/🟦️.ts`, `storage/🧹️cleanup/🎛️command/🧪️tests/🟦️.ts` and `storage/🧱️fixed/🧪️tests/🟦️.ts`, with their existing owned schema/corpus documents and script registration. The existing explicit publication aggregate can cover the Repo Original/purpose wrapper. No new runtime/test entry import or additional Source file is assumed necessary; review current ownership before editing. Actual complete tooling/Bazel captures and FD equality must follow the combined Source hold because existing512 and256 caps remain authoritative. Keeping file count unchanged does not prove transitive closure count unchanged.

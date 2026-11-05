# Private Cleanup API Follow-up

Read-only source inspection. No gates, process births, or source edits.

## Existing Authority

Process routing (process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:343–366) provides withAcceptedPhysicalCleanupV1, beginAcceptedPhysicalCleanupV1, checkAcceptedPhysicalCleanupV1 and withAcceptedPhysicalCleanupOperationV1. Enrollment must occur before effects while the original control is live. For finite callers it carves the body deadline before the unchanged whole-operation deadline; terminal checks use the privately accepted token and cleanup signal, independently of ordinary cancellation. It does not issue a replacement ResourceLeaseControl. begin must drain body work before terminal effects, and failures must be retained while terminal cleanup still runs. The finite reserve is not a fresh deadline.

Native already uses this exact structure in runNativeDisposableImageV1 and runNativeFundedImageV1 (storage/🧹️cleanup/♻️retirement/📋️contract/🟦️.ts). Their real backend ports preserve preaccepted EX/creator custody into terminal retirement, and bind the actual allocation. Public platform child factories require a genuine parent; funded factories additionally require the actual parent reservation. These are the correct reuse points for already owned disposable children.

captureNativePlatformImageCleanupV1 and withNativePlatformImageCleanupReadV1 retain actual existing receiver custody, but return only immutable backing facts plus recheck. Their NativeImageCleanupReadV1 type explicitly confers no port, control, release, or deletion authority. They cannot authorize the current whole-workspace deletion. Recomputable retirement also demands a live control and is not a cancellation bypass.

## Consequence for Current Receiver

Current receiver finally calls retirePrivateRepositoryRootV1(workspace, original, control). That function requires a live control, then opens partition images, obtains maintenance writers, recovers reservations, retires allocations, closes images, and starts physical backing retirement. Those are new ordinary acquisitions. Wrapping this call in withAcceptedPhysicalCleanupOperationV1 does not make them legal after cancellation. Passing a token as a control, reopening under a fresh control, or weakening existing control checks would be incorrect.

There is no inspected drop-in existing API for terminal retirement of an arbitrary dynamically populated private workspace after cancellation. Recommended structure: retain genuine resource-owning disposable/funded factory scopes from before each image birth through body completion, using their preaccepted terminal retirement; drain body then settle those exact owners, lease image last. Root-directory removal can be one closed terminal operation bound to captured directory identity after every retained Native owner has proven settlement. If workspace partition opening currently hides these owners, extend that owning layer with a closed preaccepted aggregate terminal port rather than teaching retirement to reopen cancelled stores. Keep the existing live-control recovery path for later recovery, clearly separate from same-process terminal settlement.

Ordinary cancellation can execute finally and settle preaccepted owners. SIGKILL, host crash, or termination before child finally cannot; the actual durable claims/receipts/journals must be recovered under a subsequent genuine invocation. Parent fresh-root cleanup intentionally cannot recursively erase a nonfresh Native store.

## Bounded Metadata Interface

Existing readNativeImageRetirementFileV1(path, maximumBytes), at process/📦️artifacts/🏗️native-build/💾️storage/📄️retirement-file/🟦️.ts, verifies non-symlink ancestry, single-link regular file type and size, opens a no-follow descriptor when available, and checks descriptor/path identity before and after reading. Reuse for metadata (4096 bytes) and a schema-declared short owner PID record bound. It uses readFileSync(fd) after the size check; concurrent growth could still transiently allocate beyond the original size, so an absolute acquisition bound would require strengthening this shared interface to a maximumBytes+1 bounded descriptor read. Windows O_NOFOLLOW fallback is covered by path/descriptor identity comparison, not a universal OS no-follow guarantee.

## Smallest Portable Verification

1. Schema corpus for cancellation/finite cutoff/drain/terminal states, checked by internal validator and Ajv (existing model tests also use SQLite as independent EX oracle).
2. Existing Native cleanup operation model test: cancel the exact body signal after genuine lease acquisition, verify ordinary writer proof rejects while requireAcceptedPhysicalCleanupWriterProofV1 succeeds, terminal operation runs, and SQLite confirms EX remains until final release. Existing test is named “canceled native cleanup retains actual EX and token without reviving its Original” in storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts.
3. Add receiver success/failure/cancellation cases through its existing real OS witness. Assert original unchanged, no post-cutoff store acquisition, terminal retirement before receipt/root absence, original cancellation retained, and reserve exhaustion leaves recoverable evidence rather than fabricating success. Avoid hard kill in the portable smallest gate; it needs a separate recovery integration case.
4. Shared bounded-read corpus: exact bound accepted; oversized regular file, directory, symlink (where supported), hard link (where supported), replaced identity, and malformed/oversized owner PID rejected before receiver store effects.

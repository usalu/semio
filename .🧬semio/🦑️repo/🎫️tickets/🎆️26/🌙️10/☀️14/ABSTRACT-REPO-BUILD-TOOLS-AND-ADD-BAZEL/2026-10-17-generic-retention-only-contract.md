# Generic Retention-Only Ordinary Endpoint Contract

This proposed additive Source feature belongs only to the existing generic filesystem descriptor module/schema/public facade, existing artifact tests, and existing stdio helper. It adds no Source input file to the backend or authorizer closures. No Native, installer, protected asset or actual old workspace payload operation is performed by the implementing agent.

## Exact Surface and Meaning

The existing original-root receiver exposes `retainPreservation(expected, intent)`. Its expectation must come from the existing `capturePreservation(name)` in the exact same live root frame, for a physical ordinary file or directory. Missing, link, special, copied, foreign-frame, nested-frame, escaped and consumed expectations refuse. Capture and binding expectations continue to share the existing32 counter; the same frame may perform at most one preservation or retention capsule operation.

The proposed first-party result type is `ArtifactRelativeRetainedPreservationV1` with frozen fields:

- `name`: the original single basename;
- `absent: true`: the actual named absence established by this operation, not historical authority;
- `parent`: the actual final original-root seven-field identity;
- `retained`: `{capsule, payload:"p", intent:"i", receipt:"r", kind, identity, intentSha256, receiptSha256}`, where identity is the actual moved original object after its own permitted rename.

This is nominal completion DATA from the genuine live operation, not an adoption, recovery, deletion, funding, Store, reservation, readiness or independent absence capability. The domain owner must keep it private, establish its own current whole-store/token/global-exclusion/all-scope relation, and freshly prove named absence and exact parent before a separate absent-only materializer. No serialized result or existing capsule is accepted as an input to this operation.

No directory target or replacement binding is accepted or created. No old payload content is opened, enumerated, read, copied or hashed. The source root may be a20hex generation directory under a genuine `j` receiver; no generation-name ownership meaning exists in this generic API. A directory without a ready record is not thereby inactive: the domain must separately establish current alias/reader constraints or an explicit repair/readers-refuse contract before moving a referenced endpoint.

## Admission, Custody and Transition

Reuse the existing root-only private preservation proof map and directory-descriptor registry. Check synchronous actual control, exact receiver, quiescent accepting frame, full captured parent7, captured complete parent basename roster, full current leaf7, and the unchanged path256/name/intent/capsule limits before effects. Consume/reserve the one terminal preservation operation synchronously before scheduling. Cancellation remains checked at each syscall/read/write/barrier and after awaits. Unawaited accepted operations retain actual drainage; late escaped work cannot reopen admission.

The capsule is exactly `.p` plus the first8hex of the complete UTF8 intent SHA256. It is exclusively created, never reopened/adopted; a collision refuses. Capsule10 plus `/p` is12units, no longer than the unchanged minimum ordinary source basename12. Intent remains1..4096UTF8 bytes and receipt at most4096 bytes. Shared32 descriptors and existing file write chunk bounds are unchanged.

Hold a nofollow FD to the original ordinary object before writing the intent, through the move, moved-object proof, durable receipt and final checks. Write and fsync exclusive capsule/i, then sync capsule and original parent before any name move. Capsule/p must be absent. Invoke real descriptor-relative no-replace rename from original-parent/sourceName to capsule/p. Never emulate no-replace using a precheck plus overwriting rename. No prepared symlink, exchange, unlink, rollback or payload deletion occurs.

Immediately compare the held original FD and actual capsule/p to the exact captured leaf. All seven metadata facts must remain equal except nondecreasing ctime caused by the owned rename; dev/ino/kind/mode/nlink/size/mtime must remain. Validate actual named source absence, exact parent roster `(before minus sourceName plus capsule)`, exact capsule roster, and only own parent/capsule metadata deltas before retaining updated private metadata. A foreign object moved by a race remains in the capsule and the operation refuses without rollback or completion issuance.

Write exclusive durable capsule/r identifying the new purpose `retain-current-ordinary-entry-without-binding`, original name, capsule, actual moved identity and full intent SHA; include explicit `sourceAbsent:true`, `deletePayload:false`, `adoptPayload:false`. Sync both parent directories and verify immutable records, moved FD/name facts, source absence, exact final rosters and root identities. Return only after existing create/FD/library drainage and closure succeeds. Any write/sync/close failure retains visible objects, preserves arbitrary primary identity and adds truthful cleanup failure; no success result escapes.

## Platform Scope and Primary ABI Sources

Darwin uses `renameatx_np(oldfd, oldname, newfd, newname, RENAME_EXCL=4)`; the installed SDK sys/stdio.h declares this exact signature and constant. Apple's [current XNU sys/stdio.h](https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/sys/stdio.h) is the primary upstream declaration. Linux uses the existing system-library `renameat2` symbol with `RENAME_NOREPLACE=1`, defined by the [Linux UAPI filesystem header](https://github.com/torvalds/linux/blob/master/include/uapi/linux/fs.h). Unsupported syscall/filesystem semantics refuse; no fallback overwriting rename is allowed. Runtime qualification is actual Generated Darwin only in this environment; Linux exact-ABI Source modeling does not claim physical Linux acceptance.

Windows adds an honest early refusal for this new purpose before any acquisition or move until rooted rename/sharing is qualified. Existing strict binding and publish-preserving APIs keep their current semantics; prior read/write/remove function bodies remain unchanged.

## Schema-First and Actual Test Plan

Extend only the existing descriptor schema with a closed retention protocol, no-replace ABI constants, receipt and portable cases. Extend the existing artifact test module/stdio helper Source modes, not new script files. First observe actual missing method/type semantic RED on a genuine Generated root.

Portable Ajv/SQLite cases include file/directory/hardlink/unawaited positives; missing/link/special; copied/foreign/escaped/nested/consumed expectations; wrong arity/receiver/intent; shared expectation limit; path256; capsule and payload collision; parent/leaf drift; same-frame second/concurrent operation; cancellation before/after intent/move; undefined/frozen primaries; partial/zero writes; intent/receipt close and sync failures; final parent barrier; foreign move and late unknown record; unchanged source contents outside the API claim; and Windows pre-effect refusal.

Actual Generated Darwin fixtures use real exclusive descriptors, real no-replace syscalls and injected actual syscall/write/sync/close faults through the established test-only system ABI boundary. Python independently reopens exact known fixture names/dir_fd to compare seven facts, full original test bytes/hashes, absence and bounded durable records. The implementation never inspects those old bytes. SQLite independently derives admission/outcome and event order; current strict binding/preserving/create/relative/read regressions and strict TypeScript must pass before an independent held-byte review. No Native/installer acceptance is inferred.

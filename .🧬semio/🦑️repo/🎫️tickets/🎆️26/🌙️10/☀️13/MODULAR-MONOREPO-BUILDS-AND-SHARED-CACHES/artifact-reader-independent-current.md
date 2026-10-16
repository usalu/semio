# Independent Artifact Reader Audit

Read-only source review of the current opaque reader implementation and canonical Core control/Native path and reservation implementations. No tests, builds, cache operations, or runtime confirmations were performed. Caller migration remains in flight and was excluded.

## Findings

1. **High: phase control can bypass the base loan deadline and cancellation.** `withRepositoryArtifactV1` authenticates both controls against the exact invocation, but never establishes that `phaseControl` descends from or is at least as narrow as `control`. Core permits independently issued sibling controls for one invocation. Supplying a short or additionally canceled base storage control and a longer sibling phase causes lease acquisition, hashing, and consumer tasks to run under only the sibling. `storage.recheck()` does not substitute for preserving the original base control continuously. Bind the operation to the actual base loan and privately prove phase ancestry or compose both controls without widening either. Add a negative proof with two genuine sibling controls of the same invocation, one carrying the shorter deadline/additional signal.

2. **High: manifest read is not bounded by the checked descriptor.** The marker gets an `lstat` size/regular-file check, followed by a separate path-based `readFile`. A replacement or growth between those calls can cause allocation/read of an arbitrarily large file before any byte budget enforcement. This also means the parsed declaration is not proven to come from the descriptor subsequently included in the tree digest. Read the manifest using an opened descriptor, compare original/pre/post identity, impose the byte cap during reading, and bind the parsed bytes to the inspected manifest digest. The artifact file digest already demonstrates the bounded-descriptor pattern that the manifest needs.

3. **Medium: directory traversal lacks pre/post descriptor or path identity continuity.** `inspectArtifactV1` checks a directory with `lstat` and then opens it separately using `opendir(path)`. Directory identity is neither recorded in the returned fingerprint nor compared after traversal. Replacement of an empty directory, or replacement of directory ancestors containing otherwise identical inventory, is invisible to final recheck; concurrent replacement also separates the opened listing from recursively resolved child paths. If the required contract includes original physical tree identity, capture and compare directory identities before/after traversal and retain them in the snapshot. Otherwise narrow the documented identity promise explicitly to regular files.

## Confirmed Source Properties

- Tokens use a private WeakMap and frozen opaque object; copied receipts do not authenticate.
- Exact invocation and original reservation authentication are present. Core reservation authentication compares the original control signal.
- Core task enrollment defers effects into a microtask, so reader pending registration precedes consumer execution.
- The producer SH lease remains held across the body, accepted-task drainage, and final recheck, with release in finally.
- Reader failure arrays preserve rejection with `undefined`, including callback rejection; length rather than truthiness determines failure.
- Per-file hashing uses bounded chunks and regular-file pre/post descriptor/path identity comparisons. Physical traversal and accepted tasks have finite protocol caps.
- A task error closes further reader task admission and is retained for outer failure even when the caller catches the task rejection.

These are source conclusions only. Runtime proofs are still required, especially cancellation/deadline narrowing and replacement races.

## Planned Conservative Core Refinement Guard

Reviewed the proposed algorithm against actual `ControlScope`, `controlFor`, `remaining`, `track`, and `admit` source. The guard had not yet appeared in Source at the time of this follow-up read; these conclusions apply to the exact proposed algorithm, not a verified final implementation.

The algorithm is sound as a conservative ancestry proof under the current canonical issuance model. A shared private control ensures phase `remaining()` walks the same finite deadlines and cancellation ancestry. Each skipped base control with `requestedMilliseconds === 0` contributes no independent elapsed allowance; exact `signal === invocation.control.signal` excludes any additional cancellation signal. Any meaningful restriction inherited above such a skipped control is still encountered by walking its base ancestry. If there is no shared control, accepting only an entirely unrestricted base ancestry remains safe because both private controls share the exact original invocation, whose deadline and signal are always checked by `remaining()`.

This intentionally refuses numerically tighter independent siblings when the base has its own finite allowance, and refuses custom-signal siblings. That is conservative rather than an authority widening. `AbortSignal.any` creates a distinct signal for additional-signal issuance, so even superficially equivalent signal combinations do not accidentally pass the exact-identity check.

Registering the reader operation under both actual base and actual phase retains both scopes while its asynchronous operation runs. `track` defers effects until a microtask and registers pending work synchronously; nested enrollment therefore occurs before acquisition effects. The implementation should call the guard before physical effects, use the proved phase for the lease and all borrowed tasks, and keep the SH release outside the inner loan and task drain. Existing Core admission checks remain necessary; the refinement helper need not independently reopen or modify admission.

No new correctness flaw found in the proposed guard. Existing manifest descriptor/bounded-read and directory identity findings remain independent. No tests or runtime checks performed.

## Actual Revised Implementation Review

Re-read the actual Core refinement helper and reader after the bounded manifest and directory changes landed. No runtime execution performed.

- **Resolved:** actual refinement implementation matches the reviewed conservative algorithm. Actual reader enrolls the outer operation under base and phase and preserves SH through inner scope settlement. No further control narrowing flaw found.
- **Resolved:** manifest allocation is now bounded by the request/protocol minimum. Descriptor identity, bounded reads, extra-byte growth detection, strict UTF-8 decoding, and finally-close are present. The original unbounded `readFile` finding no longer applies.
- **Partially resolved:** directory identity is now included in snapshots, so stable replacement between inspections is detected. Traversal still lacks post-directory identity comparison; initial `facts.isDirectory()` and subsequent bigint identity/open are separate observations. Replacement during an inspection is not proven to be one continuous original directory tree.

### Remaining Concrete Gaps

1. **Manifest parse and initial snapshot are still separate identities.** `readArtifactManifestV1` returns only parsed JSON. Its checked descriptor identity/digest is discarded. `inspectArtifactV1` later captures whatever manifest occupies the path at that time without revalidating its owner/declaration against the parsed value. For example, replacing a valid owner-A manifest after parsing with an owner-B manifest having the same files lets the body receive owner A while the stable baseline contains owner B. Final recheck passes if that replacement remains stable. Return the manifest fingerprint with the parsed value and require the initial snapshot to match that exact fingerprint (or derive declaration from the same bounded descriptor used to establish the snapshot).

2. **Aggregate byte accounting uses a different stat from digest accounting.** `inspectArtifactV1` adds the first `facts.size`; `artifactDigestV1` then obtains another independent identity and may accept a larger file up to the full request budget. Growth in that interval means the initially accepted snapshot can contain a sum above `maximumBytes`, although a later final inspection may reject it. Return the digest descriptor size and use that exact size for aggregate accounting before admitting the consumer.

No new error/undefined or pending-enrollment race found: rejection is retained by array length, reader pending addition precedes Core deferred callback execution, and accepted tasks are drained before final recheck and SH release. The report retains earlier sections as audit history; this section supersedes resolved findings above.

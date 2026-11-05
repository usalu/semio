# Pending Page Integration Review

Read-only review of PendingPublicationPage and JsonBorrowedSinkCursor. Native tests are queued, not executed; no production changes or jobs were made by this review.

## Concrete refusal-state inconsistency

PendingPublicationPage::write_step returns OwnershipLimit when the 4096-byte backing is full and the cursor still has pending octets, before setting refused=true. The early write_demands error return also leaves write_refused false. Failures returned through the bridge instead seal refused and quote its bool write. Therefore Err does not consistently imply write_refused, and a parent relying on that getter can repeatedly retry an impossible write. Root was notified to centralize actual failure sealing with its paid metadata quote or explicitly keep early invalid-call errors separate from writer refusal.

## Parent custody obligations

The plan freezes the final token and scalar lane/receipt/count. Source variants are synchronous borrowed views and never escape their call. However Ui/Download/Raw source values are not themselves frozen or identity-checked by the cursor. The parent must retain the same immutable accepted emit/source across all framing steps; passing a new value of the same variant can mix bytes from different payloads. Do not select the current focus, current generation or mutable outbox when resuming. Fault and forwarding ValueErrors are both owning return fields and must enter explicit retained cleanup; dropping either error or a failed page owner is not an admissible integration path.

## Traversal and physical scope

Cursor phase0/1 admit frames/path backing separately, phase2 seeds the initial frame, phase3 alternates a bounded traversal atom and positive caller-output flush. Zero maximum_work returns before mutation. Output-empty pending flush pauses without advancing; pending output count is capped by remaining output and body after metadata. Partial failure reports cursor effects and keeps frames/path/pending bytes for closure. Close moves actual vectors into ControlledRetirement, then separately clears the terminal cursor. Pending page handoff requires framed plus empty traversal custody, uses the original token, and separately removes plan/metadata after backing transfer.

The generic JSON sink does not retain source references, and fixed atom storage avoids owned JSON trees. Its work count is one bounded action, distinct from copied_items. The enclosing FullOp must pay returned work and all copy/capacity/release progress, including both error paths. It must use original construction for backing/traversal births; forwarding an encoding control does not independently prove quoted physical custody. Raw cancelled framing still calls checkpoint on the forwarded encoder, so a genuinely cancelled normal observer will refuse: integration must use the original retirement receiving path/cancellation override, not create a fresh accepting control.

## State taxonomy

Keep PendingPublicationOutcome's existing fault-report phase separate. A directly owned pending page field has its own accepted intent, frozen token, partially initialized backing, traversal frontier and refusal evidence. Mixing that custody with the existing fault phase would conflate semantic outcome/reporting state with physical page admission and complicate exact terminal emptiness. No additional concrete source defect was found in this bounded pass.


## Mounted integration follow-up

The initial mounted framing failure branch returned true indefinitely while retaining failure. Runtime owner subsequently reported its repair: third context ValueError is retained, outer receiving refusal before callback is handled, and actual write refusal moves the operation to Retiring under a funded stage/metadata quote. Cancelled accepted JSON therefore follows paid failure/owner closure rather than receiving a renewed uncancelled control. The stale infinite-pause finding is superseded by that current change; semantic native execution is still required.

MountedUiCopy source uses explicit owned output, separate vector and string allocations, bounded UTF-8 chunks, and retained close cursor. It borrows the unchanged producer synchronously and restores the same copied scope on saturated outbox admission. No additional concrete copy-state or UTF-8 boundary defect was found in this bounded read.

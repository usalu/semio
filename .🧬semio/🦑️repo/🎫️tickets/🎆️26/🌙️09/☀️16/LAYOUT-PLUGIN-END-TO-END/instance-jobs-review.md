# Instance Jobs SDK Custody Review

2026-10-28. Read-only actual log13 contract diagnostics, current cancellation issuer/lease retirement, jobs close ladder, private child group and ActionBus admission source. No edits to SDK source, no compiler, test or graph. Ingress writer still assembling its packet; this report does not clear unimplemented changes.

## Cancellation Close Packet

The three Option handle corrections now explicitly require the existing canonical issuer handle for an active ToolCancellationLeaseRetirement. ToolCancellationIssuer.handle returns its actual issued slot: None means no issued canonical handle (before construction or after returning it during issuer closure), not an equivalent empty registry.

Active lease retirement uses the canonical handle to verify original state/generation/active Arc identities and to return cancellation token aliases to exact issuing scopes. Replacing None with a new/default handle or skipping closure would lose that authority. The new ok_or_else(InvariantViolated) at all three active demand/close sites is truthful: it preserves the pending owner and reports the invariant failure. No unwrap panic, forged canonical owner or pending-handle take was added.

Already terminal cursors still release their empty frame without requiring a handle. That preserves the legitimate distinction between completed owner release and unfinished alias/token retirement. Command/metadata owners still close before cancellation lease retirement, and command removal remains after the lease/cursor is empty. Retained grants, pending.validate_original(cx), exact queue identity and terminal checks remain unchanged. No premature close or lost pending lease was found in this packet.

## Private Child Group Packet

The group file only admits native fixed group frames, quotes native close work and drives existing MountedPrivateChildGroup retirement. Its nested owner and actual close/demand/terminal APIs already require SpaceMember+MemberFactory. The former ReferenceSpaceMember constraint was inherited container scope, not a reference pack operation. Current SpaceMember impl retains all actual fixed-slot, grant, original lifecycle context, terminal and ownership-graph checks; no body or pack authority changed. No actionable defect found.

## Actual ActionBus Contract for Pending Ingress Repair

ToolOperationSpec::from_owned moves the actual Box payload and owned key/schema/operation/source; it does not itself authorize those inputs. ActionBus.admit_job looks up the registered exact factory, whose erased admission verifies spec.source.matches(context). Refused admission leaves the typed payload in the caller specification; successful admission returns an erased owned job and increments dispatch_count. The real registered factory is the source of job authority, not a replacement synthetic dispatch.

The current obsolete ingress call uses removed dispatch/ToolOperationSpec::new and cannot compile. The writer is tracing an original-construction-funded Box<J>, from_owned with identity captured from the same actual StepContext, then admit_job with retained spec/input/checkpoint/output slots. Any completed packet must preserve all of those owners across denied capacity, yield and error and retire original J plus metadata through existing original custody helpers. The current function takes J by value and returns OriginalTurnFailure, so an unguarded return after moving it into a local specification would drop owned work; that is a review condition, not an approved repair.

Host continuation should reference the existing reactor-tasks tasks_extension_continuation host_for_instance owner already used by sibling task/document code. No default native context, substituted body limit, fabricated operation/generation/identity or unowned continuation constructor is acceptable. Final ingress/continuation review remains pending writer completion.

## Final SDK Packet Review

The writer selected justified deletion of the unused three-method dispatch chain rather than inventing an unsafe adapter. Exact diff removes run_framework_reserved_job/run_framework_reserved_job_inner/drive_framework_reserved_worker and their now-unused imports. Independent bounded plugin source inspection confirms remaining names are explanatory comments, fixture strings and absence assertions; route/factory source-law recognizers inspect textual legacy fixture shapes rather than invoking methods. Writer separately documented tracked/untracked code/schema/reflection inventory. No current executable consumer or schema/WIT hook was identified.

The live original producer definitions remain outside the deleted block: job/reserved.admit_original_reserved_producer and retained-command/producer.admit_retained_tool_producer(_measured_scoped). Their original donor/receiver/context APIs were not replaced by synthetic dispatch. Consequently no new local specification can lose J/metadata, no default source identity or body was invented, and no native guard was weakened by the deletion.

The current tool-run host reference resolves the already-public reactor_tasks.tasks_extension_continuation.host_for_instance. Its actual owning manifest depends on actor-host, declaration emission, ABI, framework, value and kernel, with no reverse instance-jobs dependency. The direct dependency adds the correct owning interface; no duplicate host was implemented. Existing source-law recognition behavior still needs its normal owning policy validation before broad policy-success claims; this source audit does not run those tests.

Current cancellation/privategroup and SDK packets have no actionable source custody/provenance defect found in this review and were cleared to Rust for the next owning compiler invocation. No compiler/runtime success is inferred. All SDK source remained writer-owned and unchanged by the reviewer.

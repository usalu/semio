# Source Refusal Review

Read-only review of current production patch against source-refusal-audit.md. No tests/build queues started; mounted/browser execution remains separate evidence.

## Concrete Finding

TextEditorHost/🟦️.tsx:474 builds sourceScope from controllerId, surfaceId and the entire scene.settingsJson. That JSON includes the localized label (provider text_parameter_scene emits question.label; embedded source editor likewise emits localized labels). An EN↔DE presentation/settings change under the same edit owner makes currentSourceDraft null at482 and displayedScene fall back to published scene.buffer at573-577. Thus the user loses the invalid local text/alert without its publication or explicit revert. Scope should identify edit ownership/target rather than include presentation settings wholesale. Sent directly to browser_execution.

## Verified Source Invariants

- Mapper now returns dispatch/local/refused and performs encoded-byte size preflight before split; routine overlimit no longer throws.
- Source lane bypasses ordinary pending echo edits, typing-run acceptance, shared preview and trailing selection dispatch: surface-state-and-delivery:91-94 returns immediately after onSourceCommit.
- deliver captures newest local text via onDraftChange before notifying serialized flushDelivery; the one-in-flight outbox remains stable.
- commitSource prepares canonical parent mapper action, bypasses invalid/local results, and only records admission on applied outcome with matching captured scope. admitTextEditorSourceDraftV1 only clears refusal when the admitted text still equals current draft text, protecting a newer refusal from older success.
- Controlled local draft survives ordinary unrelated scene publication: it clears only when admitted, local and published text agree.
- Source discard clears the draft explicitly. Local invalid edits have no parent dispatch. In-flight parent mutations already dispatched are not cancelled by discard; publication of such accepted edits is still possible and should be represented as canonical state.
- MountedRef suppresses late admission update after unmount. Current scope hides previous owner drafts rather than dispatching their late acknowledgement into new owner.
- Refusal alert and discard use localized labels, role=alert, aria-invalid and aria-describedby in both visible textarea fallback and WasmEditorSurface props.

Remaining review limitation: old-owner pending outbox draining across a surface/controller identity reuse is not exercised here. It depends on mounted session teardown/reset ordering; no execution claim is made.

## Stable Owner Scope Follow-up

Current sourceScope no longer includes settingsJson/display label: it uses controller/surface plus editControllerId/editAction/editArgument/editEncoding/sorted editArguments. This fixes the exact locale-label reset reported above.

Two concrete identity pitfalls remain in that intermediate source: sourceScope does not contain the mounted LocalDocumentOwnerV1.id or useLocalDocumentWindowV1 lane, whereas the existing explicit draft mechanism reads both. localDocumentOwnerIdV1 encodes pluginId/appId/sessionInstanceId/runtimeKey/clientInstanceId; owner.active supplies lifecycle validity. Reusing identical controller/surface/edit args under another document can otherwise retain the old scoped draft.

Also, procedural parameter control_arguments explicitly supplies initialValueJsonChunks in Try mode (preview-and-params-body/🦀️.rs:141). That is answer baseline content, not semantic edit identity. Including all editArguments in scope therefore resets local source draft on a baseline publication. The explicit draft scope already excludes revision. Appropriate source identity consists of mounted owner/window and semantic target fields (controller/action/argument + question IDs/key/paramKey/window identity), excluding labels, source limits, publication revision and initial answer baseline. Sent to browser_execution for their ongoing fix; no edits made.

## Final Source Review Closure

The previously reported locale/baseline/mounted-owner scope findings are resolved in current source. sourceScope now includes LocalDocumentOwner.id and the length-prefixed actual mounted window + semantic edit identity. The helper selects editIdentityArguments when supplied, excluding display labels and maximum-byte settings. Provider Try text settings supply only key/paramKey/windowId/windowKindId as identity while preserving complete initialValueJsonChunks in dispatched editArguments. Blueprint uses its existing semantic-only arguments.

commitSource checks owner.active before dispatch and again on acknowledgement, checks activeSourceScopeRef after await, and ignores post-unmount completion. Scope change clears old draft/issued refs; owner retirement invalidates active scope and clears them. A stale admission changes state only if the current draft text still equals its text. This preserves a newer invalid draft and its refusal flag.

Accepted local text is retained with admitted marker until matching scene.buffer arrives in a scene object different from the object captured at issuance; the old matching published buffer alone does not clear an outstanding reversion. changeSource tracks awaitingPublication so editing back to the old canonical value while a different mutation is pending remains a local draft rather than disappearing prematurely. Explicit discard remains user-authorized local reset.

No additional concrete defect established in this final focused source review. This closes source inspection only. Mounted/browser live recovery and native compilation remain separate pending verification; prior focused test passes were reported by the coordinator, not executed by this audit.

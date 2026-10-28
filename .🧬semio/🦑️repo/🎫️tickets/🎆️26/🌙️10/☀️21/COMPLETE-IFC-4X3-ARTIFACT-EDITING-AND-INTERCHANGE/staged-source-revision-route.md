# Authenticated Staging Source Revision Route

Source inspection found an existing active host route, so no new source Extension or fabricated bootstrap token is needed.

## Native Source And Transport

plugin/instance/plugin-app-impl/ephemeral-interaction begins a LocalInteractionLiveQuery under the exclusive mounted app. It captures self.stores.store.content_revision_now() as LocalInteractionIdentity.document_revision, alongside actual document/config/interaction snapshot reads. CapturedRoot.identity_json emits all32bytes as64lowercasehex in documentRevision. This is distinct from the interaction slice revision/generation and from UiPublicationRevision's first8byte projection.

Framework AppChannelClient.readLocalInteractionPages delegates to app-channel/local-interaction-query. Its existing client lifecycle allows one query, carries AbortSignal, uses the original request/query sequence, checks started token appInstanceId, correlates each reply token and page ordinal, acknowledges only after the consumer resolves, and completes only after native closed/root retirement. Retiring the client cancels its query. This is a frozen admitted capture; a later document mutation may make its revision stale and must invalidate staged readiness. Native form handling must always recheck the full document hash.

PluginRuntime handle-contract exposes readLocalInteraction(instanceId,signal). The active channel adapter invokes that paged route, enforces LOCAL_INTERACTION_CAPTURE_MAX_BYTES, decodes the completed capture and verifies localInteractionIdentityEquals(capture.identity, transportIdentity) before returning it. AppDocumentIdentity/readAppDocumentIdentity is a different metadata route containing only app_instance_id and parent_document_id; it does not provide the hash.

## Actual Renderer Owner

The real caller is ShellHost/focus-and-input/registers-and-context-menu publishLocalInteraction. It already awaits plugin.readLocalInteraction(instanceId), checks the current session's instance id and exact loaded plugin handle, then calls localInteraction.observe with only capture.state plus current hover. The admitted capture.identity is currently discarded.

ShellHost/shell-core-state owns this localInteraction store once per shell. Shell/shell-store LocalInteractionStoreV1 retains InteractionState only and exposes get/observe/subscribe. No existing cached canonical hash was found there. Observation runs through createLatestAsyncDispatcher on activation and after OBSERVED_INTERACTION_ACTION_IDS in the real program-dispatch funnel. That is not yet a complete canonical-source invalidation subscription for all document edits.

The clean host integration is to retain the already-admitted identity beside this same active owner, with explicit plugin handle/channel/session lifetime correlation, instead of recapturing the full model solely for its hash. InteractionIdentity.generation is the interaction store generation, not actor activation generation and not a form generation. Source-change/abort/session or channel replacement must retire readiness/proof. The browser owner remains root's scope; no browser source edits were made during this inspection.

## Native Comparison

AppOperationContext.canonical_base_revision_hex at declaration/view/operation-contexts is the existing full lowercase formatter. AppRenderOperationContext carries canonical_base_revision:[u8;32]; the native guided producer should compare the request token against that full pinned value. The source query can be stale after capture, so authenticated bootstrap is not command admission. Template semantic publication remains refused.

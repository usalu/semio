# Return Content Coverage Review

## Read-Only Findings

The current common Kernel TurnResult owns UI patches, effects, render-plane presence, next wake, status, fuel use, command ingress, lifecycle receipt and issued UI patch receipt. The current WIT record likewise includes presence. Actor's outer TurnResult has no presence field, and PluginRuntime.coerceTurnResult extracts no presence field. The replacement retained-return grammar must not reproduce this omission.

The existing generated bridge lifts the whole WIT result, spreads it into a new object and normalizes receipt and ingress fields. The current UI patch operations include upsert, component, layout, activity, children, style, accessibility, bindings, menu, removal and root selection. Their payloads include packed bytes and child arrays; paging only the outer patch array would not bound those descendants.

Effects are not just notifications. Effect::SendMessage delivers AppFrame replies to the original instance; those include Invocation results used by extension completion. The current typed-operation page path also uses SendMessage with a specific magic/header, and its acknowledgement is a semantic message. These are different from raw return-input ACKs and issued UI publication ACKs. The canonical host decoder must retain each domain owner and preserve exact original request authority without silently dropping unrecognized effect content.

## Live Intake Join

OwnedUiPatchIntake currently expects one privately bound native patch authority, then looks up the exact original aggregate's surface and offers one operation at a time. It keeps input retirement, publication acknowledgement, patch close and surface handoff as separate phases. Constructor/source validation and close refusal handling were read directly.

For paged return adoption, its operation source must become an incremental, retained input owner. It must not reconstruct a whole operations array from all return pages before offering the first operation. The parent raw owner cannot release the page while a decoder, accepted UI input or borrowed byte view still owns it. Conversely, raw return retirement cannot wait for a semantic publication ACK that needs a later execute turn.

The original response envelope must be held before pending-map removal, worker error extraction and external continuation. Current private output-cell tests cover this retention seam, not live return mounting or eventual unknown-wrapper disposal.

## Coordination

The presence omission and effect/AppFrame distinction were sent to the native/schema coordinator. Its fixed result vectors and variable semantic grammar remain the canonical authority; no schema, native, WIT, UI or live transport source was changed in this review.

The coordinator subsequently released a declaration for review at common Kernel/📤️return/📦️content. All four files (wire declaration, declaration schema, fixture and fixture schema) were read completely. The declaration now explicitly includes presence and exact WIT effect encoding, eleven UI opcodes, independent packed-field symbol tables, split UTF-8/varints and records crossing 4096-byte page boundaries. Its oracle/acceptance release was still pending at this read.

The planned outer framing decoder may retain only bounded header state while a body crosses pages. Record length remains a u64 count, not permission to allocate a whole field. Body spans and header completion must not mint input-retirement or semantic-publication authority. The actual semantic consumer needs independently admitted owned storage before the parent page ACK; a borrowed slice may not wait for the next page. The UI owner has been asked for its concrete paged-input callable before that join is implemented.

The released fixed-result schema now requires ordinary poll success to be pending/page/retired; fixed poll control responses may only be blocked/refused with a non-none fault. Malformed authority uses authority-free protocolFault rather than invented identity. The actual host codec follows those released rules.

The content declaration was subsequently accepted and read in full again. Pack is opaque length-delimited bytes with a field-owned dialect: SendMessage preserves the exact AppFrame Invocation bytes, UI fields/presence use their own store wire-value documents, and nested scene bytes retain their existing dialect. The Invocation effect and full typed presence fixture are mounted. Full presence binary parity remains a separate native oracle obligation. Outer framing/section-order work is now released; no native whole-runtime readiness is inferred.

The concrete type-only WIT return-page interface was read at plugin schema lines 987–1051. Its protocol-fault(fault) case requires the canonical malformed-control/mixed-control subset check. The pending single poll signature is (events, command-page, return-drive, budget) -> return-result without a plugin-error wrapper; that signature and live typed-source retention are not mounted yet.

This report is a source review, not runtime proof. All six apps still require fresh built artifacts, real content/interaction and exact close/reopen verification.

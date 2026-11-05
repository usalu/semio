# Admitted Selection Cancellation Route

The generic retained operation action is cancelTypedOperation (CANCEL_TYPED_OPERATION_ACTION_ID), not a confirmed generic engagementAbort. Payload is exactly operationId and generation, each a sixteen-character lowercase hexadecimal string. The native progress controls create that exact binding in plugin `⏳️operation-progress/🦀️.rs:84–99`; decode_operation_cancellation at30–42 validates it.

Generic instance dispatch in plugin `🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:171–172` routes this action through tools.dispatch_operation_cancellation before the plugin action bridge. Actor, app instance and generation witnesses must match the admitted operation. An Escape hook must obtain the real operation identity/status and must not manufacture an id from the removed local candidate.

Raster `✏️editor/📤️export/🧪️tests/🦀️.rs:163–175` demonstrates the supported route against an actual admitted retained job: render progress controls, extract operationId/generation, refuse stale generation and foreign actor/instance, accept repeated correct cancellation, drain terminal publication and verify final progress UI refresh. This is the existing test source, not a newly executed result.

The pre-submission local selection map only owns preparation/encoding. After SelectPixels admission its Escape cancellation needs this generic identity-bound route. Exact APIs and source locations were sent directly to native_selection and parent; no cancellation implementation was edited by this audit.

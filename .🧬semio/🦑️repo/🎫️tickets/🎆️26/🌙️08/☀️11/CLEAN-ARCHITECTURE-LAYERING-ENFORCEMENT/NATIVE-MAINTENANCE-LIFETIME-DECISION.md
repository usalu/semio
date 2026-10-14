# Explicit Native Maintenance Receiver Lifetime

The current common Native image maintenance interface hides a real lifetime distinction. Opened shared receivers release their reader for an exclusive attempt and remain released after either outcome. A privately provisioned image already owns its exclusive physical lifetime and retains its receiver after a successful maintenance body. Documenting universal consumption or universal retention would be false. Earlier Root assumptions are superseded by these actual defining implementations.

Use one schema-first closed result with a required explicit `receiver` discriminator. Keep actual backend ownership behavior and express it accurately through the shared interface:

| Outcome | Required result | Receiver after return |
| --- | --- | --- |
| Opened optional attempt declined | `acquired:false, receiver:"released"` | Released; no body value or callback effect |
| Opened exclusive body completed | `acquired:true, receiver:"released", value:T` | Released; actual body result retained |
| Private provisioned body completed | `acquired:true, receiver:"retained", value:T` | Original private receiver retained |

`T` is the actual callback result; a public arbitrary parser cast is not introduced. Outcome metadata does not issue Native/Store custody. Native receiver rechecks remain authoritative. Invalid request, cancellation or body failures must preserve their actual errors and paid exclusive release; no result is manufactured for a failed operation.

The Specific missing/zero-catalog startup recovery is optional maintenance. It should explicitly request `wait:false`, avoiding a required exclusive transition while an original caller may already hold a reader. The caller reopens only a released receiver, with the same actual open closure, original control, fixed physical namespace, native bounds and source lineage. It retains/rechecks an actually retained receiver. No defaults for old shapes, legacy overload, fresh timeout, fake live image or new namespace is added.

RuntimeHigh owns the narrow common contract/schema/backend result and post-await checks, portable/independent oracle laws, Specific opener, and exact native schema catalog metadata. ScopeHigh owns independent graph fact Store original-scope/queued lease investigation. Low independently reviews both. All prior actual operations are terminal; no new native/factory/initializer is authorized by this decision.

Test-first qualification must cover all three outcomes, strict unknown/missing/out-of-shape refusal, callback suppression on declined acquisition, actual receiver release/retention paths, original post-await cancellation and exact Specific reopen arguments. Existing test libraries remain test-only. Real native/image/Store execution and the cause of Actor18/CE2 queued waits remain unqualified until actual registered runs establish them. The current Source mismatch is documented separately in [the queue audit](GRAPH-FACT-QUEUE-IDENTITY-STATIC-AUDIT.md).

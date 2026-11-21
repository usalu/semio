# Hover Action Audit

The host interaction verb calls `parse_interaction_hover_targets`, which accepts both direct `targetId` and `granularityId`, and treats a present `targets` field as authoritative, including `"[]"`. The native retained hover router emits authored args on enter and `input.targets = "[]"` on leave. The host checks hover ownership before clearing. No additional refusal was identified in the inspected reserved job route.

Coverage gap: the React tree hover test and native retained router test both use the conventional `targets` batch, and check emitted intent rather than complete merged action through host dispatch. Add tests for direct authored fields on enter and a leave with both direct fields and `targets: "[]"`, through each retained intent-to-action path and the reserved host dispatch. This verifies that the input overlay survives transport and wins over authored direct fields. No tests were run in this read-only audit.

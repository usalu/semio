# Typing Policy Routing Review

Read-only current-source audit; no code edits, compiler, tests, or processes run. Native typing/metadata remains unexecuted. Eight portable policy cases are owner-reported passes.

## Normal Action and Manifest Paths

Actions lines168–169 complete the before pass before registry validation188–189; that pass admits suppress=false, so a typingCommit cannot suppress an unknown action there. The validated control pass is only entered at196. Manifest commands validate app/mode owner and classifier225–238 before control247. Pending resumes145/217 inspect the actual retained owner control_pass rather than a separately inferred before-ready flag. Host active resumes43–66 retain the same control-ready/tag/decision fields; normal before-ready and control-ready are distinct ActionIngressSource fields. Metadata remains private in the same typing owner until finish_into atomically restores metadata and ready/decision, including partial close turns.

Typing implementation50–85 admits control&&suppress, !control&&all and control-only retire_windows. Before pass captures time for lapse, while control skips unused clock. Explicit commit82 is based on retained suppression and actual addressed window. Actual absent-window retirement83 requires control-retire policy, nonempty window ID and nonempty inherited roster. Thus global window empty-string and empty roster preserve the intended no-retirement-authority behavior. No concrete metadata/ready loss was found in these paths.

## Direct Intent Validation Question

Current plugin-app-impl handle_intent_frame504 checks only action version before capture_original_direct_tool_tag506/514. That helper actions82–84 enters prepare_original_tool_dispatch, whose control typing pass may set suppression before the original command producer/catalog admission at517. Valid typing buffer plus known typingCommit sets suppress in typing pass60–64; registry.get at59 supplies view classification only, with unknown actions treated as non-view. handle_intent_frame511/514 can then admit_empty instead of calling the producer. The inspected sole runtime plugin_dispatch_intent caller typed-operations467–469 advances retained arguments/metadata then directly calls handle_intent_frame; its documented caller staleness check is not itself a registry/classification proof.

Source_lifetime and root were asked for any earlier actual intent qualification seam before calling this a complete-path defect. Without such a seam, unknown direct intents carrying valid typingCommit can be suppressed before registry validation. Normal action/manifest validation does not cover this direct method. This question needs an explicit source-bound qualification or a real negative native intent case; no broad compatibility fallback is proposed.

## Final Direct Intent Fix Review

Fresh source read verifies the direct validation gap is closed in current source. capture_original_direct_tool_tag actions82–86 now admits the actual typed source, obtains the actual DeclaredToolCatalogAlias source and requires qualify_action before shared typing control. Both handle_intent_frame capture calls use that helper.

qualify_action typed47–55 validates original source/decoder/caller, pins actual catalog and incoming pointer/length, retains the genuine OriginalToolQualification and registration, and returns false throughout qualification progression/transfer/closure. Its true branches compare actual canonical registration tool_id and app owner, final admission verb, or the retained admission owner matches_registered_action. Thus pointer/length alone is not the authorization witness, including when inline bytes change in place. Qualification/admission leases remain in the typed owner; existing close_demands/close_step close those owners and controlled-retire registration before final source reset. No cold qualifier or synthetic proof was found.

The authored selected native tests616–665 invoke real handle_intent_frame against layout_app_with_registry: unknown and known typingCommit controls, zero-grant source invariance, cancelled pending qualification with preserved args/metadata, and same-pointer/same-length action rewrite after metadata becomes private. They assert refusal without progress or Store change and controlled source/app closure. These are source-authored tests, not executed evidence here. No further concrete authorization or retirement hole was found in the bounded final packet. Source-checker passes and native execution remain distinct; actual native typing/qualification results are pending.

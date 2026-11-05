# Nested Inspector Mutation Regression

Native red run 63909 compiled the live-core tests and failed at the first ISO source case with no kind sets partNumberRule.source. This reproduced the actual inspector routing gap. Existing parent mutations already provide the correct typed semantic boundary. Root added nearest-parent resolution, preserving exact-leaf precedence, siblings and parent selectors; deeper selectors remain inside the replacement value. Missing paths and textual prefixes remain refusals.

The closed schema, twelve neutral cases and English feature are under norm/tests/nested-edit. Native tests compare output with an independent serde_json JSON Pointer oracle. Synthetic ISO value shape now matches the real kind=script/functionId/source shape. Real ISO/VDI source witnesses are documented separately; their actual native engineering integration is still required.

Atomic write bundles must resolve against the current typed staged projection, otherwise a later replacement can overwrite the earlier sibling. Root authored real typed sparse-diff/ColdMutation fixtures and batch admission tests. Red run 84548 is pending. The planned implementation uses the existing kernel ColdReplayProjection and admission-aware fold_operation_into interfaces, with typed retirement, warning preservation and whole-bundle refusal; it does not treat a JSON shadow as native execution. Production sequential staging is not yet implemented.

Browser 88312 was deliberately canceled after measuring 3.62 GB of owned browser planner memory alongside the 2.96 GB package planner and severe swap contention. Exact owned process descendants exited and leases were released; root will resume the complete browser journey after the current package planner. No completion claim is inferred from authored tests or cancellation.

## Actual Typed Batch Red and Implementation

Native run 47293 executed the real sparse-diff model: all twelve parent payload cases passed, while both batch tests failed as intended. One observed final configuration was (a=1,b=4) instead of (a=3,b=4); Error/Fatal operations were also incorrectly returned for emission. Root then implemented sequential validation with the existing kernel-owned ColdReplayProjection and admission-aware in-place fold, explicit typed disposal on refusal, current-projection numeric coercion and per-operation indices. Placement grouping is now a pure plan feeding the same typed stage. String pseudo-mutation tests were replaced with the actual sparse-diff fixture.

The first full-core green attempt exposed an incorrect retirement trait namespace; all eight references now use value::retirement::RetireOwned, preserving the derive macro at the value root. A corrected full-core run is active. The independent metadata predicate can_set_field and neutral access cases are also authored. No full-core or business-integration green is claimed yet.

Additional source audits found unsupported scalar fields still offered mutating controls and list Add omitted the required typed item. runtime_full owns a schema-first read-only scalar repair for all fifteen providers; its production/source fixture coherence is confirmed and native execution is pending. Supported list creation and structured variant changes need an actual typed staging form; schema defaults are form seeds, not proof of a valid new product. These remain completion work.

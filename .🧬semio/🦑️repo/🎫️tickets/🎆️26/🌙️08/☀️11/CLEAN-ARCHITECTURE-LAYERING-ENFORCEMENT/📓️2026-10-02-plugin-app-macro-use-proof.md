# Plugin App Macro Use Proof

Read-only source census; no compiler/test execution. Actual app inline body starts351 and closes39302 in OS plugin `🦀️.rs`; attribute350 is `#[cfg_attr(test, macro_use)]`.

| App macro | Definition | Export |
| --- | --- | --- |
| app_labels |6922 |macro_export |
| app_action_enum |12977 |macro_export |
| app_commands |13076 |macro_export |
| view_commands |13308 |macro_export |
| bounded_first_step_tool_proofs |15435 |macro_export |
| framework_reserved_job |18078 |private |
| surface_builder_forward |38675 |private |

All authored executable private-macro occurrences found in framework/S/hub are inside the app body: framework_reserved_job twenty invocations18251–18274; surface_builder_forward recursive template call38678 and invocation38694. Two outside matches in plugin-runtime-plugin-builder-contract705/2634 are documentation, not invocations. Therefore no observed external caller requires those private macros to escape app.

Outside tests use exported macros: app-terminology4 invokes app_labels; app-app-commands31/95 invokes app_commands; mutation fixture tests and runtime-builder1431 use explicit crate::bounded_first_step_tool_proofs. The five public macros already have crate-root macro namespace authority through macro_export, independent of module macro_use. Removing the test-only macro_use consequently removes redundant outward textual visibility of the two private helpers; it does not remove the five exported macro identities. Later plugin_exports/actor/extension macros are outside app and do not participate in this attribute.

This is concrete source justification for narrow authored attribute removal, preserving strict scope metadata refusal. Native compilation/runtime of affected outside unqualified macro calls remains a necessary execution receipt; source review alone is not a passing claim. Do not relax the graph attribute gate or add a general macro_use waiver.

Later execution correction: the authored-call census above missed generated consumer macro bindings. Actual post-removal plugin compilation refused432 diagnostics rooted in two unbound generated dispatch macros. Thus the source justification was incomplete and cannot establish safe removal by itself. High corrected explicit generated consumer bindings while retaining the strict scope cleanup; Native subsequently reports current actual cfg(test) host1 GREEN. Preserve this failed compile and correction as part of the proof rather than relying on the earlier authored-only inventory.

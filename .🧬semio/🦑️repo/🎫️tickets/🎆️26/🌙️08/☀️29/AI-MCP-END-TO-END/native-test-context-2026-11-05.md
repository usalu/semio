# Native Test Context

Historical test explanations moved out of the bounded Rust compilation unit. They document prior defects and fixture assumptions; they do not establish current native execution or full end-to-end success. Test code, assertions and all source boundaries are unchanged.

## fixture_catalog

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs

🧫️ The compiled note+cad fixture catalog — every test below asserting a KNOWN capability
census injects THIS, never `build_catalog()`, whose whole job since ticket
26/08/29/AI-MCP-END-TO-END is to discover whichever plugins are really installed.

## tools_list_is_the_full_real_gateway_surface

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs

🎯️ `tools/list` is 28 tools and every one of them is real: 3 core gateway + 8 mutation-protocol
+ 5 `🗿️artifact` + 3 `💡️inference` (discovery pair + the general `inference_run` execution route)
+ 4 hub-backed `💡️inference` jobs + 4 `🖥️ui` + `conversation_reply` (ticket 26/09/18 slice AC1,
the agent's own free-text turn). Ticket 26/08/29/AI-MCP-END-TO-END retired the last stub, so
there is no longer a "declared but unimplemented" bucket to assert against.

## workspace_backed_tools_degrade_to_a_retryable_plugin_unavailable_without_a_binding

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs

🪜️ Progressive enhancement, tier 1: with no workspace bound, a workspace-backed tool is
present in `tools/list` and answers a principal holding its scopes with a structured, RETRYABLE
`PLUGIN_UNAVAILABLE` naming the binding it needs — never a protocol-level failure, never fabricated data.

## the_tool_census_matches_the_registry_exactly

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs

🎯️ The census and the registry agree — no tool is registered that the census omits, and no
census name is unregistered. This is what makes `GATEWAY_TOOL_NAMES` a fact, not a comment.

## action_prepare_tool_call_returns_a_prepared_action_report_for_a_granted_scope

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs

🎬️ The exact scenario the brief's §5 live transcript demonstrates, proven deterministically:
a principal WITH `artifact.write` can `action_prepare` the cad demo capability and gets back a
real `PreparedActionReport`.

## action_prepare_tool_call_is_permission_denied_for_a_scope_the_principal_lacks

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs

🎬️ The second half of the brief's §5 live transcript: a principal WITHOUT the required scope
gets `PERMISSION_DENIED`, never a protocol-level failure.

no scopes granted

## a_hub_open_retries_only_what_the_hub_marks_retryable_and_always_terminates

Source: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️quick/🦀️.rs

⏳️ A hub whose descriptor index is mid-refresh answers a RETRYABLE `PLUGIN_UNAVAILABLE`, and
until ticket 26/09/18 slice M9 nothing retried it: the gateway process exited before
`initialize` (measured live 2026-09-21, M8 §5.4(2)). The law pins the three things that make the
repair a retry and not a hang — a non-retryable failure is never re-attempted, a retryable one is
re-attempted a BOUNDED number of times, and the total wait is finite and small.

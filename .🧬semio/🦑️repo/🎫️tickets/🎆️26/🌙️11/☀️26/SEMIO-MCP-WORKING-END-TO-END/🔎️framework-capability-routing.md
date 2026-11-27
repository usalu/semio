# Framework capability routing (headless MCP)

## Finding

`framework.*` catalog entries (`CapabilityOwner::Framework`, compiled once per distinct framework action id in
`🗂️catalog/♻️framework-dedup`) were refused by every channel:

- `RoutingArtifactChannel::select_target` / `route_for` called `resolve_route_for_capability_in`, which only accepts
  `CapabilityOwner::Plugin` (`PLUGIN_UNAVAILABLE ... owned by framework, not a plugin`).
- `ShellArtifactChannel::select_target` refused them the same way (`target capability has no plugin owner`), so the
  shell-routed lane was equally unreachable.
- Even past routing, the guest address resolver (`PluginArtifactChannel::addressed_action_invocation`) only matched
  `<plugin>.<app>.<action>` ids.

The execution side already exists: every plugin guest embeds the framework layer. `preview_addressed_action`
(`🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions`) accepts a framework reserved action id
(`framework_reserved_action_kind`) at `window_kind_id: "*"` and previews it through the reserved jobs
(`👷️job/🏛️reserved`). So a framework capability runs in the guest of the artifact's own app; only the host routing was missing.

## Fix

- `🔀️routing-and-shell-routed`: `RoutingArtifactChannel::owner_route` resolves a framework capability's route from the
  target artifact binding (`plugin_id`, `app_id`); `target.artifactRef` is required and must be a bound artifact.
  `route_for` skips the plugin-owner check for framework capabilities (the selected target already pins the route).
  `resolve_plugin_for_capability_in` now answers `INPUT_INVALID` "name it with `target.artifactRef`" for a framework id.
- `🐚️channel`: `ShellArtifactChannel::select_target` resolves a framework capability against the live instance whose
  `artifact_ref` is the target (plugin/app taken from that instance); the bound-instance check accepts framework owners.
  `ShellRoutedArtifactChannel` forwards to both, so both lanes agree.
- `📨️plugin-artifact-channel/💡️inference-route-and-actions`: `addressed_action_invocation` maps `framework.<actionId>` to
  an `ActionAddress` of the channel's own app (`window_kind_id "*"`, first window kind as window instance).
- `🗂️catalog/♻️framework-dedup`: framework capability descriptions end with "Runs on one artifact: pass
  `target.artifactRef` with action_prepare." (en/de), so `capabilities_search`/`capabilities_describe` tell agents.

## Tests

Added to `🏠️workspace/🧪️tests/🔬️quick/🧭️plugin-capability-routing/🦀️.rs`:
`framework_capabilities_route_from_the_target_artifact_binding`,
`resolve_plugin_for_capability_asks_a_framework_capability_for_its_target_artifact`.
`cargo check --tests -p semio-os-mcp-workspace` compiles all of this. The `workspace` test target currently does not
build for unrelated, pre-existing reasons in `🧪️tests/🔬️long/🦀️.rs` (missing `semio_os_store_test_support`,
`exchange_blocking(&mut &mut ..)`, `DslValue == &str`), so the new tests could not be executed.

## End-to-end verification status

Not run to completion. The `.mcp.json` launcher answers from the previously published (stale) binary and refreshes in the
background; both attempts to refresh it with this change failed on other agents' in-flight edits
(`semio_plugin_document_media::StallWait` missing in `🔌️plugin/🪜️maintenance-ladder`), and the stale binary still shows the old
`PLUGIN_UNAVAILABLE ... owned by framework` refusal. `artifact_create` also still failed with `BUDGET_EXCEEDED` on the stale
binary (the other agent's area), so no plugin artifact could be bound.

Re-run once the plugin layer builds again (the flow polls until the refreshed server publishes the new description and until
`artifact_create` succeeds):

    cd <ticket folder> && mkdir -p 🗑️generated && python3 🐍️probe-flow.py 🧪️flow-framework-capability.jsonl 14000

The flow covers action_prepare/invoke of a framework capability, transaction_begin/rollback with two handles, history_undo/redo.
Plugin-capability invoke, transaction_commit, action_cancel and job_get/job_cancel lines still need to be added once an
artifact exists (ids come from the replies).

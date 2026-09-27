# WGPU Command Panel Tree Parity

## Authority

React's `buildCommandCategoryTree` publishes every command as a `TreeDataItem`. A zero-argument item executes immediately. An argument-carrying item toggles one owner-qualified command key and never executes from that row. The expanded command moves out of the list and into a leading form section whose argument controls are tree items and whose Execute and Reset controls are section-header actions.

The WGPU body previously used a stack of generic sections. Zero-argument commands were inert text, supported OS arguments appeared as immediately committing Select controls, and Execute and Reset appeared as content buttons.

## Contract decision

`TreeSectionProps.header_toolbar` is an explicit relation to a direct horizontal `ContainerRole::Toolbar` child containing real Button records. This mirrors `TreeItemProps.inline_toolbar`, preserves disabled state, stable button identity, action bindings, accessibility, and independent hit targets, and avoids a second action protocol. The WGPU legacy projection mirrors the relation as `UiTreeSectionNode.header_toolbar` and positions it inside the section header.

## Command routing

- `executeResolvedCommand` accepts only a current, visible, zero-argument resolved command. It re-resolves the opaque owner-qualified key instead of parsing it. OS commands stay inside `apply_os_command`; plugin, app, and active-mode commands use `dispatch_command`.
- `setCommandExpanded` accepts only a current, visible argument-carrying command and toggles its owner-qualified key.
- `executeStagedCommand`, `stageCommandArg`, and `resetStagedCommand` remain the form boundary.
- Row ids use `command.{stable-key with ':' replaced by '.'}`, so identical local ids from OS, plugin, app, and mode owners cannot collide.

## Neutral evidence

The shared fixture is `Shell/🧫️fixtures/🎛️command-panel/🔣️.json`; its schema is `Shell/🧬️schema/🎛️command-panel/🔣️.json`. It covers English and German labels, four colliding zero-argument owner scopes, two explicit argument forms, singleton auto-expansion, staged readiness, treeitem roles, and Execute/Reset header actions.

## Validation

The public TreeSection relation is closed across Rust contract, typed visitor, builder, schema metadata, checked-in TypeScript contract, owned wire decoder, native and TypeScript graph validation, WGPU legacy projection, retained reconciliation, mounted layout, paint reservation, and every literal constructor. `rustfmt --edition 2021 --emit stdout` parsed every changed Rust source and test.

The mounted React oracle passed 5/5 in `🗑️generated/astra-runtime/commands29/react-4.log`. It verifies English and German rows, all four owner-qualified zero-argument identities, explicit expansion, staged required-argument gating, Execute/Reset callbacks, singleton auto-expansion, and the form's actual default-closed state.

The queued native laws are:

- `tree_section_header_toolbar_is_a_direct_horizontal_button_toolbar`
- `a_closed_tree_section_keeps_its_header_toolbar_live_and_collapses_only_its_rows`
- `command_panel_fixture_maps_every_owner_to_actionable_tree_rows`
- `resolved_zero_argument_command_executes_once_and_a_stale_key_is_rejected`
- `argument_command_ingress_toggles_only_a_live_owner_qualified_form`
- `the_command_dock_opens_the_expanded_commands_staged_form`
- `execute_is_disabled_until_every_required_argument_is_staged`
- `build_command_panel_ui_groups_rows_under_category_headers`

Native execution is pending because the current workspace build stopped upstream in unrelated XML/SVG compilation before reaching UI or renderer tests. No native pass is claimed.

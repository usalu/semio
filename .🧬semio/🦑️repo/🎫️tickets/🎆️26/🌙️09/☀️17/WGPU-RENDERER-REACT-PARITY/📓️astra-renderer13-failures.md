# Renderer13 Native Failures

1160 tests run:1145 passed,15 failed,0 skipped (38.447 seconds).

```text
        FAIL [   0.024s] ( 156/1160) semio-framework-os-renderer-wgpu dock::tests::key_event_matches_chord_respects_modifiers
    thread 'dock::tests::key_event_matches_chord_respects_modifiers' (18072598) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-unit/🦀️.rs:722:5:
    assertion failed: key_event_matches_chord(&z, &mods(false, true, false, false), "mod+z")
```

```text
        FAIL [   0.061s] ( 224/1160) semio-framework-os-renderer-wgpu engine_canvas::node_graph_attach_tests::tutorial_semantic_points_resolve_through_live_graph_geometry
    thread 'engine_canvas::node_graph_attach_tests::tutorial_semantic_points_resolve_through_live_graph_geometry' (18072826) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs:205:9:
    assertion `left == right` failed: off-surface semantic point is not targetable: Domain { id: "graph-main", domain: "slider", entity: "height", value: 0.0 }
      left: Some((-55.960037, 408.7217))
     right: None
```

```text
        FAIL [   0.040s] ( 713/1160) semio-framework-os-renderer-wgpu shell::command_registry_tests::build_command_panel_ui_groups_rows_under_category_headers
    thread 'shell::command_registry_tests::build_command_panel_ui_groups_rows_under_category_headers' (18074731) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-command-registry/🦀️.rs:933:5:
    assertion `left == right` failed
      left: 5
     right: 4
```

```text
        FAIL [   0.034s] ( 789/1160) semio-framework-os-renderer-wgpu shell::host_door_remainder_tests::the_undo_redo_chords_match_reacts_gate_and_stay_shadowable
    thread 'shell::host_door_remainder_tests::the_undo_redo_chords_match_reacts_gate_and_stay_shadowable' (18075301) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs:148:9:
    assertion `left == right` failed
      left: None
     right: Some(Undo)
```

```text
        FAIL [   0.046s] ( 833/1160) semio-framework-os-renderer-wgpu shell::panel_anchor_model_tests::a_keybinding_override_remaps_a_shell_chord_and_frees_its_default
    thread 'shell::panel_anchor_model_tests::a_keybinding_override_remaps_a_shell_chord_and_frees_its_default' (18075552) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:709:5:
    assertion `left == right` failed: ⌨️ the DEFAULT table is untouched
      left: None
     right: Some(ToggleSearch)
```

```text
        FAIL [   0.091s] ( 851/1160) semio-framework-os-renderer-wgpu shell::panel_anchor_model_tests::default_dock_matches_the_shared_react_fixture
    thread 'shell::panel_anchor_model_tests::default_dock_matches_the_shared_react_fixture' (18075741) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:248:5:
    assertion `left == right` failed: 🧭️ the whole skeleton, branch children included, must match the fixture
      left: DockSkeleton { version: 3, anchors: {"bottom-left": [DockTabSkeleton { id: "framework.category.display", children: Some([DockTabSkeleton { id: "framework.display.windows", children: None, trees: Some(["framework.display.windows.tree"]) }, DockTabSkeleton { id: "framework.display.layout", children: None, trees: Some(["framework.display.layout.tree"]) }]), trees: None }, DockTabSkeleton { id: "fixture.display", children: None, trees: Some(["fixture.display.tree"]) }, DockTabSkeleton { id: "s-sync-status", children: None, trees: Some(["s-sync-status.tree"]) }], "bottom-middle": [DockTabSkeleton { id: "framework.category.command", children: Some([DockTabSkeleton { id: "command.category.window", children: None, trees: Some(["command.category.window.tree"]) }, DockTabSkeleton { id: "command.category.appearance", children: None, trees: Some(["command.category.appearance.tree"]) }, DockTabSkeleton { id: "command.category.layout", children: None, trees: Some(["command.category.layout.tree"]) }, DockTabSkeleton { id: "command.category.language", children: None, trees: Some(["command.category.language.tree"]) }, DockTabSkeleton { id: "command.category.general", children: None, trees: Some(["command.category.general.tree"]) }]), trees: None }], "bottom-right": [DockTabSkeleton { id: "framework.settings", children: Some([DockTabSkeleton { id: "fixture.settings", children: None, trees: Some(["fixture.settings.tree"]) }, DockTabSkeleton { id: "framework.settings.general", children: None, trees: Some(["framework.settings.general.tree"]) }, DockTabSkeleton { id: "framework.settings.theme", children: None, trees: Some(["framework.settings.theme.tree"]) }, DockTabSkeleton { id: "framework.settings.keybindings", children: None, trees: Some(["framework.settings.keybindings.tree"]) }, DockTabSkeleton { id: "framework.settings.default-apps", children: None, trees: Some(["framework.settings.default-apps.tree"]) }, DockTabSkeleton { id: "framework.settings.conflicts", children: None, trees: Some(["framework.settings.conflicts.tree"]) }]), trees: None }, DockTabSkeleton { id: "framework.marketplace", children: None, trees: Some(["framework.marketplace.tree"]) }, DockTabSkeleton { id: "os.task-manager", children: None, trees: Some(["os.task-manager.tree"]) }, DockTabSkeleton { id: "framework.panel.history", children: None, trees: Some(["framework.panel.history.tree"]) }], "left-middle": [], "right-middle": [], "top-left": [DockTabSkeleton { id: "fixture.workbench", children: None, trees: Some(["fixture.workbench.tree"]) }], "top-middle": [], "top-right": [DockTabSkeleton { id: "fixture.details", children: None, trees: Some(["fixture.details.tree"]) }, DockTabSkeleton { id: "framework.chat", children: None, trees: Some(["framework.chat.tree"]) }]} }
     right: DockSkeleton { version: 3, anchors: {"bottom-left": [DockTabSkeleton { id: "framework.category.display", children: Some([DockTabSkeleton { id: "framework.display.windows", children: None, trees: Some(["framework.display.windows.tree"]) }, DockTabSkeleton { id: "framework.display.layout", children: None, trees: Some(["framework.display.layout.tree"]) }]), trees: None }, DockTabSkeleton { id: "fixture.display", children: None, trees: Some(["fixture.display.tree"]) }, DockTabSkeleton { id: "s-sync-status", children: None, trees: Some(["s-sync-status.tree"]) }], "bottom-middle": [DockTabSkeleton { id: "framework.category.command", children: Some([DockTabSkeleton { id: "command.category.window", children: None, trees: Some(["command.category.window.tree"]) }, DockTabSkeleton { id: "command.category.appearance", children: None, trees: Some(["command.category.appearance.tree"]) }, DockTabSkeleton { id: "command.category.layout", children: None, trees: Some(["command.category.layout.tree"]) }, DockTabSkeleton { id: "command.category.language", children: None, trees: Some(["command.category.language.tree"]) }]), trees: None }], "bottom-right": [DockTabSkeleton { id: "framework.settings", children: Some([DockTabSkeleton { id: "fixture.settings", children: None, trees: Some(["fixture.settings.tree"]) }, DockTabSkeleton { id: "framework.settings.general", children: None, trees: Some(["framework.settings.general.tree"]) }, DockTabSkeleton { id: "framework.settings.theme", children: None, trees: Some(["framework.settings.theme.tree"]) }, DockTabSkeleton { id: "framework.settings.keybindings", children: None, trees: Some(["framework.settings.keybindings.tree"]) }, DockTabSkeleton { id: "framework.settings.default-apps", children: None, trees: Some(["framework.settings.default-apps.tree"]) }, DockTabSkeleton { id: "framework.settings.conflicts", children: None, trees: Some(["framework.settings.conflicts.tree"]) }]), trees: None }, DockTabSkeleton { id: "framework.marketplace", children: None, trees: Some(["framework.marketplace.tree"]) }, DockTabSkeleton { id: "os.task-manager", children: None, trees: Some(["os.task-manager.tree"]) }, DockTabSkeleton { id: "framework.panel.history", children: None, trees: Some(["framework.panel.history.tree"]) }], "left-middle": [], "right-middle": [], "top-left": [DockTabSkeleton { id: "fixture.workbench", children: None, trees: Some(["fixture.workbench.tree"]) }], "top-middle": [], "top-right": [DockTabSkeleton { id: "fixture.details", children: None, trees: Some(["fixture.details.tree"]) }, DockTabSkeleton { id: "framework.chat", children: None, trees: Some(["framework.chat.tree"]) }]} }
```

```text
        FAIL [   0.079s] ( 854/1160) semio-framework-os-renderer-wgpu shell::panel_anchor_model_tests::every_shell_owned_leaf_projects_into_retained_records
    thread 'shell::panel_anchor_model_tests::every_shell_owned_leaf_projects_into_retained_records' (18075762) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:660:9:
    🧾️ framework.hub publishes a body but no anchor declares it
```

```text
        FAIL [   0.095s] ( 855/1160) semio-framework-os-renderer-wgpu shell::panel_anchor_model_tests::footer_bands_match_the_react_chrome_band_fixture
    thread 'shell::panel_anchor_model_tests::footer_bands_match_the_react_chrome_band_fixture' (18075786) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:1160:9:
    📑️ bottom-middle is centred on the footer: first=(BottomMiddle, "framework.category.command", Rect { x: 542.0, y: 0.0, w: 93.599976, h: 22.4 }), last=(BottomMiddle, "framework.category.command", Rect { x: 542.0, y: 0.0, w: 93.599976, h: 22.4 }), span=93.599976, width=1440
```

```text
        FAIL [   0.030s] ( 920/1160) semio-framework-os-renderer-wgpu shell::shell_chrome_parity_tests::every_shared_keybinding_row_routes_to_its_shell_verb_and_outranks_app_keybindings
    thread 'shell::shell_chrome_parity_tests::every_shared_keybinding_row_routes_to_its_shell_verb_and_outranks_app_keybindings' (18076255) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs:333:9:
    "playground.navbar.roles.editor": a shell chord must outrank every app-declared keybinding
```

```text
        FAIL [   0.056s] ( 947/1160) semio-framework-os-renderer-wgpu shell::shell_input_tests::display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag
    thread 'shell::shell_input_tests::display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag' (18076349) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:398:10:
    the reconciled and painted window kind publishes its transfer handle
```

```text
        FAIL [   0.052s] ( 949/1160) semio-framework-os-renderer-wgpu shell::shell_input_tests::escape_closes_the_palette_rather_than_committing_its_query_field
    thread 'shell::shell_input_tests::escape_closes_the_palette_rather_than_committing_its_query_field' (18076372) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:568:5:
    assertion `left == right` failed
      left: None
     right: Search
```

```text
        FAIL [   0.044s] ( 955/1160) semio-framework-os-renderer-wgpu shell::shell_input_tests::the_palette_chord_toggles_and_never_types_itself_into_the_query
    thread 'shell::shell_input_tests::the_palette_chord_toggles_and_never_types_itself_into_the_query' (18076426) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:544:5:
    assertion `left == right` failed: the chord opens the palette
      left: None
     right: Search
```

```text
        FAIL [   0.058s] ( 960/1160) semio-framework-os-renderer-wgpu shell::shell_input_tests::two_published_table_surfaces_transfer_through_the_shell_host
    thread 'shell::shell_input_tests::two_published_table_surfaces_transfer_through_the_shell_host' (18076440) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:506:5:
    release retires retained capture
```

```text
        FAIL [   0.076s] ( 983/1160) semio-framework-os-renderer-wgpu shell::theme_editor_and_accessibility_tests::accessibility_activation_updates_settings_switch_and_active_tab_projection
    thread 'shell::theme_editor_and_accessibility_tests::accessibility_activation_updates_settings_switch_and_active_tab_projection' (18076613) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:21:149:
    settings branch
```

```text
        FAIL [   0.061s] ( 991/1160) semio-framework-os-renderer-wgpu shell::theme_editor_and_accessibility_tests::pointer_activation_resolves_every_nested_leaf_from_root_to_target_independent_of_panel_rect
    thread 'shell::theme_editor_and_accessibility_tests::pointer_activation_resolves_every_nested_leaf_from_root_to_target_independent_of_panel_rect' (18076692) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:21:149:
    settings branch
```
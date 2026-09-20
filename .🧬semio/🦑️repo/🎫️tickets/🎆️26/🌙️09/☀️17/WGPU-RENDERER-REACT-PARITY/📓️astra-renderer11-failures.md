# Renderer11 Native Failure Census

Executed1151tests:1119passed,32failed,0skipped. Compilation succeeds. These are actual assertion/runtime failures; none has been accepted as an intentional skip. Source inclusion is governed by the compiled binary; new Display gesture closure may postdate sampling.

## 1. agent_bridge::tests::every_modelled_shell_to_gateway_fixture_round_trips_through_this_codec

```text
AgentCancel did not decode: UnknownTag(10)
```

## 2. async_boundary_tests::presenter_ack_retirement_source_mutations_are_denied

```text
assertion failed: presenter_retirement_contract(LIBRARY_SOURCE, PREPARED_SOURCE, GPU_SOURCE,
        DRAW_SOURCE, OS_HOST_SOURCE, WINT_APP_SOURCE, DRAW_LAWS_SOURCE)
```

## 3. async_boundary_tests::the_present_watchdog_signature_carries_within_item_upload_progress

```text
as the fifth term of the progress signature
```

## 4. engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack

```text
assertion `left == right` failed: 🧱️ guard 'renderer::engine_canvas': measured slot tables differ from the committed budget
      left: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 75104, owner_bytes: 16 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
     right: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 75032, owner_bytes: 16 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
```

## 5. engine_canvas::node_graph_attach_tests::node_graph_paint_publishes_its_captions_over_the_engine_raster

```text
the seven-node fixture publishes a caption per node and per port, got 16 at lod Some(String("detail"))
```

## 6. engine_canvas::node_graph_attach_tests::tutorial_semantic_points_resolve_through_live_graph_geometry

```text
known graph semantic point resolves
```

## 7. hub_connection::tests::every_verb_the_surface_dispatches_is_one_of_the_declared_hub_actions

```text
the surface dispatches an undeclared verb hubRefreshSpaces
```

## 8. scenes::admitted_surface_map_tests::admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack

```text
assertion `left == right` failed: 🧱️ guard 'renderer::scenes': measured slot tables differ from the committed budget
      left: [FixedSlotTableBudget { owner: "scenes::AdmittedSurfaceMap<World3dState>", capacity: 256, element_bytes: 23208, owner_bytes: 49528 }]
     right: [FixedSlotTableBudget { owner: "scenes::AdmittedSurfaceMap<World3dState>", capacity: 256, element_bytes: 22920, owner_bytes: 48944 }]
```

## 9. scenes::icon_render_tests::icon_render_maps_the_shared_lighting_fixture_to_the_world_environment

```text
assertion `left == right` failed
      left: Object {"azimuth": Number(120.0), "color": String("#ffd080"), "elevation": Number(25.0), "enabled": Bool(true), "intensity": Number(1.7)}
     right: Object {"azimuth": Number(120), "color": String("#ffd080"), "elevation": Number(25), "enabled": Bool(true), "intensity": Number(1.7)}
```

## 10. scenes::table_tests::shared_fixture_table_transfer_crosses_windows_and_revalidates_mime_payload_and_generation

```text
a changed source document generation retires the transfer before release
```

## 11. scenes::virtual_file_system_tests::ctrl_click_extends_the_selection_where_a_plain_click_replaces_it

```text
assertion `left == right` failed: ctrl adds to the live selection instead of replacing it
      left: ["n2"]
     right: ["n1", "n2"]
```

## 12. shell::appearance_tour_and_footer_pill_tests::the_footer_pills_are_not_gated_off_the_browser_build

```text
the footer's sync phase is no longer native-only
```

## 13. shell::appearance_tour_and_footer_pill_tests::the_introduction_read_arms_the_tour

```text
the landing read arms the tour
```

## 14. shell::command_registry_tests::build_os_commands_covers_every_wired_setting

```text
assertion `left == right` failed
      left: ["os.toggleFullscreen", "os.setAppearance", "os.setDriver", "os.setLocale", "os.setTerminology", "os.setThemeId", "os.openTaskManager", "os.resetDock"]
     right: ["os.toggleFullscreen", "os.setAppearance", "os.setDriver", "os.setLocale", "os.setTerminology", "os.setThemeId", "os.resetDock"]
```

## 15. shell::display_conflicts_marketplace_tests::a_tool_leaf_publishes_its_own_measures_under_their_own_ids

```text
assertion `left == right` failed: 🛠️ with no measures in hand React renders the section and nothing else
      left: 2
     right: 1
```

## 16. shell::hub_projection_workspace_tests::neutral_authority_and_multi_document_vectors_fold_to_the_shared_summary

```text
hub projection fixture: Error("missing field `authorization_generation`", line: 7, column: 80)
```

## 17. shell::navbar_footer_parity_tests::the_navbar_cluster_walks_to_completion_and_keeps_its_bands

```text
🧭️ TopMiddle belongs to the centered cluster
```

## 18. shell::panel_anchor_model_tests::build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme

```text
expected a stack root
```

## 19. shell::panel_anchor_model_tests::default_dock_matches_the_shared_react_fixture

```text
assertion `left == right` failed: 🧭️ anchor top-right must carry the fixture's tabs in the fixture's order
      left: ["fixture.details", "framework.chat", "framework.hub"]
     right: ["fixture.details", "framework.chat"]
```

## 20. shell::panel_anchor_model_tests::footer_bands_match_the_react_chrome_band_fixture

```text
📑️ bottom-middle is centred on the footer
```

## 21. shell::panel_anchor_model_tests::the_mobile_panel_flattens_every_anchor_in_anchor_order

```text
assertion `left == right` failed: 📱️ the flattened order is the fixture's
      left: ["fixture.workbench", "fixture.details", "framework.chat", "framework.hub", "framework.settings", "framework.marketplace", "os.task-manager", "framework.panel.history", "framework.category.command", "framework.category.display", "fixture.display", "s-sync-status"]
     right: ["fixture.workbench", "fixture.details", "framework.chat", "framework.settings", "framework.marketplace", "framework.panel.history", "framework.category.command", "framework.category.display", "fixture.display", "s-sync-status"]
```

## 22. shell::shell_input_tests::display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag

```text
the reconciled and painted window kind publishes its transfer handle
```

## 23. shell::shell_shortcuts_palette_tests::every_row_dispatches_its_verb_and_outranks_app_keybindings

```text
assertion `left == right` failed: 21 rows, eight of which spell both the ctrl and the meta accelerator
      left: 28
     right: 29
```

## 24. shell::theme_editor_and_accessibility_tests::accessibility_activation_updates_settings_switch_and_active_tab_projection

```text
assertion `left == right` failed
      left: Some(false)
     right: Some(true)
```

## 25. shell::theme_editor_and_accessibility_tests::editing_the_document_re_tokenises_the_painted_theme

```text
assertion `left == right` failed: 🎨️ an unedited document paints the built-in theme
      left: Rgba { r: 1.0, g: 0.034339808, b: 0.07818742, a: 1.0 }
     right: Rgba { r: 1.0, g: 0.03433981, b: 0.07818742, a: 1.0 }
```

## 26. shell::theme_editor_and_accessibility_tests::the_appearance_section_offers_reacts_paint_and_alpha_rows

```text
🖌️ the paint row
```

## 27. shell::theme_editor_and_accessibility_tests::every_open_theme_section_publishes_within_the_retained_document_ceiling

```text
🎨️ the all-open theme leaf publishes at row 528: measure action 'setThemeAppearancePaint' args exceed the retained contract: data did not match any variant of untagged enum UiValue
```

## 28. shell::window_actions_search_pane_tests::the_engagement_body_paints_reacts_control_row_with_reacts_ids_and_intents

```text
🎛️ the engagement body is missing React's 'ui.engagement.actions' — got ["pane-top/framework.section.engagements/window-action-pane", "pane-top/framework.section.engagements/n2", "pane-top/framework.section.engagements/engagement-control.slider.field", "pane-top/framework.section.engagements/engagement-control.slider", "pane-top/framework.section.engagements/engagement-control.stepper", "pane-top/framework.section.engagements/engagement-control.ring", "pane-top/framework.section.engagements/ring.a", "pane-top/framework.section.engagements/ring.b", "pane-top/framework.section.engagements/granularity", "pane-top/framework.section.engagements/granularity.face", "pane-top/framework.section.engagements/granularity.edge", "pane-top/framework.section.engagements/engagement-control.select", "pane-top/framework.section.engagements/n13", "pane-top/framework.section.engagements/n14", "pane-top/framework.section.engagements/action.category.actions", "pane-top/framework.section.engagements/action.addObjectKind", "pane-top/framework.section.engagements/action.category.create", "pane-top/framework.section.engagements/action.openAddObjectDialog", "pane-top/framework.section.engagements/action.category.history", "pane-top/framework.section.engagements/action.undo"]
```

## 29. shell::window_lifecycle_template_drag_tests::display_window_and_projection_rows_publish_reacts_drag_payload_without_click_actions

```text
WorkerPool: mandatory submission failed closed: Contended
```

## 30. shell::window_lifecycle_template_drag_tests::closed_world3d_retires_every_input_and_scene_owner_before_id_reuse

```text
World3dState reached Drop before retained dynamic owners reached terminal empty
```

## 31. shell::window_lifecycle_template_drag_tests::template_pointer_drag_promotes_and_escape_cancels_without_mutating_the_dock

```text
WorkerPool: mandatory submission failed closed: Contended
```

## 32. winit_app::p3c_tests::normalized_host_key_routes_the_platform_os_command_to_the_shell

```text
the real host ingress reaches apply_os_command
```

## Root Integration Repairs

The measured fixed-slot fixture is updated without changing capacities or heap/stack limits: EngineSurfaceRegistry element75032→75104 bytes (+72×256=18432), AdmittedSurfaceMap<World3dState> element22920→23208 (+288×256=73728) and owner48944→49528 (+584). The current source contains the added scene shadow/raster ownership records; per-field attribution is not asserted without a separate layout measurement.

Async source guard #2 counted exactly two scene invalidations before Hub status publish/retire added two more actual invalidations. It now requires four; its removal mutation remains required to fail. Watchdog source guard #3 now matches the six-term production signature, retaining the original fifth upload-progress term and the new sixth raster-retirement cursor. No runtime scheduling or assertion semantics were weakened. Native rerun remains pending.

AgentBridge wire drift #1: shared neutral corpus gained AgentCancel tag10; WGPU decode/encode lacked it. Added the exact SSOT string payload and raised modeled coverage6→7 while retaining four explicitly unmodeled snapshot frames. Native replay is pending. Sender state and user cancellation UI parity remain open; codec acceptance alone will not close them.

Root #17 repair: navbar law required a TopMiddle chip but constructed a world-pane fixture with no TopMiddle tabs. Added an explicit test-only middle leaf; all occupied-band and physical nonoverlap assertions remain. Root #20 footer law now prints the measured first/last rects to distinguish pixel rounding from clipping or placement defects before altering its assertion. Graph #5/#6 now name actual overlay rows and the unresolved semantic point for the next native run; no success condition changed.

Canonical React AgentBridge target passes43/43, including the actual mounted cancellation frame test. Earlier generic React target selected no tests because AgentBridge uses its dedicated target; no false pass is recorded. WGPU cancellation codec replay and UI/state producer remain pending.

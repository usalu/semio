# Current Product Whole13 Failures

Actual long owning1676 roster completed with no-fail-fast: Nx1/native100,1657passed19failed11policy-skipped50.447s. All selected tests ran. This shared current result is not a source-held or historical acceptance. Each original failure follows. Flow teardown diagnostics identify HistoryStore for allfivegesture/caret guards.

```text
    thread 'engine_canvas::node_graph_gesture_tests::a_press_on_a_node_body_selects_that_node_and_a_released_drag_publishes_its_move' (5650559) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs:79:9:
    the attached surface reaches a fixed terminal witness

    thread 'engine_canvas::node_graph_gesture_tests::a_drag_that_crosses_a_port_keeps_the_bounded_path_and_still_publishes_its_move' (5650545) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs:79:9:
    the attached surface reaches a fixed terminal witness

    thread 'engine_canvas::node_graph_gesture_tests::detaching_a_wired_input_from_its_port_publishes_disconnect' (5650557) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs:79:9:
    the attached surface reaches a fixed terminal witness

    thread 'engine_canvas::node_graph_gesture_tests::an_output_to_input_drag_publishes_connect' (5650558) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs:79:9:
    the attached surface reaches a fixed terminal witness

    thread 'kernel_runtime::kernel_runtime_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack' (5651407) panicked at 🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/../../🦀️.rs:1601:5:
    assertion `left == right` failed: 🧱️ guard 'renderer::kernel_runtime': measured slot tables differ from the committed budget
      left: [FixedSlotTableBudget { owner: "kernel_runtime::RetainedSurfaceRegistry", capacity: 64, element_bytes: 25672, owner_bytes: 32 }, FixedSlotTableBudget { owner: "kernel_runtime::CommandDocumentRetirementRegistry", capacity: 512, element_bytes: 584, owner_bytes: 4112 }]
     right: [FixedSlotTableBudget { owner: "kernel_runtime::RetainedSurfaceRegistry", capacity: 64, element_bytes: 24488, owner_bytes: 32 }, FixedSlotTableBudget { owner: "kernel_runtime::CommandDocumentRetirementRegistry", capacity: 512, element_bytes: 584, owner_bytes: 4112 }]

    thread 'renderer_io_retained_tests::native_smoke_without_a_selected_application_returns_failure' (5651901) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs:209:5:
    assertion `left == right` failed: the smoke input uses the current host channel
      left: Number(21)
     right: Number(23)

    thread 'scenes::admitted_surface_map_tests::admitted_surface_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack' (5651957) panicked at 🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/../../🦀️.rs:1601:5:
    assertion `left == right` failed: 🧱️ guard 'renderer::scenes': measured slot tables differ from the committed budget
      left: [FixedSlotTableBudget { owner: "scenes::AdmittedSurfaceMap<World3dState>", capacity: 256, element_bytes: 28728, owner_bytes: 3136 }]
     right: [FixedSlotTableBudget { owner: "scenes::AdmittedSurfaceMap<World3dState>", capacity: 256, element_bytes: 28400, owner_bytes: 3136 }]

    thread 'interpreter::ui_command_wiring_tests::accepted_node_graph_note_caret_arms_resets_and_retires_with_its_exact_scene' (5651208) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs:79:9:
    the attached surface reaches a fixed terminal witness

    thread 'shell::fault_notices_tests::a_refused_guest_channel_is_told_as_its_localized_notice' (5654394) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🧪️wgpu-fault-notices/🦀️.rs:98:13:
    older-guest en: never the raw code or the English message

    thread 'shell::panel_anchor_model_tests::host_panel_action_is_claimed_before_guest_and_preserves_the_session_roster_and_home_projection' (5655249) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:166:10:
    configured leaf is host-owned: "kernel: instance 77 is not registered; exact command owner entered bounded close"

    thread 'shell::plugin_install_tests::a_parsed_relay_kind_resolves_to_its_declared_owner' (5655492) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🎬️wgpu-plugin-install/🦀️.rs:98:5:
    assertion `left == right` failed
      left: None
     right: Some("cad")

    thread 'shell::plugin_install_tests::the_host_plugin_is_not_the_owner_of_every_kind' (5655544) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🎬️wgpu-plugin-install/🦀️.rs:30:5:
    assertion `left == right` failed
      left: None
     right: Some("cad")

    thread 'shell::settings_general_layout_tests::locale_and_terminology_changes_require_one_full_guest_refresh_and_settle' (5655878) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7025:50:
    shell terminology has explicit admitted authority

    thread 'shell::shell_boot_isolation_tests::boot_selection_opens_the_requested_variant_across_the_fixture_table' (5655929) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs:103:63:
    case "dependency-sorts-first" selects a program

    thread 'shell::shell_boot_isolation_tests::boot_without_the_requested_plugin_settles_with_a_per_plugin_status' (5655933) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs:119:5:
    assertion `left == right` failed
      left: "generation3d"
     right: "procedural"

    thread 'shell::shell_input_tests::standalone_multi_app_variants_resolve_their_declared_app' (5656833) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1138:5:
    assertion `left == right` failed
      left: "generation3d"
     right: "procedural"

    thread 'shell::shell_shortcuts_palette_tests::the_palette_rows_follow_the_react_declaration_order' (5657096) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs:636:5:
    assertion failed: shell.space_mode

    thread 'shell::tour_overlay_and_chord_glyph_tests::the_platform_door_is_wired_end_to_end' (5658112) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs:323:5:
    ⌨️ the page makes the read

    thread 'shell::ui_prefs_themes_i18n_tests::the_host_language_is_the_fallback_after_a_lock_and_the_stored_preference' (5658386) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs:256:9:
    assertion `left == right` failed: 🗣️ `normalizeUiLocale` folds "fr-CH"
      left: None
     right: Some("en")
```

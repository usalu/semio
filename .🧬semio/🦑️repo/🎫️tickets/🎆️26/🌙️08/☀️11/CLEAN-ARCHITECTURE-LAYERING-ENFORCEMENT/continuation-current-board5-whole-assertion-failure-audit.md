# Current Board 5 Whole Assertion Failures

Actual exact registered canonical Infinite whole/default-feature invocation compiled and ran all543 tests. Actual summary:537passed/6failed/0ignored/0measured/0filteredout,4.95s. This is the current owning suite count, not retired oldUI1060. The World cursor fixture compilation corrections are exercised: both original cursor laws pass; the flat-rectangle minimap test instead fails before rectangle decoding at current minimap layout availability. Owning command failure/postchecks are still completing at this observation; no final source exactness claim yet.

Exact six failures and original assertion diagnostics:

```text
failures:

---- board::ports::directed_dag::tests::dag_host_loads_demo_fixture stdout ----

thread 'board::ports::directed_dag::tests::dag_host_loads_demo_fixture' (512027) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:426:5:
assertion `left == right` failed
  left: "dag.hostDocument"
 right: "dag.host_snapshot"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- board::ports::directed_dag::tests::dag_host_slider_overlay_preserves_language_neutral_field_labels stdout ----

thread 'board::ports::directed_dag::tests::dag_host_slider_overlay_preserves_language_neutral_field_labels' (512039) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:619:13:
assertion `left == right` failed: min
  left: Number(Float(0.0))
 right: Number(UInt(0))

---- board::ports::directed_dag::tests::minimap_widget_panel_uses_square_corners stdout ----

thread 'board::ports::directed_dag::tests::minimap_widget_panel_uses_square_corners' (512072) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:1748:56:
minimap layout

---- board::ports::directed_dag::tests::selected_nodes_cursor_censuses_and_emits_one_byte_per_grant stdout ----

thread 'board::ports::directed_dag::tests::selected_nodes_cursor_censuses_and_emits_one_byte_per_grant' (512083) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/././../../🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs:29:5:
assertion `left == right` failed
  left: Some(20)
 right: Some(16)

---- world::tests::world_native_component_gumball_survives_live_guest_source_refresh stdout ----

thread 'world::tests::world_native_component_gumball_survives_live_guest_source_refresh' (512506) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:6573:13:
the actual guest refresh advanced the original view revision

---- world::tests::world_native_primary_geometry_gpu_upload_preserves_original_zero_indices stdout ----

thread 'world::tests::world_native_primary_geometry_gpu_upload_preserves_original_zero_indices' (512508) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/../../🌍️world/🧪️tests/🔬️unit/🦀️.rs:6735:37:
wire: actual original GPU upload Err("mesh upload schema was empty")


failures:
    board::ports::directed_dag::tests::dag_host_loads_demo_fixture
    board::ports::directed_dag::tests::dag_host_slider_overlay_preserves_language_neutral_field_labels
    board::ports::directed_dag::tests::minimap_widget_panel_uses_square_corners
    board::ports::directed_dag::tests::selected_nodes_cursor_censuses_and_emits_one_byte_per_grant
    world::tests::world_native_component_gumball_survives_live_guest_source_refresh
    world::tests::world_native_primary_geometry_gpu_upload_preserves_original_zero_indices

test result: FAILED. 537 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.95s

```

Full raw log remains under `🗑️generated/current-native-origin/epoch-5/board-whole5.log`. Product owning command still active. These original failures are retained without narrowing/filtering or changing assertion expectations. Fresh current code investigation is required before further corrections.

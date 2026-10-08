# GIS Native Runtime Failures

Actual uncached app-assembly suite: 315 tests executed; 294 passed, 21 failed, 0 skipped; exit 1.

## 'editor::gis2d::commands::example::tests::set_active_example_empty_then_reuse_round_trips_document' (1341296)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🧪️tests/🔬️unit/🦀️.rs:9:5:
assertion failed: !app.snapshot().expect("projection").positions.is_empty()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::example::tests::set_active_example_empty_then_reuse_round_trips_document ... FAILED
failures:
failures:
    editor::gis2d::commands::example::tests::set_active_example_empty_then_reuse_round_trips_document
```

## 'editor::gis2d::commands::example::tests::set_active_example_is_operation_under_registry_kind_discipline' (1341300)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🧪️tests/🔬️unit/🦀️.rs:40:5:
clearing a non-empty example publishes at least one delete operation per removed feature
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::example::tests::set_active_example_is_operation_under_registry_kind_discipline ... FAILED
failures:
failures:
    editor::gis2d::commands::example::tests::set_active_example_is_operation_under_registry_kind_discipline
```

## 'editor::gis2d::commands::example::tests::the_example_catalogue_resolves_declared_ids_and_faults_on_the_rest' (1341312)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🧪️tests/🔬️unit/🦀️.rs:55:60:
the declared example resolves: Fault { origin: App, code: FaultCode("app.message"), severity: Error, message: "gis map example 'demo' does not parse: TextError { kind: InvalidValue, message: \"expected LBracket, found Int '5'\", span: TextSpan { line: 1, column: 11, length: 1 }, expected: None }", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::example::tests::the_example_catalogue_resolves_declared_ids_and_faults_on_the_rest ... FAILED
failures:
failures:
    editor::gis2d::commands::example::tests::the_example_catalogue_resolves_declared_ids_and_faults_on_the_rest
```

## 'editor::gis2d::commands::example::tests::the_manifest_stages_exactly_the_catalogue_ids' (1341322)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🧪️tests/🔬️unit/🦀️.rs:74:9:
the palette must never stage an id `set_active_example` rejects: demo
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::example::tests::the_manifest_stages_exactly_the_catalogue_ids ... FAILED
failures:
failures:
    editor::gis2d::commands::example::tests::the_manifest_stages_exactly_the_catalogue_ids
```

## 'editor::gis2d::commands::features::tests::delete_feature_undo_redo_round_trips' (1341462)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🧪️tests/🔬️unit/🦀️.rs:181:45:
position
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::features::tests::delete_feature_undo_redo_round_trips ... FAILED
failures:
failures:
    editor::gis2d::commands::features::tests::delete_feature_undo_redo_round_trips
```

## 'editor::gis2d::commands::features::tests::move_feature_translates_a_route_polyline_whole' (1341466)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🧪️tests/🔬️unit/🦀️.rs:121:77:
route
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::features::tests::move_feature_translates_a_route_polyline_whole ... FAILED
failures:
failures:
    editor::gis2d::commands::features::tests::move_feature_translates_a_route_polyline_whole
```

## 'editor::gis2d::commands::features::tests::patch_routes_emits_route_patch_ops_and_updates_document' (1341498)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🧪️tests/🔬️unit/🦀️.rs:12:5:
assertion `left == right` failed: one matching route publishes one patch operation
  left: 0
 right: 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::features::tests::patch_routes_emits_route_patch_ops_and_updates_document ... FAILED
failures:
failures:
    editor::gis2d::commands::features::tests::patch_routes_emits_route_patch_ops_and_updates_document
```

## 'editor::gis2d::commands::features::tests::rename_feature_retitles_the_addressed_position_and_leaves_geometry_alone' (1341524)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️features/🧪️tests/🔬️unit/🦀️.rs:139:45:
position
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::commands::features::tests::rename_feature_retitles_the_addressed_position_and_leaves_geometry_alone ... FAILED
failures:
failures:
    editor::gis2d::commands::features::tests::rename_feature_retitles_the_addressed_position_and_leaves_geometry_alone
```

## 'editor::gis2d::maphost::tests::the_host_mirrors_the_document_features_and_the_config_camera' (1342055)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗺️maphost/🧪️tests/🔬️unit/🦀️.rs:9:5:
the reuse-map fixture seeds position features
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::maphost::tests::the_host_mirrors_the_document_features_and_the_config_camera ... FAILED
failures:
failures:
    editor::gis2d::maphost::tests::the_host_mirrors_the_document_features_and_the_config_camera
```

## 'editor::gis2d::modes::edit::windows::map::component::tests::an_interaction_hover_reaches_the_scene_as_the_popup_record' (1342065)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🧪️tests/🔬️unit/🦀️.rs:44:71:
fixture route
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::modes::edit::windows::map::component::tests::an_interaction_hover_reaches_the_scene_as_the_popup_record ... FAILED
failures:
failures:
    editor::gis2d::modes::edit::windows::map::component::tests::an_interaction_hover_reaches_the_scene_as_the_popup_record
```

## 'editor::gis2d::modes::edit::windows::map::component::tests::an_interaction_select_marks_the_feature_selected_in_the_rendered_scene' (1342069)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🧪️tests/🔬️unit/🦀️.rs:28:77:
fixture position
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::modes::edit::windows::map::component::tests::an_interaction_select_marks_the_feature_selected_in_the_rendered_scene ... FAILED
failures:
failures:
    editor::gis2d::modes::edit::windows::map::component::tests::an_interaction_select_marks_the_feature_selected_in_the_rendered_scene
```

## 'gis-map-window-ownership-law' (1342172)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️window/🦀️.rs:238:21:
GIS Map exact-window runtime law: "fitWorld: Fault { origin: Framework, code: FaultCode(\"window-config.actor\"), severity: Error, message: \"window config edit actor differs from its opened session\", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'editor::gis2d::modes::edit::windows::map::config::component::window_ownership_tests::gis_map_window_ownership_runtime_isolates_renders_and_reopens_two_map_windows' (1342171) panicked at 🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️window/🦀️.rs:242:10:
GIS Map window ownership law thread: Any { .. }
test editor::gis2d::modes::edit::windows::map::config::component::window_ownership_tests::gis_map_window_ownership_runtime_isolates_renders_and_reopens_two_map_windows ... FAILED
failures:
failures:
    editor::gis2d::modes::edit::windows::map::config::component::window_ownership_tests::gis_map_window_ownership_runtime_isolates_renders_and_reopens_two_map_windows
```

## 'editor::gis2d::panels::inspection::tests::the_inspector_detail_section_follows_the_features_selection' (1342301)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:37:49:
the default fixture carries a position
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::gis2d::panels::inspection::tests::the_inspector_detail_section_follows_the_features_selection ... FAILED
failures:
failures:
    editor::gis2d::panels::inspection::tests::the_inspector_detail_section_follows_the_features_selection
```

## 'examples::demo::tests::default_example_document_carries_addressable_features' (1342314)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs:16:57:
default example resolves: Fault { origin: App, code: FaultCode("app.message"), severity: Error, message: "gis map example 'demo' does not parse: TextError { kind: InvalidValue, message: \"expected LBracket, found Int '5'\", span: TextSpan { line: 1, column: 11, length: 1 }, expected: None }", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], params: None, retryable: false }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test examples::demo::tests::default_example_document_carries_addressable_features ... FAILED
failures:
failures:
    examples::demo::tests::default_example_document_carries_addressable_features
```

## 'examples::demo::tests::inference_determinism_law' (1342320)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs:30:83:
demo fixture parses: TextError { kind: InvalidValue, message: "expected LBracket, found Int '5'", span: TextSpan { line: 1, column: 11, length: 1 }, expected: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test examples::demo::tests::inference_determinism_law ... FAILED
failures:
failures:
    examples::demo::tests::inference_determinism_law
```

## 'standards::v1::subsets::any::io::component::binary::mutations::tests::gis_map_default_document_is_non_empty' (1342403)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:46:5:
assertion failed: !default_document().positions.is_empty()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::binary::mutations::tests::gis_map_default_document_is_non_empty ... FAILED
failures:
failures:
    standards::v1::subsets::any::io::component::binary::mutations::tests::gis_map_default_document_is_non_empty
```

## 'standards::v1::subsets::any::io::component::binary::snapshot::tests::gis_map_document_pack_agrees_with_dsl_for_bundled_reuse_example' (1342419)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:8:82:
parse reuse-map example: TextError { kind: InvalidValue, message: "expected LBracket, found Int '5'", span: TextSpan { line: 1, column: 11, length: 1 }, expected: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::binary::snapshot::tests::gis_map_document_pack_agrees_with_dsl_for_bundled_reuse_example ... FAILED
failures:
failures:
    standards::v1::subsets::any::io::component::binary::snapshot::tests::gis_map_document_pack_agrees_with_dsl_for_bundled_reuse_example
```

## 'standards::v1::subsets::any::io::component::tests::geojson_io::geojson_crate_reads_the_bundled_example_export' (1342848)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs:139:9:
assertion `left == right` failed
  left: (0, 0)
 right: (152, 149)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::tests::geojson_io::geojson_crate_reads_the_bundled_example_export ... FAILED
failures:
failures:
    standards::v1::subsets::any::io::component::tests::geojson_io::geojson_crate_reads_the_bundled_example_export
```

## 'standards::v1::subsets::any::io::component::text::snapshot::tests::gis_map_document_dsl_round_trips_bundled_reuse_example' (1342906)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:7:54:
parse reuse-map example: TextError { kind: InvalidValue, message: "expected LBracket, found Int '5'", span: TextSpan { line: 1, column: 11, length: 1 }, expected: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::text::snapshot::tests::gis_map_document_dsl_round_trips_bundled_reuse_example ... FAILED
failures:
failures:
    standards::v1::subsets::any::io::component::text::snapshot::tests::gis_map_document_dsl_round_trips_bundled_reuse_example
```

## 'standards::v1::subsets::any::io::component::text::snapshot::tests::print_dsl_reproduces_the_bundled_example_text_verbatim' (1342915)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:18:54:
parse reuse-map example: TextError { kind: InvalidValue, message: "expected LBracket, found Int '5'", span: TextSpan { line: 1, column: 11, length: 1 }, expected: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::text::snapshot::tests::print_dsl_reproduces_the_bundled_example_text_verbatim ... FAILED
failures:
failures:
    standards::v1::subsets::any::io::component::text::snapshot::tests::print_dsl_reproduces_the_bundled_example_text_verbatim
```

## 'standards::v1::subsets::any::schema::component::relocated_engine_tests::svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge' (1342933)

```text
🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️relocated-engine/🦀️.rs:49:5:
at least one path node rendered: <svg viewBox="0 0 256 256" width="256" height="256" xmlns="http://www.w3.org/2000/svg"><g id="layer-gis-features"/></svg>
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::schema::component::relocated_engine_tests::svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge ... FAILED
failures:
failures:
    standards::v1::subsets::any::schema::component::relocated_engine_tests::svg_export_renders_real_svg_text_through_the_stdio_drawing_bridge
```

# ISO 16757 Part Checklist

| Part | Module | evaluate() | Positive test | Negative test |
|------|--------|------------|---------------|---------------|
| 1 §3.1 | `part_1::validate_catalogue_structure` | yes | `reference_fixture_selects_one_product` | `composition_cycle_detected` |
| 1 §4.2 | `part_1::select_products` | yes | `reference_fixture_selects_one_product` | selection ambiguity in `evaluate` |
| 1 §10 | `part_1::resolve_bim_embedding` | yes | `evaluate_exercises_all_parts_with_numeric_checks` | — |
| 2 §5.3.5 | `part_2::BoundingBox::overlaps` | yes | installation clearance in `evaluate` | — |
| 2 §6.1 | `part_2::validate_geometry_graph` | yes | `geometry_bbox_volume_for_box_primitive` | — |
| 2 §7.1 | `part_2::evaluate_bounding_box` | yes | `geometry_bbox_volume_for_box_primitive` | — |
| 2 §7.4 | `part_2::project_step_entity` | yes | `evaluate_exercises_all_parts_with_numeric_checks` | — |
| 4 §4.3 | `part_4::validate_dictionary` | yes | `dictionary_controlled_values_filter_by_subject` | — |
| 4 §5.1 | `part_4::to_iso12006_mappings` | yes | `evaluate_exercises_all_parts_with_numeric_checks` | — |
| 4 §6.3.2 | `part_4::filter_controlled_values` | yes | `dictionary_controlled_values_filter_by_subject` | — |
| 5 §6.1 | `part_5::build_ifc_catalogue` / `export_ifc_step` | yes | `ifc_step_export_contains_data_section` | — |
| 5 §6.10 | `part_5::calculate_part_number` | yes | `part_number_script_is_deterministic` | — |
| 5 §8 | `part_5::DefaultScriptRuntime` | yes | `script_rejects_forbidden_import` | division-by-zero in `evaluate` |
| IO | `io::catalogue_to_json` / `catalogue_from_json` | — | `catalogue_json_round_trip` | — |
| Session | `evaluate` / `Iso16757Family` | yes | `evaluate_exercises_all_parts_with_numeric_checks` | `norm_family_evaluate_matches_host` |

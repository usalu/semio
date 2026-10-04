# Registered Current Native Route Audit

Read-only source audit. No Cargo, Nx, or test execution occurred; counts are authored test declarations matching the fixed native selector, not runtime discovery or passing receipts. Shared-file changes can change counts after this observation.

Every listed native route uses the shared `runArtifactRustPackageMain` guard: one optional mode only (`source` or `native`). The registered native target already supplies `native`, so it accepts **no additional positional or forwarded arguments**. Use `SEMIO_TEST_LEVEL=quick` for budget selection, never `--args=quick`. The native selector is fixed, not caller-forwarded Cargo arguments.

| Exact Nx target | Added args | Native selector | Authored law count | Native hook presence | Snapshot features | Nx cache |
|---|---|---|---:|---|---|---|
| `@semio-tech/trinity-rewriting-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 9 | 0 codec mounts | ["component-app-assembly"] | False |
| `@semio-tech/raster-raster-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 8 | 1 codec mounts | inherited testFeatures | True |
| `@semio-tech/flow-flow-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 8 | 1 codec mounts | inherited testFeatures | True |
| `@semio-tech/process-process3d-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 8 | 0 codec mounts | [] | False |
| `@semio-tech/dag-dag-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 12 | 1 codec mounts | inherited testFeatures | False |
| `@semio-tech/stdio-bmp-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 9 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-svg-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 10 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-deflate-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 8 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-txt-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 9 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-binary-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 8 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-json-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 17 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-gltf-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 23 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-csv-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 9 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/stdio-tsv-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 9 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/space-space-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 4 | 1 codec mounts | inherited testFeatures | unspecified |
| `@semio-tech/procedural-generation3d-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 12 | 1 codec mounts | [] | True |
| `@semio-tech/procedural-generation2d-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 12 | 1 codec mounts | [] | unspecified |
| `@semio-tech/framework-flow-flow-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 7 | 0 codec mounts | inherited testFeatures | False |
| `@semio-tech/framework-space-space-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 11 | 1 codec mounts | inherited testFeatures | False |
| `@semio-tech/framework-space-collection-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 11 | 1 codec mounts | inherited testFeatures | False |
| `@semio-tech/framework-infinite-dag-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_snapshot_ --no-fail-fast` | 7 | 0 codec mounts | inherited testFeatures | False |
| `@semio-tech/framework-workflow-workflow-rs:test-snapshot-sqlite-native` | None | `--lib sqlite_ --no-fail-fast` | 16 | 1 codec mounts | inherited testFeatures | unspecified |

Counts exclude non-test `sqlite_snapshot_codec` methods; grep for all `fn sqlite_snapshot_` overcounts packages with capability methods. Native codec mounts establish source hook presence only, not assembled runtime publication. Cached native targets need an uncached root execution to establish fresh runtime receipts.

### @semio-tech/trinity-rewriting-rs

Router: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: No explicit codec method located; inspect generated/derived ownership before treating as absent.

Authored test names:

- `sqlite_snapshot_rewriting_actual_bare_parent_owns_capability`
- `sqlite_snapshot_rewriting_binary_keeps_all_authored_strings_and_property_words`
- `sqlite_snapshot_rewriting_text_keeps_all_authored_strings_and_property_words`
- `sqlite_snapshot_rewriting_genuine_controlled_record_covers_actual_recursive_property_domain`
- `sqlite_snapshot_rewriting_declared_json_retains_layout_words_and_all_property_states`
- `sqlite_snapshot_rewriting_independent_sqlite_interprets_nine_domain_tables`
- `sqlite_snapshot_rewriting_real_large_authored_string_control_is_interior`
- `sqlite_snapshot_rewriting_actual_erased_parent_queries_independent_authored_strings`
- `sqlite_snapshot_rewriting_actual_subset_io_declaration_owns_parent_capability`

### @semio-tech/raster-raster-rs

Router: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_raster_actual_declaration_capability`
- `sqlite_snapshot_raster_both_native_encodings_and_literal_child_identity`
- `sqlite_snapshot_raster_controlled_owner_refusals`
- `sqlite_snapshot_raster_full_owned_entities_and_raw_words_both_encodings`
- `sqlite_snapshot_raster_schema_relationships_and_exact_row_bound`
- `sqlite_snapshot_raster_four_phases_interior_large_copy_cancellation`
- `sqlite_snapshot_raster_deep_flat_native_ownership_and_late_refusal`
- `sqlite_snapshot_raster_cold_retirement_releases_atomic_large_strings`

### @semio-tech/flow-flow-rs

Router: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_flow_owned_child_and_independent_edit`
- `sqlite_snapshot_flow_bad_owned_rows_and_cancellation`
- `sqlite_snapshot_flow_actual_bare_erased_encodings`
- `sqlite_snapshot_flow_exact_wildcard_guard`
- `sqlite_snapshot_flow_actual_declaration_owned_io`
- `sqlite_snapshot_flow_all_literal_reference_fields_match_native_and_sqlite`
- `sqlite_snapshot_flow_controlled_native_full_string_fields`
- `sqlite_snapshot_flow_native_each_owned_string_has_interior_admission`

### @semio-tech/process-process3d-rs

Router: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: No explicit codec method located; inspect generated/derived ownership before treating as absent.

Authored test names:

- `sqlite_snapshot_process3d_actual_parent_bare_capability_is_present`
- `sqlite_snapshot_process3d_complete_parent_native_formats_preserve_all_fields_and_words`
- `sqlite_snapshot_process3d_parent_child_addresses_are_independent_literals`
- `sqlite_snapshot_process3d_genuine_controlled_metadata_and_typed_construction_cover_every_variant`
- `sqlite_snapshot_process3d_actual_erased_parent_both_encodings_have_queryable_domain_fields`
- `sqlite_snapshot_process3d_independent_sqlite_knows_every_handwritten_table_and_scalar_width`
- `sqlite_snapshot_process3d_actual_document_declaration_exposes_parent_capability`
- `sqlite_snapshot_process3d_owned_native_large_fields_have_real_interior_controls`

### @semio-tech/dag-dag-rs

Router: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs`

Authored test names:

- `sqlite_snapshot_dag_actual_bare_parent_owns_capability`
- `sqlite_snapshot_dag_actual_io_declaration_owns_parent_capability`
- `sqlite_snapshot_dag_binary_keeps_unresolved_literal_child_addresses`
- `sqlite_snapshot_dag_text_keeps_unresolved_literal_child_addresses`
- `sqlite_snapshot_dag_attached_local_scene_does_not_replace_persisted_parent_fields`
- `sqlite_snapshot_dag_controlled_value_constructs_only_actual_parent_fields`
- `sqlite_snapshot_dag_declared_json_preserves_literal_parent_child_fields`
- `sqlite_snapshot_dag_independent_sqlite_interprets_literal_two_table_parent`
- `sqlite_snapshot_dag_actual_erased_binary_text_keeps_all_literal_child_columns`
- `sqlite_snapshot_dag_actual_semantic_rows_admit_exact_count_and_cancel_all_copy_phases`
- `sqlite_snapshot_dag_owned_relationship_and_dialect_edits_are_rejected`
- `sqlite_snapshot_dag_independent_sqlite_edits_reconstruct_literal_parent`

### @semio-tech/stdio-bmp-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_bmp_owned_io_preserves_all_header_and_rgba_fields`
- `sqlite_snapshot_bmp_header_palette_and_grid_pixels_are_semantic`
- `sqlite_snapshot_bmp_independent_sql_grid_edits_preserve_rgba_channels`
- `sqlite_snapshot_bmp_actual_factory_canonical_external_carrier`
- `sqlite_snapshot_bmp_native_input_interior_and_caller_ceilings`
- `sqlite_snapshot_bmp_long_schema_projection_and_reconstruction_are_interior_controlled`
- `sqlite_snapshot_bmp_native_output_interior_and_caller_ceilings`
- `sqlite_snapshot_bmp_actual_typed_file_preserves_complete_literal_owned_fields`
- `sqlite_snapshot_bmp_external_carrier_normalizes_unrepresented_schema`

### @semio-tech/stdio-svg-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_svg_literal_metadata_native_state_is_distinct_from_external_wire`
- `sqlite_snapshot_svg_native_long_unicode_carriers_and_interior_cancellation`
- `sqlite_snapshot_svg_native_limits_and_literal_typed_file_io`
- `sqlite_snapshot_svg_exact_declared_tiny_basic_owned_io`
- `sqlite_snapshot_svg_domain_names_and_all_native_document_fields_roundtrip`
- `sqlite_snapshot_svg_rejects_foreign_roots_shapes_graphs_and_budgets`
- `sqlite_snapshot_svg_independent_geometry_queries_and_edits_preserve_native_state`
- `sqlite_snapshot_svg_tiny_and_basic_io_validators_recheck_reconstructed_entities`
- `sqlite_snapshot_svg_owned_tiny_basic_guard_matches_all_existing_conformance_rules`
- `sqlite_snapshot_svg_erased_native_preflight_admission_and_limits`

### @semio-tech/stdio-deflate-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_deflate_header_dictionary_and_decoded_payload_roundtrip`
- `sqlite_snapshot_deflate_independent_decoded_payload_edit`
- `sqlite_snapshot_deflate_actual_factory_canonical_external_carrier`
- `sqlite_snapshot_deflate_native_input_interior_and_caller_ceilings`
- `sqlite_snapshot_deflate_long_schema_projection_and_reconstruction_are_interior_controlled`
- `sqlite_snapshot_deflate_native_output_interior_and_caller_ceilings`
- `sqlite_snapshot_deflate_actual_typed_file_preserves_complete_literal_owned_fields`
- `sqlite_snapshot_deflate_external_carrier_normalizes_unrepresented_schema`

### @semio-tech/stdio-txt-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_txt_controlled_native_matches_the_real_external_carrier`
- `sqlite_snapshot_txt_native_controls_refuse_inside_copy_and_at_exact_limits`
- `sqlite_snapshot_txt_projection_and_reconstruction_cancel_inside_long_literal_fields`
- `sqlite_snapshot_txt_surrogate_ids_and_literal_metadata_are_not_native_headers`
- `sqlite_snapshot_txt_actual_typed_io_preserves_fields_without_native_phases`
- `sqlite_snapshot_txt_actual_bare_factory_transfers_native_carriers`
- `sqlite_snapshot_text_reconstruction_respects_value_budget`
- `sqlite_snapshot_text_value_preflight_can_be_cancelled`
- `sqlite_snapshot_text_lines_preserve_order_and_line_endings`

### @semio-tech/stdio-binary-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_binary_reconstruction_respects_value_budget`
- `sqlite_snapshot_binary_bytes_are_queryable_ordered_integers`
- `sqlite_snapshot_binary_actual_factory_canonical_external_carrier`
- `sqlite_snapshot_binary_native_input_interior_and_caller_ceilings`
- `sqlite_snapshot_binary_long_schema_projection_and_reconstruction_are_interior_controlled`
- `sqlite_snapshot_binary_native_output_interior_and_caller_ceilings`
- `sqlite_snapshot_binary_actual_typed_file_preserves_complete_literal_owned_fields`
- `sqlite_snapshot_binary_external_carrier_normalizes_unrepresented_schema`

### @semio-tech/stdio-json-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📦️pack/🦀️.rs`

Authored test names:

- `sqlite_snapshot_json_exact_declared_i_json_owned_io`
- `sqlite_snapshot_json_syntax_preserves_order_arbitrary_numbers_and_primitive_kinds`
- `sqlite_snapshot_json_rejects_dangling_cycles_ownership_ordinals_and_primitive_shape`
- `sqlite_snapshot_json_independent_queries_and_edits_reconstruct_the_native_model`
- `sqlite_snapshot_json_owned_i_json_guard_matches_native_rules_and_cancels`
- `sqlite_snapshot_json_native_encoding_preflight_bounds_escaping_indentation_and_cancellation`
- `sqlite_snapshot_json_exact_geojson_owned_io_and_borrowed_profile`
- `sqlite_snapshot_json_erased_native_preserves_owned_schema_lexemes_and_duplicate_members`
- `sqlite_snapshot_json_erased_native_deep_tree_has_no_wire_depth_or_indentation_quota`
- `sqlite_snapshot_json_handwritten_logical_records_validate_owned_topology_and_identity`
- `sqlite_snapshot_json_typed_intermediate_number_strings_independent_numeric_query_and_erased_fidelity`
- `sqlite_snapshot_json_typed_number_meaning_is_owned_by_named_profile_guards`
- `sqlite_snapshot_json_owned_record_identity_matches_handwritten_definition`
- `sqlite_snapshot_json_native_decode_owner_restores_typed_state_with_interior_control`
- `sqlite_snapshot_json_controlled_flat_binding_admits_storage_and_topology_before_copy`
- `sqlite_snapshot_json_authored_grammar_recognizes_its_own_full_state_logical_frame`
- `sqlite_snapshot_json_controlled_flat_output_preserves_full_tree_and_cancels_during_unicode`

### @semio-tech/stdio-gltf-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs`

Authored test names:

- `sqlite_snapshot_gltf_independent_sql_reserialization_preserves_ieee_words_unsigned_and_duplicate_extras`
- `sqlite_snapshot_gltf_erased_binary_and_text_preserve_every_owned_field`
- `sqlite_snapshot_gltf_rejects_independently_edited_ieee_conflict_and_unowned_relationship`
- `sqlite_snapshot_gltf_exact_typed_guard_budget_and_cancellation`
- `sqlite_snapshot_gltf_full_typed_document_and_independent_buffers_reconstruct`
- `sqlite_snapshot_gltf_independent_sql_mesh_accessor_material_and_buffer_edits`
- `sqlite_snapshot_gltf_actual_factory_exposes_owned_provider_and_structural_hash`
- `sqlite_snapshot_gltf_empty_domain_tables_preserve_absence_and_present_empty_lists`
- `sqlite_snapshot_gltf_deep_owned_extras_erased_payloads_have_bounded_wire_depth`
- `sqlite_snapshot_gltf_logical_text_rejects_foreign_envelope_and_unowned_suffix`
- `sqlite_snapshot_gltf_flat_primitive_extras_keep_deep_ordered_members_and_ieee_words`
- `sqlite_snapshot_gltf_empty_image_and_texture_entities_survive_logical_native_lists`
- `sqlite_snapshot_gltf_authored_grammar_recognizes_each_owned_named_domain`
- `sqlite_snapshot_gltf_controlled_binding_retires_deep_completed_fields_after_failure_and_cancellation`
- `sqlite_snapshot_gltf_independent_bad_later_json_owner_retires_deep_completed_sibling`
- `sqlite_snapshot_gltf_independent_later_root_owner_failure_retires_completed_extensions`
- `sqlite_snapshot_gltf_curated_demo_uses_actual_owned_flat_native_records`
- `sqlite_snapshot_gltf_curated_metabolism_pack_is_current_owned_logical_state`
- `sqlite_snapshot_gltf_typed_guard_rejects_independently_changed_owned_state`
- `sqlite_snapshot_gltf_typed_guard_accepts_independently_renumbered_surrogate`
- `sqlite_snapshot_gltf_native_owned_decode_controls_physical_and_typed_materialization`
- `sqlite_snapshot_gltf_genuine_native_output_preserves_paid_full_word_state`
- `sqlite_snapshot_gltf_genuine_native_output_rejects_limits_and_cancels_during_paid_copy`

### @semio-tech/stdio-csv-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_csv_reconstruction_respects_value_budget`
- `sqlite_snapshot_csv_wide_record_preflight_can_be_cancelled`
- `sqlite_snapshot_csv_records_fields_and_quoting_are_relational`
- `sqlite_snapshot_csv_actual_factory_canonical_external_carrier`
- `sqlite_snapshot_csv_native_input_interior_and_caller_ceilings`
- `sqlite_snapshot_csv_long_schema_projection_and_reconstruction_are_interior_controlled`
- `sqlite_snapshot_csv_native_output_interior_and_caller_ceilings`
- `sqlite_snapshot_csv_actual_typed_file_preserves_complete_literal_owned_fields`
- `sqlite_snapshot_csv_external_carrier_normalizes_unrepresented_schema`

### @semio-tech/stdio-tsv-rs

Router: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_tsv_reconstruction_respects_value_budget`
- `sqlite_snapshot_tsv_wide_record_preflight_can_be_cancelled`
- `sqlite_snapshot_tsv_records_fields_and_line_endings_are_relational`
- `sqlite_snapshot_tsv_actual_factory_canonical_external_carrier`
- `sqlite_snapshot_tsv_native_input_interior_and_caller_ceilings`
- `sqlite_snapshot_tsv_long_schema_projection_and_reconstruction_are_interior_controlled`
- `sqlite_snapshot_tsv_native_output_interior_and_caller_ceilings`
- `sqlite_snapshot_tsv_actual_typed_file_preserves_complete_literal_owned_fields`
- `sqlite_snapshot_tsv_external_carrier_normalizes_unrepresented_schema`

### @semio-tech/space-space-rs

Router: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`

Authored test names:

- `sqlite_snapshot_space_controlled_native_output_retains_exact_fields_and_cancels_inside_unicode`
- `sqlite_snapshot_space_full_occurrence_fields_and_independent_sqlite_edit`
- `sqlite_snapshot_space_complete_guard_aliases_and_erased_native_controls`
- `sqlite_snapshot_space_actual_declared_typed_and_registry_erased_io`

### @semio-tech/procedural-generation3d-rs

Router: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs`

Authored test names:

- `sqlite_snapshot_procedural_generation3d_wire_primitive_controlled_binding_preserves_literal_ownership`
- `sqlite_snapshot_generation_native_row_frontier_matches_complete_relational_entities`
- `sqlite_snapshot_procedural_generation3d_independent_entity_renumbering_preserves_owned_state`
- `sqlite_snapshot_procedural_generation3d_controlled_native_input_preserves_complete_owned_state`
- `sqlite_snapshot_procedural_generation3d_controlled_native_output_preserves_exact_physical_bytes`
- `sqlite_snapshot_procedural_generation3d_native_controls_cancel_inside_large_owned_literals`
- `sqlite_snapshot_procedural_generation3d_full_owned_state`
- `sqlite_snapshot_procedural_generation3d_independent_sqlite_edit`
- `sqlite_snapshot_procedural_generation3d_bad_structure_and_admission`
- `sqlite_snapshot_procedural_generation3d_erased_native_encodings_and_retained_refusals`
- `sqlite_snapshot_procedural_generation3d_actual_declaration_owned_io`
- `sqlite_snapshot_procedural_generation3d_deep_owned_answers_and_partial_cancellation`

### @semio-tech/procedural-generation2d-rs

Router: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs`

Authored test names:

- `sqlite_snapshot_generation2d_wide_owned_answer_row_frontier_and_interior_cancel`
- `sqlite_snapshot_generation_native_row_frontier_matches_complete_relational_entities`
- `sqlite_snapshot_procedural_generation2d_independent_entity_renumbering_preserves_owned_state`
- `sqlite_snapshot_procedural_generation2d_controlled_native_input_preserves_complete_owned_state`
- `sqlite_snapshot_procedural_generation2d_controlled_native_output_preserves_exact_physical_bytes`
- `sqlite_snapshot_procedural_generation2d_native_controls_cancel_inside_large_owned_literals`
- `sqlite_snapshot_procedural_generation2d_full_owned_state`
- `sqlite_snapshot_procedural_generation2d_independent_sqlite_edit`
- `sqlite_snapshot_procedural_generation2d_bad_structure_and_admission`
- `sqlite_snapshot_procedural_generation2d_erased_native_encodings_and_retained_refusals`
- `sqlite_snapshot_procedural_generation2d_actual_declaration_owned_io`
- `sqlite_snapshot_procedural_generation2d_deep_owned_answers_and_partial_cancellation`

### @semio-tech/framework-flow-flow-rs

Router: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: No explicit codec method located; inspect generated/derived ownership before treating as absent.

Authored test names:

- `sqlite_snapshot_framework_flow_bare_persisted_owner_has_semantic_capability`
- `sqlite_snapshot_framework_flow_public_identity_matches_actual_native_carrier`
- `sqlite_snapshot_framework_flow_binary_keeps_every_widget_cluster_gui_and_word`
- `sqlite_snapshot_framework_flow_text_keeps_every_widget_cluster_gui_and_word`
- `sqlite_snapshot_framework_flow_controlled_value_keeps_actual_owned_state`
- `sqlite_snapshot_framework_flow_erased_both_formats_expose_persisted_entities`
- `sqlite_snapshot_framework_flow_native_output_cancels_inside_owned_text_copy`

### @semio-tech/framework-space-space-rs

Router: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs`

Authored test names:

- `sqlite_snapshot_framework_space_builtin_creation_and_reload_publish_io_without_manual_registration`
- `sqlite_snapshot_framework_space_same_control_retained_materialization_budget_is_cumulative`
- `sqlite_snapshot_framework_space_actual_typed_io_file_metadata_and_route`
- `sqlite_snapshot_framework_space_independent_surrogate_renumber_preserves_complete_logical_state`
- `sqlite_snapshot_framework_space_actual_factory_capability`
- `sqlite_snapshot_framework_space_ordinary_native_complete_state`
- `sqlite_snapshot_framework_space_erased_both_directions_preserve_all_semantic_cells`
- `sqlite_snapshot_framework_space_independent_sql_edit_retains_literal_typed_state`
- `sqlite_snapshot_framework_space_authored_schema_and_exact_semantic_row_admission`
- `sqlite_snapshot_framework_space_all_owned_phases_cancel_and_wrong_dialect_refuses`
- `sqlite_snapshot_framework_space_native_input_retained_materialization_uses_same_caller_ledger`

### @semio-tech/framework-space-collection-rs

Router: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs`

Authored test names:

- `sqlite_snapshot_framework_collection_builtin_creation_and_reload_publish_io_without_manual_registration`
- `sqlite_snapshot_framework_collection_same_control_retained_materialization_budget_is_cumulative`
- `sqlite_snapshot_framework_collection_actual_typed_io_file_metadata_and_route`
- `sqlite_snapshot_framework_collection_independent_surrogate_renumber_preserves_complete_logical_state`
- `sqlite_snapshot_framework_collection_actual_factory_capability`
- `sqlite_snapshot_framework_collection_ordinary_native_complete_state`
- `sqlite_snapshot_framework_collection_erased_both_directions_preserve_all_semantic_cells`
- `sqlite_snapshot_framework_collection_independent_sql_edit_retains_literal_typed_state`
- `sqlite_snapshot_framework_collection_authored_schema_and_exact_semantic_row_admission`
- `sqlite_snapshot_framework_collection_all_owned_phases_cancel_and_wrong_dialect_refuses`
- `sqlite_snapshot_framework_collection_native_input_retained_materialization_uses_same_caller_ledger`

### @semio-tech/framework-infinite-dag-rs

Router: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: No explicit codec method located; inspect generated/derived ownership before treating as absent.

Authored test names:

- `sqlite_snapshot_framework_dag_bare_persisted_owner_has_semantic_capability`
- `sqlite_snapshot_framework_dag_public_identity_matches_actual_native_carrier`
- `sqlite_snapshot_framework_dag_binary_keeps_all_variant_fields_and_exact_words`
- `sqlite_snapshot_framework_dag_text_keeps_all_variant_fields_and_exact_words`
- `sqlite_snapshot_framework_dag_controlled_value_keeps_actual_owned_state`
- `sqlite_snapshot_framework_dag_erased_both_native_formats_have_queryable_entities`
- `sqlite_snapshot_framework_dag_native_output_cancels_inside_owned_text_copy`

### @semio-tech/framework-workflow-workflow-rs

Router: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust/📜️script.ts`. Registered command: `bun ./📜️script.ts test-snapshot-sqlite native`.

Codec mount source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`

Authored test names:

- `sqlite_snapshot_workflow_media_contract_binds_literal_values_under_both_native_controls`
- `sqlite_snapshot_workflow_media_port_binds_literal_values_under_both_native_controls`
- `sqlite_snapshot_workflow_input_binds_literal_values_and_cancels_owned_utf8`
- `sqlite_snapshot_workflow_manual_metadata_preserves_neutral_fields_and_enum_labels_under_both_controls`
- `sqlite_snapshot_workflow_bare_native_codec_requires_the_complete_semantic_owner`
- `sqlite_snapshot_workflow_complete_neutral_native_fields_retain_both_declared_encodings`
- `sqlite_snapshot_workflow_all_native_geometry_and_optional_parameter_words_are_exact`
- `sqlite_snapshot_workflow_complete_relational_root_and_empty_collections_restore`
- `sqlite_snapshot_workflow_independent_sql_queries_and_surrogate_edits_preserve_all_fields`
- `sqlite_snapshot_workflow_exact_geometry_and_optional_numeric_words_reach_both_erased_native_directions`
- `sqlite_snapshot_workflow_every_declared_media_vocabulary_and_optional_literal_kind_restore`
- `sqlite_snapshot_workflow_independent_malformed_edits_refuse_every_owned_relationship`
- `sqlite_snapshot_workflow_exact_row_limits_and_tiny_owned_native_frontiers_are_enforced`
- `sqlite_snapshot_workflow_long_literal_unicode_cancellation_reaches_all_four_owned_phases`
- `sqlite_snapshot_workflow_caller_cancellation_reaches_interior_known_node_collections`
- `sqlite_snapshot_workflow_imperative_registration_routes_actual_queryable_files`


History: no project with a History-named artifact was found in this scoped cohort; root must provide its authoritative route before executing it. Framework Workflow is separately listed for clarity and uses a custom **`sqlite_`** selector rather than `sqlite_snapshot_`; its table count currently counts only `sqlite_snapshot_` declarations.

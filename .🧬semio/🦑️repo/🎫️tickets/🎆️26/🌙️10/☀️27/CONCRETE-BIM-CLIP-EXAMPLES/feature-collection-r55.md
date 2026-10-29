# Registered Feature Hook Collection R55

Collection only on 2026-10-29 through runpy.run_path(main) and main load_tests(unittest.defaultTestLoader, unittest.TestSuite(), None). No test execution, setUp, or building_geometry call. This is not a final source-identity verification.

The actual hook collected 342 entries: 341 real tests and 1 failed import placeholder. All remaining registered groups imported and collected. No _ErrorHolder was present.

## Collection Failure

The single unittest.loader._FailedTest is the registered axon test module. Import path: axon tests → axon producer → drafting → energy authoring fonts → output.configure_manim → from manim import config. The isolated Pyodide harness has no manim package. This must be addressed in the test-runtime interface before claiming successful full hook execution; the module must not be silently omitted. No production or test files were changed by this preflight.

```text
Failed to import test module: 🧪️tests
Traceback (most recent call last):
  File "/lib/python313.zip/unittest/loader.py", line 137, in loadTestsFromName
    module = __import__(module_name)
  File "/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🧪️tests.py", line 10, in <module>
    _axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️")
            ~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/lib/python313.zip/importlib/__init__.py", line 88, in import_module
    return _bootstrap._gcd_import(name[level:], package, level)
           ~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🐍️.py", line 13, in <module>
    _d = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
         ~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/lib/python313.zip/importlib/__init__.py", line 88, in import_module
    return _bootstrap._gcd_import(name[level:], package, level)
           ~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🐍️.py", line 10, in <module>
    _fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
             ~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/lib/python313.zip/importlib/__init__.py", line 88, in import_module
    return _bootstrap._gcd_import(name[level:], package, level)
           ~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/repo/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py", line 18, in <module>
    importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").configure_manim()
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/repo/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/📁️output/🐍️.py", line 33, in configure_manim
    from manim import config
ModuleNotFoundError: No module named 'manim'

```

## Class and Function Inventory

| Registered class/group | Functions |
| --- | ---: |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🐍️.SparseSupportContract | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️parking.ParkingContract | 9 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🧩️chapter-source.🐍️.BimChapterSourceContract | 17 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔌️exchange.🧪️tests.🐍️slab.SlabConsumers | 6 |
| unittest.loader._FailedTest | 1 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.📏️geometry.AxonPhysicalGeometryContract | 3 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🧪️surfaces.SurfaceRoleContract | 1 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧮️analysis.🧪️profiles.🐍️.MeshTrueProfileContract | 2 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📍️placements.LocalPlacementContract | 4 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️slabs.SlabPartitionCacheContract | 1 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️partitions.SlabPartitionContract | 4 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️beams.BeamSharedOwnershipContract | 2 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️joints.BeamJointOwnershipContract | 37 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️bearing.RoofPostBearingContract | 6 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔺️frustums.FrustumSolidContract | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪚️ownership.MaterialOwnershipContract | 7 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕳️openings.SlabOpeningContract | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️services.CellarServicesContract | 8 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚰️services.ServiceProfileContract | 8 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧊️sections.SectionExtrusionContract | 3 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.☀️rooflights.RooflightConstruction | 3 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🌬️garden.GardenConstruction | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️ramps.RampGeometryContract | 6 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️curtains.NativeCurtainContract | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🕳️clashes.OccupiedClashContract | 9 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕳️profiles.HollowProfileContract | 13 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🕳️passages.FixedBeamPassageContract | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🪟️fillings.NativeFillingStepContract | 6 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel.FunctionTestCase | 18 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛁️details.FunctionTestCase | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📐️representations.FunctionTestCase | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕳️cellar.FunctionTestCase | 2 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️foundations.FunctionTestCase | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️architecture.FunctionTestCase | 7 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️suite.FunctionTestCase | 6 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪨️regions.FunctionTestCase | 6 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧱️walls.FunctionTestCase | 6 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️fillings.FunctionTestCase | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪚️joinery.FunctionTestCase | 3 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚪️garage.FunctionTestCase | 3 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚪️roof.FunctionTestCase | 5 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground.FunctionTestCase | 7 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️chimneys.FunctionTestCase | 4 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️prisms.FunctionTestCase | 4 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪜️stair-parts.FunctionTestCase | 7 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪜️stairs.FunctionTestCase | 6 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕸️mesh-clipping.FunctionTestCase | 2 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🌀️wall-routing.FunctionTestCase | 4 |
| 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️facade.FunctionTestCase | 8 |
| <run_path>.FunctionTestCase | 2 |
| test_cells.WallCellContract | 1 |
| test_defaults.DetailDefaultContract | 1 |
| test_defaults.LevelLabelContract | 1 |
| test_frustums.FrustumDrawingContract | 9 |
| test_profile_consumer.ProfileDetailConsumer | 2 |
| test_profiles.ProfileCutContract | 3 |
| test_screens.ScreenDrawingContract | 5 |
| test_services.ServiceEnvelopeContract | 1 |
| test_services_native.ServiceDrawingContract | 4 |
| test_slab_regions.RegionalSlabContract | 3 |
| test_stairs.StairDrawingContract | 6 |

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🐍️.SparseSupportContract (5)

- test_authored_storeys_have_separate_complete_support_partitions
- test_bearing_profile_intersection_matches_shapely
- test_nonconvex_cut_is_not_its_convex_hull
- test_portable_voronoi_matches_scipy_nearest_neighbours
- test_translation_is_a_complete_first_floor_mesh_with_voids

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️parking.ParkingContract (9)

- test_fixture_is_bound_to_current_physical_source
- test_native_collision_band_projects_actual_section_occupancy
- test_native_vehicle_checks_actual_hoist_and_reuses_isolated_geometry
- test_numpy_corners_and_javascript_metrics_match_geos
- test_physical_oracle_matches_native_height_intersections
- test_source_signature_selects_native_mesh_height_band
- test_three_car_demonstrations_are_rejected_without_infeasibility_claim
- test_two_car_control_is_physically_verified
- test_vehicle_checks_synthetic_height_intersecting_service

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🧩️chapter-source.🐍️.BimChapterSourceContract (17)

- test_analysis_narration_preserves_selected_births_and_unsolved_scope
- test_bilingual_narration_matches_accessible_transcripts
- test_chapter_callbacks_reference_defined_globals
- test_chapters_use_the_current_geometry_contract
- test_discipline_tables_and_loss_rows_partition_native_identities
- test_elevation_labels_preserve_source_millimetres_and_negative_signs
- test_entrance_cut_includes_only_material_at_its_native_height
- test_exchange_count_card_reports_registered_products_without_historic_range
- test_members_card_uses_actual_clipped_mesh_counts_and_selected_supports
- test_native_class_matrix_and_chimney_meanings_include_every_discipline
- test_provenance_helper_distinguishes_all_native_bases_in_both_languages
- test_roundtrip_units_accept_native_dimensionless_station_fractions
- test_service_placement_oracle_checks_native_parts_without_requiring_an_axis
- test_source_roles_and_paving_replace_removed_fictions
- test_standard_hook_registers_native_contracts_exactly_once
- test_takeoff_glass_hatches_use_native_filling_hosts
- test_window_highlights_filter_fill_class

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔌️exchange.🧪️tests.🐍️slab.SlabConsumers (6)

- test_all_openings_drawn_and_named
- test_crop_against_independent_geometry
- test_disconnected_glass
- test_exchange_room_context
- test_grid_dimension_is_source_based
- test_source_opening_areas

### unittest.loader._FailedTest (1)

- 🧪️tests

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.📏️geometry.AxonPhysicalGeometryContract (3)

- test_buried_columns_split_at_the_terrain_occlusion_plane
- test_mixed_slab_keeps_each_regional_depth
- test_rectangular_column_is_never_replaced_by_circle

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🧪️surfaces.SurfaceRoleContract (1)

- test_portable_material_dispatch

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧮️analysis.🧪️profiles.🐍️.MeshTrueProfileContract (2)

- test_analytical_plan_marker_uses_the_actual_member_section
- test_true_shape_follows_rectangular_section_rather_than_bounding_circle

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📍️placements.LocalPlacementContract (4)

- test_explicit_stair_steps_and_landings_have_distinct_origins
- test_generic_local_parts_resolve_world_datums
- test_level_roof_finish_accounts_for_regional_depth
- test_ramp_flight_carries_the_authored_six_centimetre_slab

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️slabs.SlabPartitionCacheContract (1)

- test_same_geometry_reuses_partition_without_stale_mutations

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️partitions.SlabPartitionContract (4)

- test_clipping_retains_authored_reference_precision
- test_editable_holes_cannot_hide_ambiguous_region_ownership
- test_near_boundary_and_disjoint_holes_match_independent_union
- test_source_region_surfaces_and_sleeves_match_independent_geos

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️beams.BeamSharedOwnershipContract (2)

- test_shared_and_exclusive_parts_remain_distinct_and_form_complete_beam
- test_solid_embedded_beam_strip_has_full_concrete_self_weight

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️joints.BeamJointOwnershipContract (37)

- test_BEAM_PRIMARY_F_13_14
- test_BEAM_PRIMARY_F_13_25
- test_BEAM_PRIMARY_F_15_20
- test_BEAM_PRIMARY_F_1_6
- test_BEAM_PRIMARY_F_20_32
- test_BEAM_PRIMARY_F_24_30
- test_BEAM_PRIMARY_F_25_28
- test_BEAM_PRIMARY_F_26_28
- test_BEAM_PRIMARY_F_28_31
- test_BEAM_PRIMARY_F_28_CROSS
- test_BEAM_PRIMARY_F_2_7
- test_BEAM_PRIMARY_F_3_34
- test_BEAM_PRIMARY_F_4_10
- test_BEAM_PRIMARY_F_5_33
- test_BEAM_PRIMARY_F_6_29
- test_BEAM_PRIMARY_F_7_24
- test_BEAM_PRIMARY_F_8_9
- test_BEAM_PRIMARY_R_10_11
- test_BEAM_PRIMARY_R_10_15
- test_BEAM_PRIMARY_R_14_26
- test_BEAM_PRIMARY_R_1_6
- test_BEAM_PRIMARY_R_20_32
- test_BEAM_PRIMARY_R_22_29
- test_BEAM_PRIMARY_R_24_30
- test_BEAM_PRIMARY_R_28_31
- test_BEAM_PRIMARY_R_2_7
- test_BEAM_PRIMARY_R_34_10
- test_BEAM_PRIMARY_R_34_35
- test_BEAM_PRIMARY_R_3_34
- test_BEAM_PRIMARY_R_4_10
- test_BEAM_PRIMARY_R_5_33
- test_BEAM_PRIMARY_R_6_22
- test_BEAM_PRIMARY_R_6_34
- test_complete_nonauxiliary_beam_column_register
- test_exact_pair_register_matches_independent_geos_and_scipy
- test_invalid_and_stale_joint_owners_fail
- test_native_body_declares_member_and_material_volume_scopes

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️bearing.RoofPostBearingContract (6)

- test_a_structural_reserve_need_not_reach_the_separate_finish
- test_actual_native_posts_have_continuous_rc_material_through_roof_deck
- test_analytical_slab_domain_keeps_native_column_supports_and_full_load_surface
- test_cache_tracks_reserved_columns_and_returns_isolated_parts
- test_source_embedded_junction_is_explicitly_inferred_and_reinforcement_unassessed
- test_structural_and_weathering_material_are_owned_once_and_conserve_union

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔺️frustums.FrustumSolidContract (5)

- test_every_representation_preserves_the_taper
- test_inclined_section_has_true_sides
- test_invalid_loft_correspondence_is_rejected
- test_tapered_union_matches_independent_integrated_slice_union
- test_tapered_volumes_and_vertices_match_independent_hulls

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪚️ownership.MaterialOwnershipContract (7)

- test_coplanar_cuts_keep_connected_boundaries_instead_of_render_fragments
- test_disjoint_cutters_do_not_fragment_occupied_material
- test_exact_collinear_spikes_preserve_thin_material
- test_prism_subtraction_preserves_metadata_and_disjoint_material_volume
- test_source_curtain_material_rings_are_valid_without_material_loss
- test_source_masonry_excludes_registered_concrete_columns
- test_source_wall_ownership_matches_independent_material_difference

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕳️openings.SlabOpeningContract (5)

- test_native_opening_removes_registered_slab_material
- test_opening_edits_invalidate_host_cache
- test_opening_quantity_and_all_projections_keep_regional_depths
- test_openings_must_be_inside_host_and_disjoint
- test_slab_and_wall_hosts_are_distinct_schema_contracts

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️services.CellarServicesContract (8)

- test_actual_cellar_wall_mouths_are_open_and_closed_covers_are_separate
- test_hollow_service_solids_and_loft_volumes_match_independent_libraries
- test_native_hatches_remove_host_even_without_covers
- test_physical_service_solids_do_not_overlap_columns_or_door_fillings
- test_registered_projections_clear_actual_numbered_structure
- test_registered_projections_have_native_connected_apertures
- test_service_types_classify_and_aggregate_native_occurrences
- test_spatial_type_material_and_partial_model_ownership

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚰️services.ServiceProfileContract (8)

- test_every_representation_keeps_native_material_and_hollow_cross_sections
- test_historical_schema_rejects_conflicting_state_and_proposal_as_presence
- test_inferred_cellar_plant_is_supported_and_clear_of_stair
- test_invalid_wall_and_nonorthogonal_axis_are_rejected
- test_mitred_exit_has_connected_empty_bore_and_disjoint_physical_material
- test_original_identity_and_registered_visible_service_families
- test_polyhedral_faces_reject_open_or_nonplanar_shells
- test_three_native_axes_preserve_hollow_material_and_exact_endpoints

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧊️sections.SectionExtrusionContract (3)

- test_invalid_and_ambiguous_source_sections_are_rejected
- test_production_plan_section_and_axon_share_exact_geometry
- test_sections_volumes_vertices_and_every_projection

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.☀️rooflights.RooflightConstruction (3)

- test_actual_production_material_and_plan_callbacks
- test_mixed_material_inventory_and_rooflight_mass
- test_registered_aperture_and_material_assembly

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🌬️garden.GardenConstruction (5)

- test_actual_cap_roof_plan_and_exact_surface_projections
- test_concave_cap_is_separate_material_owned_geometry
- test_garden_auxiliaries_and_wall_leaves_respect_post_ownership
- test_support_graph_tracks_actual_garden_and_kiosk_hosts
- test_three_functional_post_sections

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️ramps.RampGeometryContract (6)

- test_diagonal_flight_has_true_sloping_bottom_and_independent_slices
- test_invalid_convention_and_borrowed_landing_drift_fail
- test_native_ramps_keep_source_products_and_aggregation_ownership
- test_owned_source_inventory_has_no_landing_or_flight_double_material
- test_production_callbacks_and_vehicle_obstacles_use_the_same_parts_once
- test_source_products_have_real_shared_geometry_and_all_representations

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️curtains.NativeCurtainContract (5)

- test_building_fillings_are_real_apertures_and_vehicle_curtain_exists_once
- test_insulated_metal_wings_do_not_turn_the_timber_leaf_core_into_metal
- test_native_host_domain_and_aperture_share_exact_world_geometry
- test_physical_panes_are_planar_and_material_never_fills_the_aperture
- test_source_host_schema_is_exactly_one_native_domain

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🕳️clashes.OccupiedClashContract (9)

- test_convex_distance_matches_independent_scipy_halfspaces
- test_empty_pipe_lumen_is_not_reported_as_solid_pipe_material
- test_host_relations_do_not_hide_filled_bore_interference
- test_independent_material_oracle_exercises_genuine_3d_halfspaces
- test_progress_result_and_source_schemas_are_explicit
- test_source_has_explicit_native_host_relations_and_policy
- test_source_hosted_conduits_do_not_create_false_clashes
- test_stale_host_relation_is_invalid_and_work_is_cancellable
- test_unhosted_nearby_pipe_still_violates_the_external_clearance

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕳️profiles.HollowProfileContract (13)

- test_belowgrade_exit_is_open_and_does_not_bore_the_footing
- test_bore_quantities_are_localized_and_surface_ownership_is_explicit
- test_hollow_mesh_is_closed_and_its_cap_matches_geos_difference
- test_invalid_bores_are_rejected_and_native_evidence_is_explicit
- test_material_cuts_and_collision_slices_preserve_bore_air
- test_native_column_representations_retain_voids
- test_native_step_uses_hollow_and_arbitrary_void_profiles
- test_net_profile_material_and_second_moments
- test_offset_bore_centroid_moments_match_geos_scipy_integration
- test_only_attested_pipe_route_sections_are_bored
- test_production_plan_section_and_axon_show_a_real_bore
- test_source_pipe_and_elbow_material_do_not_overlap_authored_column_material
- test_structural_bearing_requires_actual_material_contact_outside_the_bore

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🧪️tests.🕳️passages.FixedBeamPassageContract (5)

- test_all_beams_including_cellar_are_disjoint_from_actual_pipe_material
- test_each_fixed_cut_is_exact_and_bounded_by_its_raw_beam
- test_fixed_profiles_are_exact_native_openings_and_only_four_beams
- test_passages_have_nominal_ten_mm_radial_gap_and_are_fixed_under_pipe_edits
- test_portable_slice_and_material_volume_oracle_detects_a_plug

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🪟️fillings.NativeFillingStepContract (6)

- test_all_exported_solid_parts_match_independent_polygon_volume
- test_full_source_schema_accepts_normalized_panel_stations
- test_material_constituents_address_every_solid_exactly_once
- test_native_entities_types_and_attribute_positions
- test_public_filling_interface_matches_native_body_source
- test_spatial_free_excerpt_has_native_occurrence_attributes_and_constituents

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️remodel.FunctionTestCase (18)

- test_actual_column_profiles_bear_their_assigned_slabs
- test_bath_chaise_clears_actual_upper_column
- test_chauffeur_programme_partitions_the_real_diagonal_studio
- test_covered_garden_canopy_preserves_open_terrace
- test_flat_entry_bay_has_real_glass_interruption
- test_guest_door_full_body_width_and_public_approach
- test_landscape_aperture_is_not_covered_by_curved_screen
- test_pantry_hatch_is_a_real_empty_hosted_opening
- test_parents_bath_has_private_lower_boundary
- test_planters_leave_two_real_garage_rooflights_clear
- test_private_corridor_supports_continuous_person_width
- test_remodel_contract
- test_remodel_spaces_do_not_overlap
- test_remodel_third_party_polygons
- test_salon_radiator_and_terminal_pipe_are_in_salon
- test_solarium_right_cap_bulges_outward
- test_son_door_opens_to_public_hall
- test_vehicle_sweep_against_independent_geometry

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛁️details.FunctionTestCase (5)

- test_detail_placement_and_independent_quantities
- test_details_have_actual_hollow_and_occupied_volumes
- test_details_leave_person_routes_and_door_approaches_clear
- test_guest_washing_bowl_is_actually_under_zenith_light
- test_stair_guards_follow_flights_with_connected_tube_and_open_arrival

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.📐️representations.FunctionTestCase (5)

- test_polygon_sections_handle_concavity_and_boundary_cuts
- test_polygon_union_area_handles_crossings_and_concavity
- test_sections_and_elevations_use_actual_xz_solids
- test_sections_preserve_real_cavities
- test_stair_quantities_equal_occupied_solid_union_including_landings

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕳️cellar.FunctionTestCase (2)

- test_cellar_contract
- test_cellar_shapely

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️foundations.FunctionTestCase (5)

- test_foundation_frustum_quantities_match_scipy_convex_hulls
- test_foundation_material_totals_match_portable_oracle
- test_foundation_materials_and_vertical_contacts_are_physical
- test_foundations_cover_actual_exterior_supports_once
- test_foundations_preserve_cellar_material_ownership

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️architecture.FunctionTestCase (7)

- test_finish_solids_are_nominal_slices_below_the_floor_datum
- test_garden_paving_and_grass_exactly_cover_the_real_floor
- test_guest_floor_finish_zones_follow_the_integrated_wardrobe
- test_kitchen_patio_has_empty_facade_frame_instead_of_false_glass
- test_kitchen_patio_is_a_real_open_air_room_and_roof_void
- test_office_access_is_clear_between_hall_and_kitchen
- test_service_entrance_is_an_independent_free_portal

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏛️suite.FunctionTestCase (6)

- test_alcove_uses_registered_circular_posts_and_existing_frame
- test_parent_room_contains_full_registered_alcove_bay
- test_parents_bath_opens_physically_into_bedroom
- test_son_public_route_does_not_cross_parents_suite
- test_u_handrails_have_connected_physical_sections_and_nominal_volume
- test_upper_stair_arrival_is_supported_and_clear_of_its_guards

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪨️regions.FunctionTestCase (6)

- test_measured_archival_regions_have_actual_structural_tops_and_common_soffits
- test_regional_partition_rejects_overlap_gaps_and_invalid_depths
- test_regional_representations_preserve_depths_voids_and_actual_section_plane
- test_regional_slabs_match_analytical_and_independent_occupied_volumes
- test_regional_soffit_uses_conservative_lowest_surface
- test_step_exports_every_regional_solid_with_actual_depth_and_void_subtraction

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧱️walls.FunctionTestCase (6)

- test_cavity_wall_parts_preserve_opening_and_exclude_air
- test_source_facade_families_keep_solid_and_cavity_dimensions_distinct
- test_step_wall_extrusions_use_occupied_leaves
- test_vertical_material_regions_follow_opening_sill_and_keep_outer_plane
- test_wall_axis_reversal_preserves_solid_ownership
- test_wall_body_uses_occupied_parts

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️fillings.FunctionTestCase (5)

- test_filling_perimeter_solids_and_clear_aperture
- test_glass_face_area_excludes_frames_and_includes_glazed_doors
- test_material_relations_do_not_reassign_typed_fillings_to_concrete
- test_native_filling_materials_and_clear_widths_are_explicit
- test_registered_curtain_has_real_planar_members_and_native_portals

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪚️joinery.FunctionTestCase (3)

- test_salon_has_hollow_two_lane_tracks_and_real_sashes
- test_sashes_and_mullions_occupy_glass_divisions_without_intersection
- test_source_glazed_joinery_parts_and_quantities_agree_with_geos

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚪️garage.FunctionTestCase (3)

- test_garage_head_tracks_its_actual_slab_soffit
- test_garage_portal_has_no_false_intermediate_masonry_piers
- test_garage_two_train_panels_are_frameless_and_material_unverified

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚪️roof.FunctionTestCase (5)

- test_corner_aperture_keeps_finite_station_end_planes
- test_mixed_filling_cells_conserve_independent_material_and_clear_leaf
- test_mixed_filling_rejects_uncovered_or_overlapping_layout_cells
- test_roof_access_is_on_the_registered_vertical_host_segment
- test_roof_exit_leaf_has_its_own_external_timber_evidence

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground.FunctionTestCase (7)

- test_ground_basin_registration_and_garage_portal_are_physically_consistent
- test_ground_paths_match_independent_registered_controls
- test_ground_plate_preserves_cellar_ownership_and_registered_apertures
- test_ground_registered_room_fittings_and_private_portals_remain_accessible
- test_ground_spaces_are_valid_distinct_and_contain_registered_basin_and_hoist
- test_northern_circulation_passes_the_registered_basin_without_moving_source_structure
- test_rejected_garage_hall_door_is_absent_and_did_intersect_the_source_ramp

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️chimneys.FunctionTestCase (4)

- test_actual_production_catalogue_and_storey_projections_keep_spanning_chimney
- test_chimney_and_real_hall_route_have_no_occupied_native_clash
- test_chimney_material_matches_independent_annulus_and_hulls
- test_original_chimney_registration_and_native_sleeves

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚬️prisms.FunctionTestCase (4)

- test_actual_registered_roof_chimney_sections_and_support
- test_finish_reserve_rejects_missing_or_incomplete_native_owner
- test_native_composite_service_and_finish_owner_partition
- test_production_roof_profile_and_above_cut_wire_keep_material_scope

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪜️stair-parts.FunctionTestCase (7)

- test_native_solid_mesh_keeps_authored_material_role
- test_native_stair_step_preserves_all_material_parts
- test_polyhedron_boundary_planes_keep_all_vertices
- test_public_stair_parts_preserve_mesh_and_material
- test_stair_geometry_requires_explicit_occupied_parts
- test_stair_occupied_volume_matches_independent_hull_and_slices
- test_stair_source_has_no_unused_turn_synthesis_fields

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪜️stairs.FunctionTestCase (6)

- test_cellar_first_tread_has_real_door_and_standing_approach
- test_complete_u_trains_and_registered_curve_centre
- test_native_waist_volume_and_continuous_connections
- test_original_roof_ramp_guard_is_open_tubular_construction
- test_roof_stair_cubicle_is_materially_hollow_and_has_real_access
- test_shared_slab_void_is_explicit_and_union_owned

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🕸️mesh-clipping.FunctionTestCase (2)

- test_mesh_node_publication_obeys_only_declared_half_cell_error
- test_unrounded_mesh_clip_matches_independent_geos_domain

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🌀️wall-routing.FunctionTestCase (4)

- test_curved_standard_wall_native_step_preserves_whole_body
- test_curved_standard_wall_public_views_keep_material_domain
- test_explicit_closed_wall_keeps_its_final_material_segment
- test_standard_curved_wall_reaches_complete_shared_catalogue

### 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏗️facade.FunctionTestCase (8)

- test_auxiliary_concrete_metal_and_material_leaves_never_share_volume
- test_auxiliary_union_matches_geos_difference_of_primary_structure
- test_garden_topology_keeps_open_frame_and_covered_kiosk_distinct
- test_profile_sections_match_independent_geos_area_and_sweep
- test_shelf_and_upstand_have_independent_source_thicknesses
- test_source_auxiliary_parts_are_disjoint_and_order_independent
- test_source_clear_apertures_and_all_representations_survive
- test_support_graph_distinguishes_corners_lower_stiffeners_and_cavity_ties

### <run_path>.FunctionTestCase (2)

- <lambda>
- <lambda>

### test_cells.WallCellContract (1)

- test_cells

### test_defaults.DetailDefaultContract (1)

- test_default_supports_exist

### test_defaults.LevelLabelContract (1)

- test_source_precision_and_locale

### test_frustums.FrustumDrawingContract (9)

- test_architectural_ground_plan_hides_buried_foundations
- test_axon_side_faces_match_scipy_hull_planes_and_actual_top
- test_complete_elevation_provider_uses_actual_tapered_faces
- test_detail_reveal_preserves_actual_slope_and_section_station
- test_elevation_painter_records_inclined_world_vertices
- test_live_extrusion_recomputes_inclined_normals
- test_plan_records_actual_cut_and_visible_caps
- test_section_records_real_inclined_cut_and_partial_height
- test_staged_services_keep_the_true_band_cap_rings

### test_profile_consumer.ProfileDetailConsumer (2)

- test_actual_outline_and_part_heights
- test_regional_strip_uses_occupied_parts

### test_profiles.ProfileCutContract (3)

- test_cellar_levels_and_slabs
- test_finish_and_footing_visibility
- test_profile_spans

### test_screens.ScreenDrawingContract (5)

- test_axon_provider_uses_same_native_parts
- test_complete_elevation_provider_projects_aperture_wall_and_cap
- test_plan_provider_records_native_aperture_slices
- test_section_provider_records_material_below_and_above_window
- test_solarium_detail_records_shelf_head_and_sloping_cap

### test_services.ServiceEnvelopeContract (1)

- test_service_bounds

### test_services_native.ServiceDrawingContract (4)

- test_axon_provider_emits_exact_material_without_axis_padding
- test_complete_service_elevation_uses_physical_faces
- test_plan_provider_preserves_actual_hollow_horizontal_slices
- test_section_provider_keeps_annular_air_and_inclined_end_planes

### test_slab_regions.RegionalSlabContract (3)

- test_exposed_region_faces
- test_level_finish_from_source_parts
- test_region_cuts

### test_stairs.StairDrawingContract (6)

- test_axon_collection_preserves_waist_faces_and_finish_parts_once
- test_elevation_includes_native_waist_silhouette
- test_material_roles_preserve_source_tile_and_concrete
- test_path_wall_door_catalogue_and_native_plan_section
- test_plan_includes_waist_cut_between_treads
- test_section_includes_inclined_waist_material

The cheap worker was disposed after recording this inventory. Root owns the final hash and full feature execution freeze.

## Dependency Diagnosis and Proposed Scope

The missing producer contains three tests: NumPy area/opening-cell checks, complete stair primitive/landing coverage, and all-storey authored primitive identity coverage. They call _wall_cells, _c.geometry and prims; they do not invoke a painter, font layout, text shaping or native frame rendering. The existing R11 preparation interface compiles unchanged AST methods with narrow data dependencies; it does not provide a general Manim runtime substitute. Existing axon/📏️geometry.py likewise extracts four exact primitive helpers for NumPy/GEOS tests.

Merely deferring fonts.configure_manim is insufficient: fonts additionally eagerly installs Text/Pango behavior and resolves bundled BODY_FONT. Drafting declares Drawing/Live subclasses of Manim VGroup; axon imports both drafting and core plus Manim. Blind config/Text/VGroup stubs would hide several independent runtime boundaries. No such stubs were introduced.

Recommended production scope, proposed to root before changes: isolate renderer-independent primitive assembly in the axon domain, retain real model geometry and membership readers, have both renderer and tests consume the same pure module, and leave shared energy typography untouched. An alternative ticket-only exact AST loader follows the authorized R11 interface but certifies extracted physical logic rather than an ordinary producer import. The root's decision is pending; no production file was changed by this diagnosis. Native font/layout/frame gates remain separate.

## Approved Production Separation and Green Collection

Root approved a scoped production separation, now implemented without Manim stubs or shared energy/font/framework edits. Actual assembly lives in axon/🧩️primitives.py, physical readers in core/📐️geometry.py, and pure coordinate/area measures in drafting/📏️geometry.py. Rendering providers explicitly import the same moved functions/types. Actual axon tests import the pure production module. Existing exact-source extraction consumers were updated together; no duplicate implementations or compatibility wrappers were added.

Fresh isolated actual load_tests collection now yields345 real tests, zero _FailedTest/_ErrorHolder, and no manim module imported. The cheap worker was disposed. This collection is not execution or a final-source identity assertion.

Execution of the three real axon contracts exposed two pre-existing failures behind the old import barrier: _build used nonexistent basement stage taxonomy for actual cellar slabs; _solid assumed exactly eight vertices and crashed for native triangular wall partitions. Root approved replacing the key with cellar and generalizing native polygon caps/sides. Four-corner coordinates/normals remain exactly equal. Portable triangle/pentagon/concave fixture volumes now match GEOS area×height and SciPy hulls for convex cases.

The three axon contracts, four physical geometry contracts, one material dispatch contract and15 orthographic frustum/stair regressions pass (23 focused tests). A same-geometry complete primitive comparison contains12,094 pieces and matches SHA25647d7fc5423191b885093846799472a20cf043e3d3bd767c0bededb8d38b96491 before/after movement with the two authorized bug fixes isolated in the baseline: old input cellar labels mapped to its old basement key, and corrected n-vertex _solid substituted to bypass the old crash. This is an explicitly normalized comparison, not a claim that the original broken whole-building producer ran successfully.

An additional28-test exact-definition consumer check passed24 and revealed four independent gates: two section tests need jsonschema in the test harness; exchange slab tests have stale opening areas and an overbroad ['path'] source rejection. Root/R11 were notified for their scope. No failing tests were omitted. Native text layout and rendered frames remain separate gates. Full final feature execution awaits root's final source/code freeze.

## Final Validator Preparation

Root added the real pinned jsonschema4.23.0 Pyodide wheel and dependencies through the existing test-only interface. Initial dependency traversal failed because the lock lists dependency jsonschema_specifications but keys the package jsonschema-specifications. The interface now normalizes package names to lowercase and collapses dash/underscore/dot runs before lookup; pinned hashes remain checked. Fresh isolated validator imports the real jsonschema4.23.0 successfully. No stubs, product runtime dependency or full-cohort execution were introduced. The prepared worker awaits root's final source/code hash.

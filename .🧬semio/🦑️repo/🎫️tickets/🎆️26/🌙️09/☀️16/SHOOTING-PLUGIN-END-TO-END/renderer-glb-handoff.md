# Renderer GLB Schema Handoff

The native renderer lost compilation at the asset probe transition from GLB semantic validation to materialization. The retained renderer build log reported two privacy errors at the probe call because the caller accessed `GlbSchemaCursor::output` and passed it to `GlbMaterializeCursor::new`, while `GlbSchemaOutput` was visible only within `glb_decode`.

The probe now consumes the validated cursor with `into_materializer`. This transfers the owned schema inside `glb_decode` without cloning it. The output field and materializer constructor both have the same module visibility as the output type. The probe still enters materialization only upon successful semantic validation. The conversion asserts that invariant in debug builds.

Scoped `repo tools exec git -- diff --check` passed for the three changed source files. The existing renderer async-boundary coverage includes `a_fetched_glb_becomes_the_resident_world_mesh_its_url_names` and `a_real_catalogued_glb_streams_through_the_surfaces_own_asset_lane_into_its_mesh_table`. The immediate native compile target is `semio-renderer-wgpu-asset-decode`. This handoff did not start a concurrent Cargo run.

The repository source closure scanner measured 11 files and 3,309 active lines for the asset-decode library, and 13 files and 3,397 active lines with test configuration. Both fit the native limits of 32 files and 4,000 active lines. The longest edited absolute path is 166 characters (229 UTF-8 bytes). The largest edited file is 33,570 bytes. The existing native renderer retry exercises this owner.

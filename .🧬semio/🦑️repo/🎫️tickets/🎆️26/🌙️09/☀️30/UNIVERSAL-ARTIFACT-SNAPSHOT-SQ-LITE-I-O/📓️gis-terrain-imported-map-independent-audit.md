# GIS Terrain Imported Map Independent Audit

Read-only production inspection; no Cargo, Nx, Git, runtime, or test execution. Root, s, and GIS AGENTS.md read. Only this report was created.

## Concrete Schema Decisions

- Keep `importedMap: Option<ImportedMap>` so absent and present-empty maps differ. Do not equate either with null records or empty nested objects. The current empty string is not a useful durable domain; malformed media should fail at the explicit import boundary rather than become a string carrier.
- Positions, routes, and regions must store complete ordered intrinsic object records, including unknown record members. A narrow `(id, lon, lat)` struct would lose real upstream fields. Preserve array occurrence order and repeated IDs; neither upstream producer nor terrain consumer prohibits duplicate IDs.
- Unknown root members belong to an ordered `IntrinsicMember` collection, with reserved `positions`, `routes`, and `regions` excluded to prevent competing representations. Define duplicate member names deliberately: current serde_json object admission collapses duplicates and its default map ordering is not source lexical ordering, whereas an intrinsic member array can represent more. Do not claim source JSON duplicate-key or lexical-order fidelity.
- If collection presence is semantically relevant, distinguish missing collection from explicit empty array. `MapDescriptorJson` treats missing collections as empty, but rejects explicit null. A three-Vec ImportedMap deliberately normalizes missing to empty; document that import rule and test it. Optional pin string fields accept missing and null with the renderer; retaining full object records preserves their distinction.
- Nine intrinsic kinds and exact full u64/i64/IEEE bits/raw bytes are a first-party snapshot domain. JSON media cannot express bytes, signed positive integer tagging, infinities, or NaN payload bits. The import/output boundary needs explicit admissibility or diagnostic rules; a serde_json Value roundtrip cannot prove these domains.

## Observed Real Admission and Consumption

`gismap/.../🧬️schema/🦀️.rs:252-283` imports only collection arrays, retaining entries whose `id` is a string, and stores the complete entry in MapFeature.data. It does not validate lon, lat, points, ring, label, or properties. Export uses the stored data directly and emits only positions/routes/regions at the root. Thus full record preservation is justified; a required coordinate schema at durable snapshot level would narrow upstream admission.

`framework/.../🗺️tiled-map/🦀️.rs:1507-1560` is a rendering projection: PositionData requires string id and f64 lon/lat; optional label, name, kind, icon, source_url accept missing/null, with sourceUrl alias in serde. RouteData requires id and Vec<[f64;2]> points; stroke_width defaults. RegionData requires id and ring. Unknown members are ignored. `sync_map_json:2691` inserts records into ID-keyed maps, losing repeated ID occurrences only in render state. No coordinate geographic-range, ring closure, minimum-point, or ID uniqueness constraints are declared. Do not import such constraints into durable storage from geometry conventions.

`gisterrain/.../✏️editor/🦀️.rs:593-602` currently accepts every Structured payload string without checking schema, JSON syntax, media type, or descriptor shape. This is an admission bug to replace, not a reason for an opaque fallback. MediaPayload itself is transport vocabulary in framework manifest; it need not become the durable imported map domain.

Terrain inference has two independently permissive projections. `💡️inferences/🦀️.rs:173-190` requires string id and numeric lon/lat to render pins, and uses only optional string label/icon. `💡️inferences/📦bounds/🦀️.rs:29-38` accepts numeric lon/lat without requiring id. Keep that deliberate distinction or explicitly resolve it; sharing a new decoder which requires id would change bounds/mesh behavior for currently stored object records lacking id. Both projections ignore routes/regions, but durable import must retain them.

## Mesh Identity and Ownership

`gisterrain/🦀️.rs:63-73` creates a mesh child handle from an initial content key; snapshot SQLite persists child_id and target independently. `gis_terrain_mesh_from_snapshot:82` derives placeholder quad bounds from imported positions but does not imply the persisted child handle should be rebuilt during decode or later imported-map mutations. Maintain independently authored child identity through exact projection/reconstruction; the proposed fixture's differing childId/targetArtifactId is useful and must not be normalized into equality.

## Current Fixture Review and Test Gaps

The new adjacent imported-map JSON fixture inspected contains full records, arbitrary Unicode/NUL names, unknown region property, raw bytes, u64 max, i64 min, IEEE signed zero/subnormal/max/infinity/NaNs, and independent mesh IDs. It is appropriate for the intrinsic durable domain, not a claim of JSON media admission. Keep JSON tag objects only as a language-neutral fixture representation, never a production carrier.

Its unsigned/signed schema currently validates decimal syntax but not 64-bit bounds, and permits signed negative zero. A typed fixture decoder must reject overflow and define canonical decimal spelling; schema validation alone cannot establish those assertions. The schema also does not validate record id/geometry shapes, intentionally consistent with a broad durable object domain.

Add language-neutral cases or explicit test construction for absent versus present-empty import, missing versus null members, repeated feature IDs, collection order and member order, empty object/array, integer boundaries in nested records, all nine kinds inside feature records as well as root properties, and reserved root collisions. Verify separate object/array ownership and occurrence ordinals in SQL; foreign keys alone do not establish single ownership or contiguous ordinals. Reject dangling values, unused variant rows, variant kind mismatches, unexpected multiple map owners, or sharing one value row between two intrinsic parents if the representation is an ownership tree.

Third-party SQLite querying/editing can verify relational exposure and independent reconstruction. Use bit words/decimal strings/byte hex as its oracle inputs, avoiding JS Number conversion for u64/i64 and NaN equivalence. No tests were run by this auditor.

# Original Visible Services and Historical Identity R21

## Result and Physical Scope

The model now declares one historical target: original-completed-1930-1931. It distinguishes original attestation, inferred reconstruction and unverified teaching envelopes. Later survey, restoration observation and proposal evidence remain distinguishable; a proposal or restoration observation alone cannot claim original presence. Bilingual labels and an explicit evidence policy are schema-validated. Services carry Pset_HistoricalEvidence with TargetState, EvidenceStatus, SourceReferences and ReconstructionUncertainty through information() and native STEP.

The old diameter-pipe implementation produced filled rectangular boxes and expanded every axis end in Z by its radius. It is replaced by actual hollow material sectors on straight X, Y and Z axes. Source profiles declare geometric diameter, material wall thickness and facet budget. The native service_geometry(key,model) interface exposes the same occupied physical parts used by quantities and all projections. There is no runtime library dependency, no fabricated solid centre and no arbitrary Z expansion. Nonorthogonal straight axes are explicitly rejected; sloping arbitrary network routes are not silently approximated by boxes.

## Primary Evidence Recovered

The [official indexed primary book](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1), p116, identifies insulated sheet-metal heating risers beside supports6/11 and a water riser sharing11; archival references include FLC H1-13 131,19439,19569,19570 and U1(14)050/052. Page117 explicitly calls40 cm boiler flue,13 cm soil stack and55 mm stair tube **diameters**, resolving the diameter-versus-section ambiguity. The soil stack serves two WCs. Page69 explains the enlarged25×25 cm buried stems2/4 by their internal8 cm rainwater drains. That wording justifies below-grade drainage within the stems, rather than claiming solid stems beneath a ground-level pipe cutoff.

No blocked full-book fetch was retried. Primary installation drawing pixels and complete roof-terminal footprints were not recovered. Existing later-observation and original-state distinctions remain explicit; modern absence is not evidence of original absence.

## Authored Original Service Families

- DP-1 and DP-2 now follow the actual axes of supports2/4. Their sourced nominal diameter is80 mm; geometric80 mm outer diameter,3 mm material wall and100 mm sleeve clearance remain inferred. The four arbitrary exposed perimeter drops and unsupported PS-1/2/3 distribution chain are removed. DR-1/2 are inferred hollow receiving collars aligned with the attested stacks, not a verified original roof-inlet schedule.
- HEAT-RISER-6/11-CASING and -INSULATION are native IfcCovering occurrences. Separate heating pipe occurrences and WATER-RISER-11 distinguish services from structural columns and rainwater. Original presence and adjacency are attested; casing180 mm diameter, exact offsets, insulation material/thickness and internal pipe positions are inferred. Only visible ground-storey portions are represented; no buried heating-distribution network is invented.
- SOIL-STAIR is a distinct nominal130 mm sanitary stack beside the spiral stair. BOILER-FLUE-VISIBLE is a separate IfcChimney with a nominal400 mm diameter and an explicitly bounded ground/first-storey run. Their exact plan coordinates, material wall thickness and unresolved connections remain inferred. The three original high chimney terminals are **not** assigned to unidentified CMN roof rectangles; terminal footprints/heights and complete branches remain unregistered.
- BL-1 is an explicitly inferred cellar heating-plant envelope at XY(6.8,14.7), with retained unregistered0.4×0.6×1.2 m dimensions. Its cellar placement is a declared inference from basement heating/fuel documentation, not a claim that plant drawing pixels were inspected. Floor support and clearance from all stair solids pass independent tests. No gas/condensing boiler type, exact original plant count or complete fuel connection is asserted.

## Real Penetrations and Below-Grade Continuity

Six native slab-host IfcOpeningElement occurrences provide actual material voids: OP-DP-1/2-FLOOR/ROOF, OP-FLUE-FLOOR and OP-SOIL-FLOOR. Pipes reference their penetrations without inventing IfcRelFillsElement relationships or violating the single-opening filling inverse. The100 mm rainwater sleeve intentionally exceeds inferred80 mm geometric OD.

The final rainwater outlet depth−0.60 m and0.30 m +X stubs are explicit inferences above the footings. Merely intersecting two annular pipes would block the other pipe's lumen. Instead, matching45° end planes join vertical and horizontal hollow sectors on one elliptical annular interface. Occupied material is disjoint on opposite sides of that plane, while the central air passage remains connected and empty. Each sector is a closed convex polyhedron with8 vertices and6 planar faces. Native IFCFACETEDBREP preserves these ends exactly.

The column author separately reconstructed the250 mm buried-stem C-notch over Z−0.65..−0.55, the vertical100 mm bore above it, and the330 mm finished column bore. No footing hole or complete collector network was invented. This services change preserves that source work and its distinction between finished envelope and actual reinforced-concrete core.

## Shared Physical API and Projections

The schema now admits explicit tube end planes and closed convex mesh parts. Public part_vertices, part_faces, part_outline_at, part_sections, part_regions, part_footprint, part_elevation, part_solid and part_body preserve actual planes and material. Closed-face validation rejects invalid indices, open shells, nonplanar faces and nonconvex solids. Existing column profile voids, planar section extrusions and tapered loft interfaces are preserved.

Plan cuts contain the actual material slice; service parts away from the plan plane are projected as light wire outlines, preventing lower half-shell silhouettes from masquerading as filled cut material. Section uses actual occupied cut polygons. Axon and all four elevations consume real faces, including horizontal tubes and inclined miter caps. Native service STEP uses correct XZ/YZ placements for horizontal sweeps and SolidModel/Brep representations for miters. The facade author's exact circle-void and hollow-column branches remain intact.

Source pipe profiles use96 facets. The largest new tube radius0.20 m has a maximum chord sagitta of approximately0.107 mm. Quantities and exported occupied material use the same faceted physical geometry. The circular sleeve budget was preserved; algorithmic slab broad-phase/partition improvements were implemented by the parent rather than reducing geometric fidelity.

## Verification

The original four portable service contracts failed before implementation. Final **12/12 portable tests pass in4.263 s**: eight model/schema/geometry gates and four production projection recorders. GEOS/Shapely independently verifies annular areas and hollow joint interfaces; SciPy convex hulls independently verify physical volumes, actual axis bounds and empty lumen samples. Historical-state conflict, unsupported proposal-as-presence, malformed shell and inferred cellar support cases are included. Production recorders cover X/Y/Z tubes and an actual mitered outlet in plan, section, axon and all four elevations.

Pinned independent web-ifc0.0.78 passes **16/16 English/German cases** for source downpipe, mitered outlet, casing, sanitary stack, boiler flue, cellar boiler and synthetic X/Y tubes. Native classes, localized names, historical property sets, real placed XYZ bounds and triangulated occupied volumes round-trip. Each tube has96 material solids/1152 triangles; the boiler has1 solid/12 triangles. Example independent volumes: DP-1=0.005221366924 m³ versus model0.005221367258; flue=0.406031006346 m³ versus model0.406030997861. These are material volumes, not filled bore envelopes.

Final complete building_geometry passes in18.715 s with17 service-geometry items. Complete source JSON Schema2020-12 check_schema and validation pass with0 errors. Metadata service-domain membership additionally includes HOIST-JIB and is not interchangeable with the class-selected geometry count or a full installation inventory. Native Manim rendering remains unverified because the repository launcher is blocked.

## Files and Reproduction

Product changes: model/🐍️.py, model/🔣️.json, model/🧬️schema/🔣️.schema.json, model/🔌️exchange/🐍️.py; the shared visual core plus plan, section and axon providers. Tests: model/🧪️tests/🚰️services.{json,py}; visuals/🏛️savoye/🧪️ortho-remodel/services.json and test_services_native.py; the shared recorder scope now binds the public model interface for native section/mesh faces.

Retained ticket inputs: native-services-r21.py exports fresh source samples; native-services-r21.mjs validates them through the independent IFC interface. Tool outputs and temporary performance diagnostics remain only under the ticket generated folder for parent cleanup. No checked-in source file was copied into the ticket. No shell bypass, modifying Git command, worktree, runtime external library or AGENTS edit was used.

This is a completed physical/API change for the explicitly bounded visible services and declared historical identity. Full original installation routing, exact plant layout, original roof-terminal profiles, hidden heating bundles and detailed inlet/outlet schedules remain source-registration work, not falsely completed inventory.

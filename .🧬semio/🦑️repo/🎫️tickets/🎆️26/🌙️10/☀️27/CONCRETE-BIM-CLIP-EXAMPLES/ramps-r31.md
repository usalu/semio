# Native Ramp and Ramp Flight Geometry R31

Implemented shared physical ramp/flight geometry, all seven public product representations, actual native IFC product export and single-owner landing material. The449source products retain their positions, GUIDs, names, nominal slab thicknesses, flight paths and landing outlines. No speculative source relocation or normal-thickness reinterpretation was made.

## Evidence and Thickness Convention

Read ramp-representation-gap-r30.md, the source _flights/quantities implementation, primary2022article§8, complete Annex5 longitudinal section pixels, and recovered/visually inspected originalFigure13/FLC-H1-13-305 pixels from author-r8.pdf page15. The embedded original detail is502×491pixels. It shows plan, CoupeA-B and reinforcement details; the available resolution does not resolve a confident normal-versus-vertical6cm dimension leader. The article establishes a6cm floor slab, and separately a6cm central scissor-shaped screen, without specifying the floor thickness measurement direction.

The existing6cm vertical offset is therefore retained as an explicit inferred modeling convention. Source Ramp and RampFlight geometry now carries thickness_direction=vertical; its Pset_GeometryConvention expressly distinguishes this direction inference from the sourced nominal6cm dimension. No claim is made that the original6cm dimension has been measured perpendicular or vertical from the recovered pixels. Details and bounded source-reader inputs are recorded in ramp-evidence-r31.md and ramp-r31-source.cjs.

## Shared Public Geometry Contract

Public ramp_geometry(key,model) supports actual IfcRamp and IfcRampFlight products. Flights use closed native convex polyhedra with real inclined upper and lower faces, plan width normal to the source path and the declared vertical thickness offset. Arbitrary non-axis-aligned straight paths are supported; no axis-aligned proxy or rectangular-envelope approximation is used.

For a parent ramp, parts contains only its owned landing bodies; child_parts contains real flight slabs with child owner tag/GUID; shared_parts contains a borrowed landing with its actual owner GUID and source component tag; full_parts contains the complete selected assembly. Flights expose their own slab in parts/full_parts. Children retain native IfcRampFlight identities. Returned flight and landing descriptors also carry these same physical parts for drawing consumers.

The source described the same first-floor landing twice: RM1's top and RM2's bottom. Explicit landing owner and component tag fields now assign that physical LANDING-FIRST slab to RM1 while preserving its participation in both full assemblies. Borrowed landing references must match the owner's registered outline, thickness and actual elevation; drift fails. Invalid thickness conventions or nonnative flight parents fail.

Public plan, section, elevation,3d,axis,footprint and body representations consume full_parts deliberately. Their derived.scope distinguishes assembly from product, and primitive owner/tag metadata preserves material ownership. Native source axes are3D paths. Mesh representations declare SolidModel and expose true BRep faces rather than flattening the underside.

quantities.volume is the net complete assembly/product volume. exclusive_volume, shared_volume and child_volume partition that selected semantic volume. material_quantities, weight and mass describe only owned material. Full parent volumes therefore must not be added to their child inventories. Each parent full assembly is1.677354m³; RM1 owns0.430458m³ of landings, RM2 owns0.314118m³ and references the0.11634m³ first-floor landing. Each of four flights owns0.623448m³. The nine distinct owned solids together contain3.238368m³.

## Production Consumers

building_geometry now derives ramp descriptors from the shared helper. Actual axon, plan and section/elevation callbacks consume its physical flight/landing parts. The former private width/normal/upper-face reconstruction and axis-dependent section approximation were removed. The shared first-floor landing is drawn once, retaining parent selection aliases. Ground landing rendering uses its actual slab body rather than a substitute plane.

Vehicle obstacles now enumerate each ramp's owned parts plus child_parts once through the same interface. The old axis-aligned-only private flight reconstruction is removed. Borrowed landing material is not duplicated in obstacle records. Generic polyhedron height-band projection supplies actual inclined occupancy, including diagonal flights.

## Native IFC and Selected Fragment Scope

STEP export supports actual IfcRamp and IfcRampFlight class/GUID products. A selected parent owns only its landing Body and aggregates actual child flight products through the authored IfcRelAggregates GUID/name. Child bodies are independent native BReps; they are never copied into the parent Body. Scoped excerpt IDs, entity locations and token spans include the real child fragments.

Pset_MaterialOwnership states exclusive_owned_material Body scope, full selected assembly/product scope, full/owned/shared/child volumes, explicit external shared landing owner-GUID/component references, vertical thickness direction, inferred direction basis and nominal slab thickness. RM2's selected fragment contains its two owned landings and two child flights; the externally owned first-floor landing remains an explicit reference. These are selected-fragment inventory semantics, not a claim that the exporter now writes a normalized entire building.

Independent pinned web-ifc0.0.78 decoded14samples: all six source products in English/German, plus a diagonal synthetic flight in both languages. It verified source class/GUID/localized name, authored aggregation identity and child product references, unique body counts, ownership quantities/conventions, actual placed bounds and mesh volumes. Source parent excerpts contain5owned solids for RM1 and4for RM2, including their two child flight bodies. Selected flights contain one body each.

|Selected native fragment|Owned bodies including children|Native inventory m³|API selected inventory m³|
|---|---:|---:|---:|
|RM1|5|1.677354149185|1.677354|
|RM2|4|1.561014154568|1.561014|
|Each source flight|1|0.623448078112|0.623448|
|Diagonal synthetic flight|1|2.000000054836|2.000000|

The larger RM2 full semantic assembly volume1.677354m³ includes the separately referenced0.11634m³ landing; native fragment material therefore correctly totals1.561014m³. Maximum assembled reader volume error is approximately1.55e-7m³. No proxy class or adapter product was introduced.

## Test-First Verification

Schema and portable 🛝️ramps.json were authored before implementation. The first three tests failed for missing public ramp geometry and unsupported native IfcRamp export. Final six ramp contracts pass in5.664s. Independent SciPy ConvexHull verifies source and diagonal volumes and true sloping lower faces; independently intersected hull edges plus GEOS verify diagonal horizontal and vertical cuts. A5m run,2m width,0.2m vertical thickness synthetic flight has2m³ volume.

Independent linear programming and SciPy HalfspaceIntersection examine every pair of the nine owned source solids. They prove no positive shared flight/landing material, including both flight endpoints and the formerly duplicated inter-ramp landing. The source paths meet landing boundary faces; no positive endpoint joint material needed an additional partition. Summed distinct owned volume is3.238368m³.

Actual production callback tests execute the real axon _build, plan _plan_ramps, section _sec_ramps and vehicle _vehicle_parts functions with geometry-recording drawing substitutes. They confirm nine owned axon solids, two upper flight footprints, two first-storey landing outlines, seven actual longitudinal section cuts and nine unique vehicle records/four native flight products. These are callback geometry proofs, not native rendered frames.

Final combined regression: six ramp contracts,35joint contracts and the two existing beam contracts all pass,43tests in26.304s. All five modified production Python modules compile. Full repository cohort, source-schema integration and analytical snapshot consistency remain parent/clips_r11 integration gates. No native Manim frame was claimed; its existing repository launcher gate remains unresolved.

## Modified Files and Handoff

Product files created or updated:

- CLIP/🧱️model/🐍️.py: ramp_geometry/_ramp_geometry, ramp quantities/representations, building ramp descriptors, owner metadata propagation and vehicle ramp records.
- CLIP/🧱️model/🔣️.json: six ramp/flight convention records; explicit landing owner/component tags. No metric/position/identity changes.
- CLIP/🧱️model/🧬️schema/🔣️.schema.json: thickness direction and landing owner/tag fields.
- CLIP/🧱️model/🔌️exchange/🐍️.py: native class routing, scoped ownership properties and real child aggregation.
- CLIP/🧱️model/🧪️tests/🛝️ramps.json and🛝️ramps.py: portable cases and six contracts.
- CLIP/🎨️visuals/🏛️savoye/🧊️axon/🐍️.py,📋️plan/🐍️.py and✂️section/🐍️.py: shared physical ramp consumers.

Durable ticket inputs/reports: ramp-evidence-r31.md, ramp-r31-source.cjs, ramps-r31-native.py, ramps-r31-native.mjs and this report. Native sample outputs, source retrieval images and reader dependencies remain under 🗑️generated for parent cleanup. Main standard load_tests registration is owned by clips_r11 and was requested directly. Parent was notified of final source/API/exchange/drawing freeze.

No shell command, Git mutation, worktree, IDE, AGENTS edit, checked-in source copy, migration or external runtime library was introduced. Remaining historical uncertainty is precise original slab thickness direction and the previously existing source registration precision; these tests certify the authored reconstruction and material ownership, not new historic measurement.

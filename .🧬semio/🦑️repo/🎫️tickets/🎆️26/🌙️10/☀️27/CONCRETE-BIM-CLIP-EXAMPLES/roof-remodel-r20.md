# Solarium Roof Remodel R20

## Physical Source

SC-1 is now a continuous asymmetric piecewise cubic wall, following CMN2021 native path6707 rather than two equal semicircles. The drawing's missing subpath interval is the landscape aperture at plan-cut height, with actual occupied wall below and above. OP-SC-VIEW is an IfcOpeningElement hosted by SC-1 through an IfcRelVoidsElement relation. Its cumulative path stations physically remove the full wall thickness from +7.250 to +8.475 m. The wall body reaches +9.105 m; a separately occupied sloping cap reaches +9.325 m. The source building maximum was corrected from assumed9.40 to sourced9.325 m; roof slab datums were preserved.

SC-SILL and SC-HEAD are native reinforced-concrete IfcBeam members. Their inferred physical envelopes include the projecting sill shelf and aperture-head reinforcement visible in the primary section and official owner photograph. The screen body subtracts their occupied solids through the existing generic ownership interface, retaining their material identities and avoiding double ownership. No unsupported continuous concrete core was invented for the whole rendered screen.

## Primary Evidence and Registration

- [CMN2021 visitor document](https://www.villa-savoye.fr/content/download/9824393/file/document-visite-villa-savoye-FR-2021.pdf?inLanguage=fre-FR&version=3), page2, path6707 supplies the native asymmetric cubic contour and candidate jamb ends. Section18 identifies the landscape opening. The [official CMN garden photograph](https://www.villa-savoye.fr/decouvrir/un-jardin-suspendu) confirms a framed aperture with occupied material beneath and above it. The earlier circulation-gap interpretation is explicitly superseded by roof-aperture-topology-correction-r20.md.
- CSIC2022 DOI10.3989/ic.88625 annex5, locally inspected as register-r9-section22.png, shows printed +9.325 highest crest, +9.105 inner cap edge, +8.475 window head and +7.25 projecting sill. The +9.335 reading in older notes was a transcription error. Both upper and lower longitudinal sections attest the screen height; the lower section crosses the aperture.
- The first-floor structural columns were extracted from the actual CMN native vector operations4479–4540, independently of roof-plan dots. Repeated X centres642.7582,672.2826,701.8069,731.5392,760.8555 span the source19.00 m reference-grid extent; Y centres411.6165..530.1296 span19.00 m. Registration scales are6.215647/6.237532 PDF points per metre. Small authored column offsets remain visible in the drawing; a perfectly regular fictitious dot grid was not substituted.
- The19.30 ×21.60 m structural slab perimeter is a separate check on registration. It differs slightly from the illustrative perimeter. XY registration, traced jambs and thickness retain approximately0.15 m illustration uncertainty; native PDF decimal precision is not surveyed metric precision. Roof-plan dots remain unidentified objects, not new columns.

The0.20 m rendered wall envelope,1 mm finite cap crest, reinforcement depths and sill/head profiles are explicitly inferred. The finite crest is a narrow positive profile for a sloping native loft; its geometry is independently verified. It does not imply a surveyed1 mm historical arris. The ruled cap surfaces remain fully physical, rather than a flat9.325 m wall band.

## Shared Contract and Projections

The schema now admits native four-control cubic Bezier outline items and a typed wall cap contract. The stdlib-only path_wall_geometry(model,wall) API expands the path deterministically, constructs non-overlapping mitered bands, splits them at cumulative aperture stations and height datums, subtracts owned material, and returns the actual occupied parts plus removed opening solids. Invalid stations, self-overlapping bands and overlapping apertures are rejected. The opening's native3D polygons have actual thickness, replacing the former degenerate two-point polygons.

All seven representation modes for both wall and opening derive from those physical solids. Plan, section, four elevations and axon providers now consume the shared parts; they no longer reconstruct overlapping screen-segment boxes or a closed unperforated band. Concave material notches created by concrete ownership retain true interval sections. A dedicated solarium_detail draws the native aperture cut, shelf/head and sloping crest; source objects remain addressable by tag. Numeric annotations use the scene language infrastructure.

Native STEP now dispatches IfcWall screens as well as ordinary walls and preserves every occupied part. The cap is exported as actual IFCEXTRUDEDAREASOLIDTAPERED entities with positive corresponding end profiles. Existing facade product/type/material-aspect behavior was preserved. This is native wall-body export with a physical aperture; exporting a complete host/opening relationship context is outside the one-product excerpt API and is not claimed.

## Verification

The initial four language-neutral path-wall checks failed before implementation. The implemented contract passed all ten checks (5.788 s): GEOS miter-buffer/difference equality, full-thickness aperture3D solids, invalid station rejection, source-bounded aperture/cap levels, independent Shapely material-ownership checks and SciPy convex-hull loft volume, plus actual production plan, section, axon and detail recorders and the complete elevation provider for all four faces of the source aperture wall/cap band. A whole-screen elevation-area union was unsuitable as a test oracle because hundreds of coincident projected faces stalled GEOS union; the focused aperture-band recorder passed and native full-screen triangulation separately verified every occupied solid. The GEOS area oracle excludes zero-area projected edge polygons.

Pinned independent web-ifc0.0.78 read both English and German native STEP excerpts successfully:670 occupied solids,165 genuine tapered cap solids and8040 triangles. Triangulated volume13.412938989102866 m³ agrees with model13.412938988219171 m³. Native bounds agree to1e-5 m, including the +9.325 m crest. Full source JSON Schema2020-12 validation passed with0 errors using pinned jsonschema4.23.0. Final full building_geometry passed in7.789 s with670 occupied screen parts; final full source schema also passed after the generic ownership and concurrent cellar-section changes.

Native Manim rendering remains unverified because the repo launcher registry is blocked. Production projection recorders and independent native IFC geometry checks are recorded separately from historical fidelity.

## Solarium Walking-Floor Boundary Audit

SP-SOLARIUM's existing102.159 m² polygon is a provisional broad solarium-and-approach planning region. It does not trace the wind-wall inner face and must not be presented as a measured70 m² enclosure. Its bilingual source name and note now make that distinction explicit. The primary paving diagram shows separate circulation/maintenance regions, but a complete registered closed walking-floor boundary has not been established. The source polygon was therefore preserved rather than resized to force a historical area total. Historical70 m² solarium and110 m² hanging-garden figures remain approximate references. Floor-zone interpretation, unnamed roof vents/rooflights and the detailed stair-arrival paving still require further source tracing; they are not covered by the completed screen/aperture remodel.

## Changed Files

- model/🐍️.py; model/🔣️.json; model/🧬️schema/🔣️.schema.json
- model/🔌️exchange/🐍️.py
- model/🧪️tests/🌀️screens.json; model/🧪️tests/🌀️screens.py
- visuals/🏛️savoye/📋️plan/🐍️.py; ✂️section/🐍️.py; 🧊️axon/🐍️.py; 🔩️detail/🐍️.py
- visuals/🏛️savoye/🧪️ortho-remodel/screens.json; test_screens.py
- ticket/native-roof-r20.mjs; ticket/roof-remodel-r20.md

All edits used fresh shared files; no worktree, modifying Git command, source copy, runtime external dependency or shell bypass was introduced. Ticket-generated native samples and diagnostic output stay under the ticket's generated folder for the parent final cleanup. Input verification scripts and this audit are retained.

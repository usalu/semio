# Garden Post Zones and Concave Weather Cap R28

Implemented physical source-bound silhouettes for posts16/22 and the garden weather cap. Source now contains449elements, one more than the prior448 cohort. The added GARDEN-WEATHER-CAP is a separate architectural IfcBuildingElementProxy with kind=weathering_cap,16native convex section sweeps, concrete core and separate nominal upper render. Existing primary garden beam extents6.19–6.51m are preserved.

## Primary Source and Registration

Read physical-architecture-audit-r24.md, facade-structure-r23.md, roof-source-gaps-r19.md, source-audit-r17.md and source-materials-r10.md. Visually inspected original figure10-img_p13_1.png (FLC-H1-13-310,502×701pixels), its article text, and complete register-r9-section23.png (Annex6,2448×3168). Source crops were generated only to inspect existing pixels; enlargement recovers no missing precision.

[Zaparaín Hernández et al.,2022, DOI10.3989/ic.88625](https://doi.org/10.3989/ic.88625), §6.2/Figure10 explicitly identifies three functional sections: parapet engagement, exposed aerodynamic portion and cornice support. Figure10 displays the exposed teardrop, a separately flattened/winged parapet junction and a different upper outline with a flat base and arched head. The source distinguishes a14cm proposal, handwritten12cm reduction and the later measured15cm finished exposed width. The proposed12cm outline is not used as an executed concrete core.

Annex6 locates garden posts16/22 at the left edge of the two transverse sections and supplies sill+4.06m, head+5.21m and upper weathering+6.77m. The cap retrieval region on the full raster is approximately X232..269,Y680..704pixels. Its upper edge dips concavely between raised edges; its underside is stepped, with a projecting inner tongue. It is visibly a different silhouette from the primary rectangular beam. The original low-resolution Figure10 remains necessary for profile family identity; the later Annex6 reconstruction supplies dimensioned registration. Neither is treated as a newly recovered precise original shop drawing.

## Three Physical Post Zones

Both existing post occurrences now use native geometry.sections with explicit functional role, profile and vertical interval. The schema permits the role field, and column_geometry carries role/material into shared physical parts and native IFC constituent aspects.

| Role | Physical interval | Evidence boundary |
|---|---|---|
| Parapet engagement |3.14–4.06m | Lower winged/flattened silhouette follows Figure10; its footprint dimensions are inferred. |
| Exposed |4.06–5.21m | Existing traced approximately23cm longitudinal contour retained;15cm finished narrow width sourced. |
| Cornice support |5.21–6.34m | Distinct flat-base arched outline follows Figure10; metric outline inferred. |

The4.06/5.21m source dimensions are sill/head datums. Interpreting them as functional section transitions is explicitly an inference; no claim is made to original measured transition levels. Lower overall wing width28cm and upper approximately18×16.5cm footprint are inferred silhouette reconstruction, not sourced exact dimensions. The former uniformly extruded15cm contour no longer extends through all three zones.

TY-AERO15-TRACE now denotes the finished three-zone post family and uses existing MAT-RENDERED, a rendered-concrete composite with assumed24kN/m³ unit weight. This is a finished geometric envelope; unknown concrete/render boundaries are not silently replaced by a12cm design core. Column section_scope remains authored_geometry_net_of_voids. The analytical frame member already reports stiffness_basis=not_assessed, so the geometrical second moments do not assert validated concrete-core stiffness. Both post occurrences join the architectural model as well as the structural model.

## Weather Cap Construction

The cap follows actual W-GARDEN-E span Y6.081–15.5m, rather than extending into the adjacent salon band. Its registered XZ envelope is18.74..19.10m and6.51..6.77m. Top-edge trace points are6.770,6.762,6.754,6.751,6.752,6.756,6.765,6.770m across its width. The stepped underside preserves the projecting inner tongue and deeper outer leg. These intermediate coordinates and the horizontal position are inferred, with approximately3cm section-reading uncertainty. Setting its lower contact to the existing6.51m beam top is an inferred construction registration, distinct from the sourced6.77m weathering leader. Exact corner returns remain unresolved.

The concave/stepped section is decomposed into8disjoint vertical strips, each with concrete core and a nominal15mm upper render part, for16convex native XZ sweeps. The top render allocation is explicitly inferred, rather than claiming a measured historic finish section. It retains the concave upper surface instead of filling its convex hull. The cap and primary beam share a boundary plane without sharing occupied volume. No32cm beam is shifted upward to force the6.77m leader.

A test of the actual production roof-plan callback caught the first storey assignment hiding this architectural cap. It now belongs to the roof storey and REL-CON-roof, while its independent base_level remains first for the authored local coordinates. Actual roof plan projection covers precisely the intended footprint. The part material ownership introduced in R27 is used by production plan, section/elevation, axon and native IFC.

## Support Graph Correction

The parent found two stale pre-split support relations: REL-SUPPORT-16-FIRST and22-FIRST still pointed only to W-E, which now ends atY6.081m. A portable identity contract failed before correcting them. Post16 now names W-GARDEN-E, its garden L lintel and primary6–22beam. Post22 names the garden/kiosk walls, their two L lintels and primary6–22/22–29beams at their actual common junction. This is explicitly named inferred support mapping after the facade split. No global W-E replacement was made. Existing six hosted-service relationships and bore/C-outlet geometry were preserved.

## Verification

Language-neutral 🌬️garden.json and Python oracle were written first. Initial tests failed for the missing cap and the single repeated post profile. Five final tests pass in12.926s: independent NumPy profile areas and GEOS three distinct source footprints; full cap section union against the independently authored silhouette; SciPy convex-hull volume for every convex material part; pairwise material nonoverlap; retained beam extents and no cap/beam double ownership; actual production roof-plan callback footprint; exact production inclined section/face geometry; support identity registration; and material junction ownership.

The junction check independently finds no occupied overlap between either three-zone post and65garden auxiliary physical parts, or any of4196garden-wall material leaf pieces. Initial uncached preparation took6.095s for auxiliary ownership and12.3s for garden wall leaves in this validator session. The final5-test suite used the shared caches where available; its runtime is not a native renderer benchmark.

Full source JSON Schema2020-12 validation with jsonschema4.23.0 passed with0errors after source publication and hosted-service integration. Selected final native IFC samples were regenerated after correcting roof containment. Independent pinned web-ifc0.0.78 verified six product/language combinations:

| Product | Placed solids | Triangles | Native volume m³ | API volume m³ |
|---|---:|---:|---:|---:|
|16-FIRST, each language |3 |152 |0.097614740843 |0.097614738412 |
|22-FIRST, each language |3 |152 |0.097614740843 |0.097614738412 |
|GARDEN-WEATHER-CAP, each language |16 |192 |0.572910648233 |0.572910675000 |

Actual placed bounds agree within1e−5m. Native material constituent roles, localized material names, exact shape-aspect partition and historical state/source/uncertainty properties all passed independent decoding. The concave lower column footprint is preserved by native triangulation. These checks certify the authored reconstruction geometry, not new historic measurement precision.

## Wall Piece Counts and Phase Costs

Requested per-wall counts after R28, measured with warm shared ownership/cache state (one API wall call each):

| Wall | Occupied pieces | Seconds |
|---|---:|---:|
|W-GARDEN-E |4196 |0.396 |
|W-KIOSK-E |117 |0.333 |
|W-E |56 |0.353 |
|W-S |121 |0.408 |
|W-N |121 |0.440 |
|W-W |185 |0.338 |

The4196garden pieces are a real fragmentation cost of current exact ownership subtraction. No performance coalescence or silhouette simplification was attempted in this bounded physical remodel. Parent may profile an exact coalescence separately; current passing geometry must be preserved.

## Artifacts and Changed Files

The inspected generated diagram garden-register-r28.png/.svg juxtaposes the original source retrieval crops and actual authored sections. It is a geometry verification diagram, not a native Manim frame. It and native samples/results remain under 🗑️generated for parent final cleanup. Durable verification inputs are garden-r28-author.mjs and garden-r28-native.py; the native input reuses rooflight-r27-native.mjs/native-services-r21.mjs as a tool-independent independent reader interface. No runtime external library was added.

Changed product files: CLIP/🧱️model/🔣️.json; model/🐍️.py (column section role/material propagation only in this round); model/🧬️schema/🔣️.schema.json (section role); model/🧪️tests/🌬️garden.json and 🌬️garden.py. Main test discovery registration is owned by clips_r11. The parent owns ticket lifecycle, source count/caption final checks, analytical-link refresh and generated cleanup.

All source edits used fresh reads and narrow object replacements. The final source write used a closed r+ handle because Windows rejected truncate-style writeFile opens; no file remains held. No shell, Git mutation, worktree, IDE, AGENTS edit, source-file copy or migration was used. Remaining unregistered specifics are precise lower/upper profile measurements, original transition heights, core/render thicknesses, cap finish composition and corner returns. Native Manim fidelity remains blocked by the existing launcher/registry issue.

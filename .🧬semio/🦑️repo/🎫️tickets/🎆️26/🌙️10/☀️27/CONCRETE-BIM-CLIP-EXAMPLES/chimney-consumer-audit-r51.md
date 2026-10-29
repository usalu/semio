# Composite Chimney Consumer Audit R51

Read-only source audit,2026-10-29. No source writes or source copies. Inspected100Python/TypeScript files under the BIM tree, including all six chapter files and plan/section/elevation/axon consumers. Candidate R50 occupancy remains pending. Line numbers refer to the inspected concurrent source state and should be refreshed after R40 edits.

## Concrete Generic-Service Assumptions

In 🎬️clip/🧱️model/🐍️.py:1242, _service unconditionally reads e.geometry.points before considering geometry. Existing native service types are points-backed, but new schema-first composite prism chimneys deliberately provide generic parts. Without a composite branch this raises KeyError before all public consumers. Lines1292–1293 also reject services unless tube/size has generated occupied parts. R40 owns this producer extension.

In the same file:2326–2329, the public represent branch for IfcChimney and other services unconditionally derives cut planes and axis from g.points. A composite producer that only adds g.parts still fails here. Derive cut planes from occupied bounds, and expose a genuine authored axis only if present or explicitly derive a meaningful vertical axis for the composite family. Do not fake tube geometry merely to satisfy this branch.

At2044–2046, quantities already dispatches generic non-tube service volume through occupied parts rather than requiring solid_volume. For the proposed vertical prism parts, polygon area×height is valid. No public consumer requiring solid_volume from services was found; occurrences of that key belong to wall geometry. Composite parts containing future meshes would require the generic occupied-volume function, but that hypothetical extension is not a demonstrated blocker for these vertical prisms.

Main cohort contract 🎬️clip/🧪️tests/🐍️.py:905–909 loops every service.points after checking parts. It must check actual part vertices/bounds for every service, retaining explicit pipe endpoint checks at897–904 for their named rainwater/boiler cases. Otherwise a valid composite service is rejected by a tube-shaped test assumption.

## Public Scene Defects

🎬️clip/🧩️models/🐍️.py:327 labels the entire IfcChimney class as visible boiler-flue trunk in English/German. Adding salon and kitchen chimneys makes this classification text false. Generalize the class meaning to modeled chimney/flue bodies and preserve boiler-specific evidence only for the boiler tag. A bilingual fixture should verify the class description can describe the new identities without assigning their functions to the boiler.

The same chapter's _schema at557–560 hardcodes a class set omitting IfcChimney, IfcCovering, IfcDoor and other existing classes, then requires equality against the union of all actual discipline counts. This is a preexisting public schema-beat failure that also blocks a chimney-inclusive cohort. The chapter runs BEATS dynamically at130–134. Derive ordered classes from current counts or a complete explicit class catalogue; do not merely append the new two tags. A fixture should assert matrix class rows exactly equal the current model-class union including chimneys.

## Paths Already Generic

Plan _plan_services at490–514 iterates data.parts and uses native per-part slice outlines. Section _sec_services at413–435 iterates parts and native projections. Elevation581–585 calls add_loft per service part. Axon _service_prims317–335 iterates generic occupied parts, preserving section/mesh faces and vertical bands. These routes contain no universal points/tube requirement; retain them and test the new ring rather than adding cylindrical substitutes.

The models chapter points accesses at150–156/755 are constrained to a DP-1 collision scenario;539 filters kind downpipe before reading points. They are not universal chimney defects. Representations chapter points accesses at614/628/1117 operate explicit wall representations, not a service collection. The six chapters share the canonical geometry consumers; no additional universal service-points access was established in the other five chapter files.

## Proposed Meaningful Fixture

Use a language-agnostic generic chimney fixture with no points and no tube, outer square0.35m and square bore0.15m,12disjoint layered prism parts. Assert body volume0.1000×height, plan occupied area0.1000m² and bore center absent from the horizontal slice. Compare independent polygon union/difference using existing GEOS and compare native IFC material volume through the existing independent reader. Exercise public plan, section, elevation and axon actual part paths plus API quantities and representations. A generic-parts fixture must not silently add points to evade the exact uncovered failure. Historical dimensions and morphology remain separately qualified in R48/R50.

## Nested Material Closure Follow-Up

Read current published source and actual66-part roof-door export. All emitted per-part material tags resolve in the model materials registry: roof-door MAT-METAL,MAT-TEXTURED-GLASS,MAT-PLYWOOD,MAT-WOOD; prismatic chimney MAT-RENDER,MAT-HOLLOW-CLAY,MAT-PLASTER. No missing material reference was found. The layered timber leaf is resolved into skins and perimeter battens before native export, so no air solid or unresolved LS-DOOR-LEAF part reaches the66occupied bodies.

partial_model at current model.py2320–2332 returns discipline metadata, tags, GUIDs and class counts; it is not a standalone source/material subset and has no material-closure export to repair. Native exchange _constituents164–172 groups actual body part material/role; lines280–281 gathers every constituent material;467–468 resolves/emits each corresponding IFCMATERIAL. Layered non-constituent exports separately resolve each layer material at473–474. Thus it does not rely only on element.material for these new multipart products. No concrete missing nested-material reference or class/name defect was established in this follow-up. Root independently reports native SDK verification for the final products; this source audit is not a substitute for it.

# Horizontal Host Openings R19

The source-attested garage lifting hatch and cellar coal projection require actual horizontal slab apertures. The previous schema and API assumed every IfcOpeningElement had a vertical wall host. The new schema distinguishes a wall host from a slab host: exactly one of geometry.wall or geometry.slab is required; slab hosts additionally require an XY outline. Existing wall geometry remains its own domain contract. No compatibility path or runtime library was added.

A slab-host opening resolves its removed solid from each actual structural region, including the regional Z interval. The host structural partition and derived finish parts subtract the aperture. The host cache includes its native opening occurrences, so an outline edit invalidates the derived slab. Opening quantities retain the host, area, removed structural volume, and regional depths. All seven representation modes use those regional removed solids. A hole outside its host, or one overlapping another named or geometric hole, is rejected.

## Test-Driven Evidence

Portable fixture: model/tests/🕳️openings.json. Before implementation, all four initial checks failed: no aperture subtraction, unchanged cache, missing wall-head attributes, and no boundary/overlap validation. The implemented four geometric checks pass against independent GEOS/Shapely areas and difference unions. The fixture spans two different slab depths: 0.20m and 0.30m; removed area is 2m² and volume 0.50m³. All five checks, including full synthetic-model schema validation, pass. Schema validation is checked with pinned jsonschema4.23.0 in the portable interpreter; it also exposed the unrelated facade dimensionless-unit omission, now corrected by the facade author.

Source JSON remains held at the facade validation cohort while the chapter author runs the full baseline. No cellar projection, jib, hopper, or cover has been published by this API change. Their traced footprints and evidence are in [hoist-projections-r17.md](hoist-projections-r17.md) and its input JSON. Native IFC opening relationships and operable-cover exchange remain to be implemented; seven geometric representation modes alone are not a full native export.

## Projection Authoring Requirements

Extend actual cellar floor and ceiling outlines into the two registered annex projections; preserve the existing narrow cellar rectangle elsewhere. Use disjoint occupied regions, real three-sided shaft construction, and a connected wall aperture in W-CELLAR-E/W-CELLAR-W. The east clear hatch must remain physically open through its slab host when the separate cover is hidden; the west hopper must connect to the cellar and have an inclined physical chute, not a solid filled block. Preserve existing stair and rib geometry; no enlargement to satisfy car poses. The east projection's opening spans beyond the old ceiling boundary, so adding a hole without extending its host is deliberately invalid.

Plan-traced dimensions have approximately0.04m uncertainty. Exact extension floor levels, cover/wall thickness, materials actually built, jib pivot and arm sections are unresolved. Any reconstruction choice must carry inferred quantity or metadata provenance and must not be labelled measured as-built. Material-interface and car/door/column clearance checks are required after publication.

## Native Verification Limit

Current 374-element geometry completes in7.22s in the portable CPython verifier. Native rendering is still unverified: the installed repo launcher registry is rejected, and native repo test cannot resolve this teaching bundle. Additional primary-book searches and direct official PDF opening on2026-10-28 yielded no usable new dimensions; the PDF fetch timed out.


## R20 Shared Source Validation

After the roof source grew to377 elements, an independent full Draft202012 schema check and all5 native slab-opening contracts passed together. The opening suite completed in21.00s. The positive regional fixture still uses independent GEOS area/union/volume checks; no source hatch occurrence is claimed until the cellar author publishes its hollow construction.

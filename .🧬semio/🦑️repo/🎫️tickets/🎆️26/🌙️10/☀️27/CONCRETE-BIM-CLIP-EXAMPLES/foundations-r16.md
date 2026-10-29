# Exterior Foundations R16

## Evidence and Registration

The foundation groups are traced from CSIC2022 author PDF Annex1, local generated/archive-r8-page18.png. Mapping is x=(826-px)19/734 and y=1.25+(py-225)19/735 metres. The trace input preserves original pixel bounds, an approximate0.04m reading tolerance and separate outer blinding and inner RC-base outlines. It is an inferred image registration rather than a measured footing schedule.

The plan shows21 independent groups serving22 numbered columns:1–7,11,12,16,17,21–24,27–33. Columns7/12 share one elongated blinding footprint and a combined footing product with two separated RC bases/transitions; no undocumented tie beam is added. Eleven remaining numbered columns8,9,10,13,14,15,18,19,20,25,26 retain their cellar support paths.

[Primary book, printedp69](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1) describes lime-concrete preparation35–45cm thick,10cm reinforced bases, pyramidal transitions,22cm square buried stems except25cm at drainage columns2/4, and the stem transition at−0.30m. These dimensions support the physical component breakdown. The proposed16×25cm masonry plinths are explicitly a contractor suggestion in that passage; their execution is not established here and they are not authored as built.

## Datum Conflict and Inferences

High-resolution Annex5, generated/register-r9-section22.png, explicitly labels the exterior preparation bottom−1.65m. This implementation follows that plotted world datum. The book states firm ground−1.50m. Its possible relation to terrain−0.15m is an unverified interpretation, not a demonstrated reconciliation. The source pset records both references.

A nominal0.40m preparation thickness is inferred within the sourced35–45cm range;0.15m pyramid rise is inferred from section proportions. Accordingly the prepared interface datums are−1.65/−1.25/−1.15/−1.00/−0.30m. They do not claim that each individual footing was surveyed at these exact depths. The RC plan bounds and apparent pyramid shoulders are raster traces. Every pyramid uses an actual continuous frustum, never stacked boxes.

## Source Contract

The offline delta has42 new IfcFooting products:21 MAT-LIME-CONCRETE preparations and21 MAT-RC combined base/transition products. Parts use outline, optional top_outline, z0,z1 and ground base_level. Existing22 numbered columns receive buried square sections from the transition to−0.30m and retain their actual above-ground profiles from there. There are no duplicate stem products or extra pads under cellar-supported columns. Forty-two connection relations establish preparation→pad→column; fresh publication also updates spatial containment. All84 new product/relation identities match the canonical UUIDv5 provider.

Authoring inputs are foundations-r16-input.json, foundations-r16-assets.json and foundations-r16-author.mjs. The assets file contains only new products, relations and affected column geometry, rather than a copied source model. The publisher fresh-reads source and refuses duplicate foundation tags. Publication remains held for the parent coordination window.

## Test Evidence

Language-neutral expected coverage and datums are model/🧪️tests/🏗️foundations.json. Before implementation, the coverage test failed as expected against the frozen340-element source. All five tests in 🏗️foundations.py pass against an in-memory merge: exact numbered coverage with no cellar duplicates; independent Shapely plan containment and physical vertical contacts; every RC pad volume independently checked through SciPy3D ConvexHull; and portable per-material totals. The prepared model would contain382 elements. Added physical preparation+RC pad volume is12.004714787m³, an inferred geometric quantity, excluding the column-owned stems.

No frozen model JSON was modified by preparation or validation. Consumer rendering and source-schema checks remain required before publication acceptance.

The in-memory382 model also passes strict AJV2020 schema validation and full building_geometry generation, exposing45 total foundations (42new plus3cellar) and67 columns. Lime-concrete volume is10.584356819m³ and newRC pad volume1.420357968m³; both values are recorded in the language-neutral fixture.

## Cellar Interface Ownership

A further independent audit checked every new footing against the preserved cellar footing and masonry volumes. The raster-traced outer preparation27 crossed the existing north cellar wall by0.017665694m³. Its physical outline now has a small notch at the actual wall face, with the original uncut raster bounds retained in the input. This assigns material to the existing masonry instead of relocating the traced pad or counting two materials in one volume. All five focused tests now pass, including a regression requiring zero new-foundation overlap with cellar masonry and existing footings.

## Publication

Published after the parent released the source window:382 elements,45 total footings,22 modified existing column products and42 new connection relations. All five foundation tests pass on the saved source. All three analytical links were regenerated after changing physical column birth elevations; consistency_issues returned an empty list. The shared source window is released.

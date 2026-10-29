# Cellar Remodel R9

Source: Zaparaín Hernández, Ramos Jular, Llamazares Blanco and Barba Rodríguez, 2022, [DOI10.3989/ic.88625](https://doi.org/10.3989/ic.88625), §§2.1,2.2,9 and annex1 (author PDF page18), longitudinal/transverse sections pages22–23. Cross-checked against structural-register-r9.md.

## Authored Scope

Added the source-informed partial cellar as its own storey, two connected longitudinal bays, four perimeter masonry walls and the partial middle divider, stone/lime and lean-lime-concrete materials, 30cm-deep strip footings, 8cm floor, 7cm ceiling web with a genuine stair void,16cm ribs with a locally deeper35cm rib, stair-opening trimmers, internal column14,11×11cm stair support Pa and30×30cm footing. Added a short curved/straight15-riser stair as explicit polygonal steps; riser count, exact step shapes and room coordinates are inferred, distinct from the upper21-step full-turn abstraction. No full-footprint underground slab or invented garage hoist hatch.

The registered stair centre is (6.238420,9.263605)m. The trace uses x=19-(px-92)*19/734 and y=1.25+(py-225)*19/735, preserving the4.75m actual grid. The primary fixture stores the original pixels and coordinate transform. Current cellar exterior is x4.504087..11.234332,y5.670408..16.837755, an image-read approximation. Exact outer offsets, masonry widths, rib lengths and trim locations remain inferred. Annex small lateral projections are omitted because functions and wall-height/topology are unresolved; they are not renamed as hoist openings.

The nominal2.40m cellar height from article§2.1 is preserved as evidence. The floor-2.42m and strip-footing bottom-2.72m come from section datums. The model places the ceiling-web top at0m, so web underside-0.07m and modeled masonry clearheight2.35m are explicitly distinguished from the nominal text height. Density values are assumed.

## Contracts

Schema now includes cellar levels, four storeys, IfcFooting/IfcBeam solid parts, rectangular column profiles and optional stair step_parts. _stair_geometry uses explicit authored steps when provided. All additions have stable IFC GUIDs, bilingual names, material/type/spatial relations and architecture/structure model membership. Parent owns rectangular-column generalization, quantities union, provider geometry propagation and exchange output.

## Verification

Before authoring, the portable language-neutral fixture and Python regression failed because no cellar storey existed. After authoring, cellar construction/access contract passes. Independent Shapely validates the partial footprint, stair void containment and all footing/rib polygon validity. Shapely catches a footprint intersection between divider and north strip footing: correct FT-CELLAR union volume7.852830912m³ versus prior summed7.870316652m³. Parent notified to apply generic occupied-volume union; individual ribs and Pa footing agree exactly. Final integrated regression rerun remains required after parent semantic updates.

Files: model JSON; schema JSON; model Python minimal authored-stair step override; 🧪️tests/🕳️cellar.json and 🧪️tests/🕳️cellar.py. Retained ticket input cellar-author-r9.mjs records authored construction logic; do not rerun it against an already modified source because it appends objects.

## Access Completion

Completed four full-height stair enclosure walls with two explicitly inferred internal unfilled passages, at the lower approach and upper landing. The lower approach intersects the first tread route. The ground ceiling stair void was widened west to x5.428420m to clear the final straight tread at x5.488420m; opening trimmer and west wall moved coherently. Shapely now also verifies the complete authored stair footprint lies inside the real ceiling opening. Final source count at release:246 elements,24 spaces,4storeys.

## Final Validation and Column14 Foundation

Added native IfcFooting FT-CELLAR-C14, with an explicitly inferred0.60×0.60×0.20m RC pad above the central continuous lime-concrete strip. The strip continues north under the connecting open bay and supports column14 without an invented full-height divider. Column14 starts at padtop-2.22m. Corrected internal stair enclosure IsExternal flags to false. Schema now supports circle, rectangle, ellipse and polygon column profiles, optional plan rotation and per-height geometry.sections as requested by the column-profile implementation.

After the parent applied generic occupied-volume union, both cellar regressions pass: source contract and independent Shapely footprint, stair containment, central pad support and four footing/rib union volume checks. All new cellar GUIDs agree with the existing stable UUID5/IFC encoder. Final source at write-window release:247elements,24spaces,4storeys. Subsequent coordinator column/profile and aboveground datum remodelling may change global counts; this is the tested cellar contribution.

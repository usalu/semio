# Prismatic Chimney Registration R44

Published on 2026-10-29 after root approval and independent R48/R50 audits. Fresh source merge added four products, taking 448 to 452. Producer SHA256: 7e6b7bd056da5d80d53ec020bd697dbfaa58468d7b0fc9a916e44c693b406170. Parent owns subsequent metadata, links, native SDK verification, ticket lifecycle and generated cleanup.

## Evidence and Reconstruction Bounds

[Official Savoye as Built 2024, pp117–119](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?isAllowed=y&sequence=1) attests the salon prismatic flue and kitchen flue beside the other curved solarium wall. The kitchen outlet is absent from planned FLC19443 and present in executed FLC19572 and original photographs. Rendered hollow brick with reinforcement is the attested construction family. Reinforcement detail, caps and lower hidden networks remain unresolved.

The modern author's roof reconstruction is available through official [UVA annex metadata](https://uvadoc.uva.es/handle/10324/66393) and an observed public first-page [third-party preview](https://www.scribd.com/document/1007185038/planta-4). The preview is a transfer of Zaparaín's 2024 author reconstruction, not an original FLC plan. Its observed original bitmap is retained as generated/roof-plan-author-r44.jpg. No authentication or access restriction was bypassed. R48 independently read the hollow-square morphology and fitted the piloti grid: X=(730.2−u)/38, Y=(v−3.9263158)/37.9789474, residual ≤0.8px. The hollow square symbols differ clearly from circular column symbols. The nominal axis coordinates are salon [13.01,1.22] and kitchen [0.89,5.32]m. Both modeled outside squares are 0.35m with a 0.15m clear square bore. Source uncertainty explicitly records XY ±0.10m, raster outside/bore uncertainty ±0.04/0.03m, and inferred layer construction.

Salon top9.25m and kitchen top9.60m are bounded teaching reconstructions, each ±0.40m. They are not calibrated original heights. The building box now records the inferred occupied modeled envelope9.60m, without claiming all historical outlets have measured heights. No terminal cap or lower flue route was invented.

## Photographic Registration Limits

Retained prismatic-register-r44.json/.py contain six noncoplanar box corners plus five provisional piloti controls. Completed original1931 plate27 upper photograph alone was used; the lower construction photo and image crop edges were excluded as metric controls. Normalized DLT plus SciPy refinement gives component RMS1.174px for11 controls. All eight facade mappings were tested; global projective sign and proper camera decomposition reject reflected negative-depth mappings. Four physical alternatives were retained and compared with native roof features.

Native overlays and camera matrices are generated diagnostics. Neither the prominent barred feature451–470 nor the lower horizontal315–365 feature established a coherent unique aperture identity. Native ramp rail projection through the current SC1 landscape opening did not validate any retained camera/feature pairing. This does not establish a source SC1 defect. No solarium geometry was moved. Both salon/kitchen photo assignments were retained; width alone cannot identify function. The independent author plan resolves axes and footprints, while heights remain explicitly bounded inference. The author-plan boiler circle also agrees with R40's independently registered cylindrical outlet within its uncertainty.

## Supported Material Topology

Both chimneys start at structural compression top6.51m in SL-R regions R10/R6, beneath finished roof level6.60m. Each uses twelve disjoint native vertical prisms: four exterior-render strips (outside0.35/inside0.33m), four hollow-clay strips (0.33/0.17m), and four inner-finish strips (0.17/0.15m). These 1/8/1cm wall layers are inferred. Material volumes are 0.274m³ salon and0.309m³ kitchen.

A bore-only0.15m square IfcOpeningElement cuts the roof slab. The 0.1000m² masonry ring bears directly on the structural roof. A rejected0.37m full-footprint hole would have removed that bearing. The generic geometry.finish_voids interface instead reserves only the0.35m outer footprint in roof finish6.51–6.60m. Actual finish_parts exclude chimney ownership; shared_finish_parts identify the owner's0.009m³ neck material per chimney. full_finish_parts retains the complete semantic reference envelope. Actual material is counted once. Existing R43 column reserve and structural ownership rules remain intact.

Products: CHIMNEY-SALON-ROOF, OP-SALON-FLUE-ROOF, CHIMNEY-KITCHEN-ROOF, OP-KITCHEN-FLUE-ROOF. Relations include native slab voids and explicit support/finish ownership, roof containment and architecture/services model memberships.

## TDD and Verification

Schema-first strict chimney alternatives distinguish native tube and composite prismatic services; mixed tube/composite input is rejected. The new language-agnostic 🚬️prisms.json fixture covers sections, material rings, structural bearing, bore-only voids and finish ownership. Four Python contracts compare native results with independent NumPy/SciPy hulls and GEOS intersections. The first test failed on unconditional service points access before generic support was implemented. Production first-floor projection then failed on an incorrectly filled above-cut composite neck; the plan provider now emits the proper wire while the roof cut retains its hollow material ring.

An early candidate assembly incorrectly mutated api.element's resolved copy, leaving finish reservations unapplied. R50 caught actual roof-finish overlap0.009m³ per chimney. The retained reproduction now mutates the source element and the corrected actual export passes independently: bearing0.1000m² each, structural overlap0, exclusive-finish overlap ≤8e−16m³, empty bore ≤9e−16m³, owner finish0.009m³ each in three materials. See chimney-construction-audit-r50.md; its earlier failed result is preserved.

Fresh source and candidate schemas pass; the mixed-composite negative rejects. Full candidate building_geometry succeeds for452 products (62.773s under concurrent load). Every occupied native category was checked in common Z bands: slabs including exclusive finish, walls/screens, pilotis, beams, foundations, stairs, railings, furnishings, details, fillings and services. No new occupied intersection was found; generic0.25m column clearance passes. The existing800mm hall route and actual stair approaches remain valid. The focused cohort passes21 tests: four prism, four boiler, seven ground and six stair contracts. All seven public representation modes pass for both prisms. EN/DE native STEP excerpts use SweptSolid with twelve extruded material prisms each. Parent performs the separate native SDK volume/identity check.

## Files and Reproduction

Production changes: model/🔣️.json; model/🐍️.py (generic native composite service, representation, quantities, finish-only ownership); model/🧬️schema/🔣️.schema.json; model/🧪️tests/🚬️prisms.json and 🚬️prisms.py; clip/🧪️tests/🐍️.py module registration; visuals/🏛️savoye/📋️plan/🐍️.py above-cut wire selection. Fresh source publication preserves root's R43 bearings, R53 stair cleanup and metadata/link edits.

Retained ticket inputs: prismatic-author-r44.mjs, prismatic-physical-r44.py, prismatic-register-r44.json/.py and this report. Generated diagnostics include prismatic-patch-r44.json, prismatic-native-r44.json, prismatic-checks-r44.json, prismatic-publication-r44.json, camera/overlay artifacts and author-plan images. prismatic-physical-r44.py can verify either the prepublication candidate or already-published source without duplicating products. No shell, Git mutation, worktree, IDE, AGENTS edit or checked-source copy was used.

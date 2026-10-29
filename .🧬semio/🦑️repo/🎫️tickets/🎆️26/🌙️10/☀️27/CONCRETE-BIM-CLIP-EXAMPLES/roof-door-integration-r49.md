# Roof Door Integration R49

The approved roof exit retains all existing wall/opening/door/type identities and the unchanged enclosure outline. Publication merged only fresh source fields: f4f7b26dfbb46749c1c93a4bb2ec94be229615e118d77c99a15a923e45152020 → dc4a83066bd3360806189e4827c281783eff7267fe29c2282bb5b85a30f7480e,452products throughout. StrictAJV2020 passed. Five incorrect glass-association target lists lost the roof type; the existing timber-layer relation gained it. No product, type or material was added.

## Evidence And Declared Uncertainty

SAB2024 p206 supports the exterior timber solarium leaf and translucent surround carried by the entrance metal-profile family. The copied salon/FLC097-1 attribution and false internal/glass type were removed. Modern-restoration p207figure11 supports only the coarse central opaque leaf, two horizontally split sidelights and full-width transom topology. It does not certify1930dimensions or swing direction. The source now says this explicitly in EN/DE.

The aperture is on the closing vertical reference side atX3.06171694865,Y8.98..10.15223949926. Cumulative public-axis stations are11.854080771353614..13.026320270615726. The author/CMN register discrepancy remains±0.15m; the enclosure was not widened. Overall head2.43m meets the existing sourced room soffit9.03m. Inferred subdivisions use20/60/20clear-frame widths,88percent leaf height and one sidelight split at60percent of leaf height. Exact profile, recess, glass pattern and original leaf dimensions remain unresolved.

The central material is LS-DOOR-LEAF:5mm plywood skins/25mm hollow batten core/5mm skin, with40mm inferred edge battens. Applying the general p203family to this roof leaf is explicitly inferred. No35mm solid timber shortcut or air solid remains. The native assembly has66occupied parts in plywood/timber/metal/translucent glass. Clear leaf0.64834369956×2.1039m is reconstructed and unmeasured; it is not a certified minimum or a claim that an0.8m body passes.

## Schema And Physical Interface

Schema-first panel_layout cells carry normalized authored u/v quantities, material, thickness and operation. The native builder validates an exact cell partition, places shared metal framing once and honors layered hollow leaves with whole-leaf battens. Independent GEOS areas and SciPy hull volumes verify material inventory, disjoint solids and clear dimensions. Gap/overlap layouts reject. All7public representations consume the same actual parts.

A finite station opening at a closed host corner previously inherited the host miter beyond its endpoint. A portable unit-square case failed before the coherent fix. Public opening geometry now intersects capped aperture bands with the actual host; path-wall subtraction retains the unremoved miter wedge and layer material. The GEOS gross-minus-hole proof passes without a tolerance increase. Absolute base+head≤body_top also avoids rejecting the exact2.43m soffit height through binary subtraction. Actual closed-west-wall public-plan regression passes36.957s.

## Recorded Verification

- Original source material/placement tests:2RED,0errors before correction.
- Mixed-layout and layered-family contracts: each RED before implementation, then independent material/public-view/invalid-partition proofs pass.
- Reviewed final candidate:5PASS,35.8794341s, f4f7 raw source stable; maximum native wall overlap6.80e−15m³.
- Published actual source:5roof contracts plus main all-window/door rough-versus-clear quantity contract:6PASS,47.2964046s, dc4raw stable before/after.
- Independent R54 audit: all66parts/materials resolve; self-overlap0, wall overlap numericalzero1.73e−14m³; sourced soffit9.03. Assumed tread-to-leaf centreline and±40mm parallel foot lines remain supported and avoid walls/guards. This is a bounded route, not full-body, code-clearance or swing certification.
- Root independently decoded10nativeEN/DE IFC samples, including66door parts and280wall parts. Native scene frames/caption fit remain unverified.

Retained inputs are roof-door-candidate-r49.json and roof-door-candidate-check-r49.py; portable fixtures/tests are model/🧪️tests/🚪️roof.json and🚪️roof.py. Derived geometry/progress belongs in generated output and is not a source snapshot.

## Other Final Integration Corrections

R51 actual-callback TDD first exposed the stale class matrix and universal service-axis loop. The matrix now derives every current discipline class; chimney labels describe assemblies; the geometry oracle uses physical part vertices. All17chapter-source/helper/bilingual contracts passed. The main452manifest was updated only by declared deltas:447 +one boiler roof bore +two chimney products +two chimney bores. Counts are IfcChimney3/openings50, inferred maximum9.60m. Exact new tags and explicit discipline overlaps are listed; no Counter(actual) fixture capture was used.

The stale exchange architectural-void snapshots were replaced by independent registered rectangle/U-semicircle constructions, analytic areas, NumPy coordinates and GEOS. Gross rectangle19.30×21.60, two0.70×0.70garage lights, accepted U registration and12explicit first-storey space tags are checked. The oracle explicitly covers architectural voids, while main slab contracts cover additional native service holes. The curtain AST test permits each path-record path and rejects the retired whole-curtain singleton path. Both old tests were RED; all6SlabConsumers now pass.

Nine explicitly named service bores now have a portable ownership-label/membership test. It first failed exactly7older metadata records, then passed after the parent aligned metadata to the intentional architecture/services memberships. Their physical cuts continue to affect structural host geometry independently of discipline labels.

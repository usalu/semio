# Roof Door and Translucent Surround Audit R49

Read-only independent audit,2026-10-29. No product changes. Parent owns ticket lifecycle.

## Author Evidence

Verified public indexed author-book text [Savoye as built,2024,pp203–207](https://es.scribd.com/document/1072987232/9788413202839), corresponding to [official university publication](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1). The university PDF was not newly downloaded. Public indexed reproduction is a transfer source, not an original FLC record.

Page206 explicitly includes the solarium exit among exterior doors with wooden leaves. Both principal entrance and solarium door sit in larger translucent-glass panels carried by matching metal profiles; the solarium arrangement was unplanned. Figure11,p207 is credited to Jorge Ramos, so its caption does not make it a1930 photograph. The discussion of a glazed salon-to-vestibule leaf and FLC H1(12)097-1 belongs to another door. Pages203–205 describe the general wooden door family and conventional clear sizes; they do not independently dimension this particular roof surround.

## Exact Current Source Defects

Inspected current checked-in model in memory without copying it. D-STAIR-ROOF is correctly classified external at element level, but TY-STAIR-ROOF-DOOR has IsExternal=false, name roof stair glazed-metal leaf, material MAT-GLASS, panel_material MAT-GLASS, panel_thickness0.004m and source Teaching-glazing-thickness-inferred. Its historical Construction explicitly describes the SALON-to-vestibule glazed door, cites FLC H1(12)097-1 and photo L2(17)103; panel_stations0/1 sources say retained single salon leaf count inferred. This is demonstrable cross-door evidence contamination. The opaque wooden roof leaf must not be represented as clear glazing merely because its surrounding assembly contains translucent glass.

The type currently has one0..1 panel interval, no separate surround or sidelight role, frame width0.025m/depth0.04m, sash width0.018m/depth0.012m, inset0.04m, threshold=false. All those metric values are inferred. The entire current opening is filled as one leaf assembly; no source field separates opaque leaf from translucent sidelights or headlight. Existing frame/sash envelopes therefore cannot be renamed as a measured historic surround.

OP-STAIR-ROOF currently start0.125/end1.454114200130733m, sill0/head2.10m, hence1.329114200130733m overall authored width. None is a source-measured clear leaf dimension. W-STAIR-ROOF first path segment length1.579114200130733m joins(3.06171695,6.11466475) to(4.64079623,6.10416380). If opening stations use cumulative path arclength from the first vertex, opening endpoints are(3.18671418,6.11383351) and(4.51579900,6.10499504). Consumer semantics must be checked before any station correction.

## Independent Plan and Photo Reading

Actual author roof bitmap737×869 was inspected with nearest-neighbour crop u438..625,v219..407. With R48 registration X=(730.2−u)/38,Y=(v−3.9263158)/37.9789474, the current first path segment maps u614,v236 to554,v236. This top horizontal side of the cubicle reads as a thick wall/grid-column interval. The right vertical side has an obvious thin multi-line door/glazing assembly at approximately u613..617,v345..393, around X3.03..3.08,Y8.98..10.25m. This is a plan correspondence to investigate, not an exact leaf measurement. No unambiguous swing arc, leaf-versus-sidelight subdivision or historic profile section can be measured from those few pixels. The overall assembly and opaque leaf must remain separate questions.

The1931 completed upper photo1705 was inspected. Its roof feature at x451..470,y58..80 has dark frame/vertical subdivision but is not independently identified here as the stair door; R44 already documents that ambiguous correspondence. No glazed leaf dimension, door swing, headlight extent or profile thickness follows from it. The photograph cannot license moving the roof body or screen.

## Actionable Scope

Repair false type material/name/evidence and external classification now, preserving explicitly inferred construction dimensions until separately established. Attribute opaque wooden leaf versus translucent surrounding assembly distinctly. A larger whole-aperture glazing panel cannot be subdivided arbitrarily into an invented leaf size. Confirm opening consumer station semantics and compare the author-plan right vertical doorway before changing OP geometry; retain uncertainty in any adopted reconstruction. No requirement to solve photographic height before correcting the demonstrated material/provenance error. No exact metal-profile cross-section, surrounding glass thickness, glazing subdivisions, swing direction or leaf dimensions was established in this pass.

## Actual Figure11 Photograph Retrieved

Retrieved public HTML for the indexed book; it openly exposed298hashed page JSONP URLs. Observed page207 URL https://html.scribdassets.com/7dllg042tci600ml/pages/207-e082cd71ed.jsonp identifies the exact printed207 caption and image sprites. Its observed image URL http://html.scribd.com/7dllg042tci600ml/images/207-e082cd71ed.jpg was fetched over HTTPS. No authentication, token derivation or hidden download route was used. Retained generated image roof-door-fig11-r49.jpg is368×1157px/116277bytes. Figure11 occupies sprite crop x0..260,y809..1156.

Actual modern photograph shows a closed dark opaque CENTRAL SINGLE LEAF, fixed translucent sidelights BOTH SIDES, and a FULL-WIDTH transom above the leaf. Each sidelight has one horizontal division around60% down leaf height. The right-hand adjacent larger window is outside the door assembly and must not be included in its width. Perspective is modest but not calibrated. Approximate overall frame width164px and leaf101px support rounded proportions leaf0.61, left sidelight0.19,right0.18, with remaining frame uncertainty; use roughly0.60central leaf and0.20each side if an explicitly inferred normalized reconstruction is required. These are coarse image ratios, uncertain several percentage points, not measured millimetre profiles.

The leaf height is roughly7–8times the transom clear height in this image. That supports presence and relative shallowness of the headlight, not an absolute leaf height or head elevation. Right-side handle is visible; hinge side and swing cannot be established securely from the closed leaf. Translucent panes show a regular fine texture, but assigning a specific wired-glass product or thickness from this reproduction would be unsupported.

If a plan-derived overall assembly width around1.27m is adopted independently, a0.60leaf fraction would imply roughly0.76m leaf envelope, not a sourced clear opening measurement. The photograph may inform a qualified inferred subdivision; it does not prove original1930leaf size or that every restored member matches the original. Text supports the historic material/surround distinction; this modern photo supplies subdivision morphology. Root and R11 received the actual image and ratios before any correction.

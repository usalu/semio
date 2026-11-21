# Concave prismatic Mesh Booleans through a longitudinal support

The retained clipping cursor now accepts one convex supporting plane parallel to a concave prism's corresponding cap edges. It checks all original cap-edge pairs, requires two strict profile crossings, and carries both endpoints of each clipped original edge into the indexed mesh. The clipped shape uses the retained cap orientation and a single new wall from the exact owning cutter face. Intersect works in both operand orders. Difference reverses the cutter wall and keeps the opposing prism interval.

The mesh face receipts distinguish an original cap clipped along its certified boundary. The authored reconstruction verifies each generated cap corner lies on an original edge and transports raw UVs, named linear attributes, and authored normals using the two original endpoint samples. Oblique supports, tangent or coincident contacts, and disconnected crossings on one support still need further Boolean handling.

The neutral fixture gives Intersect volume 1 and area 7. Its complementary Difference gives volume 2 and area 11. The native registered clipping suite passed all 10 tests and validates reciprocal incidence, orientation, and the provenance of both cut wall directions. Both authored façade tests passed with nonaffine cap samples and reimport. The focused native Flow and Generate3d suites passed 18 and 9 tests, including fuels 1, 7, and 1024. Three.js independently reproduced four authored edge UV rows, and Manifold reproduced both intersection orders and the remainder. The portable clipping and packaging suite passed 18 tests with 4,797 assertions.

The transverse regression first reproduced refusal for a nonaffine corner UV. The updated edge sample transport preserves both the named and raw exact edge samples when the vertices lie on authored source edges. Interior samples still require a certified affine chart.

## Opposing longitudinal supports

The retained cursor now clips a certified concave prism against two opposing walls. It certifies each wall against both caps, orders two crossings on a shared source edge by their original edge parameter, and checks that each crossing is strictly inside the opposite support. Intersect yields one closed band. Difference publishes both surviving prisms with reversed cutter walls. Both paths preserve original owner indices and edge-interpolated samples.

The neutral slab fixture measures Intersect at area 9 and volume 1.5 in both operand orders. Difference measures area 11 and volume 1.5 across two components. Manifold independently validates the component count, zero genus, area and volume. Three.js validates the four authored UV interpolation rows. The native clipping suite passed 10 tests. The linked authored façade passed the slab law in both Intersect directions and Difference, with all raw and named UV samples and closed reimport. The portable packaging and Clipping suite passed 18 tests and 4,821 assertions.

## Transverse caps with longitudinal walls

Intersect now combines the certified longitudinal band with transverse cap planes. The same profile edge parameter determines a cut corner on either moved cap, and four original vertices supply its weighted vertex interpolation. The two moved cap faces and both longitudinal walls retain their cutter face indices. A wholly discarded band reports an empty intersection. The mixed Difference case below reuses the same seam positions.

The registered clipping suite passed 10 tests. The portable suite passed 18 tests and 4,831 assertions. Manifold reproduces area 6 and volume 0.75 with one zero-genus component. The linked authored façade confirms both operand orders, the four exact cutter material owners, affine UVs and closed reimport. The clipping owner remains within its admission limits with 11 files and 2,185 active lines.

## Isolated existing vertex crossings

A longitudinal support that strictly separates the two source edges incident to an existing profile vertex now reuses both corresponding prism vertices. The new wall enters those vertices directly. The clipped-away wall alongside the source vertex collapses to its original edge and contributes no degenerate face. Tangent or coincident contacts still require their own boundary classification.

The diagonal fixture verifies one kept component in each Intersect operand order and in Difference, opposite orientations of the cutter wall, one live topological vertex at each reused corner, and closed reciprocal edges. Native Intersect volume 2.5 and area 12.414213562373095, and Difference volume 0.5 and area 4.414213562373095 agree with Manifold. Three.js agrees on the original and interpolated UV samples. The current native clipping suite passed 10 tests. The linked authored test passed with raw and named UV samples and closed reimport, and the portable suite passed 18 tests with 4,851 assertions.

## Mixed Difference with a transverse opening

Difference preserves the full outer cap at each surviving end. It emits only the cavern opening on each cutter-owned transverse plane and stitches the surviving outer walls through the same indexed crossings used by the lateral faces. A cutter spanning the stock cap shares the authored outer cap edges. A cutter strictly between the caps produces a closed shell with one tunnel. Both cases preserve exact cutter face ownership and raw and named UVs through closed reimport.

The native regression reproduced the mixed support refusal, then all 10 Clipping tests passed. Manifold independently reproduced one component, area 17, volume 2.25 and genus 1 for the enclosed cut. The open-end cut gives one component, area 14, volume 2.25 and genus 0. Both Intersect operand orders retain area 6 and volume 0.75. The portable suite passed 18 tests and 4,871 assertions. The authored façade passed all mixed operand orders and Difference cases with verified material owners, raw and named UV samples, and exact orientation through reimport.

- [Native regression](./🗑️generated/mixed-difference-red.log)
- [Clipping tests](./🗑️generated/mixed-difference-green.log)
- [Manifold and source oracle](./🗑️generated/mixed-difference-oracle.log)
- [Authored façade](./🗑️generated/mixed-difference-authored-current.log)

This construction covers one longitudinal support or two opposing ones, alongside either or both transverse planes. General oblique Boolean arrangements and multiple disconnected cut intervals on a single longitudinal support need a different profile arrangement.

## Nonparallel longitudinal walls

The clipping cursor accepts two nonparallel supports when both cut contours lie strictly within the other support and their removed material is disjoint. The neutral fixture trims the two arms of the concave prism. Intersect returns volume 2 and area 10 in both orders. Difference returns two closed components with total volume 1 and area 8. Native reciprocal incidence and cutter orientation agree with Manifold, and the authored reimport checks the original faces and both cutter material owners. The 10-test native suite, the authored façade, and the 18-test portable suite with 4,890 assertions passed. Intersecting cut contours still need an arrangement.

## Four independent longitudinal walls

The retained wall cursor stores a bounded set of six supports. Each crossing must lie within every other support, and each original exterior vertex can be removed by at most one wall. The original edge ordering stays defined by the at most two crossing samples on one edge. The same ownership and rim construction serve each outside component and the common closed intersection.

The neutral four-arm fixture passes Intersect in both directions with volume 3, area 14 and one closed component. Difference leaves four components with total volume 2 and area 16. The native red case reproduced the previous wall-count refusal. All 10 native clipping tests now pass, including reciprocal halfedges and the orientation of every cutter face. The authored facade passes both operation directions and reimport with original named and raw UVs. The Manifold and source suite passed 18 tests and 4,910 assertions.

- [Native green witness](./🗑️generated/four-arm-longitudinal-green.log)
- [Authored UV and reimport witness](./🗑️generated/four-arm-longitudinal-authored.log)
- [Manifold and source checks](./🗑️generated/four-arm-longitudinal-oracle.log)

# Ramp Guard And Stringer Construction Constraints

## Current State

Root has not changed source guard geometry yet. The permanent four-method guard suite is registered and RED2/PASS2: 94 actual wall material witnesses and four continuous top probes. The two existing source products still contain257/194 stepped cells. The single missing attested stringer is also not published yet. Root completed the two exterior lamp repairs separately; all five combined analytic/mounting/native/source methods pass, and independent8-fragment native shared-point proof passes.

## Source Construction

FLC H1(13)306/SAB134 supports continuous6cm reinforced concrete parapets and the central scissor screen arising from superposed bands. Preserve the large external air gaps between opposing flights. Do not fill a generic full-height central wall. MAT-RENDERED actually means Rendered concrete (assumed24kN/m3), not render-only, so keeping it for source guard bodies is semantically valid and preserves their white material presentation without inventing a measured coating thickness. Existing95cm top height and10cm cap width/depth are nominal inferences unless better measurements are secured.

The upper RF2B inner edge has an attested12cm by28cm concrete upstand already represented by the last GUARD-RM-2 section. Preserve it. The absent95cm inner RF2B guard is deliberate; do not instantiate a mirrored full guard there. The lower return executed6cm parapet replaced a proposed12cm by28cm upstand, so the families must not be conflated.

The tiny SAB136 contractor L-section is a facade-side stringer condensation/drainage channel, not a top-cap groove. A separate executed handrail covering is described as grooved, but exact groove shape/count/dimensions are unmeasured. Do not transplant or fabricate its profile from the drainage sketch.

Exactly one12cm by26cm inclined downstand is attested: RF1A garage side, original supports13/18/25. Typed source handoff is ramp-stringer-support-source-r92.json. Its inferred registered centreline isX10.898, Y8.23..16.889; full width X10.838..10.958. The top follows actual RC flight top and depth is nominal vertical. Medium proposed and root confirmed RC flight plate terminates atX10.838; the beam can therefore own its full26cm depth without duplicated upper6cm plate material. Existing columns and other structure retain their original coordinates. Root will author actual exclusive stringer material around them, not four mirrored beams.

## Width And Finish Coupling

Old1.2m flight widths are inferred and leave real68–70mm gaps between plate edges and registered6cm guard faces. There are no hidden inclined bridging beams in the current capture. Medium owns the relational clear/structural edge correction in ramp-width-bearing-proposal-r92.json: clearA1.338m, Binterior1.312m, Bexterior1.282m from actual inner faces/upstand. Keep original XY walkline metadata independently of asymmetric structural edges, plus sourced finished Z levels and6cm RC thickness. Proposed RCplate extents: RF1A X9.47..10.838 (stringer starts there), RF1B8.098..9.53, RF2A9.47..10.928, RF2B8.098..9.56. These are inferred relational construction fits, not new archival metric measurements.

SAB139/187–188 supersedes the old inferred30mm interior tile/bedding: executed interior resilient covering is5mm, total finish about8mm including nominal3mm mortar. Rubber versus linoleum nomenclature remains qualified. Medium owns that correction and must send its final source batch before root publishes guard/stringer subtraction. Exterior4cm slabs plus2cm sand remains sourced.

Midlanding side stretches fromY16.889 to18.029 need continuous protection except where actual adjacent walls provide it. The existing end return starts atY18.029. A monolithic side/edge concrete contact can be a valid connection; do not invent a gravity support requirement that forces all guard footprints inside the landing. Actual common material is still assigned once. Body/cap corner and central-band overlaps must be removed by a real union/difference.

## Exact Authoring Interface

Production first-party module clip/🧱️model/🎨️finishes/🐍️.py now exposes clip_stock(raw_mesh, [], exact=True), continuing rational state through every ordered inside/outside CSG branch; publish_stock emits isolated plain JSON only at the final boundary. Keep n dot x <= offset; complements reverse signs. Plane.point or Plane.points preserve original face provenance and derive exact plane coefficients, avoiding a rounded-offset sliver. Root should use three original points for native planes; native owner is fixing remaining unrepresentable inferred board contour offsets. Do not perform per-call float publication, hull repair, snapping or arbitrary sliver deletion.

Canonical initial capture is generated/whole-owned-capture-r92.json, 512 physical products/33952 exclusive parts, stable source d575c427 but initially loaded API57ba53f6 while the file changed. Use it only with dependency reconciliation; final current capture is mandatory. Its records carry geometry including profiles, radius, holes, mesh, section and top outlines. Group structure is actual wall/primary/RC-ramp/stair material; finish includes all floor coverings. Refresh changed ramps and floor bodies from current source before final subtraction. True circular shafts must retain analytic radius semantics rather than silently accepting an inscribed polygon overlap.

## Further Recovered Missing Elements

Low has durably registered exterior-ramp35x35mm soffit battens and a false-ceiling board (material/thickness/spacing unknown; insulation conjectural), original52 metal stair nosings LD40x25x4mm (49 current tread bodies plus3 arrivals is only a candidate mapping), and a55mm black central stair tube distinct from the35mm perimeter rails and130mm soil pipe. Source-position/phase review is still required before qualified geometry is authored. These are outstanding work, not completed documentation-only outcomes.

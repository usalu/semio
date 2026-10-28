# Rectangle Extrusion Constructed Oracle Vectors

Authored language-neutral rectangle-extrusion-vectors.json (13 cases), standard-library/cross-platform rectangle-extrusion-oracle.py and generated ADD2 physical graphs/oracle.json under generated/rectangle-extrusion. Existing IfcOpenShell.geom0.9.0.post1 normal world-coordinate create_shape API executed; no reinstall, runtime dependency or production edit. Original official4 oracle remains unchanged.

Seven admitted geometric vectors produce8vertices/12triangles and match independent analytic prism volume XDim×YDim×Depth×abs(normalized extrusion localZ)×SIscale³. Shape owner remains retained while borrowing mesh arrays. Physical GlobalIds are deterministic. All original headers are ADD2; exact product/solid/profile IDs are recorded.

| Case | SI bounds | Absolute volume m³ |
| --- | --- | --- |
| base |[-1,-2,0]→[1,2,3]|24|
| profile rotation90°+offset[5,-2] |[3,-3,0]→[7,-1,3]|24|
| solid origin[10,20,30],Z[0,1,0],X[1,0,0] |[9,20,28]→[11,23,32]|24|
| nonunit oblique[3,0,4] |[-1,-2,0]→[2.8,2,2.4]|19.2|
| nonunit negativeZ[0,0,-7] |[-1,-2,-3]→[1,2,0]|24|
| millimetres2000×4000,depth3000 |[-1,-2,0]→[1,2,3]|24|
| combined profile+solid+oblique |[13,20,31]→[18.8,22.4,33]|~19.2|

Oblique extrusion length follows normalized direction; it does not preserve perpendicular height=Depth. Both caps shift by normalized direction×Depth; volume uses local normal component. NegativeZ requires outward winding adjustment, not negative volume. Oracle signed volume is positive for these admitted cases; TS may choose different triangles but should preserve closed oriented boundary/bounds/volume.

Refusal vectors zeroX,negativeY,zeroDepth,negativeDepth failed IfcOpenShell geometry creation. NonAREA profile and plane-parallel direction were accepted by its geometry engine (the latter produces volume0), so geometry acceptance is not a normative validator. Our expected refusals come from released profile AREA/positive dimensions/depth and extrusion nonperpendicular-to-Z role constraints. Record this oracle disagreement explicitly; do not weaken our admission to match it. These minimal graphs are geometry fixtures, not full EXPRESS-valid project models.

No mapped/revolved/arbitrary/hollow/tapered/BREP completeness is asserted. Next independent output tests should compare actual new mesher/GLB geometry against these physical oracle measures while testing original capture/cancel/no-partial-publication separately. Source limits, tuple dimensions, direction zero/nonfinite, invalid profile2D refs and paired solid axes still need neutral refusal/cancellation cases owned by implementation.

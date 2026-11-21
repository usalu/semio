# Convex Mesh Union Across Five Crossing Supports

The retained Union cursor now partitions across the certified crossing planes within its existing work and point limits. The previous guard refused a box pair with five crossing supports on one operand. Expanding that guard first exposed a same-directed seam. The anchored clipper had narrowed the interpolation fraction before writing coordinates, which put copies of points on the same authored plane several float32 units apart.

The shared anchored path now retains the full interpolation fraction and evaluates the edge in float64. It writes the exact authored coordinate when a plane is axial. Result welding still uses exact position, retains source interpolation, and checks reciprocal edge incidence, connectivity, outward volume and topology.

The fixture yields one positive shell with 28 live vertices, 48 edges and 22 exterior polygons, volume 22, and area 50. The compact native Clipping suite passed all three tests, including the existing Difference variants. Independent Mesh jobs and packaging passed 37 tests and 5,685 assertions. The Generation3d catalogue and geometry oracle passed 58 tests and 399 assertions. The paired authored fixture is registered. Linked verification is tracked separately.

The packaging preservation law now parses the named original Rust tests and checks their source-scoped assertion minimum. The archived proof found no assertion changes in the previous named laws. Unrelated tests had increased the file-wide count, which had falsely failed the equality check.

The latest paired native run passed all 26 selected fixtures at fuels 1, 7 and 1,024, plus early and authored-projection cancellation. The retained Flow producer passed all nine cases including the five-support Union. The refreshed Mesh Engine and authored modeling fixture passed source projection and PolygonMesh reimport.

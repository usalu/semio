# Native 3D Kernel Capability Inventory

Date: 2026-10-07. Ticket: PROCEDURAL-3D-END-TO-END. Mode: read-only audit (no source, build, or git changes; no tests run).

Purpose: a complete inventory of the native B-Rep and mesh kernels and the flow operators that expose them, to plan a full B-Rep and mesh creation, editing and analysis widget set.

## 1. Headline numbers and corrections to the brief

- B-Rep flow extension: 93 registered operator nodes. 91 map to `BrepKernel` trait methods; 2 (`brep.io.exportStep`, `brep.io.importStep`) call the STEP codec directly. The brief said about 90.
- Kernel trait `BrepKernel`: 98 methods (`C` `BREP_KERNEL_OPERATIONS`). 7 are deliberately unexposed: `scale` (covered by `scale_axes`), `kind`, `tessellate`, `dispose`, `retain`, `registry_len`, `export_gltf`. Every other trait method has exactly one node (checked programmatically).
- Mesh flow extension: 42 operator nodes (`brep.mesh.*`). The brief said 29.
- Published manifest `🔣️.json`: 142 ids = 93 B-Rep nodes + 42 mesh nodes + 7 schema ids (`brep.edge`, `brep.face`, `brep.shell`, `brep.vertex`, `brep.text`, `brep.geometry`, `brep.mesh`). No operator is missing from the manifest.
- Job-backed (progress and cancel) B-Rep operators: only `fuse`, `cut`, `intersect` (plus the tessellation preview job). Every other B-Rep operator is synchronous. Mesh operators are budgeted jobs (39 of 42; the 3 `inspect*` readers are direct).
- Mesh operator fidelity tags are not `OpQuality` values; they read `PolygonMesh`, `TessellatedMesh` or `MeshDerivedBRep` (see section 6).

## 2. Path shorthand

- `K` = `🧰️framework/🔨️modules/🧊️3d/` (kernel root)
- `E` = `K/📐️brep/⚙️engine/🦀️.rs` (BrepKernel trait and `_sync` implementations; the `E:<n>` column is the `_sync` line)
- `C` = `K/📐️brep/⚙️engine/🔖️contract/🦀️.rs` (`OpQuality`, `OPERATION_QUALITY`)
- `F` = `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` (B-Rep flow nodes; the `F:<n>` column is the registration line)
- `FM` = `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs` (mesh flow nodes)
- `MX` = `K/🥽️mesh/🦀️.rs` (`HalfedgeMesh` kernel); `MM` = `K/🥽️mesh/🛠️modeling/🦀️.rs` (mesh modeling jobs)
- `BO` = `K/📐️brep/🛠️operations/🔀️boolean/🦀️.rs`
- `MP` = `K/📐️brep/💡️queries/📏mass-properties/🦀️.rs`
- `TOP` = `K/📐️brep/📸️representation/🕸️topology/🦀️.rs`
- `MI` = `K/📐️brep/⚙️engine/📦️mesh-io/🦀️.rs`
- `ME` = `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`
- `SS` = `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs` (session, handle store, `geometry_dict`)
- `ST` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry/🦀️.rs` (STEP codec)

Quality values are the declared `OPERATION_QUALITY` entries (`C:259-388`). `ExactAnalytic` means a closed-form or exact path; `ExactNum` is `ExactNumericalWithinTolerance`; `Approx` is `ApproximateBRep`; `MeshDerived` is `MeshDerivedBRep` (tessellate, then rebuild from triangles).

## 3. B-Rep kernel operations (flow nodes)

### 3.1 Primitives

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `box_prim` | E:821 | `brep.prim3d.box` (F:1031) | ExactAnalytic | Real |
| `sphere_prim` | E:827 | `brep.prim3d.sphere` (F:1041) | ExactAnalytic | Real |
| `cylinder_prim` | E:833 | `brep.prim3d.cylinder` (F:1044) | ExactAnalytic | Real |
| `cone_prim` | E:839 | `brep.prim3d.cone` (F:1056) | ExactAnalytic | Real |
| `torus_prim` | E:845 | `brep.prim3d.torus` (F:1068) | ExactAnalytic | Real |
| `convex_hull` | E:851 | `brep.prim3d.convexHull` (F:1080) | ExactAnalytic | Real |

No wedge, pyramid, capsule, tube or prism primitive. A prism is available only by extruding a regular-polygon wire.

### 3.2 Curves and wires

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `line_curve` | E:858 | `brep.curve.line` (F:1091) | ExactAnalytic | Real |
| `circle_curve` | E:862 | `brep.curve.circle` (F:1094) | ExactAnalytic | Real |
| `arc_curve` | E:867 | `brep.curve.arc` (F:1106) | ExactAnalytic | Real |
| `ellipse_curve` | E:883 | `brep.curve.ellipse` (F:1124) | ExactAnalytic | Real as a curve, but elliptical edges are rejected as extrude and revolve profiles (section 6) |
| `polyline_wire` | E:888 | `brep.curve.polyline` (F:1134) | ExactAnalytic | Real |
| `rectangle_wire` | E:895 | `brep.curve.rectangle` (F:1137) | ExactAnalytic | Real |
| `regular_polygon_wire` | E:901 | `brep.curve.polygon` (F:1149) | ExactAnalytic | Real |
| `interpolate_curve` | E:907 | `brep.curve.interpolate` (F:1161) | Approx | Global interpolation, degree parameter |
| `approximate_curve` | E:916 | `brep.curve.approximate` (F:1173) | Approx | Least-squares fit to a control count; the error-bounded variant `approximate_curve` (`K/📐️brep/📸️representation/➰️curve/✂️curve-ops/🦀️.rs:824`) exists but is not exposed |
| `helix_curve` | E:926 | `brep.curve.helix` (F:1185) | ExactAnalytic | Real |

Not exposed: Bezier or NURBS authoring by control points and weights (the kernel has `bezier` and `nurbs` representations, but no flow node creates them directly).

### 3.3 Surfaces and faces

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `plane_surface` | E:940 | `brep.surf.plane` (F:1204) | ExactAnalytic | Real |
| `planar_face_from_points` | E:945 | `brep.surf.planarFace` (F:1216) | ExactAnalytic | Real |
| `planar_face_from_wire` | E:952 | `brep.surf.planarFaceWire` (F:1228) | ExactAnalytic | Real |
| `nurbs_surface_from_grid` | E:961 | `brep.surf.nurbsGrid` (F:1240) | Approx | Grid fit with degrees |
| `coons_patch` | E:970 | `brep.surf.coons` (F:1250) | Approx | Interpolates boundary curves |
| `offset_face` | E:989 | `brep.surf.offset` (F:1253) | ExactNum | Real; NURBS isocurve reparametrization passes through unchanged (`K/📐️brep/🛠️operations/↔️offset/🦀️.rs:735-739`) |
| `thicken_face` | E:996 | `brep.surf.thicken` (F:1265) | ExactNum | Real |

No surface trim, extend, knit, surface-of-revolution or sweep-to-surface operators.

### 3.4 2D sketch and profile

There is no 2D sketch, dimension or constraint solver on the B-Rep side (grep: no `sketch` or `constraint solver` in `K` or the B-Rep flow extension). A profile is a 3D wire built from the curve nodes above (polyline, rectangle, polygon, circle, arc, ellipse). The 2D path geometry lives in `🧰️framework/🔨️modules/📐️geometry/` (Point, Vec2, Affine, shapes, `kurbo` as test oracle) and in the draw plugin `✏️s/🔌️plugins/🖍️draw/`. Neither is reachable from a B-Rep flow node.

### 3.5 Solid construction (extrude, revolve, sweep, loft, pipe)

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `extrude_wire` | E:1003 | `brep.solid.extrude` (F:1278) | ExactAnalytic | Closed wire to solid. Open-wire shell-only extrusion rejected (`K/📐️brep/🛠️operations/➡️sweep/🦀️.rs:77`) |
| `extrude` | E:1015 | `brep.sweep.extrude` (F:1290) | ExactAnalytic | Circle profile with axis not parallel to the direction rejected (`➡️sweep/🧮️core/🦀️.rs:154`); ellipse rejected (`🧮️core/🦀️.rs:160`) |
| `revolve` | E:1023 | `brep.sweep.revolve` (F:1302) | ExactAnalytic | Ellipse rejected (`➡️sweep/🌀️revolve/🦀️.rs:148`); NURBS rejected (`:149`); profile edge on the axis rejected (`:74`); apex-touching edge rejected (`:123`) |
| `loft` | E:1030 | `brep.sweep.loft` (F:1314) | ExactAnalytic | Profiles must be knot-compatible after degree elevation (`➡️sweep/🥞️loft/🦀️.rs:96`; full knot-union harmonization not implemented) |
| `sweep` | E:1040 | `brep.sweep.sweep` (F:1326) | ExactNum | Straight or circular path fast path is exact; general path uses adaptive rotation-minimizing-frame stations (bounded error) |
| `pipe` | E:1048 | `brep.sweep.pipe` (F:1338) | ExactNum | Optional guide; same general-path caveat as `sweep` |
| `helical_sweep` | E:1060 | `brep.sweep.helical` (F:1350) | ExactNum | Real |

### 3.6 Booleans, sections and splits

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `fuse` | E:1067 | `brep.bool.fuse` (F:1368) | ExactNum | Exact imprint, classify, select, stitch engine is the default. Job-backed (budget, progress, cancel) via `boolean_operation!` (F:441) |
| `cut` | E:1075 | `brep.bool.cut` (F:1369) | ExactNum | Job-backed (F:442) |
| `intersect` (boolean common) | E:1128 (job at E:1106) | `brep.bool.intersect` (F:1372) | ExactNum | Job-backed (F:443) |
| `compound_cut` | E:1136 | `brep.bool.compoundCut` (F:1384) | ExactNum | Multiple tools; not job-backed |
| `section` | E:1375 | `brep.intersect.section` (F:1636) | ExactAnalytic (overstated) | Planar section. Its doc says curved-face sections (plane through cylinder, sphere, cone, torus) are not routed through the imprint engine (`BO:118-120`). It builds one planar face from collected points (`BO:113-121`), so multi-loop sections are not covered by the doc |
| `split` | E:1382 | `brep.intersect.split` (F:1651) | MeshDerived | Triangle-soup classification, hull fallback (`BO:164-171`); curved-face split not routed through imprint engine |

The tessellate-classify-rebuild boolean pipeline survives only as the opt-in `boolean_solid_mesh_preview`, which `fuse`, `cut`, `intersect` and `compound_cut` never call (`C:259-266` comment).

### 3.7 Intersection queries

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `curve_curve_intersect` | E:1389 | `brep.intersect.curveCurve` (F:1665) | ExactNum | Tolerance parameter |
| `curve_surface_intersect` | E:1396 | `brep.intersect.curveSurface` (F:1677) | ExactNum | Tolerance parameter |
| `surface_surface_intersect` | E:1403 | `brep.intersect.surfaceSurface` (F:1690) | ExactNum | Returns curve geometries |

### 3.8 Local features (fillet, chamfer, shell, draft, offset, defeature)

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `fillet` | E:1264 | `brep.solid.fillet` (F:1508) | ExactAnalytic | Constant radius on all edges. Faces with holes rejected (`K/📐️brep/🛠️operations/🎨️blend/🦀️.rs:1224`) |
| `fillet_variable` | E:1272 | `brep.solid.filletVariable` (F:1520) | ExactNum | One cone p-curve is interpolated and certified to 1e-8 rather than derived (per the `C` comment, `C:323-332`) |
| `fillet_edges` | E:1281 | `brep.solid.filletEdges` (F:1556) | ExactAnalytic | Selected edges by persistent label |
| `chamfer` | E:1290 | `brep.solid.chamfer` (F:1532) | ExactAnalytic | Symmetric distance |
| `chamfer_asymmetric` | E:1298 | `brep.solid.chamferAsymmetric` (F:1544) | ExactAnalytic | Two distances |
| `chamfer_edges` | E:1306 | `brep.solid.chamferEdges` (F:1568) | ExactAnalytic | Selected edges by persistent label |
| `shell` | E:1315 | `brep.solid.shell` (F:1580) | ExactNum | Thickness and list of open faces. Single thickness only |
| `draft` | E:1343 | `brep.solid.draft` (F:1592) | ExactNum (overstated) | Plane and cylinder faces are exact. Other surface kinds are rotated rigidly as a documented best effort, not a true taper (`K/📐️brep/🛠️operations/↔️offset/🦀️.rs:1332-1336`) |
| `offset_solid` | E:1357 | `brep.solid.offsetSolid` (F:1610) | ExactNum | Real |
| `defeature` | E:1364 | `brep.solid.defeature` (F:1622) | MeshDerived | Face removal through tessellate and rebuild |

Not exposed: variable-radius shell, per-face thickness, hole and counterbore features, thread features, face-level push-pull.

### 3.9 Topology construction and edits

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `vertex` | E:1601 | `brep.util.vertex` (F:1892) | ExactAnalytic | Real |
| `face_from_wire` | E:1607 | `brep.util.faceFromWire` (F:1895) | ExactAnalytic | Real |
| `sew_faces` | E:1611 | `brep.util.sew` (F:1907) | ExactAnalytic | Tolerance parameter |
| `heal_solid` | E:1621 | `brep.util.heal` (F:1919) | ExactNum | Tolerance parameter |
| `convert_to_nurbs` | E:1628 | `brep.util.convertToNurbs` (F:1931) | ExactAnalytic | Real |
| `deconstruct` | E:1637 | `brep.brep` (F:1004) | ExactAnalytic | Returns vertex, edge, face and shell handles |
| `solid_shells` | E:1881 | `brep.topology.shells` (F:1945) | ExactAnalytic | Real |
| `compound` | E:1886 | `brep.topology.compound` (F:1957) | ExactAnalytic | Real |
| `explode` | E:1894 | `brep.topology.explode` (F:1961) | ExactAnalytic | Real |
| `label` (`label_of`) | E:1850 | `brep.topology.label` (F:1976) | ExactAnalytic | Returns the `PersistentLabel` |

Not exposed: `handle_for_label` (E:1857) as a node; select-by-label works only through internal component selection (section 8.2). No move-face, push-pull, split-edge, merge-faces or unify-faces operators.

### 3.10 Transforms and patterns

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `translate` | E:1178 | `brep.xform.translate` (F:1397) | ExactAnalytic | Real |
| `rotate` | E:1186 | `brep.xform.rotate` (F:1409) | ExactAnalytic | Rotates about the world origin only |
| `rotate_about` | E:1192 | `brep.xform.rotateAbout` (F:1421) | ExactAnalytic | Explicit origin |
| `scale` | E:1199 | not exposed (`scale_axes` covers it) | ExactAnalytic | Uniform scale; exists in the kernel only |
| `scale_axes` | E:1205 | `brep.xform.scale` (F:1438) | ExactNum | Per-axis factors about a center |
| `mirror` | E:1210 | `brep.xform.mirror` (F:1450) | ExactAnalytic | Plane mirror |
| `copy_shape` | E:1219 | `brep.xform.copy` (F:1460) | ExactAnalytic | Real |
| `linear_pattern` | E:1228 | `brep.xform.linearPattern` (F:1463) | MeshDerived | Pattern through tessellate and rebuild |
| `circular_pattern` | E:1238 | `brep.xform.circularPattern` (F:1475) | MeshDerived | Pattern through tessellate and rebuild |
| `grid_pattern` | E:1249 | `brep.xform.gridPattern` (F:1487) | MeshDerived | Pattern through tessellate and rebuild |

Not exposed: a general 4x4 affine transform for B-Rep (the mesh side has one), pattern along a curve, pattern with per-instance variation.

### 3.11 Evaluation (curves and surfaces)

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `curve_point` | E:1410 | `brep.eval.curvePoint` (F:1706) | ExactAnalytic | Real |
| `curve_tangent` | E:1414 | `brep.eval.curveTangent` (F:1721) | ExactAnalytic | Real |
| `curve_domain` | E:1422 | `brep.eval.curveDomain` (F:1735) | ExactAnalytic | Real |
| `curve_curvature` | E:1428 | `brep.eval.curveCurvature` (F:1742) | ExactAnalytic | Real |
| `surface_point` | E:1432 | `brep.eval.surfPoint` (F:1757) | ExactAnalytic | Real |
| `surface_normal` | E:1437 | `brep.eval.surfNormal` (F:1772) | ExactAnalytic | Real |
| `curve_closest_parameter` | E:1444 | `brep.eval.curveClosestParameter` (F:1787) | ExactNum | Certified; analytic for line, circle and ellipse |
| `surface_closest_uv` | E:1451 | `brep.eval.surfaceClosestUv` (F:1802) | ExactNum | Certified; analytic for analytic surfaces |

### 3.12 Measurement and validation

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `volume` | E:1457 | `brep.measure.volume` (F:1817) | ExactNum | Distinct solids |
| `area` | E:1469 | `brep.measure.area` (F:1823) | ExactNum | Distinct faces |
| `length` | E:1483 | `brep.measure.length` (F:1829) | ExactNum | Curve or distinct edges |
| `center_of_mass` | E:1503 | `brep.measure.centerOfMass` (F:1835) | ExactNum | Real |
| `bounding_box` | E:1508 | `brep.measure.boundingBox` (F:1839) | ExactNum | Returns a box handle |
| `distance` | E:1524 | `brep.measure.distance` (F:1843) | ExactNum | Solid to solid |
| `closest_point` | E:1530 | `brep.measure.closestPoint` (F:1858) | ExactNum | Real |
| `classify_point` | E:1581 | `brep.measure.classify` (F:1873) | ExactNum | Inside, outside, on boundary |
| `validate` | E:1586 | `brep.measure.validate` (F:1887) | ExactAnalytic | Returns a JSON report, but `validate_body` runs on the whole Body, not the selected handle (`E:1586-1599`). The `ok` verdict reflects every entity in the session |

Not exposed as flow nodes: `solid_mass_properties` (inertia tensor and principal data, `MP:157`, `MP:205`; used only by tests), `face_area` (`MP:372`), `edge_length` (`MP:388`), `distance_solid_solid` (`MP:404`), `count_boundary_edges` (`MP:1638`), `watertightness_of_body` (`MP:1651`), `validate_gate_sync` (`E:1821`).

### 3.13 Tessellation (preview and export bridge)

| Kernel method | Kernel impl | Flow node | Quality | Maturity / caveat |
|---|---|---|---|---|
| `tessellate` | E:1773 | none (internal bridge) | ExactAnalytic | Crack-free, error-controlled triangulation of faces (`K/📐️brep/💡️queries/🧩tessellation/🦀️.rs` header). Not a node |
| `tessellate_job` | E:1788 | none (preview job) | ExactAnalytic | Budgeted and cancellable; drives mesh `fromBrep` |

## 4. Mesh operations (flow nodes, 42)

The mesh kernel is `MX` (`HalfedgeMesh`, index-based). Every node takes and returns a `mesh` dictionary (section 8.5). Job-backed means `step_plan` returns a `MeshOperatorJob` with a step budget and cancel; the three `inspect*` readers are direct evaluation.

| Flow node | Kernel call | Job | Notes |
|---|---|---|---|
| `brep.mesh.construct` | `decode_mesh` (FM:83), `PolygonMeshSource` parse (`ME:99`) | Yes | JSON vertices and faces |
| `brep.mesh.box` | `box_prim` (MX:555) | Yes | Six quads |
| `brep.mesh.plane` | `plane_prim` (MX:557) | Yes | One open quad |
| `brep.mesh.sphere` | `ico_sphere_prim` (MX:563) | Yes | Subdivisions 0 to 5 |
| `brep.mesh.cylinder` | `cylinder_prim` (MX:559) | Yes | Segments 3 to 1024 |
| `brep.mesh.cone` | `cone_prim` (MX:561) | Yes | Segments 3 to 1024 |
| `brep.mesh.fromBrep` | `MeshTessellationJob` (MX:1317) | Yes | Deflection (default 0.1); output tagged `TessellatedMesh` |
| `brep.mesh.toBrep` | `to_brep` (FM:455) | Yes | Planar B-Rep faces, no curved reconstruction; tagged `MeshDerivedBRep` |
| `brep.mesh.translate` | `translate` (MX:572) | Yes | Vector offset |
| `brep.mesh.transform` | `affine_transform_job` (MM:142), `validate_affine_matrix` (MM:149) | Yes | Column-major 4x4, inverse-transpose normals, reflection winding repair |
| `brep.mesh.rotate` | `rotate` (MX:576) | Yes | About origin, radians |
| `brep.mesh.scale` | `scale` (MX:580), `scale_job` (MM:190) | Yes | Negative factors mirror and fix winding |
| `brep.mesh.moveVertices` | `move_vertices` (MX:584) | Yes | Selected vertex indices |
| `brep.mesh.translateComponents` | `move_components_job` (MM:194) | Yes | Vertices, edges or faces; shared vertices move once |
| `brep.mesh.rotateComponents` | `rotate_components_job` (MM:201) | Yes | Pivot selection or point |
| `brep.mesh.scaleComponents` | `scale_components_job` (MM:208) | Yes | Pivot selection or point |
| `brep.mesh.bevel` | `bevel_edges_with_progress` (MX:677), `bevel_job` (MM:347) | Yes | Documented for closed convex meshes only; 1 to 64 segments |
| `brep.mesh.dissolveEdges` | `dissolve_edges` (MX:698), job (MM:166) | Yes | Joins neighboring faces |
| `brep.mesh.dissolveVertices` | `dissolve_vertices` (MX:701), job (MM:173) | Yes | Planar neighborhood only |
| `brep.mesh.mergeVertices` | `merge_vertices` (MX:692), job (MM:324) | Yes | Modes first, center, distance |
| `brep.mesh.moveProportional` | `move_vertices_proportional` (MX:647) | Yes | Linear falloff within radius |
| `brep.mesh.snapVertices` | `snap_vertices_to_grid` (MX:652), job (MM:215) | Yes | Origin-aligned grid |
| `brep.mesh.mirror` | `mirror` (MX:746), `mirror_job` (MM:339) | Yes | Axis x, y or z; welds seam within tolerance |
| `brep.mesh.decimate` | `decimate` (MX:751), `decimate_with_progress` (MX:756), `decimate_job` (MM:366) | Yes | Shortest-edge collapse; may stop before the target ratio (documented) |
| `brep.mesh.mergeCoplanar` | `merge_coplanar_faces` (MX:717), job (MM:162) | Yes | Joins coplanar neighbors |
| `brep.mesh.loopCut` | `loop_cut` (MX:683), job (MM:297) | Yes | 1 to 256 cuts |
| `brep.mesh.knifeCut` | `knife_cut` (MX:688), job (MM:236) | Yes | Projected line through two points |
| `brep.mesh.extrude` | `extrude_faces` (MX:663), `inflate` job (MM:267) | Yes | Along normals with side faces |
| `brep.mesh.inset` | `inset_faces` (MX:667), job (MM:271) | Yes | Positive amount |
| `brep.mesh.subdivide` | `subdivide_faces` (MX:708), job (MM:305) | Yes | Quad to four triangles, boundary preserved |
| `brep.mesh.flip` | `flip_faces` (MX:414), job (MM:287) | Yes | Reverses winding |
| `brep.mesh.deleteFaces` | `delete_faces` (MX:706), job (MM:316) | Yes | Renumbers all indices (rebuild) |
| `brep.mesh.triangulate` | `triangulate` (MX:713), job (MM:312) | Yes | Concave planar polygons supported |
| `brep.mesh.weld` | `weld_coincident_vertices` (MX:725), job (MM:332) | Yes | Tolerance |
| `brep.mesh.orient` | `orient_faces_consistently` (MX:732), job (MM:281) | Yes | Connected components |
| `brep.mesh.fillHoles` | `fill_holes` (MX:741), job (MM:291) | Yes | Caps open boundary loops |
| `brep.mesh.inspectVertex` | `vertex_position` (MX:349) | No | Read-only |
| `brep.mesh.inspectEdge` | `edge_endpoints` (MX:390) | No | Read-only; halfedge index |
| `brep.mesh.inspectFace` | `face_vertex_ids` (MX:359), `face_normal` (MX:378) | No | Read-only; corner ids, unit normal, mean center |
| `brep.mesh.analyze` | `analysis` (FM:507), `analyze` (FM:139) | Yes | Counts: vertices, faces, edges, triangles, boundaryEdges, nonManifoldEdges, inconsistentEdges, degenerateTriangles; area; volume (closed and consistent only); bounds |
| `brep.mesh.exportObj` | `to_obj` (MX:1525), `obj` (FM:511) | Yes | Text |
| `brep.mesh.exportJson` | `to_json` (MX:1561), `json` (FM:512) | Yes | Text (the mesh's own JSON) |

Mesh analysis verdicts: `nonManifoldEdges` and `boundaryEdges` are counts. There is no watertight or manifold boolean output; the caller derives it from counts.

## 5. Kernel capabilities with no flow node

### 5.1 Mesh kernel and mesh-engine

- `set_shading` (MX:787) and `recompute_normals` (MX:794): used only by the lowpoly editor (`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔷️mesh-edit/🦀️.rs:319-320`).
- `unwrap_uv` (MX:1065), `mark_uv_seam` (MX:915), `is_uv_seam` (MX:924): used by the lowpoly UV commands (`.../🎮️commands/🧵️uv/🦀️.rs:23`, `:50`). Not exposed to flow.
- `set_attribute` (MX:326), `set_surface_assets` (MX:318), `corner_uv` (MX:1523): attributes, materials and textures; no flow node.
- `from_indexed_triangles` (MX:416), `from_faces` (MX:485), `from_face_loops` (MX:468), `from_json` (MX:1565): constructors used internally.
- `mesh_uv_sphere` (ME:663) and `mesh_torus` (ME:795): mesh-engine primitives, not exposed (the mesh flow exposes only the icosphere).
- `mesh_to_glb` (ME:971), `mesh_from_glb` (ME:1398): GLB codec, no mesh flow node.
- `mesh_to_stl` (ME:1457), `mesh_from_stl` (ME:1480): STL codec, no mesh flow node.
- `mesh_from_obj` (ME:893): OBJ import, no mesh flow node (B-Rep `importObj` exists instead, producing faceted B-Rep).

### 5.2 B-Rep kernel

- Inertia tensor (`solid_mass_properties`, MP:157-205) and its principal-axis rotation (`rotate_inertia`, MP:340-342): kernel-only.
- Watertight probe (`count_boundary_edges` MP:1638, `watertightness_of_body` MP:1651): kernel-only and only partly used. `watertightness_stub_unchecked` (MP:1657) is a compatibility alias that always returns `NotChecked` (see section 6).
- Face and edge measures (`face_area` MP:372, `edge_length` MP:388), solid-to-solid distance (`distance_solid_solid` MP:404), signed shell volume (`shell_signed_volume` MP:67): kernel-only.
- Scale (`scale`, E:1199): kernel-only (uniform scale is `scale_axes`).
- Persistent-label internals: `handle_for_label` (E:1857), `live_handles` (E:489), `merge_representation` (E:481).

### 5.3 Collision and rigid bodies (not reachable from B-Rep or mesh flow)

- `🧿️collision/🦀️.rs`: `TriMesh`, `intersection_test` (L544), `contains_point` (L620), `contains_point_fast` (L637), `distance_to_surface` (L479). Consumed by the puzzle plugin's precompute, not by B-Rep or mesh flow nodes.
- `🌀️rigid/🦀️.rs`: `Vector3`, `Point3`, `Quaternion`, `UnitQuaternion`, `Isometry3` (poses). No flow node.

## 6. Maturity and known gaps

Each row cites the source of the gap. "Declared" means the claim comes from a doc comment or the quality table, not from a test run.

| Area | Gap | Evidence | Effect on widgets |
|---|---|---|---|
| Sweep, extrude | Elliptical profile edges rejected for extrude | `➡️sweep/🧮️core/🦀️.rs:160` | Ellipse curves cannot be swept or extruded |
| Sweep, extrude | Non-axis circular profile rejected | `🧮️core/🦀️.rs:154` | Tilted circle profile fails |
| Sweep, extrude | Open-wire shell-only extrusion rejected | `➡️sweep/🦀️.rs:77` | Open profiles cannot be extruded to a shell |
| Revolve | Elliptical, NURBS, axis-on and apex-touching profile edges rejected | `➡️sweep/🌀️revolve/🦀️.rs:74, 123, 148, 149` | Revolve has a narrow profile domain |
| Loft | Profiles must be knot-compatible after degree elevation | `➡️sweep/🥞️loft/🦀️.rs:96` | Some profile pairs fail with an error |
| Sweep, pipe, helical | General path uses bounded-error stations | `C:297-305` comment; `ExactNum` tag | Quality claim is tolerance-bounded, not exact |
| Blend | Faces with holes rejected | `K/📐️brep/🛠️operations/🎨️blend/🦀️.rs:1224` | Fillet and chamfer fail on holed faces |
| Booleans | Curved-face section and split not routed through the imprint engine | `BO:118-120`, `BO:169-171` | Section of a cylinder, sphere, cone or torus is outside the exact path |
| Booleans | Multi-loop section output unverified | `BO:113-121` (single planar face from collected points) | Needs a test before widget use on non-convex solids |
| Booleans | Split uses a classified triangle soup with a convex-hull fallback | `BO:164-171` ("hull fallback"; trigger condition not stated in the comment) | Result fidelity depends on the fallback path; verify before widget use |
| Section | Quality tagged `ExactAnalytic` but doc says curved sections are a gap | `C:` table entry for `section` vs `BO:118-120` | Overstated fidelity tag |
| Draft | Non-plane, non-cylinder faces rotated rigidly, not tapered | `K/📐️brep/🛠️operations/↔️offset/🦀️.rs:1332-1336` | Overstated `ExactNum` tag for those faces |
| Offset | NURBS isocurve reparametrization passes through unchanged | `↔️offset/🦀️.rs:735-739` | Offset of NURBS-based faces may be imprecise |
| Fillet variable | One cone p-curve interpolated | `C:297-310` | Within 1e-8 but not analytic |
| Validate | Reports the whole Body, not the selected handle | `E:1586-1599` | Verdict is session-wide, not per shape |
| Watertight | Compatibility alias `watertightness_stub_unchecked` always returns `NotChecked`; the doc calls it a compatibility alias | `MP:1655-1659` | Conflicts with the greenfield no-compat rule; remove it and expose `watertightness_of_body` |
| Mesh flow | Operator tags use a local taxonomy, not `OpQuality` | `FM:816-818` (`operation_quality` returns `PolygonMesh` for everything except `fromBrep` and `toBrep`) | No fidelity signal on 40 of 42 mesh ops |
| Mesh | Element ids are indices, renumbered by rebuild ops | `MX:698-708` (`*self = ...job...`) | Selection by index is fragile across edits |
| Mesh | JSON round trip per node | `FM:83, 115, 129` | Large meshes pay parse and encode on every node |
| Progress and cancel | Only `fuse`, `cut`, `intersect` are jobs in B-Rep | `F:422-523` (macro and job) | Expensive B-Rep ops (fillet, shell, loft, sweep, offset, section) run without progress or cancel, which the AGENTS rules require |
| Naming | Export node names mismatch the kernel method | `brep.io.exportDwg` maps to `export_mesh`, `brep.io.importDwg` to `import_mesh` (F:116-117) | Confusing for widget authors; DWG is a triangle-soup bridge |
| Doc drift | `brep/🚪️io` header says "scaffolded by W1b: structure only" while DWG functions exist | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs:1-3` | Stale comment |

## 7. Missing capabilities for a CAD and mesh end-user

Grouped by what the end-user would expect. Items marked (kernel) already exist in the kernel and need only a flow node.

B-Rep modeling
- 2D sketch with constraints and dimensions; sketch to profile on a plane. (None; the draw plugin is separate.)
- Hole, counterbore, countersink and thread features. (None.)
- Push-pull on a face and move-face. (Only surface `offset_face`; no solid push-pull.)
- Split edge, split face, merge faces, unify same-domain faces. (None.)
- Extrude and revolve for elliptical and NURBS profiles. (Rejected; section 6.)
- Trim, extend, join and offset curve operators. (None.)
- Bezier and NURBS authoring by control points and weights. (Kernel has the types; no node.) (kernel)
- Surface of revolution, extrude-to-surface, knit surfaces. (None.)
- Wedge, pyramid, capsule and tube primitives. (None; prism only via polygon extrusion.)
- Pattern along a curve, and patterns with per-instance variation. (None.)
- General 4x4 affine transform for solids. (Mesh side only.)
- Face selection by query (normal, plane, angle, size). (None; selection is by persistent label only.)
- Variable-radius shell and per-face thickness. (None.)
- Undo and history query from the flow. (`OpDelta` exists in `TOP:501`; no flow node exposes it.) (kernel)

Analysis
- Inertia tensor and principal axes. (kernel, `MP:157`.)
- Surface curvature (Gaussian, mean) and curvature plots. (None; only `curve_curvature` exists.)
- Watertight and manifold verdict as a flow node. (kernel count, `MP:1638`.)
- Interference check as an operator (today a user composes `intersect` and `volume`).
- Minimum wall thickness, draft angle analysis, face-to-face angle. (None.)
- Section profile extraction for drawings. (Section exists but is partly exact; section 6.)

Mesh modeling
- Mesh boolean (union, difference, intersection). (None in kernel or flow.)
- Subdivision surface (Catmull-Clark, Loop) and Laplacian smoothing. (None.)
- Remesh and retopology. (None.)
- UV unwrap, seams and shading as flow nodes. (kernel; lowpoly editor only.)
- Recompute normals and smooth-shading toggle as flow nodes. (kernel; lowpoly editor only.)
- Mesh primitives: torus and UV sphere. (mesh-engine only.)
- Curvature and sharp-edge analysis; self-intersection check. (None.)
- Hole boundary listing (only the fill operation exists).
- Selection by criteria (normal, plane, angle). (None; index lists only.)
- Materials and textures on mesh. (kernel attributes; no node.)

Interchange
- STEP reader accepts only a limited entity set; other entities raise `Unsupported` (`ST:963, ST:1033`). Listed supported entities: planes, cylindrical, conical, spherical and toroidal surfaces, B-spline curves and surfaces, lines, circles, ellipses, faces, edge loops, closed shells, manifold solid breps.
- STL and OBJ import produce faceted B-Rep (one planar face per triangle, `MI:1-6`), not curved geometry.
- DWG is a triangle-soup bridge (`C:258`).
- PLY is a stdio artifact codec (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🦀️.rs`) with no B-Rep or mesh flow node.
- glTF and GLB: kernel `export_glb` and `import_glb` exist (`E:1667`, `E:1672`) but are not on the trait and have no flow node; `export_gltf` is deliberately unexposed (`C` unexposed list).
- Mesh-level STL, GLB, OBJ import: codecs in mesh-engine, no mesh flow node.

Interaction and infrastructure
- Progress and cancel for every expensive B-Rep op (only three are jobs; section 6).
- Persistence of B-Rep documents: STEP round trip and the semio snapshot conversion exist behind the `conversion-brep` feature (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs`), but geometry handles are never persisted (section 8.6).

## 8. Data types and value flow

### 8.1 B-Rep shape representation

- `Body` (`TOP`) is arena-based: vertices, edges (`curve` plus this edge's parameter `range`), coedges (per-face use of an edge, with `pcurve` in face UV), loops (cycle of coedges), faces (surface, orientation), shells, solids, plus pooled `Curve3` and `Curve2` (p-curves) and `Surface` geometry.
- Geometry kinds: line, circle, ellipse, NURBS curves; plane, cylinder, cone, sphere, torus, NURBS surfaces (`C`, `TOP`).
- The session wrapper is `Brep` (`E:417`): `body: Body`, `live: BTreeMap<String, Entity>`, `pending_mutations`.
- `Entity` (`E:404`) variants: `Vertex`, `Edge`, `Wire(Wire, PersistentLabel)`, `Face`, `Shell`, `Solid`, `Compound(Vec<SolidId>, PersistentLabel)`, `Curve(Curve3, PersistentLabel)`, `Surface(Surface, PersistentLabel)`. Wires, compounds, curves and surfaces carry a label minted at registration because they have no arena identity of their own (`E:272-278` comment).

### 8.2 Handles and persistent naming

- `GeometryKind` (`E:55`): Vertex, Edge, Wire, Face, Shell, Solid, Compound, Curve, Surface.
- `GeometryHandle(String)` (`E:71`) is minted as `hash(kind, PersistentLabel)` (`E:622-628`): deterministic for a given entity, never a counter.
- `PersistentLabel(u64)` (`TOP:432`) is issued from a per-Body monotonic counter (`LabelSource`, `TOP:440-450`) and never reused. The label survives arena compaction. Labels are the identity the document layer keys off.
- `OpRecorder` (`TOP:501`) accumulates `OpDelta { generated, modified, deleted }` per operation.
- Component selection by label: a JSON array of labels is parsed (`F:415-431`) and resolved to live handles (`handle_for_label`, `E:1857`). Missing labels error: "component label ... no longer exists".
- Scope: labels are unique per Body. Handle equality across two Bodies depends on the session boundary (one `Brep` per `Session`), not on the handle scheme alone.

### 8.3 Flow values for geometry

- A geometry value is a dictionary with schema `geometry`: `{handle: text, kind: text}` (`SS:41-43`, `SS:372-379`). Lists of geometry are list dictionaries of such entries (`SS:407-411`).
- Points, vectors and numbers are dictionaries (`SS:46-65`).
- Operators read a handle with `read_geometry` (`SS:79`) and resolve it through `with_kernel` (`SS:1025`), which takes a write lock on the kernel and records the authority's claims.

### 8.4 Session lifetime and retirement

- `Session` (`SS:808`) owns `SessionState` (`SS:882-891`): `kernel: RwLock<Brep>`, `mesh_cache` (keyed by handle and tolerance), `claims` (handles per authority), `jobs` (tessellation job registry), `closed` flag.
- `with_kernel` refuses with `geometry.session-closed` after close (`SS:1025-1032`).
- Handles are released through `dispose_sync` (`E:1841`), which runs the arena GC only when no other live handle reaches the entity. `retain` drops registry entries not in a live set.
- Operator outputs are retired through the `retire_geometry_capture!` macro (`F:135-139`), used across the B-Rep flow file.

### 8.5 Mesh representation

- `HalfedgeMesh` (`MX:263`): `vertices: Vec<MeshVertex>`, `halfedges: Vec<HalfEdge>`, `faces: Vec<MeshFace>`, `uv_seams: HashSet<u32>`, `attributes: BTreeMap<String, MeshAttribute>`, `materials`, `textures`. Half-edge topology, polygon faces, n-gons allowed.
- Ids (`MX:109-124`): `VertexId(u32)`, `HalfEdgeId(u32)`, `FaceId(u32)`, `EdgeId(u32)`, all transparent indices into the arrays. Ids are not persistent. Rebuild operations (`delete_faces`, `dissolve_*`, `subdivide`, `merge_*`) replace the whole structure (`*self = ...`, `MX:698-708`), so indices change after those ops.
- Selection in flow: a JSON array of indices, parsed per call (`FM:38`), bounded to 600000 entries.
- Flow value: a dictionary with schema `mesh`, fields `data` (JSON text from `encode_mesh`, `FM:115`, `ME:95`) and `preview` (packed text, `FM:360`). Each operator decodes and re-encodes the whole mesh (`FM:83`, `FM:129-131`).
- Attributes and surface assets (normals, UVs, colors, materials, textures) exist in the kernel (`MX:318-326`) and mesh-engine (`ME:26-77`) and are carried on the mesh. The `transform` node updates normals through its affine job (`MM:142`); no other flow node edits attributes, materials or textures.

### 8.6 Persistence and interchange

- Geometry handles are session-scoped and in memory. They do not survive process restart.
- Durable forms for B-Rep: STEP AP214 (writer `ST:1129`, reader `ST:1134`, subset limited as in section 7), the semio snapshot conversion behind feature `conversion-brep`, and DWG or triangle formats (tessellated, lossy).
- Durable form for mesh: JSON (`exportJson`) and OBJ (`exportObj`) text only.

## 9. IO inventory

| Format | Import | Export | Fidelity | Path and node |
|---|---|---|---|---|
| STEP (AP214 subset) | Yes, limited entity set | Yes | ExactAnalytic (codec) | `ST:1129, 1134`; nodes `brep.io.importStep` (F:2025), `brep.io.exportStep` (F:1991) |
| STL (B-Rep) | Yes, faceted | Yes, tessellated | MeshDerived | kernel `import_stl` (E:1682), `export_stl` (E:1653); nodes F:1998, F:2028 |
| OBJ (B-Rep) | Yes, faceted | Yes, tessellated | MeshDerived | kernel `import_obj` (E:1687), `export_obj` (E:1658); nodes F:2013, F:2040 |
| glTF and GLB | Kernel only, not on trait | Kernel only, deliberately unexposed | MeshDerived | `export_glb` (E:1667), `import_glb` (E:1672), `export_gltf` (E:1663) |
| DWG | Yes, triangle bridge | Yes, triangle bridge | MeshDerived | `export_mesh` (E:1708), `import_mesh` (E:1713); nodes F:2053, F:2067; codec `🚪️io/🦀️.rs:140-159` |
| Mesh OBJ | Codec only | Yes (`to_obj`) | PolygonMesh | `MX:1525`; node `brep.mesh.exportObj` (FM) |
| Mesh JSON | Yes (`brep.mesh.construct`, parsed by `decode_mesh` at `FM:83`) | Yes (`to_json`, `MX:1561`) | PolygonMesh | nodes `brep.mesh.construct` and `brep.mesh.exportJson` |
| Mesh GLB, STL | Codec only | Codec only | PolygonMesh | `ME:971, 1398, 1457, 1480`; no flow node |
| PLY | Codec only (stdio artifact) | Codec only | n/a | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/`; no flow node |

## 10. Test and oracle notes

- Runtime dependencies are first-party only for the kernel crates (`K` Cargo manifests and `📐️geometry`, `🏗️mesh-engine`).
- Third-party oracles are dev-dependencies only: `parry3d` 0.17 (B-Rep and B-Rep flow), `kurbo` 0.13.1 (2D geometry), `gltf` 1.4.1 (mesh-engine). This matches the AGENTS rule of validating own implementation against a third-party library.
- Test trees exist for most families: primitives, curves, surfaces, booleans (with `coincident-boundary` and `coincident-seam` fixtures), sweeps, offset, blend, tessellation, mass properties (with `oracle-unit` and `oracle-standalone`), mesh modeling, mesh jobs, mesh-engine glTF oracle differential.
- Not run in this audit: no cargo, nx or test execution.

## 11. Things not verified

- Maturity claims rely on declared metadata (`OPERATION_QUALITY`, doc comments, `unsupported` error strings), not on test runs.
- The lowpoly editor consumers of mesh UV and normal functions were confirmed by grep, not by reading the full editor.
- The multi-loop section behavior (section 6) is flagged for a test, not confirmed as a bug.
- Mesh job progress and cancel were confirmed from `step_plan` structure, not from a runtime probe.

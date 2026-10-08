# 🗂️ Lane C — Geometry Widget Catalogue (AD1)

Session 6, 2026-10-07/08. Lane C of the PROCEDURAL-3D-END-TO-END fleet. Plan: `📓️brep-mesh-widget-set-2026-10-07.md` (AD1).

## 1. File layout

Folder `C` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue/`

| Path (relative to `C`) | Role |
|---|---|
| `🔣️.json` | JSON Schema draft-07 meta-schema of one category file (`Generation3dCatalogueCategory`): kind, port, selection, enum option, pick, gumball, interaction, quality |
| `🔣️<category-slug>.json` x 27 | One file per category (`{ category, kinds[] }`), slug = category id with `.` -> `-`, e.g. `🔣️brep-primitive.json` |
| `🦀️.rs` | Rust typed loader (`ToValue`/`FromValue`), `include_str!` of the 27 files, `catalogue()`, lookups, `check()` semantic laws |
| `🟦️.ts` | TypeScript types, loader (`catalogue()`, `buildCatalogue`, `resolveCatalogueText`) and `checkCatalogue()` with the same laws |
| `🧫️fixtures/🔣️.json` | Language-agnostic laws read by both runners: kernel/mesh coverage tables, lookups, a valid base category and 35 mutated copies with expected findings, 9 schema rejections |
| `🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🔬️unit/🟦️.ts` | Rust (9 tests) and TypeScript (43 tests) runners |

Wiring: artifact root `🦀️.rs` mounts `standards::v1::subsets::any::schema::catalogue` via `#[path]`; artifact root `🟦️.ts` re-exports the TS loader; `📦️packages/🟦️typescript/📜️script.ts` registers the TS suite. generation3d has no dependency on the flow extension crate.

Taxonomy: `🔣️taxonomy.json` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`) gained two `semanticDirectoryKinds`: `schema-catalogue` (emoji 🗂️, `^catalogue$`, parent `schema`) and `schema-catalogue-category` (emoji 🔣️, `^(brep|mesh|math|analysis)-[a-z]+$`, parent `schema-catalogue`). The flow artifact does NOT use a `🗂️catalogue` folder (only a `flow-operator-catalogue` kind under `members-of-modules`), so admission was required. See section 7 for verification status.

Authoring sources (kept, not part of the product): `🐍️lane-c-catalogue/` in the ticket folder (compact JS authoring tables + generator `🐍️author.mjs`, `🐍️fixture.mjs`, `🐍️mapping-report.py`). The committed JSON files are the source of truth from now on; do not regenerate over later hand edits.

**Post-delivery change by another agent (not mine, observed at the end):** the IO parts were moved out of the schema leaf into `🚪️io/📝️text/🗂️catalogue/` (`🦀️.rs` holds `CATEGORY_SOURCES`, `parse_catalogue`, `bundled_catalogue`, `catalogue() -> &'static Arc<Catalogue>`; `🟦️.ts` the TS counterpart) and my two runners moved to `🚪️io/📝️text/🗂️catalogue/🧪️tests/🔬️unit/`; the schema leaf keeps the types, `Catalogue::from_categories` and `check()`. The names listed in section 4 are unchanged, except that `Catalogue::parse`/`bundled` are now `parse_catalogue`/`bundled_catalogue` in the io leaf. The TS runner at its new location was re-run: 43 pass. The Rust numbers below are from my original layout and were not re-run after the move. Compute lanes also refined some category-file descriptions to match kernel behavior (for example `brep.feature.filletVariable`).

## 2. Ids per category (199 kinds, 27 categories)

**`brep.primitive`** (Primitives / Grundkörper), 6 kinds, file `🔣️brep-primitive.json`

`brep.primitive.box`, `brep.primitive.sphere`, `brep.primitive.cylinder`, `brep.primitive.cone`, `brep.primitive.torus`, `brep.primitive.convexHull`

**`brep.curve`** (Curves and wires / Kurven und Kantenzüge), 10 kinds, file `🔣️brep-curve.json`

`brep.curve.line`, `brep.curve.circle`, `brep.curve.arc`, `brep.curve.ellipse`, `brep.curve.polyline`, `brep.curve.rectangle`, `brep.curve.polygon`, `brep.curve.interpolate`, `brep.curve.approximate`, `brep.curve.helix`

**`brep.surface`** (Surfaces and faces / Flächen), 7 kinds, file `🔣️brep-surface.json`

`brep.surface.plane`, `brep.surface.planarFace`, `brep.surface.planarFaceFromWire`, `brep.surface.faceFromWire`, `brep.surface.nurbsGrid`, `brep.surface.coons`, `brep.surface.offset`

**`brep.solid`** (Solids from profiles / Körper aus Profilen), 7 kinds, file `🔣️brep-solid.json`

`brep.solid.extrudeWire`, `brep.solid.extrudeFace`, `brep.solid.revolve`, `brep.solid.loft`, `brep.solid.sweep`, `brep.solid.pipe`, `brep.solid.helicalSweep`

**`brep.boolean`** (Booleans / Boolesche Operationen), 4 kinds, file `🔣️brep-boolean.json`

`brep.boolean.fuse`, `brep.boolean.cut`, `brep.boolean.intersect`, `brep.boolean.compoundCut`

**`brep.feature`** (Features / Features), 11 kinds, file `🔣️brep-feature.json`

`brep.feature.fillet`, `brep.feature.filletEdges`, `brep.feature.filletVariable`, `brep.feature.chamfer`, `brep.feature.chamferEdges`, `brep.feature.chamferAsymmetric`, `brep.feature.shell`, `brep.feature.draft`, `brep.feature.offsetSolid`, `brep.feature.thicken`, `brep.feature.defeature`

**`brep.transform`** (Transforms and patterns / Transformationen und Muster), 9 kinds, file `🔣️brep-transform.json`

`brep.transform.translate`, `brep.transform.rotate`, `brep.transform.rotateAbout`, `brep.transform.scale`, `brep.transform.mirror`, `brep.transform.copy`, `brep.transform.linearPattern`, `brep.transform.circularPattern`, `brep.transform.gridPattern`

**`brep.intersect`** (Sections and intersections / Schnitte und Schnittmengen), 5 kinds, file `🔣️brep-intersect.json`

`brep.intersect.section`, `brep.intersect.split`, `brep.intersect.curveCurve`, `brep.intersect.curveSurface`, `brep.intersect.surfaceSurface`

**`brep.evaluate`** (Evaluate curves and surfaces / Kurven und Flächen auswerten), 8 kinds, file `🔣️brep-evaluate.json`

`brep.evaluate.curvePoint`, `brep.evaluate.curveTangent`, `brep.evaluate.curveDomain`, `brep.evaluate.curveCurvature`, `brep.evaluate.surfacePoint`, `brep.evaluate.surfaceNormal`, `brep.evaluate.curveClosestParameter`, `brep.evaluate.surfaceClosestUv`

**`brep.topology`** (Topology and repair / Topologie und Reparatur), 9 kinds, file `🔣️brep-topology.json`

`brep.topology.vertex`, `brep.topology.deconstruct`, `brep.topology.shells`, `brep.topology.compound`, `brep.topology.explode`, `brep.topology.label`, `brep.topology.sew`, `brep.topology.heal`, `brep.topology.convertToNurbs`

**`brep.interchange`** (Import and export / Import und Export), 8 kinds, file `🔣️brep-interchange.json`

`brep.interchange.exportStep`, `brep.interchange.importStep`, `brep.interchange.exportStl`, `brep.interchange.importStl`, `brep.interchange.exportObj`, `brep.interchange.importObj`, `brep.interchange.exportDwg`, `brep.interchange.importDwg`

**`mesh.primitive`** (Mesh primitives / Netz-Grundformen), 8 kinds, file `🔣️mesh-primitive.json`

`mesh.primitive.construct`, `mesh.primitive.box`, `mesh.primitive.plane`, `mesh.primitive.sphere`, `mesh.primitive.cylinder`, `mesh.primitive.cone`, `mesh.primitive.torus`, `mesh.primitive.uvSphere`

**`mesh.convert`** (Mesh conversion / Netz-Umwandlung), 2 kinds, file `🔣️mesh-convert.json`

`mesh.convert.fromBrep`, `mesh.convert.toBrep`

**`mesh.transform`** (Mesh transforms / Netz-Transformationen), 5 kinds, file `🔣️mesh-transform.json`

`mesh.transform.translate`, `mesh.transform.rotate`, `mesh.transform.scale`, `mesh.transform.matrix`, `mesh.transform.mirror`

**`mesh.component`** (Mesh components / Netzelemente), 6 kinds, file `🔣️mesh-component.json`

`mesh.component.moveVertices`, `mesh.component.translate`, `mesh.component.rotate`, `mesh.component.scale`, `mesh.component.moveProportional`, `mesh.component.snapToGrid`

**`mesh.edit`** (Mesh editing / Netzbearbeitung), 12 kinds, file `🔣️mesh-edit.json`

`mesh.edit.bevel`, `mesh.edit.dissolveEdges`, `mesh.edit.dissolveVertices`, `mesh.edit.mergeVertices`, `mesh.edit.loopCut`, `mesh.edit.knifeCut`, `mesh.edit.extrude`, `mesh.edit.inset`, `mesh.edit.subdivide`, `mesh.edit.flip`, `mesh.edit.deleteFaces`, `mesh.edit.triangulate`

**`mesh.repair`** (Mesh repair and cleanup / Netzreparatur und Bereinigung), 5 kinds, file `🔣️mesh-repair.json`

`mesh.repair.weld`, `mesh.repair.orient`, `mesh.repair.fillHoles`, `mesh.repair.mergeCoplanar`, `mesh.repair.decimate`

**`mesh.inspect`** (Mesh inspection / Netzinspektion), 3 kinds, file `🔣️mesh-inspect.json`

`mesh.inspect.vertex`, `mesh.inspect.edge`, `mesh.inspect.face`

**`mesh.interchange`** (Mesh import and export / Netz-Import und -Export), 7 kinds, file `🔣️mesh-interchange.json`

`mesh.interchange.exportObj`, `mesh.interchange.exportJson`, `mesh.interchange.exportStl`, `mesh.interchange.exportGlb`, `mesh.interchange.importObj`, `mesh.interchange.importStl`, `mesh.interchange.importGlb`

**`mesh.shading`** (Mesh shading / Netzschattierung), 2 kinds, file `🔣️mesh-shading.json`

`mesh.shading.setShading`, `mesh.shading.recomputeNormals`

**`mesh.uv`** (Mesh UV mapping / Netz-UV-Abwicklung), 2 kinds, file `🔣️mesh-uv.json`

`mesh.uv.markSeams`, `mesh.uv.unwrap`

**`analysis.measure`** (Measure / Messen), 11 kinds, file `🔣️analysis-measure.json`

`analysis.volume`, `analysis.area`, `analysis.length`, `analysis.centroid`, `analysis.boundingBox`, `analysis.massProperties`, `analysis.distance`, `analysis.closestPoint`, `analysis.classifyPoint`, `analysis.angle`, `analysis.selectionArea`

**`analysis.check`** (Check / Prüfen), 4 kinds, file `🔣️analysis-check.json`

`analysis.validity`, `analysis.topologyCounts`, `analysis.interference`, `analysis.meshQuality`

**`math.values`** (Values / Werte), 8 kinds, file `🔣️math-values.json`

`math.number`, `math.integer`, `math.angle`, `math.length`, `math.boolean`, `math.vector`, `math.point`, `math.plane`

**`math.arithmetic`** (Arithmetic / Arithmetik), 17 kinds, file `🔣️math-arithmetic.json`

`math.add`, `math.subtract`, `math.multiply`, `math.divide`, `math.modulo`, `math.power`, `math.negate`, `math.absolute`, `math.squareRoot`, `math.round`, `math.minimum`, `math.maximum`, `math.clamp`, `math.interpolate`, `math.sine`, `math.cosine`, `math.tangent`

**`math.vector`** (Vectors, points and planes / Vektoren, Punkte und Ebenen), 19 kinds, file `🔣️math-vector.json`

`math.vectorFromComponents`, `math.pointFromComponents`, `math.vectorComponents`, `math.pointComponents`, `math.vectorAdd`, `math.vectorSubtract`, `math.vectorScale`, `math.vectorLength`, `math.vectorNormalize`, `math.vectorDot`, `math.vectorCross`, `math.vectorAngle`, `math.pointOffset`, `math.pointDistance`, `math.pointInterpolate`, `math.vectorBetween`, `math.planeFromPointNormal`, `math.planeFromPoints`, `math.planeComponents`

**`math.list`** (Ranges and lists / Bereiche und Listen), 4 kinds, file `🔣️math-list.json`

`math.range`, `math.series`, `math.listItem`, `math.listLength`

New kinds (not in the old registrations, 64): `mesh.primitive.torus`, `mesh.primitive.uvSphere`, `mesh.interchange.exportStl`, `mesh.interchange.exportGlb`, `mesh.interchange.importObj`, `mesh.interchange.importStl`, `mesh.interchange.importGlb`, `mesh.shading.setShading`, `mesh.shading.recomputeNormals`, `mesh.uv.markSeams`, `mesh.uv.unwrap`, `analysis.massProperties`, `analysis.angle`, `analysis.selectionArea`, `analysis.topologyCounts`, `analysis.interference`, `math.number`, `math.integer`, `math.angle`, `math.length`, `math.boolean`, `math.vector`, `math.point`, `math.plane`, `math.add`, `math.subtract`, `math.multiply`, `math.divide`, `math.modulo`, `math.power`, `math.negate`, `math.absolute`, `math.squareRoot`, `math.round`, `math.minimum`, `math.maximum`, `math.clamp`, `math.interpolate`, `math.sine`, `math.cosine`, `math.tangent`, `math.vectorFromComponents`, `math.pointFromComponents`, `math.vectorComponents`, `math.pointComponents`, `math.vectorAdd`, `math.vectorSubtract`, `math.vectorScale`, `math.vectorLength`, `math.vectorNormalize`, `math.vectorDot`, `math.vectorCross`, `math.vectorAngle`, `math.pointOffset`, `math.pointDistance`, `math.pointInterpolate`, `math.vectorBetween`, `math.planeFromPointNormal`, `math.planeFromPoints`, `math.planeComponents`, `math.range`, `math.series`, `math.listItem`, `math.listLength`

## 3. Old id -> new id (135 rows; 93 B-Rep + 42 mesh, bijective)

Port columns list only differences. `in:`/`out:` use the old and new port names. Old defaults were mostly absent for points (required connection); the catalogue gives every input a default except shape/shapes/mesh inputs and optional ports. Ports not listed keep their name.

| Old id | New id | Port renames / notes |
|---|---|---|
| `brep.prim3d.box` | `brep.primitive.box` | out: `solid` -> `shape` |
| `brep.prim3d.sphere` | `brep.primitive.sphere` | out: `solid` -> `shape` |
| `brep.prim3d.cylinder` | `brep.primitive.cylinder` | out: `solid` -> `shape` |
| `brep.prim3d.cone` | `brep.primitive.cone` | out: `solid` -> `shape` |
| `brep.prim3d.torus` | `brep.primitive.torus` | out: `solid` -> `shape` |
| `brep.prim3d.convexHull` | `brep.primitive.convexHull` | out: `solid` -> `shape` |
| `brep.curve.line` | `brep.curve.line` | out: `curve` -> `shape` |
| `brep.curve.circle` | `brep.curve.circle` | out: `curve` -> `shape`; normal is a vector port (was a point port) |
| `brep.curve.arc` | `brep.curve.arc` | out: `curve` -> `shape` |
| `brep.curve.ellipse` | `brep.curve.ellipse` | out: `curve` -> `shape` |
| `brep.curve.polyline` | `brep.curve.polyline` | out: `wire` -> `shape` |
| `brep.curve.rectangle` | `brep.curve.rectangle` | out: `wire` -> `shape` |
| `brep.curve.polygon` | `brep.curve.polygon` | out: `wire` -> `shape` |
| `brep.curve.interpolate` | `brep.curve.interpolate` | out: `curve` -> `shape` |
| `brep.curve.approximate` | `brep.curve.approximate` | out: `curve` -> `shape` |
| `brep.curve.helix` | `brep.curve.helix` | out: `curve` -> `shape`; the kernel returns a polyline wire (quality tag lowered from exact-analytic to approximate), the old registration declared a curve |
| `brep.surf.plane` | `brep.surface.plane` | in: removed `origin`, `normal`; added `plane`; out: `surface` -> `shape`; origin and normal merged into one plane port |
| `brep.surf.planarFace` | `brep.surface.planarFace` | out: `face` -> `shape` |
| `brep.surf.planarFaceWire` | `brep.surface.planarFaceFromWire` | out: `face` -> `shape` |
| `brep.util.faceFromWire` | `brep.surface.faceFromWire` | out: `face` -> `shape` |
| `brep.surf.nurbsGrid` | `brep.surface.nurbsGrid` | out: `surface` -> `shape`; defaults changed to a 3x3 grid with degree 2 so a new widget evaluates |
| `brep.surf.coons` | `brep.surface.coons` | in: removed `curves`; added `bottom`, `right`, `top`, `left`; out: `surface` -> `shape`; the single nested list port becomes four point-list ports, in kernel order bottom, right, top, left |
| `brep.surf.offset` | `brep.surface.offset` | out: `faceOut` -> `shape` |
| `brep.solid.extrude` | `brep.solid.extrudeWire` | out: `solid` -> `shape` |
| `brep.sweep.extrude` | `brep.solid.extrudeFace` | out: `solid` -> `shape` |
| `brep.sweep.revolve` | `brep.solid.revolve` | out: `solid` -> `shape` |
| `brep.sweep.loft` | `brep.solid.loft` | out: `solid` -> `shape`; smooth was a number (0 or 1), now a boolean |
| `brep.sweep.sweep` | `brep.solid.sweep` | out: `solid` -> `shape` |
| `brep.sweep.pipe` | `brep.solid.pipe` | out: `solid` -> `shape` |
| `brep.sweep.helical` | `brep.solid.helicalSweep` | out: `solid` -> `shape` |
| `brep.bool.fuse` | `brep.boolean.fuse` | out: `solid` -> `shape` |
| `brep.bool.cut` | `brep.boolean.cut` | out: `solid` -> `shape` |
| `brep.bool.intersect` | `brep.boolean.intersect` | out: `solid` -> `shape` |
| `brep.bool.compoundCut` | `brep.boolean.compoundCut` | out: `solid` -> `shape` |
| `brep.solid.fillet` | `brep.feature.fillet` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.filletEdges` | `brep.feature.filletEdges` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.filletVariable` | `brep.feature.filletVariable` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.chamfer` | `brep.feature.chamfer` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.chamferEdges` | `brep.feature.chamferEdges` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.chamferAsymmetric` | `brep.feature.chamferAsymmetric` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.shell` | `brep.feature.shell` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.draft` | `brep.feature.draft` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.solid.offsetSolid` | `brep.feature.offsetSolid` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.surf.thicken` | `brep.feature.thicken` | out: `solid` -> `shape` |
| `brep.solid.defeature` | `brep.feature.defeature` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.xform.translate` | `brep.transform.translate` | in: `geometry` -> `shape`; out: `geometryOut` -> `shape`; offset is a vector port (was a point port) |
| `brep.xform.rotate` | `brep.transform.rotate` | in: `geometry` -> `shape`; out: `geometryOut` -> `shape` |
| `brep.xform.rotateAbout` | `brep.transform.rotateAbout` | in: `geometry` -> `shape`; out: `geometryOut` -> `shape` |
| `brep.xform.scale` | `brep.transform.scale` | in: `geometry` -> `shape`; out: `geometryOut` -> `shape` |
| `brep.xform.mirror` | `brep.transform.mirror` | in: removed `geometry`, `origin`, `normal`; added `shape`, `plane`; out: `geometryOut` -> `shape`; origin and normal merged into one plane port |
| `brep.xform.copy` | `brep.transform.copy` | in: `geometry` -> `shape`; out: `geometryOut` -> `shape` |
| `brep.xform.linearPattern` | `brep.transform.linearPattern` | in: `geometry` -> `shape`; out: `compound` -> `shape`; direction is a vector port (was a point port) |
| `brep.xform.circularPattern` | `brep.transform.circularPattern` | in: `geometry` -> `shape`; out: `compound` -> `shape` |
| `brep.xform.gridPattern` | `brep.transform.gridPattern` | in: `geometry` -> `shape`; out: `compound` -> `shape` |
| `brep.intersect.section` | `brep.intersect.section` | in: removed `planeOrigin`, `planeNormal`; added `plane`; planeOrigin and planeNormal merged into one plane port; faces becomes a shapes output |
| `brep.intersect.split` | `brep.intersect.split` | in: removed `planeOrigin`, `planeNormal`; added `plane`; planeOrigin and planeNormal merged into one plane port |
| `brep.intersect.curveCurve` | `brep.intersect.curveCurve` | out: added `points` |
| `brep.intersect.curveSurface` | `brep.intersect.curveSurface` | out: added `points` |
| `brep.intersect.surfaceSurface` | `brep.intersect.surfaceSurface` |  |
| `brep.eval.curvePoint` | `brep.evaluate.curvePoint` |  |
| `brep.eval.curveTangent` | `brep.evaluate.curveTangent` |  |
| `brep.eval.curveDomain` | `brep.evaluate.curveDomain` | out: added `start`, `end` |
| `brep.eval.curveCurvature` | `brep.evaluate.curveCurvature` |  |
| `brep.eval.surfPoint` | `brep.evaluate.surfacePoint` |  |
| `brep.eval.surfNormal` | `brep.evaluate.surfaceNormal` |  |
| `brep.eval.curveClosestParameter` | `brep.evaluate.curveClosestParameter` | out: `pointOut` -> `point` |
| `brep.eval.surfaceClosestUv` | `brep.evaluate.surfaceClosestUv` | out: `pointOut` -> `point` |
| `brep.util.vertex` | `brep.topology.vertex` | out: `vertex` -> `shape` |
| `brep.brep` | `brep.topology.deconstruct` | in: removed `brep`, `edgeLabels`, `faceLabels`, `sourceHandle`; added `shape`, `edges`, `faces`; out: `V,E,F,S,SE,SF,SI` -> `vertices, edges, faces, shells, selectedEdges, selectedFaces` (sourceIndex dropped); label texts become selection ports |
| `brep.topology.shells` | `brep.topology.shells` |  |
| `brep.topology.compound` | `brep.topology.compound` | out: `compound` -> `shape` |
| `brep.topology.explode` | `brep.topology.explode` |  |
| `brep.topology.label` | `brep.topology.label` | in: `geometry` -> `shape` |
| `brep.util.sew` | `brep.topology.sew` | out: `solid` -> `shape` |
| `brep.util.heal` | `brep.topology.heal` | in: `geometry` -> `shape`; out: `solid` -> `shape` |
| `brep.util.convertToNurbs` | `brep.topology.convertToNurbs` | in: `geometry` -> `shape`; out: `geometryOut` -> `shape` |
| `brep.io.exportStep` | `brep.interchange.exportStep` | in: `geometry` -> `shape` |
| `brep.io.importStep` | `brep.interchange.importStep` | out: `geometry` -> `shape` |
| `brep.io.exportStl` | `brep.interchange.exportStl` | in: `geometry` -> `shape` |
| `brep.io.importStl` | `brep.interchange.importStl` | out: `geometry` -> `shape` |
| `brep.io.exportObj` | `brep.interchange.exportObj` | in: `geometry` -> `shape` |
| `brep.io.importObj` | `brep.interchange.importObj` | out: `geometry` -> `shape` |
| `brep.io.exportDwg` | `brep.interchange.exportDwg` | in: `geometry` -> `shape` |
| `brep.io.importDwg` | `brep.interchange.importDwg` | out: `geometry` -> `shape` |
| `brep.mesh.construct` | `mesh.primitive.construct` | out: `meshOut` -> `mesh` |
| `brep.mesh.box` | `mesh.primitive.box` | out: `meshOut` -> `mesh` |
| `brep.mesh.plane` | `mesh.primitive.plane` | out: `meshOut` -> `mesh` |
| `brep.mesh.sphere` | `mesh.primitive.sphere` | out: `meshOut` -> `mesh` |
| `brep.mesh.cylinder` | `mesh.primitive.cylinder` | out: `meshOut` -> `mesh` |
| `brep.mesh.cone` | `mesh.primitive.cone` | out: `meshOut` -> `mesh` |
| `brep.mesh.fromBrep` | `mesh.convert.fromBrep` | in: `geometry` -> `shape`; out: `meshOut` -> `mesh` |
| `brep.mesh.toBrep` | `mesh.convert.toBrep` | out: `geometry` -> `shape` |
| `brep.mesh.translate` | `mesh.transform.translate` | out: `meshOut` -> `mesh` |
| `brep.mesh.rotate` | `mesh.transform.rotate` | out: `meshOut` -> `mesh`; angle default changed from 0 to a quarter pi |
| `brep.mesh.scale` | `mesh.transform.scale` | out: `meshOut` -> `mesh` |
| `brep.mesh.transform` | `mesh.transform.matrix` | out: `meshOut` -> `mesh` |
| `brep.mesh.mirror` | `mesh.transform.mirror` | out: `meshOut` -> `mesh`; axis is now an enum port (was free text) |
| `brep.mesh.moveVertices` | `mesh.component.moveVertices` | out: `meshOut` -> `mesh` |
| `brep.mesh.translateComponents` | `mesh.component.translate` | out: `meshOut` -> `mesh` |
| `brep.mesh.rotateComponents` | `mesh.component.rotate` | out: `meshOut` -> `mesh`; angle default changed from 0 to a quarter pi |
| `brep.mesh.scaleComponents` | `mesh.component.scale` | out: `meshOut` -> `mesh` |
| `brep.mesh.moveProportional` | `mesh.component.moveProportional` | in: `selection` -> `vertices`; out: `meshOut` -> `mesh` |
| `brep.mesh.snapVertices` | `mesh.component.snapToGrid` | in: `selection` -> `vertices`; out: `meshOut` -> `mesh` |
| `brep.mesh.bevel` | `mesh.edit.bevel` | out: `meshOut` -> `mesh` |
| `brep.mesh.dissolveEdges` | `mesh.edit.dissolveEdges` | out: `meshOut` -> `mesh` |
| `brep.mesh.dissolveVertices` | `mesh.edit.dissolveVertices` | in: `selection` -> `vertices`; out: `meshOut` -> `mesh` |
| `brep.mesh.mergeVertices` | `mesh.edit.mergeVertices` | in: `selection` -> `vertices`; out: `meshOut` -> `mesh`; mode is now an enum port (was free text) |
| `brep.mesh.loopCut` | `mesh.edit.loopCut` | out: `meshOut` -> `mesh` |
| `brep.mesh.knifeCut` | `mesh.edit.knifeCut` | out: `meshOut` -> `mesh` |
| `brep.mesh.extrude` | `mesh.edit.extrude` | out: `meshOut` -> `mesh` |
| `brep.mesh.inset` | `mesh.edit.inset` | out: `meshOut` -> `mesh` |
| `brep.mesh.subdivide` | `mesh.edit.subdivide` | out: `meshOut` -> `mesh` |
| `brep.mesh.flip` | `mesh.edit.flip` | out: `meshOut` -> `mesh` |
| `brep.mesh.deleteFaces` | `mesh.edit.deleteFaces` | out: `meshOut` -> `mesh` |
| `brep.mesh.triangulate` | `mesh.edit.triangulate` | out: `meshOut` -> `mesh` |
| `brep.mesh.weld` | `mesh.repair.weld` | out: `meshOut` -> `mesh` |
| `brep.mesh.orient` | `mesh.repair.orient` | out: `meshOut` -> `mesh` |
| `brep.mesh.fillHoles` | `mesh.repair.fillHoles` | out: `meshOut` -> `mesh` |
| `brep.mesh.mergeCoplanar` | `mesh.repair.mergeCoplanar` | out: `meshOut` -> `mesh` |
| `brep.mesh.decimate` | `mesh.repair.decimate` | out: `meshOut` -> `mesh` |
| `brep.mesh.inspectVertex` | `mesh.inspect.vertex` | in: `index` -> `vertex`; out: `point` -> `position` |
| `brep.mesh.inspectEdge` | `mesh.inspect.edge` | in: `index` -> `edge` |
| `brep.mesh.inspectFace` | `mesh.inspect.face` | in: `index` -> `face` |
| `brep.mesh.exportObj` | `mesh.interchange.exportObj` |  |
| `brep.mesh.exportJson` | `mesh.interchange.exportJson` |  |
| `brep.measure.volume` | `analysis.volume` | in: `geometry` -> `solid` |
| `brep.measure.area` | `analysis.area` | in: `geometry` -> `shape` |
| `brep.measure.length` | `analysis.length` | in: `geometry` -> `curve` |
| `brep.measure.centerOfMass` | `analysis.centroid` | in: `geometry` -> `shape`; out: `center` -> `centroid` |
| `brep.measure.boundingBox` | `analysis.boundingBox` | in: `geometry` -> `shape`; out: added `min`, `max`, `size` |
| `brep.measure.distance` | `analysis.distance` |  |
| `brep.measure.closestPoint` | `analysis.closestPoint` | in: `geometry` -> `shape`; out: `pointOut` -> `point` |
| `brep.measure.classify` | `analysis.classifyPoint` |  |
| `brep.measure.validate` | `analysis.validity` | in: `geometry` -> `shape`; out: added `valid`, `watertight` |
| `brep.mesh.analyze` | `analysis.meshQuality` | out: added `closed` |

Cross-cutting changes for the example rewrite:
- `geometry` input -> `shape` on transforms, features, topology, interchange exports; geometry outputs are all named `shape` (old `solid`, `curve`, `wire`, `face`, `surface`, `faceOut`, `geometryOut`, `compound`, `geometry`, `vertex`); mesh outputs are `mesh` (old `meshOut`).
- Plane inputs: `origin` + `normal` / `planeOrigin` + `planeNormal` became ONE `plane` port `{origin:[x,y,z], normal:[x,y,z]}` (surface.plane, transform.mirror, intersect.section, intersect.split).
- Directions and axes that were point ports are `vector` ports now; lengths are `length`, angles are `angle` (radians); counts/sides/degrees are `integer`; `loft.smooth` is boolean.
- Selections: B-Rep `edges`/`faces`/`openFaces` (were geometry lists) and mesh index lists (were JSON text) are `selection` ports (B-Rep persistent labels, mesh integer indices). Mesh single picks are selections with one item (`mesh.inspect.*` `vertex|edge|face`, `mesh.edit.knifeCut` `face`). Mesh `mode`/`axis`/`pivot` are enums; `mesh.transform.matrix.matrix` is a list of 16 numbers.
- `surface.coons`: the nested `curves` list becomes four point-list ports `bottom, right, top, left` (kernel order).
- `brep.topology.deconstruct`: label texts became `edges`/`faces` selections; outputs are shape lists.
- `analysis.*` outputs use descriptive names (`centroid`, `volume`, ...); `analysis.classifyPoint.classification` is an enum (inside, outside, boundary).

## 4. Loader API

Rust (`schema::catalogue`, module path `semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::catalogue`):
- Types, all `#[derive(ToValue, FromValue)]` with `deny_unknown_fields`: `Localized{en,de}` (`resolve(locale)`), `Quality`, `PortType`, `ShapeKind`, `SelectionComponent`, `Selection`, `EnumOption`, `Port`, `PickComponent`, `Pick`, `GumballMotion`, `Along`, `Gumball`, `Interaction`, `Kind`, `Category`, `CategoryFile`.
- `catalogue() -> &'static Catalogue` (parsed once from the bundled `CATEGORY_SOURCES`), `Catalogue::bundled()`, `Catalogue::parse(iter of (slug, text))` (refuses duplicate kind ids and unknown members), `categories()`, `category(id)`, `kinds()`, `kind(id)`, `port(kind_id, name)` (inputs win), `findings()`.
- `Kind::input(name)`, `output(name)`, `picks()`, `gumballs()`; `PortType::is_numeric()`.
- `check(&[CategoryFile]) -> Vec<Finding>` where `Finding{code, owner, port}`; 31 codes (text-missing, text-identical, duplicate-kind-id, duplicate-category-id, duplicate-port-name, kind-id-malformed, kind-category-mismatch, emoji-duplicate, default-missing, default-misplaced, default-type-mismatch, default-out-of-range, default-length-out-of-range, enum-options-invalid, enum-default-not-option, constraint-misplaced, bounds-inverted, list-misplaced, shape-kinds-misplaced, item-bounds-misplaced, selection-source-invalid, selection-mode-invalid, selection-multiplicity-mismatch, selection-on-output, pick-port-missing, pick-component-mismatch, gumball-port-missing, gumball-type-mismatch, gumball-along-invalid, gumball-axis-invalid, no-outputs).
- Names untouched since the coordinator's note: `catalogue()`, `Kind`, `Port`, `PortType`, `Quality`, `Localized`, `ShapeKind`, `SelectionComponent`.

TypeScript (`🟦️.ts`, re-exported from the artifact root): `catalogue()`, `buildCatalogue(files)`, `CATALOGUE_CATEGORY_FILES`, `resolveCatalogueText(text, locale)`, `checkCatalogue(files)`, types `CatalogueKind`, `CataloguePort`, `CataloguePortType`, `CatalogueQuality`, `CatalogueShapeKind`, `CatalogueSelection`, `CatalogueText`, `CatalogueCategory`, `CatalogueCategoryFile`, `CatalogueFinding`, `CataloguePick`, `CatalogueGumball`, `CatalogueInteraction`, `CatalogueOption`, `Catalogue`.

Schema conventions for compute lanes:
- Port types: number, integer, angle (radians), length, boolean, text, enum, vector, point, plane (`{origin, normal}`), shape, shapes (list of shapes), mesh, selection, plus `any` (only for the generic `math.listItem` / `math.listLength` list ports; an addition to the brief's list). `list: true` marks lists of scalars/vectors/points (never of shapes; use `shapes`). `minItems`/`maxItems` bound lists, `shapes` and selections.
- `shapeKinds` filters shape/shapes ports. `optional` input = may be unconnected; `optional` output = may be absent (`brep.intersect.curveCurve.wire`, `analysis.interference.shape`, `analysis.meshQuality.volume`, `brep.topology.deconstruct.selectedEdges/selectedFaces`).
- Selection: `{component: face|edge|vertex|mode, source: <input port>, multiple, modeFrom?}`; `mode` takes the element type from the named enum port (mesh component kinds). Default `[]` for B-Rep (user must pick), `[0]` for mesh.
- Numeric ports: `min`/`max` inclusive, `exclusiveMin` makes `min` exclusive (positive-only lengths), `step` is the UI step, angle ports carry `unit: "rad"`.
- Interaction: `pick[] {component: shape|mesh|face|edge|vertex, port}` seeds a selection port, a shape port that accepts the kind, or the mesh port; `gumball[] {motion, port, axisPort?, originPort?, along?}`: translate -> vector/point/plane (or length/number with `along: "normal"`), rotate -> angle (needs `axisPort`, optional `originPort`) or plane, scale -> vector/number/length (optional `originPort`).
- `quality` is one of exact-analytic, exact-numerical, approximate, mesh-derived-brep, polygon-mesh, tessellated-mesh, copied from the kernel `OPERATION_QUALITY` table. Deliberate deviation: `brep.curve.helix` is `approximate` (kernel returns a 32-segments-per-turn polyline wire, output kind is `wire`, not `curve`).

## 5. Tests and results

| Run | Result |
|---|---|
| `bun test ./…/🗂️catalogue/🧪️tests/🔬️unit/🟦️.ts` (TS: AJV 8 strict validation of all 27 files against the meta-schema, 9 AJV schema rejections, 35 finding-law cases, bundle has zero findings, coverage 93/93 kernel + 42/42 mesh, lookups, locale fallback, duplicate refusal) | 43 pass, 0 fail (re-run after the final edit) |
| `NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/procedural-generation3d:test --skip-nx-cache` (official TS runner: strict `tsc` over both suites, then `bun test`) | exit 0; sqlite suite 7 pass, catalogue suite 43 pass |
| Rust, standalone scratch crate that `#[path]`-includes the SAME `🦀️.rs` + test files with only `semio-framework-value`, `-value-derive`, `-pack-json` and `serde_json` (test oracle): `cargo test --offline --lib catalogue` | 9 pass, 0 fail (also re-run at the end): bundle parses with zero findings; loader agrees with serde_json on every file; folder listing equals the bundle; 35 mutated-copy finding laws equal the fixture; kernel/mesh coverage; lookups; locale resolve; `ToValue`/`FromValue` round trip; refusal of duplicates/unknown members/broken JSON |
| Rust, official: `cargo test --locked --offline --manifest-path …/📦️packages/🦀️rust/Cargo.toml --lib catalogue` with `CARGO_TARGET_DIR=…/target-g3d-lane-c` (11 attempts) | NOT RUN TO COMPLETION. Every attempt failed to compile peer crates edited concurrently by other lanes (`semio-s-artifact-stdio-contract` `FactoryPayloadRetirement` Copy bound, `semio-framework-plugin` `factory_constructor_birth_bytes` / `close_typed_operation_cursor`, `semio-framework-os-kernel` `DiffAlgebra`, `semio-framework-os-config`, `semio-framework-os-flow` `DagHost.hostDocument`, `semio-framework-artifact-flow-flow` / `-playbook-playbook` mismatched types). None of the errors is in the catalogue files. The nx runner `bun nx run @semio-tech/procedural-generation3d-rs:test --skip-nx-cache -- quick --offline --lib catalogue` compiles the same crates and was therefore not attempted separately. Re-run once the workspace compiles. |

Not proven: that the catalogue module compiles inside the real crate under workspace lints (it compiled warning-free in the scratch crate without them).

One-off audit (script in the ticket folder, not a permanent dependency): extracted all 93 ids of `NODE_KERNEL_METHOD` and the 42 mesh `definitions` from the flow extension sources: 135/135 map to exactly one catalogue kind, no kind is claimed twice, no catalogue id is missing. The kernel trait's 7 deliberately unexposed methods (`scale`, `kind`, `tessellate`, `dispose`, `retain`, `registry_len`, `export_gltf`) have no kind (uniform scale = `brep.transform.scale` with equal factors). The permanent copy of this check is `kernelCoverage` / `meshCoverage` in the fixture.

## 6. Kernel observations that shaped descriptions (for compute lanes)

- `helix_curve` returns a polyline wire. `extrude_wire`/`planar_face_from_wire` fix the face normal to +Z through the wire's first vertex: wires must lie in planes parallel to XY (documented in the kind descriptions).
- `coons_patch` takes exactly four boundary point lists (kernel order bottom, right, top, left); `nurbs_surface_from_grid` takes rows of points.
- `section` is tagged exact-analytic but curved-face sections are not routed through the imprint engine; `draft` is exact only for planes and cylinders; `validate` reports the whole body. Kinds keep the kernel's tag; descriptions state the limits.
- No kernel support for: mesh boolean, smoothing/subdivision surface, retopology, sketch/constraints, hole/thread features, push-pull. These have no kinds.
- Kinds without an existing flow registration (kernel capability exists, 64 new): mesh torus/UV sphere, STL/GLB export, OBJ/STL/GLB import, shading, UV seams/unwrap, `analysis.massProperties/angle/selectionArea/topologyCounts/interference`, and 48 `math.*` kinds. These need new computes; `analysis.interference` composes intersect + volume, `analysis.angle` and `analysis.selectionArea` need face/edge queries.

## 7. Taxonomy check

Command: `bun nx run workspace:verify-taxonomy-report --skip-nx-cache -- --scope "<C>"` (the whole-repo run was started once and stopped after 38 minutes, it needs more than 30).
- First scoped run, BEFORE the taxonomy edit: 88 errors, all from two causes: `directory-kind-unresolved` for `🗂️catalogue` and `semantic-stem-unresolved` for the 27 `🔣️<slug>.json` files (the rest, `collision-*` and `reference-edit-required`, are consequences of the unresolved stems). Nothing was reported for `🧪️tests`, `🧫️fixtures`, `🦀️.rs`, `🟦️.ts` beyond the reference consequences.
- Then the two kinds above were added to `🔣️taxonomy.json`. Four re-runs (scoped and `--scopes-from`) crashed inside the verifier's generator planning with `ENOENT lstat` on files of OTHER lanes' stdio fixture folders: `git ls-files -d` lists 300-650 tracked files that are deleted in the working tree by concurrent work, which the verifier cannot lstat. The post-edit verdict is therefore UNVERIFIED. The fifth re-run, after the deletions shrank, stopped earlier with `frozen-coordinate-evidence-invalid` for the peer-modified fixture `📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` (document digest differs from the registered bytes). Re-run the scoped command once those peer changes are committed; the expected result is zero errors, since the 88 first-run errors all traced to the two admitted kinds.

## 8. Decisions and deviations

- `type: any` added for the two generic list kinds; `exclusiveMin`, `minItems`/`maxItems`, `unit`, `along` added to the schema as needed; `pick.component` also takes `mesh`; selection `component: mode` for the mode-driven mesh component kinds.
- Analysis: `brep.measure.*` -> `analysis.*`; `brep.mesh.analyze` -> `analysis.meshQuality`; `brep.mesh.inspect*` -> `mesh.inspect.*`.
- Euler/mass-property outputs, the `plane` port and the `any` type require compute and inference lanes to define the matching value forms in `GeometryValue`.
- All texts EN then DE, authored by hand (German CAD terminology: Quader, Kantenzug, Verrunden, Fasen, Austragen, Ausformen, Entformungsschräge, Verbund). Palette emojis are unique within a category (enforced by the `emoji-duplicate` law).
- No launch.json entry added (no new executable command). Temporary logs, the scratch crate and the symlink under `🗑️generated/lane-c` are removed at the end; no `[DEBUG]` logging was added.

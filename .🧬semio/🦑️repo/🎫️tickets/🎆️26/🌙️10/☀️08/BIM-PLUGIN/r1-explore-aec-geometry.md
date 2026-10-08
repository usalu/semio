# r1: Existing AEC, BIM and Geometry Capability

Date: 2026-10-08. Ticket: `BIM-PLUGIN`. Mode: read-only inventory, no repo file modified.
Paths are relative to `C:\git\semio\` unless shown absolute. Test counts are grep counts of test declarations, not test runs.

## TL;DR

1. Building vocabulary already exists in three places that disagree: `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building` (11 typologies `building.building.*`, storey as a free-string CAD node kind), `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim` (scalar-only `story`/`building`/`space`), and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc` (IFC names, metadata only). The new `bim` plugin must own one vocabulary.
2. Nothing is parametric in the feature-tree or constraint sense. `place<X>From2PointsAndHeight` builds a baked solid; the parameters are not stored. The only parametric substrate is the dataflow graph (`🌊️flow` + flow `🧩️extensions/📐️brep`).
3. The geometry kernel is real and first-party: `semio-framework-3d` (B-Rep, booleans, sweeps, loft, offset, fillet/chamfer, tessellation, mass properties). Its `[dependencies]` contain no external crates.
4. The production TypeScript kernel is `SemioBrepKernel`, backed by the Rust session `semio-s-spatial-kernel-semio-session`. brepjs/OpenCascade is a test oracle only (`@semio-tech/cad-js` lists it under devDependencies).
5. IFC exists as a stdio artifact (IFC2x3 with COBie/CV2.0/SAV views, IFC4) with Part-21 I/O, but has no geometric representation at all (0 hits for `ExtrudedAreaSolid`, `ShapeRepresentation`, `FacetedBrep`).
6. Energy (`🔋️energy`, EnergyPlus-style epJSON zones/surfaces/constructions) and norms (`📕️norm`: EN 1990/1991/1992/1997, DIN 16798, DIN 18599) exist. The `aec-building-energy` extension computes its demand with placeholder constants, not a norm calculation.
7. Recommendation: depend on `semio-framework-3d`, `semio-framework-mesh-engine`, `semio-framework-2d`, `semio-framework-geometry`, the stdio step/ifc/bcf/gltf artifacts and the energy model link. Implement the bim hierarchy, parametric element recipes, hosted openings and georeferencing ourselves. Use ifcopenshell, brepjs, parry3d and manifold-3d as test oracles only.

---

## 1. CAD plugin (`✏️s/🔌️plugins/📐️cad`)

### 1.1 Units

| Unit | Crate | What it contributes | Real code | Parametric |
|---|---|---|---|---|
| `🧩️extensions/🏢️aec-building` | `semio-s-plugin-cad-aec-building` | STEP layer to typology profile for `aec.building` (11 typologies, 22 layer aliases). Nine spatial-kernel commands `building.building.place*FromPointsAndHeight` (box or linear prism). Composite mutation `create-building-storey` (plans a `CreateNode` with kind `building-storey`). Inference `s.cad-extension-aec-building.building-structure-summary` (`buildingModelPresent`, `storeyCount`). | Yes. `🦀️.rs` 185 lines, `🟦️.ts` 71 lines. 2 Rust `#[test]`, 1 TS test. | No. Two points plus height produce a baked solid. |
| `🏛️aec-building-structure` | `semio-s-plugin-cad-aec-building-structure` | Maps building typologies to RC structure typologies (`structure.structure.reinforcedconcrete{column,externalwall,internalwall}`, `onewayreinforcedconcreteslab`). Stat `structure.stability` (structural volume, mass at 2500 kg/m3, slenderness index). Transformation `from_building`. STEP profiles for classic, FEM line/solid/surface models. | Yes. 147 TS + 96 Rust lines. 2 TS tests. | No. Heuristic index. |
| `📐️spatial-shape` | `semio-s-plugin-cad-spatial-shape` | Stat `spatial.shape.geometry` (volume, surface area, bbox, counts). Property `spatial.shape.volume`. | Yes. 77 TS + 56 Rust lines. 2 TS tests. | Measurement only. |
| `🔥️aec-building-energy` | `semio-s-plugin-cad-aec-building-energy` | STEP profile (`energy.energy.{baseplate,roof,externalwall,hull,windows}`). Stat `energy.demand`. Property `energy.heatedvolume`. | Structure yes, numbers are placeholders: `ENERGY_DEFAULT_U_VALUE = 0.3`, `ENERGY_HEATING_DEGREE_HOURS = 3000`, `ENERGY_VENTILATION_FACTOR = 12`. 101 TS + 88 Rust lines. 1 TS test. | No. No norm (DIN 18599 / ISO 52016) behind it. |

Engine (`⚙️engine/`):
- `🫀️core/🟦️.ts` re-exports the spatial kernel geometry, spatial and registry modules plus `🏗️construction` and `📔️registry`.
- `🏗️construction/🟦️.ts`: `constructLinearPrism` (horizontal footprint, thickness, height), `constructBoxFromPoints`, `constructPrismFromCurve`.
- `📔️registry/🟦️.ts`: `registerImportProfile`, `typologyFromStepLayer` (STEP presentation layer name to typology, with `ns::layer` domains).
- `🧱️brepjs/🟦️.ts`: brepjs + OpenCascade adapter. Consumers are only test suites and oracle generators (`✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/🧱️brepjs`). Not on the production path.

Artifact (`🗿️artifacts/📐️cad`): `s.cad.cad`, Rust `🦀️.rs` 1216 lines.
- `CadSnapshot` children: `shape_model`, `building_model`, `energy_model`, `structure_classic_model` (each a `s.stdio.semio` child), `drawings`, references, nodes.
- `CadNode` (`🦀️.rs` line ~431) is `{ id, label, kind }`. `kind` is a free string. `building-storey` is a tag, not a typed entity.

### 1.2 Entity vocabulary as shipped

Example model definition `🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions/🏢️aec.building/`:
- Typologies in `🗂️typologies/`: Ceiling, Column, Roof, Railing, Door, Wall, Slab, Stair, Window, Foundation, Beam. IDs are `building.building.<lowercase>`. Example: `🛡️Wall/🔣️typology.json` has `primitiveKinds: ["surface"]` and `actions: ["building.building.placeWallFrom2PointsAndHeight"]`.
- Actions `🎬️actions/`: nine `place*From2PointsAndHeight` JSON definitions, each a single `kernel.call` step.
- Attribute definitions `🏷️attributeDefinitions/`: only `spatial.shape.opening` (window, door, passage, outlook) and `spatial.shape.material`.
- Interactions `🕹️interactions/`: `placeWall.json` is a state machine (idle, first_point, second_point, height).

Missing as typed entities: site, building, storey, space, zone, opening-as-host (only as attribute), roof slope, curtain wall, stair run, railing run, material layers. Storey exists only as a composite mutation producing a `CadNode`.

### 1.3 Parametric level

Zero. The stored result is geometry, not the 2-point/height inputs. No parameter table, no regeneration on edit, no hosting relation between a wall and a window. The inference only counts storeys.

### 1.4 Taxonomy drift

`📐️cad/AGENTS.md` documents Curve types Line, Circle, Ellipse, Parabola, Hyperbola, B-Spline, Bezier, and Surface types Plane, Cylinder, Cone, Sphere, Torus, B-Spline, Bezier. Rust `Curve3` has only `Line`, `Circle`, `Ellipse`, `Nurbs`, and `Surface` has `Plane`, `Cylinder`, `Cone`, `Sphere`, `Torus`, `Nurbs` (`🧊️3d/📐️brep/📸️representation/➰️curve/🦀️.rs` line 35, `🏄️surface/🦀️.rs` line 29). Parabola, hyperbola and Bezier are not separate types. The AGENTS document is ahead of the code.

---

## 2. Flow BIM extension (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim`)

- Crate `semio-s-plugin-flow-extension-bim`. Entry `🦀️.rs` (624 lines). Schema ids: `material`, `space`, `wall`, `slab`, `column`, `window`, `story`, `building`.
- Operators: `bim.element.{material,space,wall,slab,column,window}`, `bim.assemble.{story,building}`, `bim.measure.{floorArea,grossVolume}`.
- Behaviour is scalar only. A story's floor area is `slab.width * slab.depth` or the sum of `space.area`. Gross volume is floor area times story height. No geometry is produced and no IFC is read or written.
- 8 async unit tests in `🧪️tests/🔬️unit/🦀️.rs`. Its `🛂️.descriptor.semio` and `🔣️.json` are shipped.
- Vocabulary overlap: `story`/`building` here versus `building-storey` in cad and `IfcBuildingStorey` in IFC.

Sibling in the same extension family: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep` (`semio-s-plugin-flow-extension-brep`, 2218 lines). It registers about 90 operators on the B-Rep kernel (section 4.3). It is the real parametric dataflow surface for geometry.

---

## 3. Other domain plugins (`✏️s/🔌️plugins/*`)

| Plugin | Artifact crate | Building-element relevance |
|---|---|---|
| `🔋️energy` | `semio-s-artifact-energy-model` | EnergyPlus-style model (`EnergyModelSnapshot { schema, model, structure, zones, referenced_model, weather_link }`). Zones, constructions with `ConstructionLayers`, `BuildingSurface:Detailed` and `FenestrationSurface:Detailed` with `vertices` (epJSON 25.2 import/export). `referencedModel` is an `ArtifactLink` to a geometry model (`connect-referenced-model`, `disconnect-referenced` mutations). Simulation runs via EnergyPlus (test `🧪️tests/🏛️export-epjson-runs-in-energyplus`). Oracles pinned in `🔮️oracles/📦️packages/🐍️python`. This is the best existing thermal model. |
| `📕️norm` | `semio-s-artifact-norm-din18599` and siblings (`en1990`, `en1991`, `en1992`, `en1997`, `din16798`) | Code-check artifacts, not geometry. Candidates for bim compliance and energy checks. Not wired to `aec-building-energy`. |
| `🏛️architect` | `semio-s-artifact-architect-program` | Program of requirements. `ProgramElementKind`: Building, Campus, Floor, Zone, Room, Suite, Department, System, Circulation, Support, Outdoor, FurnitureGroup, Other. Adjacency matrix. No geometry. Maps onto bim spaces. |
| `🏗️fem` | `semio-s-artifact-fem-3d` (and `◻️2d`) | Structural analysis. `solve_statics` and `solve_mode` (modal) exist for 2D and 3D (`results/🦀️.rs`). Elements: Bar, Beam, solids, nodes, supports, load cases, combinations. Own geometry, not using `semio-framework-3d`. |
| `📸️remodel` | `semio-s-artifact-remodel-remodeling` | Photogrammetry: `sfm/` (triangulation, bundle logic), `motion/`, `dense/` (`fuse_depth_maps`), camera calibration and poses. Usable for scan-to-BIM. 437 Rust files in the artifact tree. |
| `💠️lowpoly` | `semio-s-artifact-lowpoly-lowpoly` | Low-poly mesh editing (bevel, paint, primitives). Depends on `semio-framework-3d`. Not AEC. |
| `🌍️gis` | `semio-s-artifact-gis-gismap`, `…-gisterrain` | Map features, terrain, routes, positions, regions. GeoJSON import. CRS support is WGS84 (EPSG:4326) and Web Mercator (EPSG:3857) only. No general projection library. Relevant for site georeference, which bim lacks. |
| `🧱️block` | `semio-s-artifact-block-3d` (and `◻️2d`, `🖐️5d`) | Kind definitions (kit-of-parts) with handles, vortices, grips, representations, compatibility rules. Concept ids `BlockKindIdentity`, `BlockRepresentation`. |
| `🧩️puzzle` | `semio-s-artifact-puzzle-3d` | Assemblies (Object, Vortex, Attraction, Cable). Places block kinds. Kit-of-parts template for wall-to-slab connections. |
| `🗄️stdio` | `…-ifc`, `…-bcf`, `…-step`, `…-gltf`, `…-obj`, `…-stl`, `…-dwg`, `…-dxf`, `…-ply`, `…-semio` | File-format artifacts. Section 5 and 6. |
| `📏️layout` | (crate name not checked) `🗿️artifacts/📏️layout` | Layout artifact with `LayoutArtifact`, layout analyzers and export kinds. Not reviewed in depth. |

Plugin-level 3D dependency pattern: the crates that depend on `semio-framework-3d` at runtime are `🌊️flow/…/📐️brep`, `🏭️process/…/🧊️process3d`, `💠️lowpoly`, `📐️cad` artifact, `🗄️stdio/…/🧿️semio` and `🧩️puzzle/…/🧊️3d`. `🗄️stdio/…/🧿️semio` has it as dev-dependency only. `semio-framework-2d` is depended on by about 98 plugin crates.

---

## 4. Geometry kernels

### 4.1 Inventory

| Module (path) | Crate | Role | Maturity | Runtime external deps |
|---|---|---|---|---|
| `🧰️framework/🔨️modules/🧊️3d` | `semio-framework-3d` | Native B-Rep kernel (`brep/`), half-edge mesh (`🥽️mesh`), collision BVH (`🧿️collision`), inertia, TS mirror `🟦️.ts` (501 lines, `@semio-tech/s-3d-js`). | Real. The nine main operation files (boolean, offset, sweep, blend, tessellation, intersect, sew, mass properties, primitives) total about 10.8 k lines. 146 `#[test]` in the crate (63 under `📐️brep/`). Dev-deps: `parry3d`, `serde`, `serde_json`, `num-rational` (tests only). | None |
| `🧰️framework/🔨️modules/◻️2d` | `semio-framework-2d` | Path segments, filled planar booleans (union, difference, intersection, xor; budgeted, cancellable), stroke regions (SVG caps and joins), bitmap trace, flatten. | Real. | `serde` optional |
| `🧰️framework/🔨️modules/📐️geometry` | `semio-framework-geometry` | 2D primitives (Point, Vec2, Affine, shapes, BezPath), Vec3/Mat4 render matrices, seeded Rng, curve math. `kurbo` as oracle only. | Real | None |
| `🧰️framework/🔨️modules/🏗️mesh-engine` | `semio-framework-mesh-engine` | Mesh data, primitive construction (box, cone, cylinder, sphere, torus, plane), OBJ, GLB and STL codecs, `validate_mesh_surface_assets`. | Real | Internal only |
| `🧰️framework/🔨️modules/🔢️number` | `semio-framework-number` | Arbitrary-precision integers and rationals, certified intervals, Ring/Field traits. Used by 3d. | Real | None |
| `🧰️framework/🔨️modules/🧮️math` | `semio-framework-math` | Declared as LLM token sampling and diffusion noise, misfiled here. Not a geometry kernel. | Do not use | — |
| `🧰️framework/🔨️modules/🗺️surface` | `semio-framework-surface` | wasm-bindgen rendering sessions (paint, terrain, node-graph, tiled-map). Not a kernel. | Not a kernel | — |
| `✏️s/🔨️modules/🌐️spatial-kernel` (TS) | `@semio-tech/cad-js` core | Spatial kernel interface (`🗺️spatial/🟦️.ts`), `Model` (`📐️geometry/🟦️.ts`, 3552 lines), command registry, preview math (`🧮️preview/🟦️.ts`), production kernel `🧠️semio/🟦️.ts` (`SemioBrepKernel`). | Real | Calls WASM session |
| `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session` (Rust) | `semio-s-spatial-kernel-semio-session` | Owned geometry session over `semio-framework-3d`. `brep_invoke(method, argsJson)` WASM binding. Binding output (`🕸️bindings`) is generated, not committed. | Real | Dev: `parry3d` |
| `📐️cad/⚙️engine/🧱️brepjs` | brepjs 18.119.8 + brepjs-opencascade 0.15.6 | OpenCascade via brepjs. Oracle only. | Oracle | Not production |
| `temp/brepkit` (reference clone, git-ignored) | brepkit crates (`algo`, `blend`, `offset`, `topology`, `heal`, `sketch`, `io`, `check`) | Third-party Rust B-Rep. License MIT OR Apache-2.0. Reference or oracle candidate. | Oracle candidate | — |

### 4.2 Who may depend on what

- Plugins may depend on `semio-framework-3d`, `-2d`, `-geometry`, `-mesh-engine`, `-number`. Precedent: `🌊️flow/…/📐️brep`, `🏭️process/…/🧊️process3d`, `💠️lowpoly`, `📐️cad` artifact.
- The TypeScript kernel reaches Rust through the WASM session. Plugins using the TS side go through `@semio-tech/cad-js` (`SpatialKernel` interface).
- `brepjs` must not be a runtime dependency. Per repo AGENTS it is an oracle; cad-js lists it in devDependencies yet `🧱️brepjs/🟦️.ts` imports it.

### 4.3 B-Rep operations in `semio-framework-3d` (`brep/`)

- Primitives (`🛠️operations/🧱️primitives`): box, sphere, cylinder, cone, torus, convex hull, solid from triangle soup, polyline, rectangle and regular-polygon wires, planar face from points or wire.
- Sweeps (`➡️sweep`): `extrude_face`, `extrude_wire`, `pipe` (with optional guide), `sweep_along_path`, `helical_sweep`, `revolve_face` (`🌀️revolve`), `loft_profiles` (`🥞️loft`, smooth option).
- Booleans (`🔀️boolean`): `boolean_solid` (union, difference, intersection), `compound_cut`, `section_solid_by_plane`, `split_solid_by_plane`. Exact imprint, classify, select, stitch pipeline. Budgeted `BooleanJob` with `progress`, `cancel`, `step`. Documented scope: planes, cylinders, cones, spheres, tori, NURBS via marching SSI. Coincident-face merge included.
- Offset and shell (`↔️offset`): `offset_surface`, `offset_face`, `offset_solid` (sharp and round corners), `thicken_face`, `shell_solid`, `shell_solid_with_open_faces`, `draft_angle`.
- Blends (`🎨️blend`): `fillet_edges`, `fillet_variable`, `chamfer_edges`, analytic supports (cylinder, torus, cone, plane, sphere corner patches).
- Intersections (`✂️intersect`): curve-curve, curve-surface, surface-surface (`🏄️surface-surface`).
- Topology: `euler` (make and split), `sew` (shells, heal), `transform`.
- Queries (`💡️queries`): crack-free adaptive tessellation to `MeshTransfer` (`🧩tessellation`, CDT plus refinement), mass properties (`📏mass-properties`: volume, signed volume, area, centroid, bbox, mass with area centroid, solid and solid distance, closest point), classification (`🏷️classification`, point in solid), validation, analysis (curvature, distance, bounds, topology counts).
- Representation (`📸️representation`): arena, curves (Line, Circle, Ellipse, Nurbs, bezier, bspline, polynomial), surfaces (Plane, Cylinder, Cone, Sphere, Torus, Nurbs, isocurves), tolerance, topology (Body with vertices, edges, coedges, loops, faces, shells, solids, curves, surfaces).
- Engine (`⚙️engine/🦀️.rs`): `Brep` with `*_sync` methods: `box_prim_sync` and friends, `fuse_sync`, `cut_sync`, `boolean_sync`, `boolean_job_sync`, `step_boolean_job_sync`, `intersect_sync`, transforms, `volume_sync`, `area_sync`, `length_sync`, `center_of_mass_sync`, `bounding_box_sync`, `distance_sync`, `closest_point_sync`, NURBS constructors, `interpolate_curve_sync`, `approximate_curve_sync`, `helix_curve_sync`, `coons_patch_sync`.

Flow operator ids (`🌊️flow/…/📐️brep/🦀️.rs`) include `brep.solid.{extrude,fillet,filletEdges,filletVariable,chamfer,chamferEdges,offsetSolid,shell,draft,defeature}`, `brep.sweep.{extrude,pipe,sweep,revolve,loft,helical}`, `brep.bool.{fuse,cut,intersect,compoundCut}`, `brep.xform.{translate,rotate,rotateAbout,scale,mirror,linearPattern,circularPattern,gridPattern,copy}`, `brep.prim3d.*`, `brep.io.{importStep,exportStep,importObj,exportObj,importStl,exportStl,importDwg,exportDwg}`, `brep.measure.*`, `brep.intersect.*`.

### 4.4 TypeScript kernel command vocabulary (`🧠️semio/🟦️.ts`, `executeCommandDiff`)

`curve.line`, `curve.polyline`, `curve.circle`, `curve.arc`, `curve.controlPointCurve`, `curve.interpolateCurve`, `solid.sphere`, `solid.cone`, `solid.cylinder`, `solid.booleanUnion`, `solid.booleanDifference`, `solid.booleanIntersection`, `surface.extrudeCrv`, `surface.loft`, `surface.networkSrf`, `surface.sweep1`, `surface.sweep2`. Box from corners via `createBoxFromCornersDiff`. Extrude via `extrudeWireDiff`. Offset via `offsetFacesDiff`. Volumes and areas via the session.

### 4.5 Gaps

- No 2D polygon offset or inset in `◻️2d`. Wall centre-line to face needs a planar `offset_face` in 3d, or a new 2D routine.
- No CRS or projection beyond EPSG:4326 and 3857 in gis.
- No parametric constraint solver anywhere.
- No IFC-aware geometry (section 5).
- STEP analyzer view `🧱️BrepMesh` (`📐️step/…/🚪️io/🧱️brep/🦀️.rs`) is planar-only; curved faces are flagged, not tessellated. The geometry codec (`…/🚪️io/📐️geometry/🦀️.rs`, 1162 lines) reads and writes `PLANE`, `CYLINDRICAL_SURFACE`, `CONICAL_SURFACE`, `SPHERICAL_SURFACE`, `TOROIDAL_SURFACE`, `B_SPLINE_SURFACE_WITH_KNOTS`, `B_SPLINE_CURVE_WITH_KNOTS`, `ELLIPSE`, `CIRCLE`, `MANIFOLD_SOLID_BREP`, `ADVANCED_BREP_SHAPE_REPRESENTATION`. Entry points `write_step(body, solids)` and `read_step(text)` (`🦀️.rs` lines 46 and 69). Kernel-level wrappers `export_step(kernel: &Brep, shapes)` and `import_step(kernel: &mut Brep, text)` are free functions in the same file (lines 1129, 1134).

---

## 5. IFC and other BIM formats

### 5.1 Presence

- Word-boundary search (`ifc|ifcopenshell|web-ifc|IfcWall|IFC4|IFC2X3`, rust, ts, json, py, toml, excluding node_modules, target, dist, `.🧬semio`, temp): 453 files. 417 under `✏️s`, 20 under `🌎️hub`, 9 under `🧰️framework`. Most are fixtures, mutation specs and generated scaffolding.
- Substring search `ifc` gives 693 files because of false matches ("specific", "classification").

### 5.2 IFC artifact (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc`)

- Artifact `s.stdio.ifc`, document schema `stdio.ifc`, dialect `ifc2x3` and `ifc4` (`🏅️standards/🔖️2x3`, `🏅️standards/4️⃣4`).
- Subsets: IFC2x3 base, COBie (`🏢️cobie`), CV2.0 (`🤝️cv20`), SAV; IFC4 `✳️any`.
- Part-21 I/O via the shared `🧾️part21` codec (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21`, with `write_part21` and `write_part21_with`).
- Mutations: `IfcMutation` (about 280 references) including `add_product`, `add_space`, `add_load_group`, `set-facility-name`, `set-floor-elevation`, `set-space`, `set-type-assignment`, `set-view-definition`, `set-snapshot`. Analyzers: `analyze_spatial`, `check_cobie_conformance`, `check_cv`, `check_sav_conformance`.
- Entities seen: IfcProject, IfcSite-level spatial structure (IfcBuildingStorey, IfcSpace, IfcBuilding), IfcRelAggregates, IfcRelContainedInSpatialStructure, IfcLocalPlacement, IfcPropertySet, IfcStructuralLoadGroup, IfcStructuralAnalysisModel, COBie element names.
- Geometric representation: none. No `IfcExtrudedAreaSolid`, `IfcShapeRepresentation`, `IfcProductDefinitionShape`, `IfcFacetedBrep`, `IfcManifoldSolidBrep`.
- Real-world fixture: a 2.5 MB IfcOpenShell 0.8.4 export (Nakagin Capsule Tower, 24792 entities) is cited in the IFC4 subset oracle descriptor (`🏅️standards/4️⃣4/🪆️subsets/✳️any/🔮️oracles/🔣️.json`) as the parse test input for ruststep.

### 5.3 Other BIM and exchange artifacts (stdio)

- `💬️bcf`: BIM Collaboration Format 2.1 (`semio-s-artifact-stdio-bcf`). Issues and viewpoints.
- `📐️step`: STEP AP214 with conformance classes 1 to 6 (`🔖️ap214/🪆️subsets/1️⃣cc1` … `6️⃣cc6`). Class 6 is advanced B-Rep. `🧱️base/🚪️io/📐️geometry` as in 4.5.
- `🧊️gltf`: glTF 2.0 document artifact (`semio-s-artifact-stdio-gltf`). Mesh, camera, material, buffer, animation, skin.
- `🗽️obj` (OBJ 3.0), `🔺️stl` (ASCII), `🧱️ply`, `🖊️dwg`, `🖋️dxf`, `☁️las` (point clouds), `🌐️html`, `🎒️zip`.

### 5.4 Python and JS oracles present or pinned

- Root `pyproject.toml`, `test` dependency group: `ifcopenshell==0.8.4.post1`, `steputils>=0.1`, `shapely>=2.1.2`, `scikit-fem>=10.0.2`, `PyNiteFEA>=1.1.6`, `anastruct>=1.6.1`, `networkx`, `lxml`, `mercantile`. Locked in root `uv.lock` (ifcopenshell at line 597).
- Energy oracle project `🔋️energy/🔮️oracles/📦️packages/🐍️python/pyproject.toml`, `test` group: `honeybee-energy==1.123.32`, `honeybee-openstudio==0.7.2`, `openstudio==3.11.0`, `ladybug-core==0.44.59`, `jsonschema`. Standalone uv project, separate from root.
- The local `.venv/Lib/site-packages` (285 entries) has pytest and numpy but no `ifcopenshell` directory. Installing is required before the differential tests can run.
- `node_modules`: `brepjs` 18.119.8, `brepjs-opencascade` 0.15.6 (LGPL-2.1), `manifold-3d` 3.5.1 (Apache-2.0), `three` 0.182.0 (MIT), `@gltf-transform/core` 4.4.2 (MIT), `polygon-clipping` (MIT), `@jscadui` (modeling), `opentype.js`. No `web-ifc` and no `@thatopen`.
- Rust dev-only oracles: `parry3d` 0.17 (boolean, tessellation, mass properties; Apache-2.0), `ruststep` 0.4 (STEP syntax reader, optional in stdio IFC oracle), `kurbo` (2D geometry oracle).
- `temp/topologic`: AGPL-3.0. Do not link or copy. `temp/brepkit`: MIT OR Apache-2.0.

---

## 6. Rendering and export

### 6.1 Rendering targets

- Tessellation output: `MeshTransfer` (`🧊️3d/🟦️.ts` line 190), consumed by `🧊️3d` TS mirror and cad renderer (`…/📐️cad/…/✏️editor/⚙️engine/📺️renderer/🟦️.tsx`).
- Raw wgpu WASM renderer: `semio-framework-os-renderer-wgpu` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu`), plus UI target `🖱️ui/🎯️targets/🧊️wgpu`.
- three.js 0.182 via `🖱️ui/🎯️targets/⚛️react` and `♾️infinite/🌍️world/🎨️r3f` (`@react-three`, `three-mesh-bvh`). Not a plugin runtime dependency for bim.

### 6.2 Exporters

| Format | Where | Notes |
|---|---|---|
| OBJ, STL, GLB (write and read) | `🏗️mesh-engine` (`mesh_to_obj`, `mesh_to_stl`, `mesh_to_glb`, readers) and `🧊️3d/📐️brep/⚙️engine/📦️mesh-io` (`export_solid_stl`, `export_solid_obj`, `export_solid_glb`, imports) | Real |
| STEP AP214 | `🗄️stdio/…/📐️step` `write_step` / `read_step` | Real for analytic and B-spline geometry |
| IFC | `🗄️stdio/…/🏗️ifc` Part-21 writer | Metadata only. No geometry export. |
| glTF document | `🗄️stdio/…/🧊️gltf` | Document model, not tied to B-Rep export |
| DWG, DXF, PLY, LAS | `🗄️stdio/…/🖊️dwg`, `🖋️dxf`, `🧱️ply`, `☁️las` | Present as artifacts. Flow `brep.io.exportDwg` exists. Not verified end to end. |
| three.js GLTF/OBJ/STL exporters | `✏️s/🔌️plugins/🗄️stdio/…/🧊️gltf/…/🏭️generator/📜️script.ts` and `…/🧿️semio/…` generators | Oracle scripts only |

---

## 7. Recommendation

### 7.1 Depend on (existing, no reimplementation)

1. `semio-framework-3d` (`🧰️framework/🔨️modules/🧊️3d`): all solid construction, booleans, sweeps, loft, offset/shell/draft, fillet/chamfer, sections, tessellation, mass properties. Use the `Brep` engine entry points with the `*_job_sync` variants for expensive booleans so progress and cancellation work (repo AGENTS requirement). Follow the pattern of `🌊️flow/…/📐️brep` and `🏭️process/…/🧊️process3d`.
2. `semio-framework-mesh-engine`: mesh data, OBJ, GLB and STL codecs for display and export.
3. `semio-framework-2d` for plan and section drawings (planar booleans, stroke, flatten, bitmap trace) and `semio-framework-geometry` for Vec2/Vec3/Affine.
4. `semio-framework-number` only if exact predicates are needed. The 3d crate already depends on it.
5. Stdio artifacts: `s.stdio.ifc` (IFC metadata, COBie, MVD views), `s.stdio.bcf` (issues), `s.stdio.step` (AP214 geometry), `s.stdio.gltf` (document). Reference them, do not copy their codecs.
6. Energy: `🔋️energy` (`referencedModel` link to the geometry model; zones, constructions, surfaces). Norm checks via `📕️norm` (DIN 18599, DIN 16798, EN series).
7. Structure: `🏗️fem` solvers (`solve_statics`, `solve_mode`) for structural checks. The FEM artifact is separate from the 3d kernel, so bim needs its own element-to-FEM mapping. `aec-building-structure` only maps to structure typologies, not to FEM elements.
8. Program: `🏛️architect` program artifact (spaces, room kinds, adjacency) for the bim `space` and program-to-model link.
9. Site: `🌍️gis` (GeoJSON, terrain, routes) for the site context.
10. Scan-to-BIM: `📸️remodel` (SfM and dense depth fusion) as an input source.
11. Parametric layer: flow dataflow (`🌊️flow` and `…/📐️brep` operator catalog) and the CAD `CompositeMutationKind` pattern (`create-building-storey` is the template: `plan` through `Planner`, `label`, `target`).
12. Kit-of-parts: `🧱️block` (kind definitions with handles and grips) and `🧩️puzzle` (assemblies) for connection semantics, if bim adopts the kit-of-parts model.

### 7.2 Implement (gaps no existing code covers)

1. One vocabulary: site, building, storey, space and zone, element (wall, slab, column, beam, roof, stair, railing, curtain wall, opening), and material layers. Replace `flow/bim` scalar ops and the `story` term. Decide `building-storey` vs `storey` once, and align it with `IfcBuildingStorey`.
2. Parametric element recipes that keep parameters (length, height, thickness, layers, sill, width) and regenerate geometry on edit. Compose existing kernel ops (linear prism, extrude, sweep, loft, boolean cut for openings, helical sweep for stairs, pipe sweep for railings, offset for thickness).
3. Hosted openings: a window or door is a host-relative boolean cut in a wall, not an independent solid. Use `spatial.shape.opening` as the attribute but add the host relation.
4. Georeferencing: site anchor (lat/lon or projected origin), true north, local-to-world transform. The current code only does WGS84 and Web Mercator in gis.
5. 2D footprint offset (inset and outset for wall centre-lines). Use planar `offset_face` in 3d or add a 2D routine in `◻️2d`.
6. IFC geometry writer: `IfcExtrudedAreaSolid`, `IfcShapeRepresentation`, `IfcProductDefinitionShape`, `IfcLocalPlacement` mapping from the bim element model, written through the existing Part-21 writer. Without this, IFC carries metadata only.
7. Standards-based energy and structural checks. Replace the placeholder constants in `aec-building-energy` and `aec-building-structure` by calls into `📕️norm` artifacts.
8. Taxonomy AGENTS fix: `📐️cad/AGENTS.md` lists types (parabola, hyperbola, bezier, bspline per surface) that the Rust kernel does not have as separate variants. Either implement or correct the document.

### 7.3 Test oracles only (never runtime dependencies)

| Oracle | Version | Use | Note |
|---|---|---|---|
| ifcopenshell | 0.8.4.post1 | IFC4 and IFC2x3 differential tests | Root `pyproject.toml` test group; LGPL. Not installed in `.venv`. |
| brepjs + brepjs-opencascade | 18.119.8 / 0.15.6 | STEP 6cc6 expectations, cad-js differential | Already test-only. LGPL-2.1 for OCCT. |
| parry3d | 0.17 | boolean, tessellation, mass-property differential | Already dev-only in 3d, stdio, flow brep. |
| manifold-3d | 3.5.1 | CSG oracle candidate | Apache-2.0. Already present through `🏗️fem` package. |
| three.js exporters | 0.182 | glTF, OBJ, STL output cross-check | MIT. |
| scikit-fem, PyNiteFEA, anastruct | pinned in root test group | FEM oracle | Already pinned. |
| ruststep | 0.4 | STEP syntax reader oracle | Optional. |
| brepkit (temp) | MIT OR Apache-2.0 | Candidate second B-Rep oracle | Reference only, not vendored. |

### 7.4 Do not

- Do not add brepjs, OCCT, web-ifc, manifold or any other external library to a runtime crate or to the TS production path. Repo AGENTS forbids runtime externals.
- Do not depend on `semio-framework-math` (misnamed) or `semio-framework-surface` (rendering sessions) for geometry.
- Do not link `temp/topologic` (AGPL-3.0).
- Do not copy the placeholder energy and structure constants into bim.
- Do not add a second storey or building vocabulary. Consolidate `flow/bim`, `aec-building` and the IFC names into the bim taxonomy when bim lands. Greenfield rules allow the rename, but do not leave legacy shims.

### 7.5 Suggested dependency shape for `bim`

- Rust: `semio-framework-3d`, `-mesh-engine`, `-2d`, `-geometry`, plus framework plugin crates. Stdio step, ifc and bcf artifacts as dependencies on the artifact side. Energy and norm referenced through their artifact links.
- Plugin topology: `depends_on("cad", …)` as `aec-building` does, or a standalone `bim` plugin that contributes mutations to `s.cad.cad` the same way (contract freeze §3/§4 in `🌊️flow`/`📐️cad` manifests).
- TS: depend on `@semio-tech/cad-js` (`SpatialKernel`) for the spatial kernel path. Do not import brepjs.

---

## Appendix: key file references

- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🦀️.rs` (composite mutation, inference, 22 typology aliases)
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🟦️.ts` (9 building commands, STEP profile)
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧬️schema/🧬️mutations/🏢️create-building-storey/🦀️.rs` (planning template)
- `✏️s/🔌️plugins/📐️cad/⚙️engine/🏗️construction/🟦️.ts` (linear prism, box, prism from curve)
- `✏️s/🔌️plugins/📐️cad/⚙️engine/🧱️brepjs/🟦️.ts` (oracle adapter; devDependency brepjs)
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts` (production `SemioBrepKernel`, command vocabulary)
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🦀️.rs` and submodules (`🔀️boolean`, `↔️offset`, `🎨️blend`, `➡️sweep`, `✂️intersect`, `🧱️primitives`)
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🦀️.rs` (adaptive CDT tessellation)
- `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️.rs` (`Brep` `*_sync` API)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️.rs` (scalar bim operators)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs` (about 90 geometry operators)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🦀️.rs` and `…/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie` (IFC metadata, COBie)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/…/🧱️base/🚪️io/📐️geometry/🦀️.rs` (STEP read and write)
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema` (`EnergyModelSnapshot.referenced_model`)
- `pyproject.toml` lines 26-50 (test group incl. `ifcopenshell==0.8.4.post1`), `uv.lock` line 597

# Flow Extensions Inventory

**Generated:** 2026-08-29  
**Repository:** semio  
**Scope:** Complete inventory of FLOW EXTENSION neuron kinds contributing to the procedural graph

## Executive Summary

- **Total Extensions:** 10 (9 built, 1 fixture/test)
- **Total Neuron Kinds:** 148+ across all extensions
- **3D-Geometry-Bearing Extensions:** 2 (BREP, Draw)
- **Pure-Data Extensions:** 8 (Math, Text, List, Dictionary, Logic, Primitive, BIM, Draw)
- **Status:** Most extensions built and registered; full wiring validated

---

## 1. Extension Structure Overview

### Source Locations

| Extension | Plugin Source | Built Module | Status |
|-----------|---------------|--------------|--------|
| 🏗️ BIM | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️extension-modules/flow-extension-bim/` | ✅ Built |
| 📐️ BREP | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/` | `🧰️framework/.../🔨️modules/🧑️‍💻️dev/🔌️extension-modules/flow-extension-brep/` | ✅ Built |
| 📖️ Dictionary | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/` | `🧰️framework/.../flow-extension-dictionary/` | ✅ Built |
| 🖍️ Draw | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/` | `🧰️framework/.../flow-extension-draw/` | ✅ Built |
| 🧠️ Logic | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/` | `🧰️framework/.../flow-extension-logic/` | ✅ Built |
| 📃️ List | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/` | `🧰️framework/.../flow-extension-list/` | ✅ Built |
| 🧮️ Math | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/` | `🧰️framework/.../flow-extension-math/` | ✅ Built |
| 🔤️ Primitive | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/` | `🧰️framework/.../flow-extension-primitive/` | ✅ Built |
| 📝️ Text | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/` | `🧰️framework/.../flow-extension-text/` | ✅ Built |
| 🧪️ Fixtures | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧪️fixtures/` | (Test/fixture data) | 🧪 Test |

---

## 2. Complete Neuron Kind Inventory by Extension

### 🏗️ BIM (Building Information Modeling)

**Category:** Domain-specific modeling  
**3D-Bearing:** No (pure structural/semantic data)

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| `bim.element.material` | Element | name, density, conductivity, strength | Material | Building material properties |
| `bim.element.space` | Element | name, area, height | Space | Occupiable space definition |
| `bim.element.wall` | Element | length, height, thickness | Wall | Wall element |
| `bim.element.slab` | Element | width, depth, thickness | Slab | Horizontal slab element |
| `bim.element.column` | Element | width, depth, height | Column | Vertical structural column |
| `bim.element.window` | Element | width, height, sill | Window | Glazed opening |
| `bim.assemble.story` | Assembly | elevation, height, slab; [variadic: elements] | Story | Assemble story from elements |
| `bim.assemble.building` | Assembly | name; [variadic: stories] | Building | Assemble building from stories |
| `bim.measure.floorArea` | Measurement | building | FloorArea | Total floor area across all stories |
| `bim.measure.grossVolume` | Measurement | building | GrossVolume | Gross building volume |

**Total:** 10 neuron kinds

---

### 📐️ BREP (Boundary Representation / CAD Geometry)

**Category:** 3D computational geometry  
**3D-Bearing:** YES ✅ (handles, brep/solid/surface/curve references)

| Neuron Kind ID | Type | Output | Description |
|---|---|---|---|
| **Primitives** | | | |
| `brep.prim.box` | Geometry | solid | Create axis-aligned box |
| `brep.prim.sphere` | Geometry | solid | Create sphere |
| `brep.prim.cylinder` | Geometry | solid | Create cylinder |
| `brep.prim.cone` | Geometry | solid | Create cone |
| `brep.prim.torus` | Geometry | solid | Create torus |
| `brep.prim.convex` | Geometry | solid | Convex hull from points |
| **Curves** | | | |
| `brep.curve.line` | Geometry | curve | Line segment |
| `brep.curve.circle` | Geometry | curve | Circle arc |
| `brep.curve.arc` | Geometry | curve | Arc on circle |
| `brep.curve.ellipse` | Geometry | curve | Ellipse |
| `brep.curve.polyline` | Geometry | wire | Polyline wireframe |
| `brep.curve.polygon` | Geometry | wire | Regular polygon |
| `brep.curve.rectangle` | Geometry | wire | Rectangle wireframe |
| `brep.curve.interpolate` | Geometry | curve | Interpolated spline curve |
| `brep.curve.approximate` | Geometry | curve | Approximated spline curve |
| `brep.curve.helix` | Geometry | curve | Helix curve |
| **Surfaces** | | | |
| `brep.surf.plane` | Geometry | surface | Planar surface |
| `brep.surf.planar` | Geometry | surface | Planar bounded surface |
| `brep.surf.offset` | Geometry | surface | Offset surface |
| `brep.surf.thicken` | Geometry | solid | Thicken face into solid |
| `brep.surf.coons` | Geometry | surface | Coons patch surface |
| `brep.surf.nurbs` | Geometry | surface | NURBS surface |
| **Sweep Operations** | | | |
| `brep.sweep.extrude` | Geometry | solid | Linear extrusion |
| `brep.sweep.revolve` | Geometry | solid | Revolve around axis |
| `brep.sweep.loft` | Geometry | solid | Loft between profiles |
| `brep.sweep.sweep` | Geometry | solid | Sweep profile along path |
| `brep.sweep.pipe` | Geometry | solid | Pipe sweep operation |
| `brep.sweep.helical` | Geometry | solid | Helical sweep |
| **Boolean Operations** | | | |
| `brep.bool.fuse` | Geometry | solid | Boolean union |
| `brep.bool.cut` | Geometry | solid | Boolean difference |
| `brep.bool.intersect` | Geometry | solid | Boolean intersection |
| `brep.bool.compound` | Geometry | solid | Compound solid |
| **Modifications** | | | |
| `brep.solid.fillet` | Geometry | solid | Fillet edges |
| `brep.solid.chamfer` | Geometry | solid | Chamfer edges |
| `brep.solid.offset` | Geometry | solid | Offset solid |
| `brep.solid.shell` | Geometry | solid | Create shell (hollowed solid) |
| `brep.solid.extrude` | Geometry | solid | Extrude face |
| `brep.solid.draft` | Geometry | solid | Apply draft angle |
| `brep.solid.defeature` | Geometry | solid | Remove small features |
| **Transformations** | | | |
| `brep.xform.translate` | Geometry | solid/surface/curve | Translate geometry |
| `brep.xform.rotate` | Geometry | solid/surface/curve | Rotate geometry |
| `brep.xform.scale` | Geometry | solid/surface/curve | Scale geometry |
| `brep.xform.mirror` | Geometry | solid/surface/curve | Mirror across plane |
| `brep.xform.linear` | Geometry | solid | Linear array |
| `brep.xform.circular` | Geometry | solid | Circular array |
| `brep.xform.grid` | Geometry | solid | Grid array |
| `brep.xform.copy` | Geometry | solid | Copy |
| **Intersections** | | | |
| `brep.intersect.curve` | Geometry | point | Curve-curve intersection |
| `brep.intersect.surface` | Geometry | curve | Surface-surface intersection |
| `brep.intersect.section` | Geometry | curve | Planar section |
| `brep.intersect.split` | Geometry | solid/surface/curve | Split by plane |
| **Measurements** | | | |
| `brep.measure.volume` | Data | number | Solid volume |
| `brep.measure.area` | Data | number | Surface area |
| `brep.measure.length` | Data | number | Curve length |
| `brep.measure.distance` | Data | number | Geometric distance |
| `brep.measure.closest` | Data | point | Closest point |
| `brep.measure.center` | Data | point | Center of mass |
| `brep.measure.bounding` | Data | solid | Bounding box |
| `brep.measure.validate` | Data | boolean | Validity check |
| `brep.measure.classify` | Data | text | Geometric classification |
| **Evaluations** | | | |
| `brep.eval.curve` | Data | point/vector | Evaluate curve at parameter |
| `brep.eval.surf` | Data | point/vector | Evaluate surface at parameters |
| **Utilities** | | | |
| `brep.util.heal` | Geometry | solid/surface | Heal geometry |
| `brep.util.sew` | Geometry | solid/surface | Sew faces/shells |
| `brep.util.face` | Geometry | surface | Extract face(s) |
| `brep.util.vertex` | Geometry | point | Extract vertex/vertices |
| `brep.util.convert` | Geometry | wire/surface/solid | Convert geometry type |
| **I/O** | | | |
| `brep.io.import` | I/O | solid/surface/curve | Import from file |
| `brep.io.export` | I/O | file | Export to format |
| **Geometry Routing** | | | |
| `brep.geometry` | Routing | (passthrough) | Geometry passthrough |
| `brep.brep` | Routing | (passthrough) | BREP data passthrough |

**Total:** 61+ neuron kinds

---

### 📖️ Dictionary (Generic Key-Value Operations)

**Category:** Data structure manipulation  
**3D-Bearing:** No

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| `dictionary.pack` | Operator | [variadic inputs] | dictionary | Wrap inputs into dictionary |
| `dictionary.unpack` | Operator | dictionary | dictionary | Forward dictionary unchanged |
| `dictionary.get` | Operator | dictionary, key | value | Read value by key |
| `dictionary.set` | Operator | dictionary, key, value | dictionary | Insert/replace key |
| `dictionary.remove` | Operator | dictionary, key | dictionary | Remove key |
| `dictionary.has` | Operator | dictionary, key | boolean | Key existence check |
| `dictionary.keys` | Operator | dictionary | text | Comma-separated key list |
| `dictionary.size` | Operator | dictionary | number | Count keys |
| `dictionary.merge` | Operator | [variadic: dictionaries] | dictionary | Merge ordered dictionaries |

**Total:** 9 neuron kinds

---

### 🖍️ Draw (2D Vector Graphics)

**Category:** 2D graphics  
**3D-Bearing:** Partially (drawing objects with handles; 2D-only)

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| **Shapes** | | | | |
| `draw.shape.rect` | Graphics | x, y, width, height | draw.drawing | Axis-aligned rectangle |
| `draw.shape.ellipse` | Graphics | cx, cy, rx, ry | draw.drawing | Ellipse |
| `draw.shape.circle` | Graphics | cx, cy, r | draw.drawing | Circle |
| `draw.shape.line` | Graphics | x1, y1, x2, y2 | draw.drawing | Line segment |
| `draw.shape.polygon` | Graphics | points (list) | draw.drawing | Closed polygon |
| **Paths** | | | | |
| `draw.path.rect` | Graphics | x, y, width, height | draw.drawing | Rectangle path |
| `draw.path.polyline` | Graphics | points (list) | draw.drawing | Open polyline |
| **Styling** | | | | |
| `draw.style.fill` | Graphics | drawing, colorR/G/B/A | draw.drawing | Solid fill |
| `draw.style.stroke` | Graphics | drawing, width, colorR/G/B/A | draw.drawing | Stroke outline |
| `draw.gradient.linear` | Graphics | drawing, x1/y1/x2/y2, startColor, endColor | draw.drawing | Linear gradient fill |
| **Transformations** | | | | |
| `draw.xform.translate` | Graphics | drawing, dx, dy | draw.drawing | Translate drawing |
| `draw.xform.rotate` | Graphics | drawing, angle | draw.drawing | Rotate drawing |
| `draw.xform.scale` | Graphics | drawing, sx, sy | draw.drawing | Scale drawing |
| **Grouping** | | | | |
| `draw.group.merge` | Graphics | drawing a, drawing b | draw.drawing | Merge into group |
| **Boolean Operations** | | | | |
| `draw.bool.union` | Graphics | drawing a, drawing b | draw.drawing | Boolean union |
| `draw.bool.difference` | Graphics | drawing a, drawing b | draw.drawing | Boolean difference |
| `draw.bool.intersection` | Graphics | drawing a, drawing b | draw.drawing | Boolean intersection |
| **Text** | | | | |
| `draw.text` | Graphics | x, y, text, size | draw.drawing | Text label |
| **Clipping** | | | | |
| `draw.clip.apply` | Graphics | target drawing, clip path | draw.drawing | Apply clip path |

**Total:** 20 neuron kinds

---

### 🧠️ Logic (Boolean Operators)

**Category:** Boolean logic  
**3D-Bearing:** No

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| `logic.greater` | Operator | a (number), b (number) | boolean | True when a > b |
| `logic.not` | Operator | boolean | boolean | Inverts boolean |

**Total:** 2 neuron kinds

---

### 📃️ List (List/Array Operations)

**Category:** Collection manipulation  
**3D-Bearing:** No

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| `list.empty` | Operator | (none) | list | Create empty list |
| `list.pack` | Operator | [variadic] | list | Wrap items into list |
| `list.get` | Operator | list, index, [wrap, count] | [variadic] | Read value by index |
| `list.set` | Operator | list, index, value | list | Replace at index |
| `list.append` | Operator | list, value | list | Append value |
| `list.remove` | Operator | list, index | list | Remove and reindex |
| `list.size` | Operator | list | number | Count items |
| `list.range` | Operator | start, step, count | list | Arithmetic sequence |
| `list.reverse` | Operator | list | list | Reverse order |
| `list.keys` | Operator | list | text | Comma-separated indices |
| `list.clone` | Operator | list | list | Clone list |
| `list.schema` | Operator | list | text | Report schema |

**Total:** 12 neuron kinds

---

### 🧮️ Math (Arithmetic & Vector Operations)

**Category:** Numerical computation  
**3D-Bearing:** Partially (vector/point support; pure data output)

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| **Arithmetic** | | | | |
| `math.add` | Operator | a, b | sum | Add numbers, points, or vectors |
| `math.addVariadic` | Operator | [variadic: items] | sum | Add any number of values |
| `math.subtract` | Operator | a, b | difference | Subtract numbers, points, or vectors |
| `math.multiply` | Operator | a, b | product | Multiply numbers |
| `math.divide` | Operator | a, b | quotient | Divide a by b |
| `math.modulo` | Operator | a, b | modulo | Remainder of a / b |
| `math.power` | Operator | a, b | power | a to power of b |
| **Unary** | | | | |
| `math.negate` | Operator | number | negated | Negate number |
| `math.abs` | Operator | number | absolute | Absolute value |
| `math.sqrt` | Operator | number | root | Square root |
| `math.floor` | Operator | number | floor | Floor |
| `math.ceil` | Operator | number | ceiling | Ceiling |
| `math.round` | Operator | number | rounded | Round to integer |
| **Trigonometric** | | | | |
| `math.sin` | Operator | number (radians) | sine | Sine |
| `math.cos` | Operator | number (radians) | cosine | Cosine |
| `math.tan` | Operator | number (radians) | tangent | Tangent |
| **Comparison** | | | | |
| `math.min` | Operator | a, b | minimum | Minimum of two numbers |
| `math.max` | Operator | a, b | maximum | Maximum of two numbers |
| **Advanced** | | | | |
| `math.remap` | Operator | value, fromMin/Max, toMin/Max | remapped | Remap value range |
| `math.random` | Operator | seed, min, max | random | Seeded random number |
| `math.sum` | Operator | list | sum | Sum list values |
| **Transform** | | | | |
| `math.move` | Operator | subject (point/vector), vector | point/vector | Translate point or vector |
| **Utilities** | | | | |
| `math.passThrough` | Operator | number | number | Forward number |
| `math.vector` | Operator | x, y, z | vector | Construct vector from components |

**Total:** 25 neuron kinds

---

### 🔤️ Primitive (Core Value Types)

**Category:** Fundamental types / constructors  
**3D-Bearing:** No

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| `core.number` | Constructor | value | number | Produce number dictionary |
| `core.text` | Constructor | value | text | Produce text dictionary |
| `core.boolean` | Constructor | value | boolean | Produce boolean dictionary |
| `core.image` | Constructor | dataUrl | image | Produce image dictionary |
| `core.variable` | Router | [wildcard] | [wildcard] | Relay named typed dictionary |

**Total:** 5 neuron kinds

---

### 📝️ Text (String Operations)

**Category:** Text/string manipulation  
**3D-Bearing:** No

| Neuron Kind ID | Type | Inputs | Outputs | Description |
|---|---|---|---|---|
| `text.concat` | Operator | a (text), b (text) | text | Join two text values |
| `text.upper` | Operator | text | text | Uppercase text |

**Total:** 2 neuron kinds

---

## 3. Summary Statistics

### By Category

| Category | Extension | Count | 3D-Bearing |
|----------|-----------|-------|-----------|
| Domain Modeling | BIM | 10 | No |
| Geometry (CAD) | BREP | 61+ | ✅ YES |
| Graphics (2D) | Draw | 20 | Partial |
| Data Structures | Dictionary, List | 21 | No |
| Arithmetic | Math | 25 | Partial (vectors only) |
| Logic/Control | Logic | 2 | No |
| Type Constructors | Primitive | 5 | No |
| Text | Text | 2 | No |
| **TOTAL** | | **146+** | **61+ (42%)** |

### By Output Type

| Output Type | Count | Examples |
|---|---|---|
| 3D Geometry (brep/solid/surface/curve/wire) | 61+ | brep.prim.box, brep.surf.plane, brep.sweep.extrude |
| 2D Graphics (draw.drawing handles) | 20 | draw.shape.rect, draw.bool.union |
| Pure Numbers | 35+ | math.add, brep.measure.volume, list.size |
| Points/Vectors | 8+ | math.vector, brep.measure.center |
| Text/Strings | 10+ | dictionary.keys, text.concat |
| Booleans | 3+ | logic.greater, dictionary.has |
| Collections (list/dict) | 20+ | list.pack, dictionary.set |
| **TOTAL** | 157+ | |

---

## 4. Extension Registration & Runtime Wiring

### Registration Mechanism

**Source:** `✏️s/🔌️plugins/🌊️flow/🧩️extensions/*/🦀️component.rs`

Each extension implements:
- `pub fn register(registry: &mut Registry)` - Registers neuron kinds
- `pub fn extension_manifest_json() -> String` - Contributes manifest
- `pub fn module_registry() -> Registry` - Builds in-process registry

**Runtime Host:** `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️component.rs`
- Orchestrates WASM component lifecycle
- Handles `flow_neuron_kind_infos_json` distribution
- Manages extension lifecycle (onStartup, handlers, etc.)

### Built Module Registry

Location: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️extension-modules/`

| Extension Module | Outputs | Status |
|---|---|---|
| `flow-extension-bim/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-brep/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-dictionary/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-draw/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-logic/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-list/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-math/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-primitive/` | JavaScript, WASM, interfaces | ✅ Built |
| `flow-extension-text/` | JavaScript, WASM, interfaces | ✅ Built |

---

## 5. Procedural3D Integration

### Example Usage Checkpoint

**Location:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/`

The procedural3d plugin:
- ✅ Has access to all 9 built flow extensions
- ✅ Can dispatch to any registered neuron kind
- ✅ Receives extension manifests via topic contributions
- ✅ Supports both 3D-geometry (BREP) and pure-data (Math, Logic, etc.) paths

### Current Wiring Status

| Extension | Procedural3D Uses | Status |
|---|---|---|
| BREP | Geometric modeling | ✅ Full support |
| Math | Numeric computation | ✅ Full support |
| Draw | 2D visualization (if enabled) | ✅ Available |
| List/Dictionary | Data structure operations | ✅ Available |
| BIM | Semantic building modeling | ✅ Available |
| Logic | Boolean logic gates | ✅ Available |
| Primitive | Core type constructors | ✅ Available |
| Text | String operations | ✅ Available |

**Note:** Most extensions built and available; usage depends on explicit graph wiring in procedural3d examples/fixtures.

---

## 6. Key Observations

1. **BREP Dominance:** BREP extension accounts for ~42% of all neuron kinds; highly specialized for 3D CAD workflows
2. **Math Completeness:** 25+ operators covering full arithmetic, trig, and vector support
3. **Data Structure Layers:** Primitive → Dictionary/List → higher-order operations (BIM, procedural modeling)
4. **Extension Architecture:** Consistent pattern (Operator impl + Registry registration + WASM wrapping)
5. **Type Safety:** All neuron kinds explicitly declare input/output channel types
6. **Runtime Flexibility:** Extension host supports variadic inputs, schema routing, and wildcard channels

---

## 7. Not Wired / Potential Gaps

- **fixtures extension:** Present as test/example data only; not procedurally exposed
- **Conditional extensions:** Logic (only 2 operators) — may need expansion for complex procedural graphs
- **3D Visualization:** Draw extension 2D-only; no 3D rendering neuron kinds found
- **Advanced Mesh Operations:** Basic mesh support via BREP; specialized mesh algorithms not exposed

---

## 8. File References

| File | Size | Purpose |
|---|---|---|
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs` | 834 lines | BIM semantics |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs` | 1930 lines | 3D geometry engine |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` | 1179 lines | 2D graphics |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️component.rs` | 768 lines | Arithmetic/vectors |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️component.rs` | 462 lines | List operations |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🦀️component.rs` | 381 lines | Key-value operations |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️component.rs` | 270 lines | Core types |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/🦀️component.rs` | 191 lines | Boolean logic |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🦀️component.rs` | 180 lines | String ops |

---


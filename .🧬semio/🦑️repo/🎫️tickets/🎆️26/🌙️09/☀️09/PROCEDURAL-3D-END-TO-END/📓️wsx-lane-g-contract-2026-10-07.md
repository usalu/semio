# Lane G contract — generation3d geometry inference (for compute lanes O1–O4)

Status: published 2026-10-08, ahead of the code (types land in the next hour; section 9 lists the build state and is updated as they compile). Names below are final unless section 9 says otherwise.

Roots (repo-relative):

- `GEO` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry`
- Rust path: `crate::standards::v1::subsets::any::schema::inferences::geometry` (alias `geometry` below).
- Catalogue: `crate::standards::v1::subsets::any::schema::catalogue::{catalogue, Kind, Port, PortType, Quality, Localized, ShapeKind, SelectionComponent}` (lane C, read-only for you).

## 1. What a compute lane writes

One Rust module per catalogue category, one registration table per module, keyed by catalogue kind id. Nothing else is touched except one mounting line pair in `GEO/🗃️registry/🦀️.rs` (section 2).

```rust
use super::super::prelude::*; // the category module is a child of `registry`, itself a child of `geometry`

fn box_outputs(inputs: &WidgetInputs) -> Result<Outputs, WidgetFault> {
    let (w, d, h) = (inputs.number("width")?, inputs.number("depth")?, inputs.number("height")?);
    let mut session = KernelSession::new();
    let handle = session.brep().box_prim_sync(w, d, h).map_err(|error| kernel_fault(&error))?;
    Ok(outputs([("shape", GeometryValue::shape(session.export(&handle)?))]))
}

fn box_(kind: &'static Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, box_outputs(&inputs))
}

pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "brep.primitive.box", start: box_ }];
```

Rules:

1. A compute is `fn(&'static Kind, WidgetInputs) -> Box<dyn WidgetJob>`; it is pure over its inputs (no clocks, no randomness, no globals). Determinism is mandatory: the `DepHash` cache keys on kind + params + wiring, never on outputs.
2. Cheap computes (math, vector, list, queries under the 8 ms ceiling) return `finish(kind, Result<Outputs, WidgetFault>)`, which is an already-`Done` job.
3. A kernel call that can exceed the 8 ms interactive ceiling MUST be a stepped `WidgetJob` (section 5): wrap lane J's `BrepOperationJob` / lane K's `ShapeTessellationJob` / mesh jobs; never call a one-shot long kernel operation from `start`.
4. Outputs must use exactly the output port names of the catalogue kind and the value variant that matches the port type (section 3.2). A missing or mistyped output is a contract violation and is turned into a fault by the engine (`generation3d.geometry.output-contract`), so compute tests assert full outputs.
5. Never `unwrap`/`expect`/`panic` on user data: every refusal is a `WidgetFault` with EN+DE messages. Kernel errors go through `kernel_fault(&BrepError)`.
6. Never read the document or other widgets: everything a compute needs arrives resolved in `WidgetInputs`.

## 2. Folder and registration convention

Module files (one `🦀️.rs` per folder, tests in `🧪️tests/🔬️unit/🦀️.rs` beside it, language-agnostic fixture in `🧫️fixtures/🔣️.json`):

| Category | Folder under `GEO/` | Owner |
|---|---|---|
| math-values | `🔢️math-values` | G (seed) |
| brep-primitive | `🧊️brep-primitive` | G seeded box and sphere; O2 adds cylinder, cone, torus, convexHull to the same table |
| math-arithmetic | `🧮️math-arithmetic` | O1 |
| math-vector | `➡️math-vector` | O1 |
| math-list | `📚️math-list` | O1 |
| analysis-measure | `📏️analysis-measure` | O1/Y |
| analysis-check | `✅️analysis-check` | O1/Y |
| brep-curve | `〰️brep-curve` | O2 |
| brep-surface | `🏳️brep-surface` | O2 |
| brep-solid | `🏗️brep-solid` | O2 |
| brep-boolean | `🔗️brep-boolean` | O2 |
| brep-feature | `🛠️brep-feature` | O3 |
| brep-transform | `🔁️brep-transform` | O3 |
| brep-intersect | `✂️brep-intersect` | O3 |
| brep-evaluate | `🎯️brep-evaluate` | O3 |
| brep-topology | `🐚️brep-topology` | O3 |
| brep-interchange | `💾️brep-interchange` | O3 |
| mesh-primitive | `🥽️mesh-primitive` | O4 |
| mesh-convert | `🔀️mesh-convert` | O4 |
| mesh-transform | `↔️mesh-transform` | O4 |
| mesh-component | `🎚️mesh-component` | O4 |
| mesh-edit | `✏️mesh-edit` | O4 |
| mesh-repair | `🩹️mesh-repair` | O4 |
| mesh-inspect | `🔎️mesh-inspect` | O4 |
| mesh-interchange | `📼️mesh-interchange` | O4 |
| mesh-shading | `🌗️mesh-shading` | O4 |
| mesh-uv | `🗺️mesh-uv` | O4 |

(The split between O1–O4 is the coordinator's; the table only fixes folder names so lanes never collide. Folder emojis are chosen unique among siblings and distinct from the family's own folders `💎️value 🔌️inputs ⚙️compute 🪄️widgets 🗃️registry 🛰️service`.)

Each category module exports exactly `pub const COMPUTES: &[ComputeEntry]` (all kinds of that category, no others; the ids must be the catalogue's).

Registration: `GEO/🗃️registry/🦀️.rs` mounts the category modules and lists their tables in one place:

```rust
#[path = "../🔢️math-values/🦀️.rs"] mod math_values;
// ... one `#[path]` line per category module
const TABLES: &[&[ComputeEntry]] = &[math_values::COMPUTES, brep_primitive::COMPUTES /* , yours */];
```

The seed modules `🔢️math-values` and `🧊️brep-primitive` are working templates (fixture in `🧫️fixtures/🔣️.json`, Rust test in `🧪️tests/🔬️unit/🦀️.rs` through `🧪️tests/🧰️oracle-support/🦀️.rs`, three.js oracle in `🧪️tests/🔬️unit/🟦️.ts`). Add your two lines (the `mod` and the `TABLES` element) with a targeted edit; do not reorder or reformat. The registry builds one `BTreeMap<&str, StartFn>` at first use and refuses (test) duplicate ids, unknown ids, and ids of another category's table.

## 3. Value types (`geometry::value`)

### 3.1 `GeometryValue`

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum GeometryValue {
    Number(f64), Integer(i64), Boolean(bool), Text(String),
    Vector([f64; 3]), Point([f64; 3]), Plane(PlaneValue),
    Shape(Arc<ShapeValue>), Mesh(Arc<HalfedgeMesh>),
    Selection(SelectionValue), List(Vec<GeometryValue>),
}
pub struct PlaneValue { pub origin: [f64; 3], pub normal: [f64; 3] }
pub enum SelectionKind { Face, Edge, Vertex }
pub struct SelectionValue { pub component: SelectionKind, pub ids: Vec<u64> }   // B-Rep PersistentLabel numbers, or mesh component indices
impl GeometryValue {
    pub fn shape(value: ShapeValue) -> Self;  pub fn mesh(value: HalfedgeMesh) -> Self;
    pub fn kind_name(&self) -> &'static str;           // "number" | "integer" | ... | "list"
    pub fn byte_len(&self) -> usize;                   // cache accounting estimate
}
```

Angles are radians-or-degrees exactly as the catalogue port `unit` says; `Angle`, `Length` ports carry `Number` values. `Integer` values are accepted by `Number`/`Length`/`Angle` ports; a `Number` is never silently narrowed to an `Integer` port.

### 3.2 Port type to value variant

| `PortType` | variant | `PortType` | variant |
|---|---|---|---|
| number, length, angle | `Number` (`Integer` accepted) | plane | `Plane` |
| integer | `Integer` | shape | `Shape` (kind checked against `shape_kinds`) |
| boolean | `Boolean` | shapes | `List` of `Shape` |
| text, enum | `Text` (enum: one of `options[].value`) | mesh | `Mesh` |
| vector | `Vector` | selection | `Selection` |
| point | `Point` | any | any variant |
| `list: true` on a port | `List` of the element variant | | |

### 3.3 Evaluation and faults

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetEvaluation {
    pub outputs: BTreeMap<String, GeometryValue>,
    pub fault: Option<WidgetFault>,
    pub quality: Quality,                       // catalogue Quality
}
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetFault { pub code: String, pub message: Localized, pub port: Option<String> }
impl WidgetFault {
    pub fn new(code: impl Into<String>, en: impl Into<String>, de: impl Into<String>) -> Self;
    pub fn at(self, port: impl Into<String>) -> Self;
}
pub fn kernel_fault(error: &BrepError) -> WidgetFault;     // code "generation3d.geometry.kernel", EN+DE, kernel message appended
```

A faulted evaluation has `outputs` empty. Downstream widgets that consume a faulted widget fail with `generation3d.geometry.upstream` (EN+DE, `port` = the consuming port) and never run their compute.

Fault code vocabulary (engine-raised, `generation3d.geometry.` prefix): `input-missing`, `input-type`, `input-range`, `input-literal`, `input-multiple`, `input-items`, `input-option`, `shape-kind`, `upstream`, `cycle`, `kind-unknown`, `compute-missing`, `output-contract`, `kernel`, `cancelled`, `widget-unsupported`. Computes add their own domain codes with the same prefix (`generation3d.geometry.boolean-empty`, ...). Every fault carries non-empty, different EN and DE text.

## 4. Typed inputs (`geometry::inputs`)

Computes never resolve wiring, literals or defaults. The engine builds `WidgetInputs` for every catalogue input port, in this order, first hit wins:

1. connected synapse(s) with `to == widget` and `to_port == port` (synapse document order): the parent's output named `from_port` (empty `from_port` means the parent's only output; a parent with several outputs and an empty `from_port` is `input-missing`). A port with `list: true` or type `shapes` gathers all its synapses into one `List` (a connected `List` output is spliced in); every other port with more than one synapse is `input-multiple`.
2. a literal in `Widget::Neuron.params[port]`, decoded per port type (typed literal format below).
3. the catalogue `default` (`DslValue`, decoded per port type: scalar, `[x,y,z]` vector/point, `{origin:[..],normal:[..]}` plane, array for lists/selections).
4. an `optional` port stays absent.
5. otherwise `input-missing`.

Then the engine type-checks (section 3.2), checks numeric `min`/`max`/`exclusiveMin` (`input-range`), list `minItems`/`maxItems` (`input-items`), enum `options` (`input-option`), `shapeKinds` (`shape-kind`). A refusal is a `WidgetFault` with `port` set, localized EN+DE, and the compute does not run.

Typed literal format in `params` (existing, produced by `change-widget-input`): `{ "$schema": "number", value }`, `text`, `boolean`, `point`/`vector` as `{ "$schema", x, y, z }`, lists as `{ "$schema": "list", "0": <literal>, "1": ... }`. Additions this lane decodes (the mutation lane must write the same shapes): `{ "$schema": "plane", "origin": <point literal>, "normal": <vector literal> }` and selections as a list literal of `number` items (ids). An `integer`/`angle`/`length` port uses the `number` literal; an `enum` port uses `text`.

```rust
pub struct WidgetInputs { /* widget id, &'static Kind, resolved values */ }
impl WidgetInputs {
    pub fn widget(&self) -> &str;                       pub fn kind(&self) -> &'static Kind;
    pub fn get(&self, port: &str) -> Option<&GeometryValue>;           // optional ports
    pub fn number(&self, port: &str) -> Result<f64, WidgetFault>;      // Number or Integer
    pub fn integer(&self, port: &str) -> Result<i64, WidgetFault>;
    pub fn boolean(&self, port: &str) -> Result<bool, WidgetFault>;
    pub fn text(&self, port: &str) -> Result<&str, WidgetFault>;
    pub fn vector(&self, port: &str) -> Result<[f64; 3], WidgetFault>;
    pub fn point(&self, port: &str) -> Result<[f64; 3], WidgetFault>;
    pub fn plane(&self, port: &str) -> Result<PlaneValue, WidgetFault>;
    pub fn shape(&self, port: &str) -> Result<&Arc<ShapeValue>, WidgetFault>;
    pub fn shapes(&self, port: &str) -> Result<Vec<&Arc<ShapeValue>>, WidgetFault>;
    pub fn mesh(&self, port: &str) -> Result<&Arc<HalfedgeMesh>, WidgetFault>;
    pub fn selection(&self, port: &str) -> Result<&SelectionValue, WidgetFault>;
    pub fn list(&self, port: &str) -> Result<&[GeometryValue], WidgetFault>;
    pub fn numbers(&self, port: &str) -> Result<Vec<f64>, WidgetFault>;
}
```

The accessors fail with `input-missing`/`input-type` for an absent or other-typed port, so a compute may use `?` throughout.

## 5. Job contract (`geometry::compute`)

```rust
pub enum WidgetStep { Working { progress: f32 /* 0..=1 */ }, Done(WidgetEvaluation) }
pub trait WidgetJob: Send {
    fn step(&mut self, fuel: usize) -> WidgetStep;   // fuel >= 1; does at most `fuel` units of work
    fn cancel(&mut self);                           // idempotent; the job is dropped afterwards
}
pub type StartFn = fn(&'static Kind, WidgetInputs) -> Box<dyn WidgetJob>;
pub struct ComputeEntry { pub id: &'static str, pub start: StartFn }
pub type Outputs = BTreeMap<String, GeometryValue>;
pub fn outputs<const N: usize>(entries: [(&str, GeometryValue); N]) -> Outputs;
pub fn finish(kind: &'static Kind, result: Result<Outputs, WidgetFault>) -> Box<dyn WidgetJob>;            // quality = kind.quality
pub fn finish_with_quality(kind: &'static Kind, result: Result<(Outputs, Quality), WidgetFault>) -> Box<dyn WidgetJob>;
pub fn failed(fault: WidgetFault, quality: Quality) -> Box<dyn WidgetJob>;
```

Cargo workspace: the generation3d crate now has its own workspace, so run cargo inside `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`.

Fuel accounting: one unit is "about one cheap kernel step" (one face tessellated, one boolean pair, one mesh batch). `Working` consumes the whole grant; `Done` consumes one unit and leaves the rest of the grant to the next widget. A job must return within the interactive wall budget for the grant it received, so map `fuel` to the kernel job's own `budget` (`ShapeTessellationJob::step(budget)`). `cancel()` must release kernel sessions; after `Done` the job is dropped. Progress is reported as `Working { progress }` and summed by the engine into the `widget-step` progress unit.

Kernel helpers (same module):

```rust
pub struct KernelSession { /* owns a fresh Brep */ }
impl KernelSession {
    pub fn new() -> Self;                                           pub fn brep(&mut self) -> &mut Brep;
    pub fn import(&mut self, shape: &ShapeValue) -> Result<ImportedShape, WidgetFault>;   // lane K import_shape_mapped
    pub fn export(&self, handle: &GeometryHandle) -> Result<ShapeValue, WidgetFault>;      // lane K export_shape
}
```

Every compute that uses the kernel starts from `KernelSession::new()` (a fresh session) and imports its input `ShapeValue`s; a result is always exported to a `ShapeValue`. Persisted selections are labels of the value they were taken from (`ImportedShape::session_label`).

## 6. Dispatch registry (`geometry::registry`)

```rust
pub fn lookup(kind_id: &str) -> Option<StartFn>;
pub fn start(kind: &'static Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob>;   // unknown compute => failed(compute-missing)
pub fn missing_kinds() -> Vec<&'static str>;                                      // catalogue kinds without a compute
pub fn registered() -> Vec<&'static str>;
```

Exhaustiveness: the test `geometry::registry::tests::expected_red_until_o1_to_o4_every_catalogue_kind_has_a_compute` asserts `missing_kinds().is_empty()`. It is EXPECTED RED from lane G's landing until O1–O4 have delivered all 199 kinds; its failure message prints the missing count and ids. Do not silence it, do not add placeholder computes. Run it with `quick --offline --lib inferences::geometry::registry`.

## 7. Non-neuron widgets (engine-owned, not registered)

| Widget | outputs |
|---|---|
| `InputSlider` | `number`: `Number(value)` |
| `InputNote` | `text`: `Text(text)` |
| `InputImage` | `image`: `Text(src)` |
| `Variable` | `value`: `Text(name)` |
| `OutputPreview` | passes through: the connected parent output (`from_port` empty: all of the parent's outputs; faults pass through unchanged) |
| `OutputAction`, `OutputExport` | no outputs, no fault |
| `Cluster` | fault `widget-unsupported` |

Cycles: every widget on or downstream of a cycle gets fault `generation3d.geometry.cycle` (EN+DE) and no compute runs.

## 8. Engine, cache and service (consumed by lane P, not by compute lanes)

- `InferredField` `geometry::Generation3dGeometry` over `Generation3dSnapshot`: `FIELD_ID = "s.procedural.generation3d.inference.geometry"`, `Key = String` (widget id), `Value = Arc<WidgetEvaluation>`, `plan` = Kahn order over synapses (cycle members trail as roots), `dep_input` = algorithm version + widget variant + kind + kind catalogue hash + literal params + incoming wiring, `reads` = `["hostSnapshot/widgets", "hostSnapshot/synapses"]` (lane M `DiffRegions` paths `hostSnapshot/widgets/<id>`, `hostSnapshot/synapses/<id>`). `Generation3dInference { topology, geometry }` gains the geometry field; the heavy values live in `GeometryHost`, the inference record carries the per-widget summaries (`fault`, `quality`, output kinds, content hashes).
- `GeometryHost` (`geometry::engine`) is the instance-owned engine: typed `InferenceCache`, retained base snapshot, retained `BTreeMap<widget, Arc<WidgetEvaluation>>`, stepping cursor. `Generation3dInstanceOperationOwner.geometry: GeometryHost`.
- Service `s.procedural.generation3d.geometry` (`ArtifactInferenceService::new_contextual`, context = `&GeometryHost`), payload contract files `GEO/🛰️service/📥️request.json` and `📤️result.json`, progress unit `widget-step`, `work_units` = fuel, resumable through `previous_state` = the previous result payload, cache modes `incremental` (use the cache), `cold` (clear then use), `bypass` (no cache read or write).

## 9. Build state

(updated by lane G as the code lands; compute lanes: do not rely on a name until its row says "compiles")

| Piece | State |
|---|---|
| Framework stepped driver + typed cache (`protocol::{infer_field_step, try_infer_field, InferenceCursor, ComputeStep, InferencePending, InferenceFault, InferenceError}`) | compiles, 26/26 `os_inference` tests pass (run) |
| `geometry::{value, inputs, compute, registry, widgets}` and `prelude` | written to this contract; NOT yet compiled: the dependency graph (neural-engine retirement API, os-flow, os-infinite) is mid-edit by peers. Names are final; if a compile shows drift, this row says so |
| Seed computes (`math-values` 8 kinds, `brep-primitive` box and sphere) | written, not yet compiled; fixture-driven TS oracle tests pass (6/6, run) |
| Engine (`engine::{GeometryEngine, GeometryHost, CacheMode}`) + service (`service::geometry_inference_service`) | written, not yet compiled |

Compute lanes: write against the signatures above; the first green `cargo check -p semio-s-artifact-procedural-generation3d --lib` run from the generation3d workspace will be recorded here.

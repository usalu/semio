# Flow Evaluation Contract & 3D Geometry Rendering

## 1. Public API Surface

### 1.1 Type Definitions

**FlowFixture** (`artifact/🦀️component.rs:253`)
```rust
pub struct FlowFixture {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    pub layout: OrderedMap<WidgetLayout>,
}
```

**Widget** (`artifact/🦀️component.rs:180`)
```rust
pub enum Widget {
    Neuron {
        id: String,
        neuron_kind: String,
        params: Dictionary,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        preview: bool,
    },
    InputSlider { id: String, label: String, value: f64, min: f64, max: f64, step: f64 },
    InputNote { id: String, text: String },
    InputImage { id: String, src: String },
    Variable { id: String, name: String, schema: String },
    OutputPreview { id: String, preview: Dictionary, expanded: OrderedSet },
    OutputAction { id: String, action: String },
    OutputExport { id: String, format: String },
    Cluster { id: String, name: String, tree: Tree, flow: FlowGui },
}
```

**FlowHost** (`host/🦀️component.rs:135`)
```rust
pub struct FlowHost {
    pub fixture: FlowFixture,
    pub dag: DagHost,
    pub outputs: BTreeMap<String, Dictionary>,
    pub last_eval_json: String,
    // ... internal state
}
```

**FlowEvalSession** (`host/🦀️component.rs:2265`)
```rust
pub struct FlowEvalSession { state: std::mem::ManuallyDrop<FlowEvalSessionState> }
```

### 1.2 Core Methods

**FlowHost::evaluate()** (`host/🦀️component.rs:301`)
```rust
pub fn evaluate(&mut self) -> Result<String, FlowCoreError> {
    self.evaluate_internal();
    Ok(self.last_eval_json.clone())
}
```

**tessellate_geometry()** (`brep-geometry/🦀️component.rs:501`)
```rust
pub fn tessellate_geometry(handle: &str, tolerance: f64) -> Result<semio_framework::MeshData, String>
```

**flow_neuron_kind_infos_json()** (`catalogue/🦀️component.rs:159`)
```rust
pub fn flow_neuron_kind_infos_json() -> String {
    let registry = flow_extension_registry();
    serde_json::to_string(&registry.operator_infos().collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into())
}
```

---

## 2. Exact JSON Shape of evaluate() Output

The JSON shape is built by `build_channel_eval_json()` (`bridge/🦀️component.rs:287`):

```rust
pub(crate) fn build_channel_eval_json(
    fixture: &FlowFixture,
    channels: &EvalChannels,
    kind_infos: &HashMap<String, OperatorInfo>
) -> String {
    let mut widgets = serde_json::Map::new();
    for widget in &fixture.widgets {
        let id = widget_id_for(widget);
        let mut entry = serde_json::Map::new();
        entry.insert("in".into(), serde_json::Value::Object(input_ports_json(&input_dict, kind_info)));
        entry.insert("out".into(), serde_json::Value::Object(output_dict.map(output_ports_json).unwrap_or_default()));
        if let Some(error) = output_dict.get("error").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
            entry.insert("error".into(), serde_json::Value::String(error.to_string()));
        }
        widgets.insert(id.to_string(), serde_json::Value::Object(entry));
    }
    serde_json::to_string(&widgets).unwrap_or_else(|_| "{}".into())
}
```

### JSON Structure

**Top Level:** Keyed by widget ID (string)

```json
{
  "widget1": {
    "in": { /* input ports */ },
    "out": { /* output ports */ },
    "error": "optional error message"
  },
  "widget2": { ... }
}
```

**Input Ports Object** (`bridge/🦀️component.rs:36`): Contains channel name → value mappings:
- For `Widget::Neuron`: merged from both neuron params and evaluated input channels
- For other widgets: just the evaluated input channels
- Variadic inputs via `variadic.slot_key` dictionary

**Output Ports Object** (`bridge/🦀️component.rs:61`): All keys from evaluated output dictionary

Example output for a math.add neuron:
```json
{
  "slider": {
    "in": {},
    "out": { "number": { "value": 3.0 } }
  },
  "add": {
    "in": { "a": { "value": 3.0 }, "b": { "value": 0.0 } },
    "out": { "sum": { "value": 3.0 } }
  },
  "preview": {
    "in": { "sum": { "value": 3.0 } },
    "out": {}
  }
}
```

---

## 3. Channels: Definition, Enumeration, Values

### 3.1 What is a Channel?

A **channel** is a named port on an operator (neuron kind) that carries typed data. Channels have:
- **name** (port identifier string, e.g., "a", "sum", "number")
- **schema/operators** (value type constraints, e.g., "geometry", "number")
- **cardinality** (ZeroOrMore, OneOrMore, Optional, Required)
- Optional **default** value

### 3.2 How Channels are Enumerated

Channels for a neuron kind come from `OperatorInfo`:

```rust
pub struct OperatorInfo {
    pub id: String,
    pub inputs: Vec<ChannelSpec>,
    pub outputs: Vec<ChannelSpec>,
    pub variadic_input: Option<VariadicSpec>,
    pub variadic_output: Option<VariadicSpec>,
    // ...
}

pub struct ChannelSpec {
    pub code: String,
    pub abbreviation: String,
    pub name: String,
    pub full_name: String,
    pub operators: Vec<String>,  // schema constraints
    pub default: Option<Value>,
    pub label: Option<String>,
    pub cardinality: Cardinality,
}
```

The `flow_neuron_kind_infos_json()` returns JSON-serialized `OperatorInfo[]` for all registered neuron kinds in the extension registry.

**For variadic inputs/outputs**, the actual port list is dynamic:
- `VariadicSpec` specifies slot_key, min, and optional max
- Input/output ports stored as string indices ("0", "1", "2", ...) in `Widget::{Neuron.input_ports, Neuron.output_ports}`

### 3.3 Channel Value Shapes

Channel values can carry:

**Plain Scalars:**
- `{ "value": <f64> }` (numbers)
- `{ "value": <string> }` (text)

**Spatial Types:**
- **Point**: `{ "x": f64, "y": f64, "z": f64 }`
- **Vector**: `{ "x": f64, "y": f64, "z": f64 }`

**Geometry** (carries 3D rendering):
- `{ "handle": "<string>", "kind": "solid|wire|face|edge|surface|curve|vertex|compound" }`
  - Handle is a Blake3 hex digest or prefixed ID (e.g., "solid-xyz")
  - Identifies an opaque brep object in kernel

**Collections:**
- **List**: `{ "0": <value>, "1": <value>, ... }` with schema="list"
- **Nested Lists**: Lists containing Lists containing typed values

**Complex Dictionaries:**
- Any Dictionary with optional schema field
- Can nest arbitrarily

---

## 4. Renderable Geometry Values

### 4.1 Which Values Carry Geometry

Only dictionaries with a **"handle"** field identify geometry:

```rust
fn is_brep_geometry_handle(handle: &str) -> bool {
    if handle.is_empty() { return false; }
    if ["vertex-", "edge-", "wire-", "face-", "shell-", "solid-", "compound-", "curve-", "surface-"]
        .iter().any(|prefix| handle.starts_with(prefix)) {
        return true;
    }
    handle.len() == 64 && handle.as_bytes().iter().all(u8::is_ascii_hexdigit)
}
```

**Geometry Collection** (`bridge/🦀️component.rs:328`):
```rust
pub(crate) fn collect_geometry_handles_from_dictionary(dict: &Dictionary, handles: &mut Vec<String>) {
    if let Some(handle) = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()) {
        if is_brep_geometry_handle(handle) {
            handles.push(handle.to_string());
        }
    }
    for key in dict.keys() {
        if let Some(value) = dict.get(key) {
            collect_geometry_handles_from_value(value, handles);
        }
    }
}
```

### 4.2 tessellate_geometry() Signature & Return Type

```rust
pub fn tessellate_geometry(handle: &str, tolerance: f64) -> Result<semio_framework::MeshData, String>
```

**MeshData** fields (from semio_framework):
- `positions: Vec<f64>` — flat array of (x, y, z) triples
- `normals: Vec<f64>` — flat array of (nx, ny, nz) triples
- `indices: Vec<u32>` — triangle vertex indices into positions
- `edgePositions` or `edge_positions: Vec<f64>` — optional edge geometry

**Supported Handle Kinds:**
- `solid` → tessellates to MeshData with triangles + normals
- `wire` → returns edge positions only
- `face` → tessellates to MeshData
- `edge` → returns edge positions
- `curve` → returns curve positions
- Non-geometry values → returns error String

### 4.3 Current 3D Rendering Support

**Renderable Today:**
- ✅ Brep geometry (solids, faces, shells, wires, curves via tessellation → MeshData)
- ✅ Points (3-component dicts with x,y,z)
- ✅ Vectors (3-component dicts with x,y,z)

**Not Yet Renderable:**
- ❌ Plain numbers (scalars)
- ❌ Text/strings
- ❌ Arbitrary dictionaries without "handle"
- ❌ Lists (though can contain geometry)
- ❌ Images (data URLs)
- ❌ Nested structures without geometry handles

---

## 5. Extensions & Registry

Neuron kinds are registered via the **flow extension registry** (`registry/🦀️component.rs`).

**Extension Structure:**
1. Plugin provides `OperatorInfo[]` + `Operator` implementations
2. Registered via `register_operator(info, impls, produces)`
3. First-party extensions: math, text, logic, dictionary, list, brep
4. Runtime-installable via `semio_s_plugin_flow_extension_*` crates

**For Procedural Plugin:**
- Call `flow_neuron_kind_infos_json()` to enumerate available kinds
- Pass JSON to `FlowHost::set_neuron_kind_infos_json()`
- Parse returned `evaluate()` JSON to extract geometry handles
- Call `tessellate_geometry(handle, tolerance)` for each geometry handle
- Render resulting MeshData


//! 🔷 Flow brep module: brepkit-backed geometry operators.

use geometry_brep_brepkit::BrepkitKernel;
use geometry_brep_engine::{block_on, BrepKernel, GeometryHandle, Vec3};
use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use std::sync::{Mutex, OnceLock};

static KERNEL: OnceLock<Mutex<BrepkitKernel>> = OnceLock::new();

fn kernel() -> &'static Mutex<BrepkitKernel> {
    KERNEL.get_or_init(|| Mutex::new(BrepkitKernel::new()))
}

// #region 🔖Helpers
fn with_kernel<T>(f: impl FnOnce(&mut BrepkitKernel) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = kernel().lock().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    f(&mut guard)
}

fn geometry_dictionary(handle: &GeometryHandle) -> Dictionary {
    Dictionary::with_schema("geometry")
        .insert("handle", Value::Atom(Atom::String(handle.as_str().to_string())))
        .insert("kind", Value::Atom(Atom::String("solid".into())))
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value")
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_geometry(input: &Dictionary, key: &str) -> Result<GeometryHandle, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let handle = dict
        .get("handle")
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_str())
        .ok_or_else(|| EvalError::MissingInput(format!("{key}.handle")))?;
    Ok(GeometryHandle(handle.to_string()))
}

fn read_xyz(input: &Dictionary, key: &str) -> Result<Vec3, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    Ok([
        dict.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
        dict.get("y").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
        dict.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
    ])
}

fn map_kernel_error(error: geometry_brep_engine::BrepError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

fn geometry_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id])
}

fn out_geometry() -> ChannelSpec {
    ChannelSpec::provides("out", vec![])
}

fn operator_info(id: &str, name: &str, abbreviation: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        module: "brep".into(),
        name: name.into(),
        abbreviation: abbreviation.into(),
        icon: icon.into(),
        summary: summary.into(),
        inputs,
        outputs: vec![out_geometry()],
        group: group.iter().map(|entry| (*entry).to_string()).collect(),
        ..Default::default()
    }
}

fn register_untyped(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operation }], produces);
}

fn geometry_schema() -> Schema {
    Schema {
        id: "geometry".into(),
        module: "brep".into(),
        name: "Geometry".into(),
        icon: "emoji:🔷".into(),
        summary: "Opaque brep geometry handle".into(),
        fields: vec![
            FieldSpec::new("handle", ValueType::Text),
            FieldSpec::new("kind", ValueType::Text).with_default(Value::Atom(Atom::String("solid".into()))),
        ],
    }
}
// #endregion 🔖Helpers

// #region 🔖Primitives
struct BoxPrim;
impl Operation for BoxPrim {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.box_prim(
                read_channel_number(input, "width")?,
                read_channel_number(input, "depth")?,
                read_channel_number(input, "height")?,
            ))
            .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct SpherePrim;
impl Operation for SpherePrim {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.sphere_prim(read_channel_number(input, "radius")?)).map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct CylinderPrim;
impl Operation for CylinderPrim {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.cylinder_prim(read_channel_number(input, "radius")?, read_channel_number(input, "height")?))
                .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct ConePrim;
impl Operation for ConePrim {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.cone_prim(read_channel_number(input, "radius")?, read_channel_number(input, "height")?))
                .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct TorusPrim;
impl Operation for TorusPrim {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.torus_prim(read_channel_number(input, "major")?, read_channel_number(input, "minor")?))
                .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}
// #endregion 🔖Primitives

// #region 🔖Booleans
struct Fuse;
impl Operation for Fuse {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.fuse(&read_geometry(input, "a")?, &read_geometry(input, "b")?)).map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct Cut;
impl Operation for Cut {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.cut(&read_geometry(input, "a")?, &read_geometry(input, "b")?)).map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct Intersect;
impl Operation for Intersect {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.intersect(&read_geometry(input, "a")?, &read_geometry(input, "b")?)).map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}
// #endregion 🔖Booleans

// #region 🔖Transforms
struct Translate;
impl Operation for Translate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.translate(&read_geometry(input, "geometry")?, read_xyz(input, "offset")?))
                .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct Rotate;
impl Operation for Rotate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.rotate(
                &read_geometry(input, "geometry")?,
                read_xyz(input, "axis")?,
                read_channel_number(input, "angle")?,
            ))
            .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct Scale;
impl Operation for Scale {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.scale(
                &read_geometry(input, "geometry")?,
                read_channel_number(input, "factor")?,
                read_xyz(input, "center")?,
            ))
            .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct Mirror;
impl Operation for Mirror {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.mirror(
                &read_geometry(input, "geometry")?,
                read_xyz(input, "origin")?,
                read_xyz(input, "normal")?,
            ))
            .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}
// #endregion 🔖Transforms

// #region 🔖Solid
struct Fillet;
impl Operation for Fillet {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.fillet(&read_geometry(input, "geometry")?, read_channel_number(input, "radius")?))
                .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}

struct Chamfer;
impl Operation for Chamfer {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let handle = block_on(kernel.chamfer(&read_geometry(input, "geometry")?, read_channel_number(input, "distance")?))
                .map_err(map_kernel_error)?;
            Ok(geometry_dictionary(&handle))
        })
    }
}
// #endregion 🔖Solid

// #region 🔖Measure
struct Volume;
impl Operation for Volume {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let value = block_on(kernel.volume(&read_geometry(input, "geometry")?)).map_err(map_kernel_error)?;
            Ok(number_dictionary(value))
        })
    }
}
// #endregion 🔖Measure

/// 📦 Registers brep geometry schema and operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(geometry_schema());

    register_untyped(
        registry,
        operator_info(
            "brep.prim3d.box",
            "Box",
            "Box",
            "emoji:📦",
            "Axis-aligned box solid",
            vec![
                number_channel("width", "brep.prim3d.box", 1.0),
                number_channel("depth", "brep.prim3d.box", 1.0),
                number_channel("height", "brep.prim3d.box", 1.0),
            ],
            &["Primitives 3D"],
        ),
        Box::new(BoxPrim),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.prim3d.sphere",
            "Sphere",
            "Sphere",
            "emoji:⚪",
            "Sphere solid",
            vec![number_channel("radius", "brep.prim3d.sphere", 1.0)],
            &["Primitives 3D"],
        ),
        Box::new(SpherePrim),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.prim3d.cylinder",
            "Cylinder",
            "Cylinder",
            "emoji:🛢️",
            "Cylinder solid",
            vec![number_channel("radius", "brep.prim3d.cylinder", 1.0), number_channel("height", "brep.prim3d.cylinder", 1.0)],
            &["Primitives 3D"],
        ),
        Box::new(CylinderPrim),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.prim3d.cone",
            "Cone",
            "Cone",
            "emoji:🛢️",
            "Cone solid",
            vec![number_channel("radius", "brep.prim3d.cone", 1.0), number_channel("height", "brep.prim3d.cone", 1.0)],
            &["Primitives 3D"],
        ),
        Box::new(ConePrim),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.prim3d.torus",
            "Torus",
            "Torus",
            "emoji:🛢️",
            "Torus solid",
            vec![number_channel("major", "brep.prim3d.torus", 2.0), number_channel("minor", "brep.prim3d.torus", 0.5)],
            &["Primitives 3D"],
        ),
        Box::new(TorusPrim),
        &["geometry"],
    );

    register_untyped(
        registry,
        operator_info(
            "brep.bool.fuse",
            "Fuse",
            "Fuse",
            "emoji:🔗",
            "Boolean union",
            vec![geometry_channel("a", "brep.bool.fuse"), geometry_channel("b", "brep.bool.fuse")],
            &["Booleans"],
        ),
        Box::new(Fuse),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.bool.cut",
            "Cut",
            "Cut",
            "emoji:🔗",
            "Boolean difference",
            vec![geometry_channel("a", "brep.bool.cut"), geometry_channel("b", "brep.bool.cut")],
            &["Booleans"],
        ),
        Box::new(Cut),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.bool.intersect",
            "Intersect",
            "Intersect",
            "emoji:🔗",
            "Boolean intersection",
            vec![geometry_channel("a", "brep.bool.intersect"), geometry_channel("b", "brep.bool.intersect")],
            &["Booleans"],
        ),
        Box::new(Intersect),
        &["geometry"],
    );

    register_untyped(
        registry,
        operator_info(
            "brep.xform.translate",
            "Translate",
            "Translate",
            "emoji:🔁",
            "Translate geometry",
            vec![geometry_channel("geometry", "brep.xform.translate"), ChannelSpec::requires("offset", &["math.move"])],
            &["Transforms"],
        ),
        Box::new(Translate),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.xform.rotate",
            "Rotate",
            "Rotate",
            "emoji:🔁",
            "Rotate geometry",
            vec![
                geometry_channel("geometry", "brep.xform.rotate"),
                number_channel("angle", "brep.xform.rotate", std::f64::consts::FRAC_PI_4),
                ChannelSpec::requires("axis", &["brep.xform.rotate"]),
            ],
            &["Transforms"],
        ),
        Box::new(Rotate),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.xform.scale",
            "Scale",
            "Scale",
            "emoji:🔁",
            "Scale geometry",
            vec![
                geometry_channel("geometry", "brep.xform.scale"),
                number_channel("factor", "brep.xform.scale", 2.0),
                ChannelSpec::requires("center", &["brep.xform.scale"]),
            ],
            &["Transforms"],
        ),
        Box::new(Scale),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.xform.mirror",
            "Mirror",
            "Mirror",
            "emoji:🔁",
            "Mirror geometry",
            vec![
                geometry_channel("geometry", "brep.xform.mirror"),
                ChannelSpec::requires("origin", &["brep.xform.mirror"]),
                ChannelSpec::requires("normal", &["brep.xform.mirror"]),
            ],
            &["Transforms"],
        ),
        Box::new(Mirror),
        &["geometry"],
    );

    register_untyped(
        registry,
        operator_info(
            "brep.solid.fillet",
            "Fillet",
            "Fillet",
            "emoji:🧱",
            "Fillet all solid edges",
            vec![geometry_channel("geometry", "brep.solid.fillet"), number_channel("radius", "brep.solid.fillet", 0.1)],
            &["Solid"],
        ),
        Box::new(Fillet),
        &["geometry"],
    );
    register_untyped(
        registry,
        operator_info(
            "brep.solid.chamfer",
            "Chamfer",
            "Chamfer",
            "emoji:🧱",
            "Chamfer all solid edges",
            vec![geometry_channel("geometry", "brep.solid.chamfer"), number_channel("distance", "brep.solid.chamfer", 0.1)],
            &["Solid"],
        ),
        Box::new(Chamfer),
        &["geometry"],
    );

    registry.register_operator(
        operator_info(
            "brep.measure.volume",
            "Volume",
            "Volume",
            "emoji:📐",
            "Solid volume",
            vec![geometry_channel("geometry", "brep.measure.volume")],
            &["Measure"],
        ),
        vec![OperatorImpl {
            schemas: vec![],
            operation: Box::new(Volume),
        }],
        &["number"],
    );

    registry.finalize();
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::{build_manifest_json, evaluate_json};

    #[test]
    fn box_emits_geometry_handle() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("width", Value::Dictionary(number_dictionary(2.0)))
            .insert("depth", Value::Dictionary(number_dictionary(3.0)))
            .insert("height", Value::Dictionary(number_dictionary(4.0)));
        let out = reg.dispatch("brep.prim3d.box", &input).unwrap();
        assert_eq!(out.schema(), Some("geometry"));
        assert!(out.get("handle").and_then(|v| v.as_atom()).and_then(|a| a.as_str()).unwrap().starts_with("solid-"));
    }

    #[test]
    fn fillet_translate_chain() {
        let mut reg = Registry::new();
        register(&mut reg);
        let box_out = reg
            .dispatch(
                "brep.prim3d.box",
                &Dictionary::new()
                    .insert("width", Value::Dictionary(number_dictionary(2.0)))
                    .insert("depth", Value::Dictionary(number_dictionary(2.0)))
                    .insert("height", Value::Dictionary(number_dictionary(2.0))),
            )
            .unwrap();
        let fillet_out = reg
            .dispatch(
                "brep.solid.fillet",
                &Dictionary::new()
                    .insert("geometry", Value::Dictionary(box_out))
                    .insert("radius", Value::Dictionary(number_dictionary(0.1))),
            )
            .unwrap();
        let vector = Dictionary::with_schema("vector")
            .insert("x", Value::Atom(Atom::Decimal(1.0)))
            .insert("y", Value::Atom(Atom::Decimal(0.0)))
            .insert("z", Value::Atom(Atom::Decimal(0.0)));
        let moved = reg
            .dispatch(
                "brep.xform.translate",
                &Dictionary::new()
                    .insert("geometry", Value::Dictionary(fillet_out))
                    .insert("offset", Value::Dictionary(vector)),
            )
            .unwrap();
        assert_eq!(moved.schema(), Some("geometry"));
    }

    #[test]
    fn manifest_lists_brep_operators() {
        let json = build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        assert!(json.contains("brep.prim3d.box"));
        assert!(json.contains("\"operators\""));
        assert!(json.contains("brep.xform.translate"));
    }

    #[test]
    fn evaluate_json_box() {
        let reg = module_registry();
        let input = Dictionary::new()
            .insert("width", Value::Dictionary(number_dictionary(1.0)))
            .insert("depth", Value::Dictionary(number_dictionary(1.0)))
            .insert("height", Value::Dictionary(number_dictionary(1.0)));
        let out_json = evaluate_json(&reg, "brep.prim3d.box", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.schema(), Some("geometry"));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(target_arch = "wasm32")]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json};
    use geometry_brep_engine::{BrepKernel, GeometryHandle};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json("brep", "Brep", "0.3.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
    }

    #[wasm_bindgen]
    pub fn evaluate(kind_id: &str, input_json: &str) -> String {
        evaluate_json(&module_registry(), kind_id, input_json)
    }

    #[wasm_bindgen]
    pub fn command(command_id: &str, args_json: &str) -> String {
        command_json(command_id, args_json)
    }

    #[wasm_bindgen]
    pub fn tessellate(handle: &str, tolerance: f64) -> String {
        super::kernel()
            .lock()
            .ok()
            .and_then(|kernel| {
                let geometry = GeometryHandle(handle.to_string());
                match block_on(kernel.tessellate(&geometry, tolerance)) {
                    Ok(mesh) => Some(serde_json::to_string(&mesh).unwrap_or_else(|_| "{}".into())),
                    Err(error) => Some(serde_json::json!({ "error": error.to_string() }).to_string()),
                }
            })
            .unwrap_or_else(|| serde_json::json!({ "error": "brep kernel unavailable" }).to_string())
    }

    #[wasm_bindgen]
    pub fn dispose(handle: &str) {
        if let Ok(mut kernel) = super::kernel().lock() {
            block_on(kernel.dispose(&GeometryHandle(handle.to_string())));
        }
    }

    #[wasm_bindgen]
    pub fn activate() {}

    #[wasm_bindgen]
    pub fn deactivate() {}
}
// #endregion 🔖WasmExt

//! ➕️ Flow math module: schema-dispatched arithmetic operators.

#[path = "⏱️retained/🦀️.rs"]
mod retained;
use retained::{MathAction, MathOutput, math_operator};

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, VariadicSpec};

// #region 🔖️Add
/// ➕️ Adds numbers, points, or vectors.
pub struct Add;

math_operator!(Add);
// #endregion 🔖️Add

// #region 🔖️Subtract
/// ➖️ Subtracts numbers, points, or vectors.
pub struct Subtract;

math_operator!(Subtract);
// #endregion 🔖️Subtract

// #region 🔖️Move
/// 🚚️ Moves a point or vector by a vector.
pub struct Move;

math_operator!(Move);
// #endregion 🔖️Move

// #region 🔖️Scalar
/// ✖️ Multiplies two numbers.
pub struct Multiply;

math_operator!(Multiply);

/// ➗️ Divides a by b.
pub struct Divide;

math_operator!(Divide);

/// ⚡️ Raises a to the power of b.
pub struct Power;

math_operator!(Power);

/// 🧮️ Remainder of a divided by b.
pub struct Modulo;

math_operator!(Modulo);

/// ↔ Negates a number.
pub struct Negate;

math_operator!(Negate);

/// 📏️ Absolute value.
pub struct Abs;

math_operator!(Abs);

/// √ Square root.
pub struct Sqrt;

math_operator!(Sqrt);

/// ⬇️ Minimum of two numbers.
pub struct Min;

math_operator!(Min);

/// ⬆️ Maximum of two numbers.
pub struct Max;

math_operator!(Max);

/// ⬇️ Floor of a number.
pub struct Floor;

math_operator!(Floor);

/// ⬆️ Ceiling of a number.
pub struct Ceil;

math_operator!(Ceil);

/// ⭕️ Rounds a number.
pub struct Round;

math_operator!(Round);

/// 〰 Sine in radians.
pub struct Sin;

math_operator!(Sin);

/// 〰 Cosine in radians.
pub struct Cos;

math_operator!(Cos);

/// 〰 Tangent in radians.
pub struct Tan;

math_operator!(Tan);

/// 🗺️ Remaps a value from one range to another.
pub struct Remap;

math_operator!(Remap);

/// 🎲️ Seeded or entropy-backed random number in [min, max].
pub struct Random;

math_operator!(Random);

/// ➡️ Forwards the number input unchanged.
pub struct PassThrough;

math_operator!(PassThrough);

/// ∑ Sums all numbers in a list dictionary.
pub struct Sum;

math_operator!(Sum);
// #endregion 🔖️Scalar

fn compute(action: MathAction, input: &Dictionary) -> Result<MathOutput, EvalError> {
    match action {
        MathAction::Add => {
            let a = read_dict(input, "a")?;
            let b = read_dict(input, "b")?;
            match a.schema() {
                Some("point") => Ok(MathOutput::Coordinates("sum", "point", read_xyz(a)? + read_xyz(b)?)),
                Some("vector") => Ok(MathOutput::Coordinates("sum", "vector", read_xyz(a)? + read_xyz(b)?)),
                _ => Ok(MathOutput::Number("sum", read_value_number(a)? + read_value_number(b)?)),
        }
        }
        MathAction::Subtract => {
            let a = read_dict(input, "a")?;
            let b = read_dict(input, "b")?;
            match a.schema() {
                Some("point") => Ok(MathOutput::Coordinates("difference", "point", read_xyz(a)? - read_xyz(b)?)),
                Some("vector") => Ok(MathOutput::Coordinates("difference", "vector", read_xyz(a)? - read_xyz(b)?)),
                _ => Ok(MathOutput::Number("difference", read_value_number(a)? - read_value_number(b)?)),
        }
        }
        MathAction::Move => {
            let subject = read_dict(input, "subject")?;
            let offset = read_dict(input, "offset")?;
            Ok(MathOutput::Moved(read_xyz(subject)? + read_xyz(offset)?))
        }
        MathAction::Multiply => {
            Ok(MathOutput::Number("product", read_channel_number(input, "a")? * read_channel_number(input, "b")?))
        }
        MathAction::Divide => {
            let b = read_channel_number(input, "b")?;
            if b.abs() < f64::EPSILON {
                return Err(EvalError::InvalidInput("divide by zero".into()));
        }
        Ok(MathOutput::Number("quotient", read_channel_number(input, "a")? / b))
        }
        MathAction::Power => {
            Ok(MathOutput::Number("power", read_channel_number(input, "a")?.powf(read_channel_number(input, "b")?)))
        }
        MathAction::Modulo => {
            let b = read_channel_number(input, "b")?;
            if b.abs() < f64::EPSILON {
                return Err(EvalError::InvalidInput("modulo by zero".into()));
        }
        Ok(MathOutput::Number("modulo", read_channel_number(input, "a")? % b))
        }
        MathAction::Negate => {
            Ok(MathOutput::Number("negated", -read_channel_number(input, "number")?))
        }
        MathAction::Abs => {
            Ok(MathOutput::Number("absolute", read_channel_number(input, "number")?.abs()))
        }
        MathAction::Sqrt => {
            Ok(MathOutput::Number("root", read_channel_number(input, "number")?.sqrt()))
        }
        MathAction::Min => {
            Ok(MathOutput::Number("minimum", read_channel_number(input, "a")?.min(read_channel_number(input, "b")?)))
        }
        MathAction::Max => {
            Ok(MathOutput::Number("maximum", read_channel_number(input, "a")?.max(read_channel_number(input, "b")?)))
        }
        MathAction::Floor => {
            Ok(MathOutput::Number("floor", read_channel_number(input, "number")?.floor()))
        }
        MathAction::Ceil => {
            Ok(MathOutput::Number("ceiling", read_channel_number(input, "number")?.ceil()))
        }
        MathAction::Round => {
            Ok(MathOutput::Number("rounded", read_channel_number(input, "number")?.round()))
        }
        MathAction::Sin => {
            Ok(MathOutput::Number("sine", read_channel_number(input, "number")?.sin()))
        }
        MathAction::Cos => {
            Ok(MathOutput::Number("cosine", read_channel_number(input, "number")?.cos()))
        }
        MathAction::Tan => {
            Ok(MathOutput::Number("tangent", read_channel_number(input, "number")?.tan()))
        }
        MathAction::Remap => {
            let value = read_channel_number(input, "value")?;
            let from_min = read_channel_number(input, "fromMin")?;
            let from_max = read_channel_number(input, "fromMax")?;
            let to_min = read_channel_number(input, "toMin")?;
            let to_max = read_channel_number(input, "toMax")?;
            let span = from_max - from_min;
            if span.abs() < f64::EPSILON {
                return Err(EvalError::InvalidInput("remap span is zero".into()));
        }
        Ok(MathOutput::Number("remapped", to_min + ((value - from_min) / span) * (to_max - to_min)))
        }
        MathAction::Random => {
            let min = read_channel_number(input, "min")?;
            let max = read_channel_number(input, "max")?;
            let seed = input.get("seed").and_then(Value::as_dictionary).and_then(optional_value_number).map(f64::to_bits);
            Ok(MathOutput::Number("random", min + (max - min) * next_random_unit(seed)))
        }
        MathAction::PassThrough => {
            Ok(MathOutput::Number("numberOut", read_channel_number(input, "number")?))
        }
        MathAction::Sum => unreachable!("list sum is evaluated by the fold"),
    }
}


// #region 🔖️Helpers
#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn xyz_dictionary(schema: &str, value: Vec3) -> Dictionary {
    Dictionary::with_schema(schema).insert("x", Value::Atom(Atom::Decimal(value.x))).insert("y", Value::Atom(Atom::Decimal(value.y))).insert("z", Value::Atom(Atom::Decimal(value.z)))
}

fn read_dict<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    read_value_number(read_dict(input, key)?)
}

fn optional_value_number(input: &Dictionary) -> Option<f64> {
    input.get("value").and_then(Value::as_atom).and_then(Atom::as_f64)
}

fn read_value_number(input: &Dictionary) -> Result<f64, EvalError> {
    optional_value_number(input).ok_or_else(|| EvalError::MissingInput("value".into()))
}

fn read_xyz(input: &Dictionary) -> Result<Vec3, EvalError> {
    Ok(Vec3::new(read_field_number(input, "x")?, read_field_number(input, "y")?, read_field_number(input, "z")?))
}

fn read_field_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn splitmix64(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn random_unit(seed: u64) -> f64 {
    splitmix64(seed) as f64 / u64::MAX as f64
}

fn entropy_seed() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0x0C0F_FEE0_0D15_EA5E)
}

fn next_random_unit(explicit_seed: Option<u64>) -> f64 {
    random_unit(explicit_seed.unwrap_or_else(entropy_seed))
}

fn number_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::number_default(id, 0.0, &[operator_id])
}

fn sum_out() -> ChannelSpec {
    ChannelSpec::named("S", "Sum", "sum", "Sum")
}

fn difference_out() -> ChannelSpec {
    ChannelSpec::named("D", "Dif", "difference", "Difference")
}

fn product_out() -> ChannelSpec {
    ChannelSpec::named("P", "Prd", "product", "Product")
}

fn quotient_out() -> ChannelSpec {
    ChannelSpec::named("Q", "Quo", "quotient", "Quotient")
}

fn power_out() -> ChannelSpec {
    ChannelSpec::named("P", "Pow", "power", "Power")
}

fn modulo_out() -> ChannelSpec {
    ChannelSpec::named("M", "Mod", "modulo", "Modulo")
}

fn negated_out() -> ChannelSpec {
    ChannelSpec::named("N", "Neg", "negated", "Negated")
}

fn absolute_out() -> ChannelSpec {
    ChannelSpec::named("A", "Abs", "absolute", "Absolute")
}

fn root_out() -> ChannelSpec {
    ChannelSpec::named("R", "Roo", "root", "Root")
}

fn floor_out() -> ChannelSpec {
    ChannelSpec::named("F", "Flr", "floor", "Floor")
}

fn ceiling_out() -> ChannelSpec {
    ChannelSpec::named("C", "Ceil", "ceiling", "Ceiling")
}

fn rounded_out() -> ChannelSpec {
    ChannelSpec::named("R", "Rnd", "rounded", "Rounded")
}

fn sine_out() -> ChannelSpec {
    ChannelSpec::named("S", "Sin", "sine", "Sine")
}

fn cosine_out() -> ChannelSpec {
    ChannelSpec::named("C", "Cos", "cosine", "Cosine")
}

fn tangent_out() -> ChannelSpec {
    ChannelSpec::named("T", "Tan", "tangent", "Tangent")
}

fn number_out() -> ChannelSpec {
    ChannelSpec::named("N", "Num", "numberOut", "Number")
}

fn minimum_out() -> ChannelSpec {
    ChannelSpec::named("Mi", "Min", "minimum", "Minimum")
}

fn maximum_out() -> ChannelSpec {
    ChannelSpec::named("Ma", "Max", "maximum", "Maximum")
}

fn remapped_out() -> ChannelSpec {
    ChannelSpec::named("R", "Rem", "remapped", "Remapped")
}

fn random_out() -> ChannelSpec {
    ChannelSpec::named("R", "Rnd", "random", "Random")
}

fn vector_out() -> ChannelSpec {
    ChannelSpec::named("V", "Vec", "vector", "Vector")
}

fn point_out() -> ChannelSpec {
    ChannelSpec::named("P", "Pnt", "point", "Point")
}

fn move_out() -> Vec<ChannelSpec> {
    vec![point_out(), vector_out()]
}

fn schema(id: &str, name: &str) -> Schema {
    Schema {
        id: id.into(),
        module: "math".into(),
        name: name.into(),
        icon: "emoji:🧭️".into(),
        summary: format!("{name} with x, y, z decimal fields"),
        fields: vec![FieldSpec::decimal_default("x", 0.0), FieldSpec::decimal_default("y", 0.0), FieldSpec::decimal_default("z", 0.0)],
    }
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "math".into(), name: name.into(), abbreviation: abbreviation.into(), icon: "emoji:➕️".into(), summary: summary.into(), inputs, outputs, ..Default::default() }
}

fn register_simple<O: Operator + 'static>(registry: &mut Registry, info: OperatorInfo, operation: O, schemas: Vec<&str>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: schemas.into_iter().map(str::to_string).collect(), operator: Box::new(operation) }], produces);
}

// #endregion 🔖️Helpers

/// 📦️ Registers all math schemas and operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(schema("point", "Point"));
    registry.register_schema(schema("vector", "Vector"));

    let scalar = vec![number_channel("a", "math.add"), number_channel("b", "math.add")];
    let sum_output = vec![sum_out()];
    registry.register_operator(
        operator_info("math.add", "Add", "Add", "Adds numbers, points, or vectors", scalar, sum_output.clone()),
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(Add) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operator: Box::new(Add) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operator: Box::new(Add) },
        ],
        &["number", "point", "vector"],
    );
    registry.register_operator(
        OperatorInfo { variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }), ..operator_info("math.addVariadic", "Add Variadic", "Add", "Adds any number of numbers, points, or vectors", vec![], sum_output.clone()) },
        vec![
            OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Add) },
            OperatorImpl { schemas: vec!["point".into()], operator: Box::new(Add) },
            OperatorImpl { schemas: vec!["vector".into()], operator: Box::new(Add) },
        ],
        &["number", "point", "vector"],
    );
    let subtract_scalar = vec![number_channel("a", "math.subtract"), number_channel("b", "math.subtract")];
    registry.register_operator(
        operator_info("math.subtract", "Subtract", "Sub", "Subtracts numbers, points, or vectors", subtract_scalar, vec![difference_out()]),
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(Subtract) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operator: Box::new(Subtract) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operator: Box::new(Subtract) },
        ],
        &["number", "point", "vector"],
    );
    let binary_scalar = vec![number_channel("a", "math.multiply"), number_channel("b", "math.multiply")];
    register_simple(registry, operator_info("math.multiply", "Multiply", "Mul", "Multiplies two numbers", binary_scalar.clone(), vec![product_out()]), Multiply, vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.divide", "Divide", "Div", "Divides a by b", binary_scalar.clone(), vec![quotient_out()]), Divide, vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.power", "Power", "Pow", "Raises a to the power of b", binary_scalar.clone(), vec![power_out()]), Power, vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.modulo", "Modulo", "Mod", "Remainder of a divided by b", binary_scalar, vec![modulo_out()]), Modulo, vec!["number", "number"], &["number"]);

    register_simple(registry, operator_info("math.negate", "Negate", "Neg", "Negates a number", vec![number_channel("number", "math.negate")], vec![negated_out()]), Negate, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.abs", "Abs", "Abs", "Absolute value", vec![number_channel("number", "math.abs")], vec![absolute_out()]), Abs, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.sqrt", "Sqrt", "Sqrt", "Square root", vec![number_channel("number", "math.sqrt")], vec![root_out()]), Sqrt, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.floor", "Floor", "Flr", "Floor of a number", vec![number_channel("number", "math.floor")], vec![floor_out()]), Floor, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.ceil", "Ceil", "Ceil", "Ceiling of a number", vec![number_channel("number", "math.ceil")], vec![ceiling_out()]), Ceil, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.round", "Round", "Rnd", "Rounds a number", vec![number_channel("number", "math.round")], vec![rounded_out()]), Round, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.sin", "Sin", "Sin", "Sine in radians", vec![number_channel("number", "math.sin")], vec![sine_out()]), Sin, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.cos", "Cos", "Cos", "Cosine in radians", vec![number_channel("number", "math.cos")], vec![cosine_out()]), Cos, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.tan", "Tan", "Tan", "Tangent in radians", vec![number_channel("number", "math.tan")], vec![tangent_out()]), Tan, vec!["number"], &["number"]);
    register_simple(registry, operator_info("math.passThrough", "PassThrough", "Pass", "Forwards a number", vec![number_channel("number", "math.passThrough")], vec![number_out()]), PassThrough, vec!["number"], &["number"]);

    register_simple(registry, operator_info("math.min", "Min", "Min", "Minimum of two numbers", vec![number_channel("a", "math.min"), number_channel("b", "math.min")], vec![minimum_out()]), Min, vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.max", "Max", "Max", "Maximum of two numbers", vec![number_channel("a", "math.max"), number_channel("b", "math.max")], vec![maximum_out()]), Max, vec!["number", "number"], &["number"]);
    register_simple(
        registry,
        operator_info(
            "math.remap",
            "Remap",
            "Map",
            "Remaps a value from one range to another",
            vec![number_channel("value", "math.remap"), number_channel("fromMin", "math.remap"), number_channel("fromMax", "math.remap"), number_channel("toMin", "math.remap"), number_channel("toMax", "math.remap")],
            vec![remapped_out()],
        ),
        Remap,
        vec!["number", "number", "number", "number", "number"],
        &["number"],
    );
    register_simple(
        registry,
        operator_info(
            "math.random",
            "Random",
            "Rnd",
            "Random number in range with optional seed",
            vec![number_channel("seed", "math.random"), number_channel("min", "math.random"), ChannelSpec::number_default("max", 1.0, &["math.random"])],
            vec![random_out()],
        ),
        Random,
        vec!["number", "number", "number"],
        &["number"],
    );
    register_simple(registry, operator_info("math.sum", "Sum", "Sum", "Sums numbers in a list dictionary", vec![ChannelSpec::list("list", &["math.sum"])], sum_output), Sum, vec!["list"], &["number"]);
    registry.register_operator(
        operator_info("math.move", "Move", "Move", "Moves a point or vector by a vector", vec![ChannelSpec::requires("subject", &["math.move"]), ChannelSpec::requires("offset", &["math.move"])], move_out()),
        vec![OperatorImpl { schemas: vec!["point".into(), "vector".into()], operator: Box::new(Move) }, OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operator: Box::new(Move) }],
        &["point", "vector"],
    );
    registry.finalize();
}

// #region 🔖️Manifest
/// 📦️ Flow extension manifest JSON contributed to host catalogues.
pub fn extension_manifest_json() -> String {
    use flow_extension_sdk::{build_manifest_json, FlowExtensionCommand, FlowExtensionSetting};
    build_manifest_json(
        "math",
        "Math",
        env!("CARGO_PKG_VERSION"),
        &neural_engine::ColdOwner::new(module_registry()),
        vec!["onStartup".into()],
        vec![],
        vec![FlowExtensionCommand { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
        vec![FlowExtensionSetting { id: "math.defaultPrecision".into(), setting_type: "number".into(), default: flow_extension_sdk::integer_setting_default(1), description: "Decimal places for number preview".into() }],
    )
}

/// 🌊️ Builds an in-process operator registry for this extension.
pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Manifest

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔖️ExtensionGuest
/// 🧩️ Runtime-installable flow extension bundle for `math`.
pub mod extension_guest {
    use super::{extension_manifest_json, module_registry};
    use flow_extension_sdk::flow_extension_topic_contribution;
    use semio_plugin::{ExecutionMode, ExtensionBundle};

    const FLOW_APP_ID: &str = "flow-play";
    const PROCEDURAL3D_APP_ID: &str = "procedural3d-play";
    const EXTENSION_ID: &str = "math";
    const EXTENSION_LABEL: &str = "Math";

    // 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires
    // a plain sync fn). `.mode`/`.contributes_topic`/`.handler` are still `async fn` in
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's
    // path_scope); bridged via `semio_os_kernel::io::resolve_ready` — see this packet's lease-request.
    // See R9.
    pub fn bundle() -> ExtensionBundle {
        let manifest_json = extension_manifest_json();
        let flow_topic = flow_extension_topic_contribution(FLOW_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "math", &manifest_json);
        let procedural3d_topic = flow_extension_topic_contribution(PROCEDURAL3D_APP_ID, EXTENSION_ID, EXTENSION_LABEL, "math", &manifest_json);
        let bundle = ExtensionBundle::new("flow-extension-math", EXTENSION_LABEL, env!("CARGO_PKG_VERSION")).extends("flow").depends_on("flow", semio_framework::tree_pin!());
        let bundle = bundle.mode(ExecutionMode::Linked);
        let bundle = bundle.contributes_topic(flow_topic.topic, flow_topic.payload);
        let bundle = bundle.contributes_topic(procedural3d_topic.topic, procedural3d_topic.payload);
        bundle.resource_owner(flow_extension_sdk::ExtensionEvaluationResources::new(module_registry())).owned_handler("evaluate")
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️extension-guest-standalone/🦀️.rs");

}
// #endregion 🔖️ExtensionGuest

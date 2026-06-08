//! ➕ Flow math module: schema-dispatched arithmetic operators.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType, VariadicSpec};
use std::cell::Cell;

// #region 🔖Add
/// ➕ Adds numbers, points, or vectors.
pub struct Add;

impl Operation for Add {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if let Some(items) = input.get("items").and_then(|v| v.as_dictionary()) {
            return add_items(items);
        }
        let a = read_dict(input, "a")?;
        let b = read_dict(input, "b")?;
        match a.schema() {
            Some("point") => Ok(xyz_dictionary("point", read_xyz(a)? + read_xyz(b)?)),
            Some("vector") => Ok(xyz_dictionary("vector", read_xyz(a)? + read_xyz(b)?)),
            _ => Ok(number_dictionary(read_value_number(a)? + read_value_number(b)?)),
        }
    }
}
// #endregion 🔖Add

// #region 🔖Subtract
/// ➖ Subtracts numbers, points, or vectors.
pub struct Subtract;

impl Operation for Subtract {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_dict(input, "a")?;
        let b = read_dict(input, "b")?;
        match a.schema() {
            Some("point") => Ok(xyz_dictionary("point", read_xyz(a)? - read_xyz(b)?)),
            Some("vector") => Ok(xyz_dictionary("vector", read_xyz(a)? - read_xyz(b)?)),
            _ => Ok(number_dictionary(read_value_number(a)? - read_value_number(b)?)),
        }
    }
}
// #endregion 🔖Subtract

// #region 🔖ConstructXyz
/// 🧭 Constructs a vector dictionary from x, y, z numbers.
pub struct ConstructVector;

impl Operation for ConstructVector {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(xyz_dictionary("vector", Vec3::new(read_channel_number(input, "x")?, read_channel_number(input, "y")?, read_channel_number(input, "z")?)))
    }
}

/// 📍 Constructs a point dictionary from x, y, z numbers.
pub struct ConstructPoint;

impl Operation for ConstructPoint {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(xyz_dictionary("point", Vec3::new(read_channel_number(input, "x")?, read_channel_number(input, "y")?, read_channel_number(input, "z")?)))
    }
}
// #endregion 🔖ConstructXyz

// #region 🔖Move
/// 🚚 Moves a point or vector by a vector.
pub struct Move;

impl Operation for Move {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let subject = read_dict(input, "subject")?;
        let vector = read_dict(input, "vector")?;
        Ok(xyz_dictionary(subject.schema().unwrap_or("vector"), read_xyz(subject)? + read_xyz(vector)?))
    }
}
// #endregion 🔖Move

// #region 🔖Scalar
/// ✖️ Multiplies two numbers.
pub struct Multiply;

impl Operation for Multiply {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "a")? * read_channel_number(input, "b")?))
    }
}

/// ➗ Divides a by b.
pub struct Divide;

impl Operation for Divide {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let b = read_channel_number(input, "b")?;
        if b.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("divide by zero".into()));
        }
        Ok(number_dictionary(read_channel_number(input, "a")? / b))
    }
}

/// ⚡ Raises a to the power of b.
pub struct Power;

impl Operation for Power {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "a")?.powf(read_channel_number(input, "b")?)))
    }
}

/// 🧮 Remainder of a divided by b.
pub struct Modulo;

impl Operation for Modulo {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let b = read_channel_number(input, "b")?;
        if b.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("modulo by zero".into()));
        }
        Ok(number_dictionary(read_channel_number(input, "a")? % b))
    }
}

/// ↔️ Negates a number.
pub struct Negate;

impl Operation for Negate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(-read_channel_number(input, "number")?))
    }
}

/// 📏 Absolute value.
pub struct Abs;

impl Operation for Abs {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.abs()))
    }
}

/// √ Square root.
pub struct Sqrt;

impl Operation for Sqrt {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.sqrt()))
    }
}

/// ⬇️ Minimum of two numbers.
pub struct Min;

impl Operation for Min {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "a")?.min(read_channel_number(input, "b")?)))
    }
}

/// ⬆️ Maximum of two numbers.
pub struct Max;

impl Operation for Max {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "a")?.max(read_channel_number(input, "b")?)))
    }
}

/// ⬇️ Floor of a number.
pub struct Floor;

impl Operation for Floor {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.floor()))
    }
}

/// ⬆️ Ceiling of a number.
pub struct Ceil;

impl Operation for Ceil {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.ceil()))
    }
}

/// ⭕ Rounds a number.
pub struct Round;

impl Operation for Round {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.round()))
    }
}

/// 〰️ Sine in radians.
pub struct Sin;

impl Operation for Sin {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.sin()))
    }
}

/// 〰️ Cosine in radians.
pub struct Cos;

impl Operation for Cos {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.cos()))
    }
}

/// 〰️ Tangent in radians.
pub struct Tan;

impl Operation for Tan {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?.tan()))
    }
}

/// 🗺️ Remaps a value from one range to another.
pub struct Remap;

impl Operation for Remap {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let value = read_channel_number(input, "value")?;
        let from_min = read_channel_number(input, "fromMin")?;
        let from_max = read_channel_number(input, "fromMax")?;
        let to_min = read_channel_number(input, "toMin")?;
        let to_max = read_channel_number(input, "toMax")?;
        let span = from_max - from_min;
        if span.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("remap span is zero".into()));
        }
        Ok(number_dictionary(to_min + ((value - from_min) / span) * (to_max - to_min)))
    }
}

/// 🎲 Seeded or entropy-backed random number in [min, max].
pub struct Random;

impl Operation for Random {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let min = read_channel_number(input, "min")?;
        let max = read_channel_number(input, "max")?;
        let seed = read_channel_number(input, "seed").ok().map(f64::to_bits);
        Ok(number_dictionary(min + (max - min) * next_random_unit(seed)))
    }
}

/// ➡️ Forwards the number input unchanged.
pub struct PassThrough;

impl Operation for PassThrough {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(number_dictionary(read_channel_number(input, "number")?))
    }
}

/// ∑ Sums all numbers in a list dictionary.
pub struct Sum;

impl Operation for Sum {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_dict(input, "list")?;
        let mut total = 0.0;
        for index in list_indices(list) {
            let value = list
                .get(&index.to_string())
                .and_then(|v| v.as_dictionary())
                .map(read_value_number)
                .transpose()?
                .ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
            total += value;
        }
        Ok(number_dictionary(total))
    }
}
// #endregion 🔖Scalar

// #region 🔖Helpers
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

thread_local! {
    static ENTROPY_SEED: Cell<u64> = const { Cell::new(0) };
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn xyz_dictionary(schema: &str, value: Vec3) -> Dictionary {
    Dictionary::with_schema(schema)
        .insert("x", Value::Atom(Atom::Decimal(value.x)))
        .insert("y", Value::Atom(Atom::Decimal(value.y)))
        .insert("z", Value::Atom(Atom::Decimal(value.z)))
}

fn add_items(items: &Dictionary) -> Result<Dictionary, EvalError> {
    let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    let first = indices.first().ok_or_else(|| EvalError::MissingInput("items".into()))?;
    let first_dict = read_dict(items, &first.to_string())?;
    match first_dict.schema() {
        Some("point") | Some("vector") => {
            let mut total = Vec3::new(0.0, 0.0, 0.0);
            for index in indices {
                total = total + read_xyz(read_dict(items, &index.to_string())?)?;
            }
            Ok(xyz_dictionary(first_dict.schema().unwrap_or("vector"), total))
        }
        _ => {
            let mut total = 0.0;
            for index in indices {
                total += read_value_number(read_dict(items, &index.to_string())?)?;
            }
            Ok(number_dictionary(total))
        }
    }
}

fn read_dict<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    read_value_number(read_dict(input, key)?)
}

fn read_value_number(input: &Dictionary) -> Result<f64, EvalError> {
    input
        .get("value")
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput("value".into()))
}

fn read_xyz(input: &Dictionary) -> Result<Vec3, EvalError> {
    Ok(Vec3::new(read_field_number(input, "x")?, read_field_number(input, "y")?, read_field_number(input, "z")?))
}

fn read_field_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
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
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Math::random() * u64::MAX as f64) as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0)
    }
}

fn next_random_unit(explicit_seed: Option<u64>) -> f64 {
    if let Some(seed) = explicit_seed {
        return random_unit(seed);
    }
    ENTROPY_SEED.with(|cell| {
        let seed = cell.get();
        let next = if seed == 0 { entropy_seed() } else { splitmix64(seed) };
        cell.set(next);
        random_unit(next)
    })
}

fn number_channel(id: &str) -> ChannelSpec {
    ChannelSpec::number_default(id, 0.0)
}

fn schema(id: &str, name: &str) -> Schema {
    Schema {
        id: id.into(),
        module: "math".into(),
        name: name.into(),
        icon: "emoji:🧭".into(),
        summary: format!("{name} with x, y, z decimal fields"),
        fields: vec![FieldSpec::decimal_default("x", 0.0), FieldSpec::decimal_default("y", 0.0), FieldSpec::decimal_default("z", 0.0)],
    }
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        module: "math".into(),
        name: name.into(),
        abbreviation: abbreviation.into(),
        icon: "emoji:➕".into(),
        summary: summary.into(),
        inputs,
        outputs,
        ..Default::default()
    }
}

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>, schemas: Vec<&str>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: schemas.into_iter().map(str::to_string).collect(), operation }]);
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖Helpers

/// 📦 Registers all math schemas and operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(schema("point", "Point"));
    registry.register_schema(schema("vector", "Vector"));

    let scalar = vec![number_channel("a"), number_channel("b")];
    let scalar_out = vec![ChannelSpec::number("out")];
    registry.register_operator(
        operator_info("math.add", "Add", "Add", "Adds numbers, points, or vectors", scalar.clone(), scalar_out.clone()),
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Add) },
        ],
    );
    registry.register_operator(
        OperatorInfo {
            variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
            ..operator_info("math.addVariadic", "Add Variadic", "Add", "Adds any number of numbers, points, or vectors", vec![], scalar_out.clone())
        },
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Add) },
        ],
    );
    registry.register_operator(
        operator_info("math.subtract", "Subtract", "Sub", "Subtracts numbers, points, or vectors", scalar.clone(), scalar_out.clone()),
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(Subtract) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operation: Box::new(Subtract) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Subtract) },
        ],
    );
    register_simple(registry, operator_info("math.multiply", "Multiply", "Mul", "Multiplies two numbers", scalar.clone(), scalar_out.clone()), Box::new(Multiply), vec!["number", "number"]);
    register_simple(registry, operator_info("math.divide", "Divide", "Div", "Divides a by b", scalar.clone(), scalar_out.clone()), Box::new(Divide), vec!["number", "number"]);
    register_simple(registry, operator_info("math.power", "Power", "Pow", "Raises a to the power of b", scalar.clone(), scalar_out.clone()), Box::new(Power), vec!["number", "number"]);
    register_simple(registry, operator_info("math.modulo", "Modulo", "Mod", "Remainder of a divided by b", scalar.clone(), scalar_out.clone()), Box::new(Modulo), vec!["number", "number"]);

    for (id, name, abbreviation, summary, op) in [
        ("math.negate", "Negate", "Neg", "Negates a number", Box::new(Negate) as Box<dyn Operation>),
        ("math.abs", "Abs", "Abs", "Absolute value", Box::new(Abs)),
        ("math.sqrt", "Sqrt", "Sqrt", "Square root", Box::new(Sqrt)),
        ("math.floor", "Floor", "Flr", "Floor of a number", Box::new(Floor)),
        ("math.ceil", "Ceil", "Ceil", "Ceiling of a number", Box::new(Ceil)),
        ("math.round", "Round", "Rnd", "Rounds a number", Box::new(Round)),
        ("math.sin", "Sin", "Sin", "Sine in radians", Box::new(Sin)),
        ("math.cos", "Cos", "Cos", "Cosine in radians", Box::new(Cos)),
        ("math.tan", "Tan", "Tan", "Tangent in radians", Box::new(Tan)),
        ("math.passThrough", "PassThrough", "Pass", "Forwards a number", Box::new(PassThrough)),
    ] {
        register_simple(registry, operator_info(id, name, abbreviation, summary, vec![number_channel("number")], scalar_out.clone()), op, vec!["number"]);
    }

    register_simple(registry, operator_info("math.min", "Min", "Min", "Minimum of two numbers", scalar.clone(), scalar_out.clone()), Box::new(Min), vec!["number", "number"]);
    register_simple(registry, operator_info("math.max", "Max", "Max", "Maximum of two numbers", scalar.clone(), scalar_out.clone()), Box::new(Max), vec!["number", "number"]);
    register_simple(
        registry,
        operator_info(
            "math.remap",
            "Remap",
            "Map",
            "Remaps a value from one range to another",
            vec![number_channel("value"), number_channel("fromMin"), number_channel("fromMax"), number_channel("toMin"), number_channel("toMax")],
            scalar_out.clone(),
        ),
        Box::new(Remap),
        vec!["number", "number", "number", "number", "number"],
    );
    register_simple(
        registry,
        operator_info("math.random", "Random", "Rnd", "Random number in range with optional seed", vec![number_channel("seed"), number_channel("min"), ChannelSpec::number_default("max", 1.0)], scalar_out.clone()),
        Box::new(Random),
        vec!["number", "number", "number"],
    );
    register_simple(registry, operator_info("math.sum", "Sum", "Sum", "Sums numbers in a list dictionary", vec![ChannelSpec::list("list")], scalar_out.clone()), Box::new(Sum), vec!["list"]);

    let xyz_inputs = vec![number_channel("x"), number_channel("y"), number_channel("z")];
    register_simple(
        registry,
        operator_info("math.constructVector", "Construct Vector", "Vec", "Builds a vector from x, y, z", xyz_inputs.clone(), vec![ChannelSpec::new("out", ValueType::Schema("vector".into()))]),
        Box::new(ConstructVector),
        vec!["number", "number", "number"],
    );
    register_simple(
        registry,
        operator_info("math.constructPoint", "Construct Point", "Point", "Builds a point from x, y, z", xyz_inputs, vec![ChannelSpec::new("out", ValueType::Schema("point".into()))]),
        Box::new(ConstructPoint),
        vec!["number", "number", "number"],
    );
    registry.register_operator(
        operator_info(
            "math.move",
            "Move",
            "Move",
            "Moves a point or vector by a vector",
            vec![ChannelSpec::new("subject", ValueType::Any), ChannelSpec::new("vector", ValueType::Schema("vector".into()))],
            vec![ChannelSpec::new("out", ValueType::Any)],
        ),
        vec![
            OperatorImpl { schemas: vec!["point".into(), "vector".into()], operation: Box::new(Move) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Move) },
        ],
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1, FlowModuleSettingV1};

    #[test]
    fn add_sums_number_dictionaries() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(3.0))).insert("b", Value::Dictionary(number_dictionary(1.1)));
        let out = reg.dispatch("math.add", &input).unwrap();
        assert_eq!(out.schema(), Some("number"));
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.1));
    }

    #[test]
    fn construct_vector_uses_xyz_channels() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("x", Value::Dictionary(number_dictionary(1.0)))
            .insert("y", Value::Dictionary(number_dictionary(2.0)))
            .insert("z", Value::Dictionary(number_dictionary(3.0)));
        let out = reg.dispatch("math.constructVector", &input).unwrap();
        assert_eq!(out.schema(), Some("vector"));
        assert_eq!(out.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn move_translates_point() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("subject", Value::Dictionary(xyz_dictionary("point", Vec3::new(1.0, 2.0, 3.0))))
            .insert("vector", Value::Dictionary(xyz_dictionary("vector", Vec3::new(4.0, 5.0, 6.0))));
        let out = reg.dispatch("math.move", &input).unwrap();
        assert_eq!(out.schema(), Some("point"));
        assert_eq!(out.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(5.0));
    }

    #[test]
    fn manifest_lists_math_operators_and_schemas() {
        let json = build_manifest_json(
            "math",
            "Math",
            "0.2.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
            vec![FlowModuleSettingV1 {
                id: "math.defaultPrecision".into(),
                setting_type: "number".into(),
                default: serde_json::json!(1),
                description: "Decimal places for number preview".into(),
            }],
        );
        assert!(json.contains("flow.module/v1"));
        assert!(json.contains("math.constructVector"));
        assert!(json.contains("vector"));
    }

    #[test]
    fn evaluate_json_adds_numbers() {
        let reg = module_registry();
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(2.0))).insert("b", Value::Dictionary(number_dictionary(1.0)));
        let out_json = evaluate_json(&reg, "math.add", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.schema(), Some("number"));
        assert_eq!(out.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn random_is_deterministic_with_seed() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("seed", Value::Dictionary(number_dictionary(42.0)))
            .insert("min", Value::Dictionary(number_dictionary(0.0)))
            .insert("max", Value::Dictionary(number_dictionary(1.0)));
        let first = reg.dispatch("math.random", &input).unwrap();
        let second = reg.dispatch("math.random", &input).unwrap();
        assert_eq!(first.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), second.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()));
    }

    #[test]
    fn divide_rejects_zero() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(1.0))).insert("b", Value::Dictionary(number_dictionary(0.0)));
        assert!(reg.dispatch("math.divide", &input).is_err());
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(target_arch = "wasm32")]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json, FlowModuleCommandV1, FlowModuleSettingV1};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json(
            "math",
            "Math",
            "0.2.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommandV1 { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
            vec![FlowModuleSettingV1 {
                id: "math.defaultPrecision".into(),
                setting_type: "number".into(),
                default: serde_json::json!(1),
                description: "Decimal places for number preview".into(),
            }],
        )
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
    pub fn activate() {}

    #[wasm_bindgen]
    pub fn deactivate() {}
}
// #endregion 🔖WasmExt

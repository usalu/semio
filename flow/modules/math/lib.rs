//! ➕ Flow math module: neuron kinds for arithmetic.

use neural_engine::{Atom, Dictionary, EvalError, Function, InputSpec, NeuronKindInfo, Registry, Value};
use std::cell::Cell;

// #region 🔖Add
/// ➕ Sums two number inputs into one number output.
pub struct Add;

impl Function for Add {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b").unwrap_or(0.0);
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a + b))))
    }
}
// #endregion 🔖Add

// #region 🔖Subtract
/// ➖ Subtracts b from a.
pub struct Subtract;

impl Function for Subtract {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b").unwrap_or(0.0);
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a - b))))
    }
}
// #endregion 🔖Subtract

// #region 🔖Multiply
/// ✖️ Multiplies two number inputs.
pub struct Multiply;

impl Function for Multiply {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a")?;
        let b = read_number(input, "b")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a * b))))
    }
}
// #endregion 🔖Multiply

// #region 🔖Divide
/// ➗ Divides a by b.
pub struct Divide;

impl Function for Divide {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b")?;
        if b.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("divide by zero".into()));
        }
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a / b))))
    }
}
// #endregion 🔖Divide

// #region 🔖Power
/// ⚡ Raises a to the power of b.
pub struct Power;

impl Function for Power {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a.powf(b)))))
    }
}
// #endregion 🔖Power

// #region 🔖Modulo
/// 🧮 Remainder of a divided by b.
pub struct Modulo;

impl Function for Modulo {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b")?;
        if b.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("modulo by zero".into()));
        }
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a % b))))
    }
}
// #endregion 🔖Modulo

// #region 🔖Negate
/// ↔️ Negates a number.
pub struct Negate;

impl Function for Negate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(-n))))
    }
}
// #endregion 🔖Negate

// #region 🔖Abs
/// 📏 Absolute value.
pub struct Abs;

impl Function for Abs {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.abs()))))
    }
}
// #endregion 🔖Abs

// #region 🔖Sqrt
/// √ Square root.
pub struct Sqrt;

impl Function for Sqrt {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.sqrt()))))
    }
}
// #endregion 🔖Sqrt

// #region 🔖Min
/// ⬇️ Minimum of two numbers.
pub struct Min;

impl Function for Min {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a.min(b)))))
    }
}
// #endregion 🔖Min

// #region 🔖Max
/// ⬆️ Maximum of two numbers.
pub struct Max;

impl Function for Max {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_number(input, "a").or_else(|_| read_number(input, "number"))?;
        let b = read_number(input, "b")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(a.max(b)))))
    }
}
// #endregion 🔖Max

// #region 🔖Floor
/// ⬇️ Floor of a number.
pub struct Floor;

impl Function for Floor {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.floor()))))
    }
}
// #endregion 🔖Floor

// #region 🔖Ceil
/// ⬆️ Ceiling of a number.
pub struct Ceil;

impl Function for Ceil {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.ceil()))))
    }
}
// #endregion 🔖Ceil

// #region 🔖Round
/// ⭕ Rounds a number.
pub struct Round;

impl Function for Round {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.round()))))
    }
}
// #endregion 🔖Round

// #region 🔖Sin
/// 〰️ Sine in radians.
pub struct Sin;

impl Function for Sin {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.sin()))))
    }
}
// #endregion 🔖Sin

// #region 🔖Cos
/// 〰️ Cosine in radians.
pub struct Cos;

impl Function for Cos {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.cos()))))
    }
}
// #endregion 🔖Cos

// #region 🔖Tan
/// 〰️ Tangent in radians.
pub struct Tan;

impl Function for Tan {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number").or_else(|_| read_number(input, "a"))?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n.tan()))))
    }
}
// #endregion 🔖Tan

// #region 🔖Remap
/// 🗺️ Remaps a value from one range to another.
pub struct Remap;

impl Function for Remap {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let value = read_number(input, "value").or_else(|_| read_number(input, "number"))?;
        let from_min = read_number(input, "fromMin").or_else(|_| read_number(input, "a"))?;
        let from_max = read_number(input, "fromMax").or_else(|_| read_number(input, "b"))?;
        let to_min = read_optional_number(input, "toMin").unwrap_or(0.0);
        let to_max = read_optional_number(input, "toMax").unwrap_or(1.0);
        let span = from_max - from_min;
        if span.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("remap span is zero".into()));
        }
        let t = (value - from_min) / span;
        let mapped = to_min + (to_max - to_min) * t;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(mapped))))
    }
}
// #endregion 🔖Remap

// #region 🔖Random
/// 🎲 Seeded or entropy-backed random number in [min, max].
pub struct Random;

impl Function for Random {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let min = read_optional_number(input, "min").unwrap_or(0.0);
        let max = read_optional_number(input, "max").unwrap_or(1.0);
        let seed = read_optional_number(input, "seed").map(f64::to_bits);
        let unit = next_random_unit(seed);
        let value = min + (max - min) * unit;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(value))))
    }
}
// #endregion 🔖Random

// #region 🔖PassThrough
/// ➡️ Forwards the number input unchanged.
pub struct PassThrough;

impl Function for PassThrough {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let n = read_number(input, "number")?;
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(n))))
    }
}
// #endregion 🔖PassThrough

// #region 🔖Sum
/// ∑ Sums all numbers in a list dictionary.
pub struct Sum;

impl Function for Sum {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_list(input, "list")?;
        let mut total = 0.0;
        for index in list_indices(list) {
            let value = list
                .get(&index.to_string())
                .and_then(|v| v.as_atom())
                .and_then(|a| a.as_f64())
                .ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
            total += value;
        }
        Ok(Dictionary::new().insert("number", Value::Atom(Atom::Decimal(total))))
    }
}
// #endregion 🔖Sum

thread_local! {
    static ENTROPY_SEED: Cell<u64> = const { Cell::new(0) };
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
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0)
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

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_atom())
        .and_then(|a| a.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_optional_number(input: &Dictionary, key: &str) -> Option<f64> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64())
}

fn read_list<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input
        .get(key)
        .and_then(|v| v.as_dictionary())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn register_kind(
    registry: &mut Registry,
    id: &str,
    name: &str,
    abbreviation: &str,
    icon: &str,
    summary: &str,
    inputs: Vec<InputSpec>,
    function: Box<dyn Function>,
) {
    registry.register(
        NeuronKindInfo {
            id: id.into(),
            module: "math".into(),
            name: name.into(),
            abbreviation: abbreviation.into(),
            icon: icon.into(),
            summary: summary.into(),
            inputs,
            outputs: vec!["number".into()],
            ..Default::default()
        },
        function,
    );
}

fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}

/// 📦 Registers all math neuron kinds on the registry.
pub fn register(registry: &mut Registry) {
    register_kind(
        registry,
        "math.add",
        "Add",
        "Add",
        "emoji:➕",
        "Sums two numbers",
        vec![InputSpec::number("a"), InputSpec::number_default("b", 0.0)],
        Box::new(Add),
    );
    register_kind(
        registry,
        "math.subtract",
        "Subtract",
        "Sub",
        "emoji:➖",
        "Subtracts b from a",
        vec![InputSpec::number("a"), InputSpec::number_default("b", 0.0)],
        Box::new(Subtract),
    );
    register_kind(
        registry,
        "math.multiply",
        "Multiply",
        "Mul",
        "emoji:✖️",
        "Multiplies two numbers",
        vec![InputSpec::number_default("a", 0.0), InputSpec::number_default("b", 1.0)],
        Box::new(Multiply),
    );
    register_kind(
        registry,
        "math.divide",
        "Divide",
        "Div",
        "emoji:➗",
        "Divides a by b",
        vec![InputSpec::number_default("a", 0.0), InputSpec::number_default("b", 1.0)],
        Box::new(Divide),
    );
    register_kind(
        registry,
        "math.power",
        "Power",
        "Pow",
        "emoji:⚡",
        "Raises a to the power of b",
        vec![InputSpec::number_default("a", 0.0), InputSpec::number_default("b", 1.0)],
        Box::new(Power),
    );
    register_kind(
        registry,
        "math.modulo",
        "Modulo",
        "Mod",
        "emoji:🧮",
        "Remainder of a divided by b",
        vec![InputSpec::number_default("a", 0.0), InputSpec::number_default("b", 1.0)],
        Box::new(Modulo),
    );
    register_kind(
        registry,
        "math.negate",
        "Negate",
        "Neg",
        "emoji:↔️",
        "Negates a number",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Negate),
    );
    register_kind(
        registry,
        "math.abs",
        "Abs",
        "Abs",
        "emoji:📏",
        "Absolute value",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Abs),
    );
    register_kind(
        registry,
        "math.sqrt",
        "Sqrt",
        "Sqrt",
        "emoji:√",
        "Square root",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Sqrt),
    );
    register_kind(
        registry,
        "math.min",
        "Min",
        "Min",
        "emoji:⬇️",
        "Minimum of two numbers",
        vec![InputSpec::number_default("a", 0.0), InputSpec::number_default("b", 0.0)],
        Box::new(Min),
    );
    register_kind(
        registry,
        "math.max",
        "Max",
        "Max",
        "emoji:⬆️",
        "Maximum of two numbers",
        vec![InputSpec::number_default("a", 0.0), InputSpec::number_default("b", 0.0)],
        Box::new(Max),
    );
    register_kind(
        registry,
        "math.floor",
        "Floor",
        "Flr",
        "emoji:⬇️",
        "Floor of a number",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Floor),
    );
    register_kind(
        registry,
        "math.ceil",
        "Ceil",
        "Ceil",
        "emoji:⬆️",
        "Ceiling of a number",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Ceil),
    );
    register_kind(
        registry,
        "math.round",
        "Round",
        "Rnd",
        "emoji:⭕",
        "Rounds a number",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Round),
    );
    register_kind(
        registry,
        "math.sin",
        "Sin",
        "Sin",
        "emoji:〰️",
        "Sine in radians",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Sin),
    );
    register_kind(
        registry,
        "math.cos",
        "Cos",
        "Cos",
        "emoji:〰️",
        "Cosine in radians",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Cos),
    );
    register_kind(
        registry,
        "math.tan",
        "Tan",
        "Tan",
        "emoji:〰️",
        "Tangent in radians",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(Tan),
    );
    register_kind(
        registry,
        "math.remap",
        "Remap",
        "Map",
        "emoji:🗺️",
        "Remaps a value from one range to another",
        vec![
            InputSpec::number_default("value", 0.0),
            InputSpec::number_default("fromMin", 0.0),
            InputSpec::number_default("fromMax", 1.0),
            InputSpec::number_default("toMin", 0.0),
            InputSpec::number_default("toMax", 1.0),
        ],
        Box::new(Remap),
    );
    register_kind(
        registry,
        "math.random",
        "Random",
        "Rnd",
        "emoji:🎲",
        "Random number in range with optional seed",
        vec![
            InputSpec::number_default("seed", 0.0),
            InputSpec::number_default("min", 0.0),
            InputSpec::number_default("max", 1.0),
        ],
        Box::new(Random),
    );
    register_kind(
        registry,
        "math.passThrough",
        "PassThrough",
        "Pass",
        "emoji:➡️",
        "Forwards a number",
        vec![InputSpec::number_default("number", 0.0)],
        Box::new(PassThrough),
    );
    register_kind(
        registry,
        "math.sum",
        "Sum",
        "Sum",
        "emoji:🔢",
        "Sums numbers in a list dictionary",
        vec![InputSpec::list("list")],
        Box::new(Sum),
    );
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommandV1, FlowModuleSettingV1};

    #[test]
    fn add_sums_inputs() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(3.0))).insert("b", Value::Atom(Atom::Decimal(1.1)));
        let out = reg.get("math.add").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.1));
    }

    #[test]
    fn manifest_lists_math_kinds() {
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
        assert!(json.contains("math.add"));
        assert!(json.contains("math.random"));
    }

    #[test]
    fn evaluate_json_adds_numbers() {
        let reg = module_registry();
        let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(2.0))).insert("b", Value::Atom(Atom::Decimal(1.0)));
        let out_json = evaluate_json(&reg, "math.add", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn sum_totals_list_numbers() {
        let mut reg = Registry::new();
        register(&mut reg);
        let list = Dictionary::new()
            .insert("0", Value::Atom(Atom::Decimal(1.0)))
            .insert("1", Value::Atom(Atom::Decimal(2.5)))
            .insert("2", Value::Atom(Atom::Decimal(3.0)));
        let input = Dictionary::new().insert("list", Value::Dictionary(list));
        let out = reg.get("math.sum").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.5));
    }

    #[test]
    fn random_is_deterministic_with_seed() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("seed", Value::Atom(Atom::Decimal(42.0)))
            .insert("min", Value::Atom(Atom::Decimal(0.0)))
            .insert("max", Value::Atom(Atom::Decimal(1.0)));
        let first = reg.get("math.random").unwrap().evaluate(&input).unwrap();
        let second = reg.get("math.random").unwrap().evaluate(&input).unwrap();
        assert_eq!(
            first.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()),
            second.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64())
        );
    }

    #[test]
    fn remap_maps_range() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("value", Value::Atom(Atom::Decimal(5.0)))
            .insert("fromMin", Value::Atom(Atom::Decimal(0.0)))
            .insert("fromMax", Value::Atom(Atom::Decimal(10.0)))
            .insert("toMin", Value::Atom(Atom::Decimal(0.0)))
            .insert("toMax", Value::Atom(Atom::Decimal(100.0)));
        let out = reg.get("math.remap").unwrap().evaluate(&input).unwrap();
        assert_eq!(out.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(50.0));
    }

    #[test]
    fn divide_rejects_zero() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new()
            .insert("a", Value::Atom(Atom::Decimal(1.0)))
            .insert("b", Value::Atom(Atom::Decimal(0.0)));
        assert!(reg.get("math.divide").unwrap().evaluate(&input).is_err());
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

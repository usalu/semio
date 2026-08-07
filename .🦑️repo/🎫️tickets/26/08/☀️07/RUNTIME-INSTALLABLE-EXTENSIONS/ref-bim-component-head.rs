//! 🏗️ Flow bim module: semantic building information modeling operators.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType, VariadicSpec};

// #region 🔖️Schemas
fn material_schema() -> Schema {
    Schema {
        id: "material".into(),
        module: "bim".into(),
        name: "Material".into(),
        icon: "emoji:🧱️".into(),
        summary: "Building material with thermal and structural properties".into(),
        fields: vec![FieldSpec::new("name", ValueType::Text), FieldSpec::decimal_default("density", 2400.0), FieldSpec::decimal_default("conductivity", 1.4), FieldSpec::decimal_default("strength", 30.0)],
    }
}

fn space_schema() -> Schema {
    Schema {
        id: "space".into(),
        module: "bim".into(),
        name: "Space".into(),
        icon: "emoji:🏠️".into(),
        summary: "Occupiable space with area and height".into(),
        fields: vec![FieldSpec::new("name", ValueType::Text), FieldSpec::decimal_default("area", 20.0), FieldSpec::decimal_default("height", 2.8)],
    }
}

fn wall_schema() -> Schema {
    Schema {
        id: "wall".into(),
        module: "bim".into(),
        name: "Wall".into(),
        icon: "emoji:🧱️".into(),
        summary: "Structural or partition wall".into(),
        fields: vec![FieldSpec::decimal_default("length", 4.0), FieldSpec::decimal_default("height", 2.8), FieldSpec::decimal_default("thickness", 0.2)],
    }
}

fn slab_schema() -> Schema {
    Schema {
        id: "slab".into(),
        module: "bim".into(),
        name: "Slab".into(),
        icon: "emoji:⬜️".into(),
        summary: "Horizontal slab element".into(),
        fields: vec![FieldSpec::decimal_default("width", 10.0), FieldSpec::decimal_default("depth", 8.0), FieldSpec::decimal_default("thickness", 0.25)],
    }
}

fn column_schema() -> Schema {
    Schema {
        id: "column".into(),
        module: "bim".into(),
        name: "Column".into(),
        icon: "emoji:🏛️".into(),
        summary: "Vertical structural column".into(),
        fields: vec![FieldSpec::decimal_default("width", 0.4), FieldSpec::decimal_default("depth", 0.4), FieldSpec::decimal_default("height", 3.0)],
    }
}

fn window_schema() -> Schema {
    Schema {
        id: "window".into(),
        module: "bim".into(),
        name: "Window".into(),
        icon: "emoji:🪟️".into(),
        summary: "Glazed opening".into(),
        fields: vec![FieldSpec::decimal_default("width", 1.2), FieldSpec::decimal_default("height", 1.4), FieldSpec::decimal_default("sill", 0.9)],
    }
}

fn story_schema() -> Schema {
    Schema {
        id: "story".into(),
        module: "bim".into(),
        name: "Story".into(),
        icon: "emoji:🏢️".into(),
        summary: "Building story with elements and spaces".into(),
        fields: vec![
            FieldSpec::decimal_default("elevation", 0.0),
            FieldSpec::decimal_default("height", 3.0),
            FieldSpec::new("elements", ValueType::List(Box::new(ValueType::Any))),
            FieldSpec::new("spaces", ValueType::List(Box::new(ValueType::Schema("space".into())))),
        ],
    }
}

fn building_schema() -> Schema {
    Schema {
        id: "building".into(),
        module: "bim".into(),
        name: "Building".into(),
        icon: "emoji:🏗️".into(),
        summary: "Assembled building model".into(),
        fields: vec![FieldSpec::new("name", ValueType::Text), FieldSpec::new("stories", ValueType::List(Box::new(ValueType::Schema("story".into()))))],
    }
}
// #endregion 🔖️Schemas

// #region 🔖️Helpers
fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn text_dictionary(value: impl Into<String>) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value.into())))
}

fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

fn text_channel(id: &str, operator_id: &str, default: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id]).with_default(Value::Dictionary(text_dictionary(default)))
}

fn out_material() -> ChannelSpec {
    ChannelSpec::named("M", "Mat", "material", "Material")
}

fn out_space() -> ChannelSpec {
    ChannelSpec::named("S", "Spc", "space", "Space")
}

fn out_wall() -> ChannelSpec {
    ChannelSpec::named("W", "Wal", "wall", "Wall")
}

fn out_slab() -> ChannelSpec {
    ChannelSpec::named("S", "Slb", "slab", "Slab")
}

fn out_column() -> ChannelSpec {
    ChannelSpec::named("C", "Col", "column", "Column")
}

fn out_window() -> ChannelSpec {
    ChannelSpec::named("W", "Win", "window", "Window")
}

fn out_story() -> ChannelSpec {
    ChannelSpec::named("S", "Sty", "story", "Story")
}

fn out_building() -> ChannelSpec {
    ChannelSpec::named("B", "Bld", "building", "Building")
}

fn out_floor_area() -> ChannelSpec {
    ChannelSpec::named("A", "FlA", "floorArea", "FloorArea")
}

fn out_gross_volume() -> ChannelSpec {
    ChannelSpec::named("V", "GrV", "grossVolume", "GrossVolume")
}

/// 🏷️ Descriptive metadata for a bim operator, grouped to keep `operator_info` under clippy's arg-count limit.
/// `Copy` (all fields are borrowed `&str`) so `operator_info` can take it by value without tripping
/// `clippy::needless_pass_by_value` — found via a real `cargo clippy --all-targets -- -D warnings` run
/// while de-sandwiching this crate's package layout (pre-existing, unrelated to the relocation itself;
/// this lint combination had apparently never run against this crate before).
#[derive(Clone, Copy)]
struct OperatorMeta<'a> {
    id: &'a str,
    name: &'a str,
    abbreviation: &'a str,
    icon: &'a str,
    summary: &'a str,
}

fn operator_info(meta: OperatorMeta<'_>, inputs: Vec<ChannelSpec>, output: ChannelSpec, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: meta.id.into(),
        extension: "bim".into(),
        name: meta.name.into(),
        abbreviation: meta.abbreviation.into(),
        icon: meta.icon.into(),
        summary: meta.summary.into(),
        inputs,
        outputs: vec![output],
        group: group.iter().map(|entry| (*entry).to_string()).collect(),
        ..Default::default()
    }
}

fn register_element(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>, schema_id: &str) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operation }], &[schema_id, "element"]);
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_field_number(dict: &Dictionary, key: &str) -> Option<f64> {
    dict.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64())
}

#[cfg(test)]
fn read_field_text(dict: &Dictionary, key: &str) -> Option<String> {
    dict.get(key).and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string)
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn list_from_variadic(items: Option<&Dictionary>) -> Dictionary {
    let mut out = Dictionary::with_schema("list");
    let Some(items) = items else {

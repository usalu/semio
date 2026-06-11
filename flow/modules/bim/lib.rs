//! 🏗️ Flow bim module: semantic building information modeling operators.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType, VariadicSpec};

// #region 🔖Schemas
fn material_schema() -> Schema {
    Schema {
        id: "material".into(),
        module: "bim".into(),
        name: "Material".into(),
        icon: "emoji:🧱".into(),
        summary: "Building material with thermal and structural properties".into(),
        fields: vec![
            FieldSpec::new("name", ValueType::Text),
            FieldSpec::decimal_default("density", 2400.0),
            FieldSpec::decimal_default("conductivity", 1.4),
            FieldSpec::decimal_default("strength", 30.0),
        ],
    }
}

fn space_schema() -> Schema {
    Schema {
        id: "space".into(),
        module: "bim".into(),
        name: "Space".into(),
        icon: "emoji:🏠".into(),
        summary: "Occupiable space with area and height".into(),
        fields: vec![
            FieldSpec::new("name", ValueType::Text),
            FieldSpec::decimal_default("area", 20.0),
            FieldSpec::decimal_default("height", 2.8),
        ],
    }
}

fn wall_schema() -> Schema {
    Schema {
        id: "wall".into(),
        module: "bim".into(),
        name: "Wall".into(),
        icon: "emoji:🧱".into(),
        summary: "Structural or partition wall".into(),
        fields: vec![
            FieldSpec::decimal_default("length", 4.0),
            FieldSpec::decimal_default("height", 2.8),
            FieldSpec::decimal_default("thickness", 0.2),
        ],
    }
}

fn slab_schema() -> Schema {
    Schema {
        id: "slab".into(),
        module: "bim".into(),
        name: "Slab".into(),
        icon: "emoji:⬜".into(),
        summary: "Horizontal slab element".into(),
        fields: vec![
            FieldSpec::decimal_default("width", 10.0),
            FieldSpec::decimal_default("depth", 8.0),
            FieldSpec::decimal_default("thickness", 0.25),
        ],
    }
}

fn column_schema() -> Schema {
    Schema {
        id: "column".into(),
        module: "bim".into(),
        name: "Column".into(),
        icon: "emoji:🏛️".into(),
        summary: "Vertical structural column".into(),
        fields: vec![
            FieldSpec::decimal_default("width", 0.4),
            FieldSpec::decimal_default("depth", 0.4),
            FieldSpec::decimal_default("height", 3.0),
        ],
    }
}

fn window_schema() -> Schema {
    Schema {
        id: "window".into(),
        module: "bim".into(),
        name: "Window".into(),
        icon: "emoji:🪟".into(),
        summary: "Glazed opening".into(),
        fields: vec![
            FieldSpec::decimal_default("width", 1.2),
            FieldSpec::decimal_default("height", 1.4),
            FieldSpec::decimal_default("sill", 0.9),
        ],
    }
}

fn story_schema() -> Schema {
    Schema {
        id: "story".into(),
        module: "bim".into(),
        name: "Story".into(),
        icon: "emoji:🏢".into(),
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
        fields: vec![
            FieldSpec::new("name", ValueType::Text),
            FieldSpec::new("stories", ValueType::List(Box::new(ValueType::Schema("story".into())))),
        ],
    }
}
// #endregion 🔖Schemas

// #region 🔖Helpers
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

fn operator_info(
    id: &str,
    name: &str,
    abbreviation: &str,
    icon: &str,
    summary: &str,
    inputs: Vec<ChannelSpec>,
    output: ChannelSpec,
    group: &[&str],
) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        module: "bim".into(),
        name: name.into(),
        abbreviation: abbreviation.into(),
        icon: icon.into(),
        summary: summary.into(),
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
    dict.get("value")
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_f64())
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value")
        .and_then(|value| value.as_atom())
        .and_then(|atom| atom.as_str())
        .map(str::to_string)
        .ok_or_else(|| EvalError::MissingInput(key.into()))
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
        return out;
    };
    let mut indices: Vec<String> = items.keys().cloned().collect();
    indices.sort();
    for (next, key) in indices.into_iter().enumerate() {
        if let Some(value) = items.get(&key) {
            out = out.insert(next.to_string(), value.clone());
        }
    }
    out
}

fn collect_elements_and_spaces(items: Option<&Dictionary>) -> (Dictionary, Dictionary) {
    let mut elements = Dictionary::with_schema("list");
    let mut spaces = Dictionary::with_schema("list");
    let Some(items) = items else {
        return (elements, spaces);
    };
    let mut element_index = 0usize;
    let mut space_index = 0usize;
    let mut keys: Vec<String> = items.keys().cloned().collect();
    keys.sort();
    for key in keys {
        let Some(dict) = items.get(&key).and_then(|value| value.as_dictionary()) else {
            continue;
        };
        if dict.schema() == Some("space") {
            spaces = spaces.insert(space_index.to_string(), Value::Dictionary(dict.clone()));
            space_index += 1;
        } else {
            elements = elements.insert(element_index.to_string(), Value::Dictionary(dict.clone()));
            element_index += 1;
        }
    }
    (elements, spaces)
}

fn story_floor_area(story: &Dictionary) -> f64 {
    if let Some(slab) = story.get("slab").and_then(|value| value.as_dictionary()) {
        return read_field_number(slab, "width").unwrap_or(0.0) * read_field_number(slab, "depth").unwrap_or(0.0);
    }
    let mut total = 0.0;
    if let Some(spaces) = story.get("spaces").and_then(|value| value.as_dictionary()) {
        for index in list_indices(spaces) {
            if let Some(space) = spaces.get(&index.to_string()).and_then(|value| value.as_dictionary()) {
                total += read_field_number(space, "area").unwrap_or(0.0);
            }
        }
    }
    total
}

fn story_gross_volume(story: &Dictionary) -> f64 {
    story_floor_area(story) * read_field_number(story, "height").unwrap_or(0.0)
}

fn building_stories(building: &Dictionary) -> Option<&Dictionary> {
    building.get("stories").and_then(|value| value.as_dictionary())
}
// #endregion 🔖Helpers

// #region 🔖Elements
struct MaterialElement;

impl Operation for MaterialElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "material",
            Dictionary::with_schema("material")
                .insert("name", Value::Atom(Atom::String(read_channel_text(input, "name").unwrap_or_else(|_| "Concrete".into()))))
                .insert("density", Value::Atom(Atom::Decimal(read_channel_number(input, "density")?)))
                .insert("conductivity", Value::Atom(Atom::Decimal(read_channel_number(input, "conductivity")?)))
                .insert("strength", Value::Atom(Atom::Decimal(read_channel_number(input, "strength")?))),
        ))
    }
}

struct SpaceElement;

impl Operation for SpaceElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "space",
            Dictionary::with_schema("space")
                .insert("name", Value::Atom(Atom::String(read_channel_text(input, "name").unwrap_or_else(|_| "Space".into()))))
                .insert("area", Value::Atom(Atom::Decimal(read_channel_number(input, "area")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?))),
        ))
    }
}

struct WallElement;

impl Operation for WallElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "wall",
            Dictionary::with_schema("wall")
                .insert("length", Value::Atom(Atom::Decimal(read_channel_number(input, "length")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?)))
                .insert("thickness", Value::Atom(Atom::Decimal(read_channel_number(input, "thickness")?))),
        ))
    }
}

struct SlabElement;

impl Operation for SlabElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "slab",
            Dictionary::with_schema("slab")
                .insert("width", Value::Atom(Atom::Decimal(read_channel_number(input, "width")?)))
                .insert("depth", Value::Atom(Atom::Decimal(read_channel_number(input, "depth")?)))
                .insert("thickness", Value::Atom(Atom::Decimal(read_channel_number(input, "thickness")?))),
        ))
    }
}

struct ColumnElement;

impl Operation for ColumnElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "column",
            Dictionary::with_schema("column")
                .insert("width", Value::Atom(Atom::Decimal(read_channel_number(input, "width")?)))
                .insert("depth", Value::Atom(Atom::Decimal(read_channel_number(input, "depth")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?))),
        ))
    }
}

struct WindowElement;

impl Operation for WindowElement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output(
            "window",
            Dictionary::with_schema("window")
                .insert("width", Value::Atom(Atom::Decimal(read_channel_number(input, "width")?)))
                .insert("height", Value::Atom(Atom::Decimal(read_channel_number(input, "height")?)))
                .insert("sill", Value::Atom(Atom::Decimal(read_channel_number(input, "sill")?))),
        ))
    }
}
// #endregion 🔖Elements

// #region 🔖Assembly
struct AssembleStory;

impl Operation for AssembleStory {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let elevation = read_channel_number(input, "elevation").unwrap_or(0.0);
        let height = read_channel_number(input, "height")?;
        let (elements, spaces) = collect_elements_and_spaces(input.get("elements").and_then(|value| value.as_dictionary()));
        let mut story = Dictionary::with_schema("story")
            .insert("elevation", Value::Atom(Atom::Decimal(elevation)))
            .insert("height", Value::Atom(Atom::Decimal(height)))
            .insert("elements", Value::Dictionary(elements))
            .insert("spaces", Value::Dictionary(spaces));
        if let Some(slab) = input.get("slab").and_then(|value| value.as_dictionary()) {
            story = story.insert("slab", Value::Dictionary(slab.clone()));
        }
        Ok(channel_output("story", story))
    }
}

struct AssembleBuilding;

impl Operation for AssembleBuilding {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let name = read_channel_text(input, "name").unwrap_or_else(|_| "Building".into());
        let stories = list_from_variadic(input.get("stories").and_then(|value| value.as_dictionary()));
        Ok(channel_output(
            "building",
            Dictionary::with_schema("building")
                .insert("name", Value::Atom(Atom::String(name)))
                .insert("stories", Value::Dictionary(stories)),
        ))
    }
}
// #endregion 🔖Assembly

// #region 🔖Measure
struct FloorArea;

impl Operation for FloorArea {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let building = read_building(input, "building")?;
        let mut total = 0.0;
        if let Some(stories) = building_stories(building) {
            for index in list_indices(stories) {
                if let Some(story) = stories.get(&index.to_string()).and_then(|value| value.as_dictionary()) {
                    total += story_floor_area(story);
                }
            }
        }
        Ok(channel_output("floorArea", number_dictionary(total)))
    }
}

struct GrossVolume;

impl Operation for GrossVolume {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let building = read_building(input, "building")?;
        let mut total = 0.0;
        if let Some(stories) = building_stories(building) {
            for index in list_indices(stories) {
                if let Some(story) = stories.get(&index.to_string()).and_then(|value| value.as_dictionary()) {
                    total += story_gross_volume(story);
                }
            }
        }
        Ok(channel_output("grossVolume", number_dictionary(total)))
    }
}

fn read_building<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input
        .get(key)
        .and_then(|value| value.as_dictionary())
        .filter(|dict| dict.schema() == Some("building"))
        .ok_or_else(|| EvalError::MissingInput(key.into()))
}
// #endregion 🔖Measure

/// 📦 Registers bim schemas and operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(material_schema());
    registry.register_schema(space_schema());
    registry.register_schema(wall_schema());
    registry.register_schema(slab_schema());
    registry.register_schema(column_schema());
    registry.register_schema(window_schema());
    registry.register_schema(story_schema());
    registry.register_schema(building_schema());

    register_element(
        registry,
        operator_info(
            "bim.element.material",
            "Material",
            "Mat",
            "emoji:🧱",
            "Defines a building material",
            vec![
                text_channel("name", "bim.element.material", "Concrete"),
                number_channel("density", "bim.element.material", 2400.0),
                number_channel("conductivity", "bim.element.material", 1.4),
                number_channel("strength", "bim.element.material", 30.0),
            ],
            out_material(),
            &["Elements"],
        ),
        Box::new(MaterialElement),
        "material",
    );
    register_element(
        registry,
        operator_info(
            "bim.element.space",
            "Space",
            "Space",
            "emoji:🏠",
            "Defines an occupiable space",
            vec![
                text_channel("name", "bim.element.space", "Space"),
                number_channel("area", "bim.element.space", 20.0),
                number_channel("height", "bim.element.space", 2.8),
            ],
            out_space(),
            &["Elements"],
        ),
        Box::new(SpaceElement),
        "space",
    );
    register_element(
        registry,
        operator_info(
            "bim.element.wall",
            "Wall",
            "Wall",
            "emoji:🧱",
            "Defines a wall element",
            vec![
                number_channel("length", "bim.element.wall", 4.0),
                number_channel("height", "bim.element.wall", 2.8),
                number_channel("thickness", "bim.element.wall", 0.2),
            ],
            out_wall(),
            &["Elements"],
        ),
        Box::new(WallElement),
        "wall",
    );
    register_element(
        registry,
        operator_info(
            "bim.element.slab",
            "Slab",
            "Slab",
            "emoji:⬜",
            "Defines a slab element",
            vec![
                number_channel("width", "bim.element.slab", 10.0),
                number_channel("depth", "bim.element.slab", 8.0),
                number_channel("thickness", "bim.element.slab", 0.25),
            ],
            out_slab(),
            &["Elements"],
        ),
        Box::new(SlabElement),
        "slab",
    );
    register_element(
        registry,
        operator_info(
            "bim.element.column",
            "Column",
            "Col",
            "emoji:🏛️",
            "Defines a column element",
            vec![
                number_channel("width", "bim.element.column", 0.4),
                number_channel("depth", "bim.element.column", 0.4),
                number_channel("height", "bim.element.column", 3.0),
            ],
            out_column(),
            &["Elements"],
        ),
        Box::new(ColumnElement),
        "column",
    );
    register_element(
        registry,
        operator_info(
            "bim.element.window",
            "Window",
            "Win",
            "emoji:🪟",
            "Defines a window element",
            vec![
                number_channel("width", "bim.element.window", 1.2),
                number_channel("height", "bim.element.window", 1.4),
                number_channel("sill", "bim.element.window", 0.9),
            ],
            out_window(),
            &["Elements"],
        ),
        Box::new(WindowElement),
        "window",
    );

    registry.register_operator(
        OperatorInfo {
            variadic_input: Some(VariadicSpec { slot_key: "elements".into(), min: 0, max: None }),
            ..operator_info(
                "bim.assemble.story",
                "Assemble Story",
                "Story",
                "emoji:🏢",
                "Assembles a story from elements and optional slab",
                vec![
                    number_channel("elevation", "bim.assemble.story", 0.0),
                    number_channel("height", "bim.assemble.story", 3.0),
                    ChannelSpec::requires("slab", &["bim.element.slab"]),
                ],
                out_story(),
                &["Assembly"],
            )
        },
        vec![OperatorImpl { schemas: vec![], operation: Box::new(AssembleStory) }],
        &["story"],
    );
    registry.register_operator(
        OperatorInfo {
            variadic_input: Some(VariadicSpec { slot_key: "stories".into(), min: 1, max: None }),
            ..operator_info(
                "bim.assemble.building",
                "Assemble Building",
                "Building",
                "emoji:🏗️",
                "Assembles a building from stories",
                vec![text_channel("name", "bim.assemble.building", "Building")],
                out_building(),
                &["Assembly"],
            )
        },
        vec![OperatorImpl { schemas: vec![], operation: Box::new(AssembleBuilding) }],
        &["building"],
    );

    registry.register_operator(
        operator_info(
            "bim.measure.floorArea",
            "Floor Area",
            "Area",
            "emoji:📐",
            "Total floor area across all stories",
            vec![ChannelSpec::requires("building", &["bim.assemble.building"])],
            out_floor_area(),
            &["Measure"],
        ),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(FloorArea) }],
        &["number"],
    );
    registry.register_operator(
        operator_info(
            "bim.measure.grossVolume",
            "Gross Volume",
            "Vol",
            "emoji:📦",
            "Gross building volume across all stories",
            vec![ChannelSpec::requires("building", &["bim.assemble.building"])],
            out_gross_volume(),
            &["Measure"],
        ),
        vec![OperatorImpl { schemas: vec![], operation: Box::new(GrossVolume) }],
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

    fn channel_payload(out: &Dictionary, channel: &str) -> Dictionary {
        out.get(channel).and_then(|v| v.as_dictionary()).cloned().expect("channel payload")
    }

    #[test]
    fn wall_element_emits_wall_schema() {
        let mut reg = Registry::new();
        register(&mut reg);
        let out = reg
            .dispatch(
                "bim.element.wall",
                &Dictionary::new()
                    .insert("length", Value::Dictionary(number_dictionary(5.0)))
                    .insert("height", Value::Dictionary(number_dictionary(3.0)))
                    .insert("thickness", Value::Dictionary(number_dictionary(0.2))),
            )
            .unwrap();
        let wall = channel_payload(&out, "wall");
        assert_eq!(wall.schema(), Some("wall"));
        assert_eq!(read_field_number(&wall, "length"), Some(5.0));
    }

    #[test]
    fn assemble_story_splits_spaces() {
        let mut reg = Registry::new();
        register(&mut reg);
        let wall = channel_payload(
            &reg.dispatch(
                "bim.element.wall",
                &Dictionary::new()
                    .insert("length", Value::Dictionary(number_dictionary(4.0)))
                    .insert("height", Value::Dictionary(number_dictionary(2.8)))
                    .insert("thickness", Value::Dictionary(number_dictionary(0.2))),
            )
            .unwrap(),
            "wall",
        );
        let space = channel_payload(
            &reg.dispatch(
                "bim.element.space",
                &Dictionary::new()
                    .insert("name", Value::Dictionary(text_dictionary("Lobby")))
                    .insert("area", Value::Dictionary(number_dictionary(40.0)))
                    .insert("height", Value::Dictionary(number_dictionary(3.0))),
            )
            .unwrap(),
            "space",
        );
        let slab = channel_payload(
            &reg.dispatch(
                "bim.element.slab",
                &Dictionary::new()
                    .insert("width", Value::Dictionary(number_dictionary(10.0)))
                    .insert("depth", Value::Dictionary(number_dictionary(8.0)))
                    .insert("thickness", Value::Dictionary(number_dictionary(0.25))),
            )
            .unwrap(),
            "slab",
        );
        let story = channel_payload(
            &reg.dispatch(
                "bim.assemble.story",
                &Dictionary::new()
                    .insert("elevation", Value::Dictionary(number_dictionary(0.0)))
                    .insert("height", Value::Dictionary(number_dictionary(3.0)))
                    .insert("slab", Value::Dictionary(slab))
                    .insert(
                        "elements",
                        Value::Dictionary(
                            Dictionary::new()
                                .insert("0", Value::Dictionary(wall))
                                .insert("1", Value::Dictionary(space)),
                        ),
                    ),
            )
            .unwrap(),
            "story",
        );
        assert_eq!(story.schema(), Some("story"));
        let elements = story.get("elements").and_then(|value| value.as_dictionary()).unwrap();
        let spaces = story.get("spaces").and_then(|value| value.as_dictionary()).unwrap();
        assert_eq!(list_indices(elements).len(), 1);
        assert_eq!(list_indices(spaces).len(), 1);
        assert_eq!(read_field_text(spaces.get("0").and_then(|value| value.as_dictionary()).unwrap(), "name"), Some("Lobby".into()));
    }

    #[test]
    fn assemble_building_and_measure_floor_area() {
        let mut reg = Registry::new();
        register(&mut reg);
        let slab = channel_payload(
            &reg.dispatch(
                "bim.element.slab",
                &Dictionary::new()
                    .insert("width", Value::Dictionary(number_dictionary(10.0)))
                    .insert("depth", Value::Dictionary(number_dictionary(8.0)))
                    .insert("thickness", Value::Dictionary(number_dictionary(0.25))),
            )
            .unwrap(),
            "slab",
        );
        let story = channel_payload(
            &reg.dispatch(
                "bim.assemble.story",
                &Dictionary::new()
                    .insert("height", Value::Dictionary(number_dictionary(3.0)))
                    .insert("slab", Value::Dictionary(slab))
                    .insert("elements", Value::Dictionary(Dictionary::new())),
            )
            .unwrap(),
            "story",
        );
        let building = channel_payload(
            &reg.dispatch(
                "bim.assemble.building",
                &Dictionary::new()
                    .insert("name", Value::Dictionary(text_dictionary("Tower")))
                    .insert("stories", Value::Dictionary(Dictionary::new().insert("0", Value::Dictionary(story)))),
            )
            .unwrap(),
            "building",
        );
        assert_eq!(building.schema(), Some("building"));
        let area = channel_payload(
            &reg.dispatch("bim.measure.floorArea", &Dictionary::new().insert("building", Value::Dictionary(building))).unwrap(),
            "floorArea",
        );
        assert_eq!(area.schema(), Some("number"));
        assert_eq!(read_field_number(&area, "value"), Some(80.0));
    }

    #[test]
    fn measure_gross_volume() {
        let mut reg = Registry::new();
        register(&mut reg);
        let slab = channel_payload(
            &reg.dispatch(
                "bim.element.slab",
                &Dictionary::new()
                    .insert("width", Value::Dictionary(number_dictionary(10.0)))
                    .insert("depth", Value::Dictionary(number_dictionary(10.0)))
                    .insert("thickness", Value::Dictionary(number_dictionary(0.25))),
            )
            .unwrap(),
            "slab",
        );
        let story = channel_payload(
            &reg.dispatch(
                "bim.assemble.story",
                &Dictionary::new()
                    .insert("height", Value::Dictionary(number_dictionary(3.0)))
                    .insert("slab", Value::Dictionary(slab))
                    .insert("elements", Value::Dictionary(Dictionary::new())),
            )
            .unwrap(),
            "story",
        );
        let building = channel_payload(
            &reg.dispatch(
                "bim.assemble.building",
                &Dictionary::new()
                    .insert("name", Value::Dictionary(text_dictionary("Block")))
                    .insert("stories", Value::Dictionary(Dictionary::new().insert("0", Value::Dictionary(story)))),
            )
            .unwrap(),
            "building",
        );
        let volume = channel_payload(
            &reg.dispatch("bim.measure.grossVolume", &Dictionary::new().insert("building", Value::Dictionary(building))).unwrap(),
            "grossVolume",
        );
        assert_eq!(read_field_number(&volume, "value"), Some(300.0));
    }

    #[test]
    fn manifest_lists_bim_operators() {
        let json = build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![]);
        assert!(json.contains("flow.module/v1"));
        assert!(json.contains("bim.element.wall"));
        assert!(json.contains("bim.assemble.building"));
        assert!(json.contains("bim.measure.floorArea"));
        assert!(json.contains("\"building\""));
    }

    #[test]
    fn evaluate_json_wall() {
        let reg = module_registry();
        let input = Dictionary::new()
            .insert("length", Value::Dictionary(number_dictionary(4.0)))
            .insert("height", Value::Dictionary(number_dictionary(2.8)))
            .insert("thickness", Value::Dictionary(number_dictionary(0.2)));
        let out_json = evaluate_json(&reg, "bim.element.wall", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(channel_payload(&out, "wall").schema(), Some("wall"));
    }
}
// #endregion 🔖Tests

// #region 🔖WasmExt
#[cfg(all(target_arch = "wasm32", feature = "standalone-wasm"))]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json("bim", "Bim", "0.1.0", &module_registry(), vec!["onStartup".into()], vec![], vec![], vec![])
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

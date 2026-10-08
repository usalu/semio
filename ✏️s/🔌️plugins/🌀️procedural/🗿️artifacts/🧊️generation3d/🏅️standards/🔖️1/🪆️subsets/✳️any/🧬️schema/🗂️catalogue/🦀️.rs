//! 🗂️ Generation3d geometry widget catalogue — the typed loader of the schema-first kind declarations in this folder.
//!
//! The category files beside this module are the single source for the palette, inspector controls, history labels,
//! accessible names and validation of every geometry widget; the meta-schema in `🔣️.json` describes their shape.

use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// 📚️ Every bundled category file with its slug, in the order of the folder listing.
pub const CATEGORY_SOURCES: [(&str, &str); 27] = [
    ("brep-primitive", include_str!("🔣️brep-primitive.json")),
    ("brep-curve", include_str!("🔣️brep-curve.json")),
    ("brep-surface", include_str!("🔣️brep-surface.json")),
    ("brep-solid", include_str!("🔣️brep-solid.json")),
    ("brep-boolean", include_str!("🔣️brep-boolean.json")),
    ("brep-feature", include_str!("🔣️brep-feature.json")),
    ("brep-transform", include_str!("🔣️brep-transform.json")),
    ("brep-intersect", include_str!("🔣️brep-intersect.json")),
    ("brep-evaluate", include_str!("🔣️brep-evaluate.json")),
    ("brep-topology", include_str!("🔣️brep-topology.json")),
    ("brep-interchange", include_str!("🔣️brep-interchange.json")),
    ("mesh-primitive", include_str!("🔣️mesh-primitive.json")),
    ("mesh-convert", include_str!("🔣️mesh-convert.json")),
    ("mesh-transform", include_str!("🔣️mesh-transform.json")),
    ("mesh-component", include_str!("🔣️mesh-component.json")),
    ("mesh-edit", include_str!("🔣️mesh-edit.json")),
    ("mesh-repair", include_str!("🔣️mesh-repair.json")),
    ("mesh-inspect", include_str!("🔣️mesh-inspect.json")),
    ("mesh-interchange", include_str!("🔣️mesh-interchange.json")),
    ("mesh-shading", include_str!("🔣️mesh-shading.json")),
    ("mesh-uv", include_str!("🔣️mesh-uv.json")),
    ("analysis-measure", include_str!("🔣️analysis-measure.json")),
    ("analysis-check", include_str!("🔣️analysis-check.json")),
    ("math-values", include_str!("🔣️math-values.json")),
    ("math-arithmetic", include_str!("🔣️math-arithmetic.json")),
    ("math-vector", include_str!("🔣️math-vector.json")),
    ("math-list", include_str!("🔣️math-list.json")),
];

fn is_false(value: &bool) -> bool {
    !*value
}

/// 🗣️ A user-facing string in every supported language, English first and German second.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Localized {
    pub en: String,
    pub de: String,
}

impl Localized {
    /// 🌐️ The text for a BCP 47 locale such as `de` or `de-CH`; an unsupported language reads as the first language, English.
    pub fn resolve(&self, locale: &str) -> &str {
        match locale.split(['-', '_']).next().unwrap_or_default().to_ascii_lowercase().as_str() {
            "de" => &self.de,
            _ => &self.en,
        }
    }
}

/// 🏷️ The fidelity of a kind: the kernel's operation quality for B-Rep kinds, the mesh fidelity for mesh kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum Quality {
    ExactAnalytic,
    ExactNumerical,
    Approximate,
    MeshDerivedBrep,
    PolygonMesh,
    TessellatedMesh,
}

/// 🔌️ The value type a port carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum PortType {
    Number,
    Integer,
    Angle,
    Length,
    Boolean,
    Text,
    Enum,
    Vector,
    Point,
    Plane,
    Shape,
    Shapes,
    Mesh,
    Selection,
    Any,
}

impl PortType {
    /// 🔢️ Whether the type is a single bounded or unbounded real or whole number.
    pub fn is_numeric(self) -> bool {
        matches!(self, Self::Number | Self::Integer | Self::Angle | Self::Length)
    }
}

/// 🧱️ The topological kind of a B-Rep shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum ShapeKind {
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Curve,
    Surface,
    Vertex,
    Compound,
}

/// 🎯️ The element type a selection port refers to; `Mode` follows the enum port named by `mode_from`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum SelectionComponent {
    Face,
    Edge,
    Vertex,
    Mode,
}

/// 🧲️ The sub-elements a selection port holds, taken from the shape or mesh on its source port.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Selection {
    pub component: SelectionComponent,
    pub source: String,
    pub multiple: bool,
    pub mode_from: Option<String>,
}

/// 🔘️ One choice of an enum port.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct EnumOption {
    pub value: String,
    pub label: Localized,
}

/// 🔌️ One input or output of a widget kind, with everything an inspector control needs.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Port {
    pub name: String,
    pub label: Localized,
    pub description: Localized,
    #[value(rename = "type")]
    pub port_type: PortType,
    pub shape_kinds: Option<Vec<ShapeKind>>,
    pub selection: Option<Selection>,
    #[value(default, skip_serializing_if = "is_false")]
    pub list: bool,
    pub min_items: Option<u32>,
    pub max_items: Option<u32>,
    pub default: Option<DslValue>,
    pub min: Option<f64>,
    #[value(default, skip_serializing_if = "is_false")]
    pub exclusive_min: bool,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub unit: Option<String>,
    pub options: Option<Vec<EnumOption>>,
    #[value(default, skip_serializing_if = "is_false")]
    pub optional: bool,
}

/// 👆️ The component a viewport pick delivers to a port.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum PickComponent {
    Shape,
    Mesh,
    Face,
    Edge,
    Vertex,
}

/// 👆️ A picked component that seeds a port when the kind is applied to the pick.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Pick {
    pub component: PickComponent,
    pub port: String,
}

/// 🧭️ The gumball motion that drives a port.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum GumballMotion {
    Translate,
    Rotate,
    Scale,
}

/// 📏️ The reference direction of a scalar translate motion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "kebab-case")]
pub enum Along {
    Normal,
}

/// 🧭️ A gumball motion mapped to the port it drives, with the ports that fix its axis and origin.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gumball {
    pub motion: GumballMotion,
    pub port: String,
    pub axis_port: Option<String>,
    pub origin_port: Option<String>,
    pub along: Option<Along>,
}

/// 🕹️ How a kind reacts to viewport picks and gumball motions.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Interaction {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub pick: Vec<Pick>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub gumball: Vec<Gumball>,
}

/// 🧩️ One geometry widget kind: its identity, texts, typed ports, fidelity and interaction hints.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Kind {
    pub id: String,
    pub category: String,
    pub emoji: String,
    pub label: Localized,
    pub description: Localized,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
    pub quality: Quality,
    pub interaction: Option<Interaction>,
    pub preview: bool,
}

impl Kind {
    /// 🔌️ The input port with this name.
    pub fn input(&self, name: &str) -> Option<&Port> {
        self.inputs.iter().find(|port| port.name == name)
    }

    /// 🔌️ The output port with this name.
    pub fn output(&self, name: &str) -> Option<&Port> {
        self.outputs.iter().find(|port| port.name == name)
    }

    /// 👆️ The pick hints, empty when the kind declares none.
    pub fn picks(&self) -> &[Pick] {
        self.interaction.as_ref().map_or(&[], |interaction| &interaction.pick)
    }

    /// 🧭️ The gumball hints, empty when the kind declares none.
    pub fn gumballs(&self) -> &[Gumball] {
        self.interaction.as_ref().map_or(&[], |interaction| &interaction.gumball)
    }
}

/// 🗂️ A palette category with its sort position and texts.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct Category {
    pub id: String,
    pub emoji: String,
    pub order: u32,
    pub label: Localized,
    pub description: Localized,
}

/// 📄️ One category file: the category and every kind it owns.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(deny_unknown_fields)]
pub struct CategoryFile {
    pub category: Category,
    pub kinds: Vec<Kind>,
}

/// 🚫️ Why a category file could not join the catalogue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatalogueError {
    pub source: String,
    pub message: String,
}

impl std::fmt::Display for CatalogueError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "geometry catalogue {}: {}", self.source, self.message)
    }
}

impl std::error::Error for CatalogueError {}

/// 📚️ Every geometry widget kind, ordered by category and indexed by id.
#[derive(Clone, Debug)]
pub struct Catalogue {
    categories: Vec<CategoryFile>,
    index: BTreeMap<String, (usize, usize)>,
}

impl Catalogue {
    /// 🏗️ Parses category sources and indexes their kinds; a duplicate kind id is refused.
    pub fn parse<'a>(sources: impl IntoIterator<Item = (&'a str, &'a str)>) -> Result<Self, CatalogueError> {
        let mut categories = Vec::new();
        for (slug, text) in sources {
            let file: CategoryFile = from_json_str(text, JsonMemberPolicy::Reject).map_err(|error| CatalogueError { source: slug.to_string(), message: error.to_string() })?;
            categories.push((slug, file));
        }
        categories.sort_by(|left, right| left.1.category.order.cmp(&right.1.category.order).then_with(|| left.1.category.id.cmp(&right.1.category.id)));
        let mut index = BTreeMap::new();
        for (category_index, (slug, file)) in categories.iter().enumerate() {
            for (kind_index, kind) in file.kinds.iter().enumerate() {
                if index.insert(kind.id.clone(), (category_index, kind_index)).is_some() {
                    return Err(CatalogueError { source: slug.to_string(), message: format!("duplicate kind id {}", kind.id) });
                }
            }
        }
        Ok(Self { categories: categories.into_iter().map(|(_, file)| file).collect(), index })
    }

    /// 📦️ The catalogue bundled into this crate.
    pub fn bundled() -> Result<Self, CatalogueError> {
        Self::parse(CATEGORY_SOURCES)
    }

    /// 🗂️ The category files in palette order.
    pub fn categories(&self) -> &[CategoryFile] {
        &self.categories
    }

    /// 🗂️ The category with this id.
    pub fn category(&self, id: &str) -> Option<&CategoryFile> {
        self.categories.iter().find(|file| file.category.id == id)
    }

    /// 🧩️ Every kind in palette order.
    pub fn kinds(&self) -> impl Iterator<Item = &Kind> {
        self.categories.iter().flat_map(|file| file.kinds.iter())
    }

    /// 🧩️ The kind with this id.
    pub fn kind(&self, id: &str) -> Option<&Kind> {
        self.index.get(id).map(|&(category, kind)| &self.categories[category].kinds[kind])
    }

    /// 🔌️ The input or output port `name` of the kind `id`; inputs win when both exist.
    pub fn port(&self, id: &str, name: &str) -> Option<&Port> {
        let kind = self.kind(id)?;
        kind.input(name).or_else(|| kind.output(name))
    }
}

/// 📦️ The process-wide bundled catalogue, parsed once; the catalogue tests prove the bundle parses.
pub fn catalogue() -> &'static Catalogue {
    static BUNDLED: OnceLock<Catalogue> = OnceLock::new();
    BUNDLED.get_or_init(|| Catalogue::bundled().expect("the bundled geometry catalogue is validated by its tests"))
}

//#region 🔖️Findings
/// 🔎️ One violated catalogue law: its code, the kind or category that owns it and the port it concerns.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Finding {
    pub code: &'static str,
    pub owner: String,
    pub port: Option<String>,
}

fn finding(findings: &mut Vec<Finding>, code: &'static str, owner: &str, port: Option<&str>) {
    findings.push(Finding { code, owner: owner.to_string(), port: port.map(str::to_string) });
}

fn blank(text: &Localized) -> bool {
    text.en.trim().is_empty() || text.de.trim().is_empty()
}

fn identifier(name: &str) -> bool {
    name.chars().next().is_some_and(|first| first.is_ascii_lowercase()) && name.chars().all(|character| character.is_ascii_alphanumeric())
}

fn id_matches(category: &str, id: &str) -> bool {
    let namespace = category.split('.').next().unwrap_or_default();
    let prefix = match namespace {
        "brep" | "mesh" => category,
        "math" | "analysis" => namespace,
        _ => return false,
    };
    id.strip_prefix(prefix).and_then(|rest| rest.strip_prefix('.')).is_some_and(identifier)
}

fn triple(value: &DslValue) -> bool {
    value.as_array().is_some_and(|items| items.len() == 3 && items.iter().all(|item| item.as_f64().is_some_and(f64::is_finite)))
}

fn plane_form(value: &DslValue) -> bool {
    value.as_object().is_some_and(|entries| entries.len() == 2 && value.get("origin").is_some_and(triple) && value.get("normal").is_some_and(triple))
}

fn element_form(port_type: PortType, value: &DslValue) -> bool {
    match port_type {
        PortType::Number | PortType::Length | PortType::Angle => value.as_f64().is_some_and(f64::is_finite),
        PortType::Integer => value.as_f64().is_some_and(|number| number.is_finite() && number.fract() == 0.0),
        PortType::Boolean => value.as_bool().is_some(),
        PortType::Text | PortType::Enum => value.as_str().is_some(),
        PortType::Vector | PortType::Point => triple(value),
        PortType::Plane => plane_form(value),
        PortType::Shape | PortType::Shapes | PortType::Mesh | PortType::Selection | PortType::Any => false,
    }
}

fn selection_form(value: &DslValue) -> bool {
    value.as_array().is_some_and(|items| items.iter().all(|item| item.as_str().is_some() || item.as_f64().is_some_and(|number| number >= 0.0 && number.fract() == 0.0)))
}

fn in_range(port: &Port, number: f64) -> bool {
    let above = match port.min {
        Some(min) if port.exclusive_min => number > min,
        Some(min) => number >= min,
        None => true,
    };
    above && port.max.is_none_or(|max| number <= max)
}

fn check_texts(findings: &mut Vec<Finding>, owner: &str, port: Option<&str>, label: &Localized, description: &Localized) {
    if blank(label) || blank(description) {
        finding(findings, "text-missing", owner, port);
    } else if description.en == description.de {
        finding(findings, "text-identical", owner, port);
    }
}

fn check_port(findings: &mut Vec<Finding>, kind: &Kind, port: &Port, input: bool) {
    let owner = kind.id.as_str();
    let name = Some(port.name.as_str());
    check_texts(findings, owner, name, &port.label, &port.description);
    let port_type = port.port_type;
    let shaped = matches!(port_type, PortType::Shape | PortType::Shapes);
    if port.shape_kinds.as_ref().is_some_and(|kinds| !shaped || kinds.is_empty()) {
        finding(findings, "shape-kinds-misplaced", owner, name);
    }
    if port.list && matches!(port_type, PortType::Shape | PortType::Shapes | PortType::Mesh | PortType::Selection | PortType::Enum) {
        finding(findings, "list-misplaced", owner, name);
    }
    let counted = port.list || port_type == PortType::Shapes || port_type == PortType::Selection;
    if (!counted && (port.min_items.is_some() || port.max_items.is_some())) || matches!((port.min_items, port.max_items), (Some(min), Some(max)) if min > max) {
        finding(findings, "item-bounds-misplaced", owner, name);
    }
    let constrained = port.min.is_some() || port.max.is_some() || port.step.is_some() || port.exclusive_min;
    if constrained && (!port_type.is_numeric() || !input) {
        finding(findings, "constraint-misplaced", owner, name);
    }
    let inverted = matches!((port.min, port.max), (Some(min), Some(max)) if min > max || (min == max && port.exclusive_min));
    if inverted || port.step.is_some_and(|step| step <= 0.0 || !step.is_finite()) || (port.exclusive_min && port.min.is_none()) {
        finding(findings, "bounds-inverted", owner, name);
    }
    let options = port.options.as_deref().unwrap_or_default();
    let mut values: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
    values.sort_unstable();
    values.dedup();
    if options.iter().any(|option| blank(&option.label)) {
        finding(findings, "text-missing", owner, name);
    }
    if (port_type == PortType::Enum) != port.options.is_some() || (port_type == PortType::Enum && (options.len() < 2 || values.len() != options.len())) {
        finding(findings, "enum-options-invalid", owner, name);
    }
    if !input {
        if port.default.is_some() {
            finding(findings, "default-misplaced", owner, name);
        }
        if port.selection.is_some() {
            finding(findings, "selection-on-output", owner, name);
        }
        return;
    }
    let valueless = matches!(port_type, PortType::Shape | PortType::Shapes | PortType::Mesh);
    match &port.default {
        Some(_) if valueless => finding(findings, "default-misplaced", owner, name),
        None if !valueless && !port.optional => finding(findings, "default-missing", owner, name),
        Some(default) => check_default(findings, owner, port, default),
        None => {}
    }
    check_selection(findings, kind, port);
}

fn check_default(findings: &mut Vec<Finding>, owner: &str, port: &Port, default: &DslValue) {
    let name = Some(port.name.as_str());
    let port_type = port.port_type;
    if port_type == PortType::Selection {
        match default.as_array() {
            Some(items) if selection_form(default) => {
                if port.min_items.is_some_and(|min| items.len() < min as usize && !items.is_empty()) || port.max_items.is_some_and(|max| items.len() > max as usize) {
                    finding(findings, "default-length-out-of-range", owner, name);
                }
            }
            _ => finding(findings, "default-type-mismatch", owner, name),
        }
        return;
    }
    let elements: Vec<&DslValue> = if port.list {
        match default.as_array() {
            Some(items) => {
                if port.min_items.is_some_and(|min| items.len() < min as usize) || port.max_items.is_some_and(|max| items.len() > max as usize) {
                    finding(findings, "default-length-out-of-range", owner, name);
                }
                items.iter().collect()
            }
            None => return finding(findings, "default-type-mismatch", owner, name),
        }
    } else {
        vec![default]
    };
    if elements.iter().any(|element| !element_form(port_type, element)) {
        return finding(findings, "default-type-mismatch", owner, name);
    }
    if port_type.is_numeric() && elements.iter().any(|element| element.as_f64().is_some_and(|number| !in_range(port, number))) {
        finding(findings, "default-out-of-range", owner, name);
    }
    if port_type == PortType::Enum && !port.options.as_deref().unwrap_or_default().iter().any(|option| Some(option.value.as_str()) == default.as_str()) {
        finding(findings, "enum-default-not-option", owner, name);
    }
}

fn check_selection(findings: &mut Vec<Finding>, kind: &Kind, port: &Port) {
    let owner = kind.id.as_str();
    let name = Some(port.name.as_str());
    let Some(selection) = &port.selection else {
        if port.port_type == PortType::Selection {
            finding(findings, "selection-source-invalid", owner, name);
        }
        return;
    };
    if port.port_type != PortType::Selection {
        return finding(findings, "selection-source-invalid", owner, name);
    }
    let source = kind.input(&selection.source).filter(|source| source.name != port.name && matches!(source.port_type, PortType::Shape | PortType::Shapes | PortType::Mesh));
    if source.is_none() {
        finding(findings, "selection-source-invalid", owner, name);
    }
    let mode = selection.mode_from.as_deref().and_then(|from| kind.input(from));
    let mode_ok = match (selection.component, mode) {
        (SelectionComponent::Mode, Some(mode)) => mode.port_type == PortType::Enum && mode.options.as_deref().unwrap_or_default().iter().all(|option| matches!(option.value.as_str(), "vertex" | "edge" | "face")),
        (SelectionComponent::Mode, None) => false,
        (_, found) => selection.mode_from.is_none() && found.is_none(),
    };
    if !mode_ok {
        finding(findings, "selection-mode-invalid", owner, name);
    }
    if !selection.multiple && port.max_items != Some(1) {
        finding(findings, "selection-multiplicity-mismatch", owner, name);
    }
}

fn selection_accepts(kind: &Kind, port: &Port, component: PickComponent) -> bool {
    let wanted = match component {
        PickComponent::Face => (SelectionComponent::Face, ShapeKind::Face, "face"),
        PickComponent::Edge => (SelectionComponent::Edge, ShapeKind::Edge, "edge"),
        PickComponent::Vertex => (SelectionComponent::Vertex, ShapeKind::Vertex, "vertex"),
        PickComponent::Shape => return matches!(port.port_type, PortType::Shape | PortType::Shapes),
        PickComponent::Mesh => return port.port_type == PortType::Mesh,
    };
    match (port.port_type, &port.selection) {
        (PortType::Selection, Some(selection)) if selection.component == wanted.0 => true,
        (PortType::Selection, Some(selection)) if selection.component == SelectionComponent::Mode => selection.mode_from.as_deref().and_then(|from| kind.input(from)).is_some_and(|mode| mode.options.as_deref().unwrap_or_default().iter().any(|option| option.value == wanted.2)),
        (PortType::Shape | PortType::Shapes, _) => port.shape_kinds.as_deref().is_some_and(|kinds| kinds.contains(&wanted.1)),
        _ => false,
    }
}

fn check_interaction(findings: &mut Vec<Finding>, kind: &Kind) {
    let owner = kind.id.as_str();
    for pick in kind.picks() {
        match kind.input(&pick.port) {
            None => finding(findings, "pick-port-missing", owner, Some(&pick.port)),
            Some(port) if !selection_accepts(kind, port, pick.component) => finding(findings, "pick-component-mismatch", owner, Some(&pick.port)),
            Some(_) => {}
        }
    }
    for gumball in kind.gumballs() {
        let name = Some(gumball.port.as_str());
        let Some(port) = kind.input(&gumball.port) else {
            finding(findings, "gumball-port-missing", owner, name);
            continue;
        };
        let accepted: &[PortType] = match gumball.motion {
            GumballMotion::Translate => &[PortType::Vector, PortType::Point, PortType::Plane, PortType::Length, PortType::Number],
            GumballMotion::Rotate => &[PortType::Angle, PortType::Plane],
            GumballMotion::Scale => &[PortType::Vector, PortType::Number, PortType::Length],
        };
        if !accepted.contains(&port.port_type) {
            finding(findings, "gumball-type-mismatch", owner, name);
        }
        let scalar = matches!(port.port_type, PortType::Length | PortType::Number);
        if gumball.motion == GumballMotion::Translate && scalar != gumball.along.is_some() {
            finding(findings, "gumball-along-invalid", owner, name);
        }
        if gumball.motion != GumballMotion::Translate && gumball.along.is_some() {
            finding(findings, "gumball-along-invalid", owner, name);
        }
        let axis_ok = match (gumball.motion, port.port_type, gumball.axis_port.as_deref()) {
            (GumballMotion::Rotate, PortType::Angle, Some(axis)) => kind.input(axis).is_some_and(|axis| axis.port_type == PortType::Vector),
            (GumballMotion::Rotate, PortType::Angle, None) => false,
            (_, _, axis) => axis.is_none(),
        };
        let origin_ok = match (gumball.motion, port.port_type, gumball.origin_port.as_deref()) {
            (GumballMotion::Rotate, PortType::Angle, Some(origin)) | (GumballMotion::Scale, _, Some(origin)) => kind.input(origin).is_some_and(|origin| origin.port_type == PortType::Point),
            (_, _, origin) => origin.is_none(),
        };
        if !axis_ok || !origin_ok {
            finding(findings, "gumball-axis-invalid", owner, name);
        }
    }
}

fn check_kind(findings: &mut Vec<Finding>, file: &CategoryFile, kind: &Kind) {
    let owner = kind.id.as_str();
    if kind.category != file.category.id {
        finding(findings, "kind-category-mismatch", owner, None);
    }
    if !id_matches(&file.category.id, &kind.id) {
        finding(findings, "kind-id-malformed", owner, None);
    }
    if kind.emoji.trim().is_empty() {
        finding(findings, "text-missing", owner, None);
    }
    check_texts(findings, owner, None, &kind.label, &kind.description);
    for (ports, input) in [(&kind.inputs, true), (&kind.outputs, false)] {
        let mut names: Vec<&str> = ports.iter().map(|port| port.name.as_str()).collect();
        names.sort_unstable();
        if let Some(duplicate) = names.windows(2).find(|pair| pair[0] == pair[1]) {
            finding(findings, "duplicate-port-name", owner, Some(duplicate[0]));
        }
        for port in ports {
            check_port(findings, kind, port, input);
        }
    }
    if kind.outputs.is_empty() {
        finding(findings, "no-outputs", owner, None);
    }
    check_interaction(findings, kind);
}

/// 🔎️ Checks the catalogue laws the JSON schema cannot express: ids, texts, defaults within bounds, enum options, selection sources, interaction targets and unique emojis. An empty answer means the files are sound.
pub fn check(files: &[CategoryFile]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut kind_ids: Vec<&str> = Vec::new();
    let mut category_ids: Vec<&str> = Vec::new();
    for file in files {
        let category = &file.category;
        check_texts(&mut findings, &category.id, None, &category.label, &category.description);
        if category_ids.contains(&category.id.as_str()) {
            finding(&mut findings, "duplicate-category-id", &category.id, None);
        }
        category_ids.push(&category.id);
        let mut emojis: Vec<&str> = Vec::new();
        for kind in &file.kinds {
            if kind_ids.contains(&kind.id.as_str()) {
                finding(&mut findings, "duplicate-kind-id", &kind.id, None);
            }
            kind_ids.push(&kind.id);
            if emojis.contains(&kind.emoji.as_str()) {
                finding(&mut findings, "emoji-duplicate", &kind.id, None);
            }
            emojis.push(&kind.emoji);
            check_kind(&mut findings, file, kind);
        }
    }
    findings
}

impl Catalogue {
    /// 🔎️ The findings of [`check`] over this catalogue's category files.
    pub fn findings(&self) -> Vec<Finding> {
        check(&self.categories)
    }
}
//#endregion 🔖️Findings

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

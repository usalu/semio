import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from mk import edit
A = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/"
ED = A + "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
SCHEMA = A + "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/"
GEO = SCHEMA + "📐️geometry/"
TNAME = "semio_s_artifact_cad_cad_model::geometry::CadObjectName"

enum = '''/// 🏷️ A cad object's name, stored language-neutrally in the event log and resolved at display time with the viewer's locale (`object_name`, `editor::cad::terminology`): a name the user typed is `Literal` text, a name the editor generated is the stable key it came from plus its ordinal, never a word of any language.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CadObjectName {
    Literal { text: String },
    Interaction {
        #[value(rename = "interactionId")]
        interaction_id: String,
        ordinal: u32,
        #[value(default)]
        part: Option<u32>,
    },
    Typology {
        #[value(default)]
        ordinal: Option<u32>,
    },
}

impl CadObjectName {
    pub fn literal(text: impl Into<String>) -> Self {
        Self::Literal { text: text.into() }
    }

    /// 🧩️ The `part`-th piece of a name generated for a multi-object commit (a polyline's segments); any other name is unchanged.
    pub fn with_part(&self, part: u32) -> Self {
        match self {
            Self::Interaction { interaction_id, ordinal, .. } => Self::Interaction { interaction_id: interaction_id.clone(), ordinal: *ordinal, part: Some(part) },
            other => other.clone(),
        }
    }
}

'''
props = '''/// 🏷️ The name properties a cad object's [`CAD_OBJECT_PSET`] carries: a typed name as `label` text, a generated name as its key and ordinals only.
fn object_name_properties(name: &CadObjectName) -> Vec<semio_s_artifact_stdio_semio_model::schema::snapshot::Property> {
    use semio_s_artifact_stdio_semio_model::schema::snapshot::{Property, PsetValue};
    let text = |key: &str, value: &str| Property { key: key.into(), value: PsetValue::Text { value: value.into() } };
    let number = |key: &str, value: u32| Property { key: key.into(), value: PsetValue::Number { value: f64::from(value) } };
    match name {
        CadObjectName::Literal { text: literal } => vec![text("label", literal)],
        CadObjectName::Interaction { interaction_id, ordinal, part } => [vec![text("nameKind", "interaction"), text("nameInteraction", interaction_id), number("nameOrdinal", *ordinal)], part.map(|part| number("namePart", part)).into_iter().collect()].concat(),
        CadObjectName::Typology { ordinal } => [vec![text("nameKind", "typology")], ordinal.map(|ordinal| number("nameOrdinal", ordinal)).into_iter().collect()].concat(),
    }
}

'''
read_label = '''        label: match text("nameKind").as_deref() {
            Some("interaction") => match (text("nameInteraction"), number("nameOrdinal")) {
                (Some(interaction_id), Some(ordinal)) => CadObjectName::Interaction { interaction_id, ordinal: ordinal as u32, part: number("namePart").map(|part| part as u32) },
                _ => CadObjectName::literal(object_label_from_id(&element.id)),
            },
            Some("typology") => CadObjectName::Typology { ordinal: number("nameOrdinal").map(|ordinal| ordinal as u32) },
            _ => CadObjectName::literal(text("label").unwrap_or_else(|| object_label_from_id(&element.id))),
        },'''
edit(GEO + "🦀️.rs", [
 ("fn default_true() -> bool {\n    true\n}\n", enum + "fn default_true() -> bool {\n    true\n}\n"),
 ("    pub id: String,\n    pub label: String,\n    pub typology: String,\n    #[value(default = \"default_true\")]", "    pub id: String,\n    pub label: CadObjectName,\n    pub typology: String,\n    #[value(default = \"default_true\")]"),
 ("                label: object_label_from_id(object_id),", "                label: CadObjectName::literal(object_label_from_id(object_id)),"),
 ("/// 🌉️ WRITE direction: one `CadObject` → one `SemioModelElement`", props + "/// 🌉️ WRITE direction: one `CadObject` → one `SemioModelElement`"),
 ('vec![text("label", &object.label), flag("visible"', 'vec![flag("visible"'),
 ("    properties.extend(object.mesh_url.as_deref()", "    properties.splice(0..0, object_name_properties(&object.label));\n    properties.extend(object.mesh_url.as_deref()"),
 ('        label: text("label").unwrap_or_else(|| object_label_from_id(&element.id)),', read_label),
], "name-geometry-1.patch")

geo_test = '''#[semio_framework_async_macros::async_test]
async fn cad_object_names_round_trip_language_neutrally() {
    let names = [
        CadObjectName::literal("U2"),
        CadObjectName::Interaction { interaction_id: "building.building.placeBeam".into(), ordinal: 3, part: Some(2) },
        CadObjectName::Interaction { interaction_id: "primitive.box".into(), ordinal: 1, part: None },
        CadObjectName::Typology { ordinal: Some(4) },
        CadObjectName::Typology { ordinal: None },
    ];
    for name in names {
        let object = CadObject { id: "object-n".into(), label: name.clone(), typology: "spatial.shape.primitive.box".into(), visible: true, locked: false, origin: [0.0; 3], orientation: None, scale: None, mesh_url: None, extent: None, solid_handle: None, primitives: Vec::new() };
        let element = model_element_from_cad_object(&object);
        let typed = matches!(name, CadObjectName::Literal { .. });
        assert_eq!(element.psets[0].properties.iter().any(|property| property.key == "label"), typed, "only a typed name is stored as text: {name:?}");
        assert_eq!(cad_object_from_model_element(&element).label, name);
    }
}

'''
edit(GEO + "🧪️tests/🔬️unit/🦀️.rs", [
 ('label: "ignored on the way in — restored from the id on the way out".into(),', 'label: CadObjectName::literal("U7"),'),
 ('label: "A".into(),', 'label: CadObjectName::literal("A"),'),
 ('label: "B".into(),', 'label: CadObjectName::literal("B"),'),
 ("#[semio_framework_async_macros::async_test]\nasync fn semio_model_snapshot_from_objects_round_trips_via_objects_from_model_snapshot() {", geo_test + "#[semio_framework_async_macros::async_test]\nasync fn semio_model_snapshot_from_objects_round_trips_via_objects_from_model_snapshot() {"),
], "name-geometry-2.patch")

object_name_fn = '''
/// 🏷️ A cad object's name as the viewer reads it: typed text as is, a generated name resolved from its stable key with the viewer's `axes`. Without a view context (`None`) a generated name reads as its language-neutral key, never as text of any language.
pub fn object_name(object: &CadObject, axes: Option<(Terminology, Locale)>) -> String {
    let numbered = |stem: String, ordinal: Option<u32>, part: Option<u32>| match (ordinal, part) {
        (Some(ordinal), Some(part)) => format!("{stem} {ordinal}.{part}"),
        (Some(ordinal), None) => format!("{stem} {ordinal}"),
        (None, _) => stem,
    };
    match &object.label {
        CadObjectName::Literal { text } => text.clone(),
        CadObjectName::Interaction { interaction_id, ordinal, part } => {
            let stem = axes.and_then(|(terminology, locale)| semio_s_artifact_cad_cad_interaction::interaction_by_id(interaction_id).map(|entry| entry.label.resolve(terminology, locale).to_string())).unwrap_or_else(|| interaction_id.clone());
            numbered(stem, Some(*ordinal), *part)
        }
        CadObjectName::Typology { ordinal } => {
            let stem = axes.map_or_else(|| object.typology.clone(), |(terminology, locale)| typology_label(&object.typology, CadLabels::labels(locale, terminology)).to_string());
            numbered(stem, *ordinal, None)
        }
    }
}
'''
edit(ED + "🗣️terminology/🦀️.rs", [
 ("use semio_framework_ui_locale::Locale;\nuse semio_framework_plugin::ViewModel;\n", "use semio_framework_ui_locale::{AppLabels, Locale, Terminology};\nuse semio_framework_plugin::ViewModel;\nuse semio_s_artifact_cad_cad_model::geometry::{CadObject, CadObjectName};\n"),
 ('        typology_external_wall: native_en "External Wall", native_de "Außenwand", reuse_en "External Wall", reuse_de "Außenwand";\n', '        typology_external_wall: native_en "External Wall", native_de "Außenwand", reuse_en "External Wall", reuse_de "Außenwand";\n        typology_hull: native_en "Hull", native_de "Hülle", reuse_en "Hull", reuse_de "Hülle";\n        typology_windows: native_en "Windows", native_de "Fenster", reuse_en "Windows", reuse_de "Fenster";\n'),
 ('        "energy.energy.externalwall" => labels.typology_external_wall.as_str(),\n', '        "energy.energy.externalwall" => labels.typology_external_wall.as_str(),\n        "energy.energy.hull" => labels.typology_hull.as_str(),\n        "energy.energy.windows" => labels.typology_windows.as_str(),\n'),
 ("//#endregion 🔖️Terminology", object_name_fn + "//#endregion 🔖️Terminology"),
], "name-terminology.patch")

edit(ED + "🧰️kit/🦀️.rs", [
 ("geometry::CadObject, geometry::cad_object_from_model_element", "geometry::CadObject, geometry::CadObjectName, geometry::cad_object_from_model_element"),
 ("use semio_framework_ui_locale::{Label, LocalizedLabel};", "use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};"),
 ("    pub interaction: CadInteractionSnapshot,\n}\nimpl CadPlayView {", "    pub interaction: CadInteractionSnapshot,\n    /// 🗣️ The viewer's `(terminology, locale)` when the view is built to be read (`render`, window chrome); `None` for headless uses (exports), where a generated object name reads as its language-neutral key.\n    pub axes: Option<(Terminology, Locale)>,\n}\nimpl CadPlayView {"),
 ("Self { document: document.clone(), panes: CadComposedPanes::compose(document, children), runtime, interaction }\n    }", "Self { document: document.clone(), panes: CadComposedPanes::compose(document, children), runtime, interaction, axes: None }\n    }\n    /// 🗣️ This view read by a viewer in `terminology` and `locale`.\n    pub fn with_axes(self, terminology: Terminology, locale: Locale) -> Self {\n        Self { axes: Some((terminology, locale)), ..self }\n    }"),
 ('label: format!("{} copy", source.label), origin', "label: source.label.clone(), origin"),
 ('    let label = TYPOLOGY_CATALOG.iter().find(|entry| entry.typology == typology).map_or("Object", |entry| entry.label);\n', ""),
 ('        label: format!("{label} {}", label_count + 1),', "        label: CadObjectName::Typology { ordinal: Some(u32::try_from(label_count).unwrap_or(u32::MAX).saturating_add(1)) },"),
 ('"label" => next.label = value.and_then(semio_framework_value::DslValue::as_str)?.to_string(),', '"label" => next.label = CadObjectName::literal(value.and_then(semio_framework_value::DslValue::as_str)?),'),
], "name-kit.patch")

edit(SCHEMA + "💡️inferences/🔄️derive-transformation/🦀️.rs", [
 ("use semio_s_artifact_cad_cad_model::geometry::CadObject;\n", "use semio_s_artifact_cad_cad_model::geometry::{CadObject, CadObjectName};\n"),
 ('label: "Hull".into(),', "label: CadObjectName::Typology { ordinal: None },", 2),
 ('        let label = rule.role.replace("externalwall", "External Wall").replace("slab", "Slab");\n', ""),
 ("            id: next_object_id(id_seed, index),\n            label,\n", "            id: next_object_id(id_seed, index),\n            label: CadObjectName::Typology { ordinal: None },\n"),
 ('label: "Windows".into(),', "label: CadObjectName::Typology { ordinal: None },"),
], "name-derive.patch")
edit(SCHEMA + "💡️inferences/🧪️tests/🔬️derive-transformation-unit/🦀️.rs", [('label: "Box".into(),', f'label: {TNAME}::literal("Box"),')], "name-derive-test.patch")
edit(A + "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🗺️geometry-import/🦀️.rs", [("CadObject { id: id.into(), label: label.into(),", "CadObject { id: id.into(), label: CadObjectName::literal(label),")], "name-import.patch")
edit(A + "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️dwg-import/🦀️.rs", [('assert_eq!(working.objects[0].label, "outline");', f'assert_eq!(working.objects[0].label, {TNAME}::literal("outline"));')], "name-dwg-test.patch")
edit(A + "🧪️tests/🧩️sample-scene/🦀️.rs", [('label: format!("Object {id}"),', f'label: {TNAME}::literal(format!("Object {{id}}")),')], "name-sample-scene.patch")
edit(ED + "⚙️engine/🧲️picking/🧪️tests/🔬️unit/🦀️.rs", [("        label: id.to_string(),", f"        label: {TNAME}::literal(id),")], "name-picking-test.patch")
edit(ED + "🧪️tests/🔬️unit/⏪️history-and-convergence/🦀️.rs", [('assert_eq!(renamed.label, "Renamed");', f'assert_eq!(renamed.label, {TNAME}::literal("Renamed"));')], "name-history-test.patch")

IMP = "use semio_s_artifact_cad_cad_kit::{editor::cad::terminology::typology_label, editor::cad::terminology::CadLabels};"
IMP2 = "use semio_s_artifact_cad_cad_kit::{editor::cad::terminology::object_name, editor::cad::terminology::typology_label, editor::cad::terminology::CadLabels};"
AX = "Option<(semio_framework_ui_locale::Terminology, semio_framework_ui_locale::Locale)>"
LN = "semio_framework_ui_locale::Locale::"
PA = ED + "📌️panels/🗿️artifact/"
edit(PA + "🦀️.rs", [
 (IMP, IMP2),
 ("object: &CadObject, labels: &CadLabels) -> UiAssemblyResult<BuiltNode> {\n    let mut item = ui::tree_item(ui_label(&object.label)?)", f"object: &CadObject, labels: &CadLabels, axes: {AX}) -> UiAssemblyResult<BuiltNode> {{\n    let mut item = ui::tree_item(ui_label(&object_name(object, axes))?)"),
 ("objects: &[CadObject], labels: &CadLabels) -> UiAssemblyResult<PanelTreeBuilder> {", f"objects: &[CadObject], labels: &CadLabels, axes: {AX}) -> UiAssemblyResult<PanelTreeBuilder> {{"),
 ("object_tree_item(windows, id_suffix, object, labels))", "object_tree_item(windows, id_suffix, object, labels, axes))"),
] + [(f"objects_of(CadPaneId::{n}), labels)?;", f"objects_of(CadPaneId::{n}), labels, envelope.axes)?;") for n in ["Shape", "Building", "Energy", "StructureClassic"]], "name-panels-artifact.patch")
panel_test = f'''#[semio_framework_async_macros::async_test]
async fn generated_object_names_resolve_per_viewer_language() {{
    let axes = |locale| Some((semio_framework_ui_locale::Terminology::Native, locale));
    let mut object = make_object_for_typology("building.building.beam", 1, CadPaneId::Shape);
    assert_eq!(object_name(&object, axes({LN}En)), "Beam 2");
    assert_eq!(object_name(&object, axes({LN}De)), "Träger 2");
    object.label = {TNAME}::Interaction {{ interaction_id: "building.building.placeBeam".into(), ordinal: 3, part: None }};
    assert_eq!(object_name(&object, axes({LN}En)), "Place Beam 3");
    assert_eq!(object_name(&object, axes({LN}De)), "Träger platzieren 3");
    assert_eq!(object_name(&object, None), "building.building.placeBeam 3", "without a view context the key is shown, in no language");
    object.label = {TNAME}::literal("U2");
    assert_eq!(object_name(&object, axes({LN}De)), "U2");
}}

'''
edit(PA + "🧪️tests/🔬️unit/🦀️.rs", [
 ('object.label = "U2".into();', f'object.label = {TNAME}::literal("U2");'),
 ('object_tree_item(&windows, "shape", &object, labels)', 'object_tree_item(&windows, "shape", &object, labels, None)'),
 ('object_tree_item(&de_windows, "shape", &object, de_labels)', f'object_tree_item(&de_windows, "shape", &object, de_labels, Some((semio_framework_ui_locale::Terminology::Native, {LN}De)))'),
 ('object_tree_item(&TreeWindows::unhosted(), "shape", &object, labels)', 'object_tree_item(&TreeWindows::unhosted(), "shape", &object, labels, None)'),
 ('"shape", &object, labels).expect("opened object row")', '"shape", &object, labels, None).expect("opened object row")'),
 ("#[semio_framework_async_macros::async_test]\nasync fn object_tree_item_shows_name_with_kind_as_secondary_label() {", panel_test + "#[semio_framework_async_macros::async_test]\nasync fn object_tree_item_shows_name_with_kind_as_secondary_label() {"),
 (IMP, IMP2),
], "name-panels-artifact-test.patch")
PI = ED + "📌️panels/🔍️inspection/"
edit(PI + "🦀️.rs", [(IMP, IMP2), ('"object.label", labels.label.as_str(), &object.label)?;', '"object.label", labels.label.as_str(), object_name(object, envelope.axes))?;')], "name-panels-inspection.patch")
edit(PI + "🧪️tests/🔬️unit/🦀️.rs", [("assert!(json.contains(&object.label));", "assert!(json.contains(&semio_s_artifact_cad_cad_kit::editor::cad::terminology::object_name(object, None)));")], "name-panels-inspection-test.patch")

import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from mk import edit, create
A = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/"
ED = A + "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
SPEC = A + "🎬️interaction-spec/"
ENG = ED + "⚙️engine/🕹️interaction/"
MODE = ED + "🎭️modes/✏️edit/"

NAME = "CadObjectName"
EXTRA_ENG = [
 ("use semio_s_artifact_cad_cad_model::{geometry::CadObject, geometry::CadPrimitiveSlot};", "use semio_s_artifact_cad_cad_model::{geometry::CadObject, geometry::CadObjectName, geometry::CadPrimitiveSlot};"),
 ("fn commit_primitive_box(kernel: &mut Brep, params: &HashMap<String, DslValue>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {", "fn commit_primitive_box(kernel: &mut Brep, params: &HashMap<String, DslValue>, name: CadObjectName, next_id: impl Fn(&str) -> String) -> Option<CadObject> {"),
 ('        label: format!("Box {}", label_count + 1),', "        label: name,"),
 ("fn commit_from_2_points_and_height(kernel: &mut Brep, params: &HashMap<String, DslValue>, label: &str, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {", "fn commit_from_2_points_and_height(kernel: &mut Brep, params: &HashMap<String, DslValue>, name: &CadObjectName, next_id: impl Fn(&str) -> String) -> Option<CadObject> {"),
 ('label: format!("{label} {}", label_count + 1),', "label: name.clone(),", 2),
 ("fn solid_object(kernel: &mut Brep, typology: &str, label: String,", "fn solid_object(kernel: &mut Brep, typology: &str, label: CadObjectName,"),
 ("fn bar_object(kernel: &mut Brep, typology: &str, label: String,", "fn bar_object(kernel: &mut Brep, typology: &str, label: CadObjectName,"),
 ("fn polyline_objects(kernel: &mut Brep, typology: &str, label: &str, points: &[[f64; 3]], closed: bool, label_count: usize, next_id: &impl Fn(&str) -> String) -> Vec<CadObject> {", "fn polyline_objects(kernel: &mut Brep, typology: &str, name: &CadObjectName, points: &[[f64; 3]], closed: bool, next_id: &impl Fn(&str) -> String) -> Vec<CadObject> {"),
 ('bar_object(kernel, typology, format!("{label} {}.{}", label_count + 1, index + 1), p0, p1, [0.02, 0.02], next_id)', "bar_object(kernel, typology, name.with_part(u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1)), p0, p1, [0.02, 0.02], next_id)"),
 ("fn commit_command_finish(kernel: &mut Brep, params: &HashMap<String, DslValue>, context: &HashMap<String, DslValue>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CommitOutcome> {", "fn commit_command_finish(kernel: &mut Brep, params: &HashMap<String, DslValue>, context: &HashMap<String, DslValue>, name: &CadObjectName, next_id: impl Fn(&str) -> String) -> Option<CommitOutcome> {"),
 ('format!("Sphere {}", label_count + 1)', "name.clone()"),
 ('format!("Cylinder {}", label_count + 1)', "name.clone()"),
 ('format!("Plane {}", label_count + 1)', "name.clone()"),
 ('let (typology, label) = if result_kind == "arc" { ("spatial.shape.curve.arc", "Arc") } else { ("spatial.shape.curve.circle", "Circle") };\n            let objects = polyline_objects(kernel, typology, label, &points, false, label_count, &next_id);', 'let typology = if result_kind == "arc" { "spatial.shape.curve.arc" } else { "spatial.shape.curve.circle" };\n            let objects = polyline_objects(kernel, typology, name, &points, false, &next_id);'),
 ('let (typology, label) = if result_kind == "curve" { ("spatial.shape.curve.control-point-curve", "Curve") } else { ("spatial.shape.curve.interpolate-curve", "Interpolated curve") };\n            let objects = polyline_objects(kernel, typology, label, &points, false, label_count, &next_id);', 'let typology = if result_kind == "curve" { "spatial.shape.curve.control-point-curve" } else { "spatial.shape.curve.interpolate-curve" };\n            let objects = polyline_objects(kernel, typology, name, &points, false, &next_id);'),
 ("    let label = spec.label.clone().unwrap_or_else(|| spec.id.clone());", "    let name = CadObjectName::Interaction { interaction_id: spec.id.clone(), ordinal: u32::try_from(label_count).unwrap_or(u32::MAX).saturating_add(1), part: None };"),
 ("commit_primitive_box(kernel, &params, label_count, next_id)", "commit_primitive_box(kernel, &params, name, next_id)"),
 ("commit_from_2_points_and_height(kernel, &params, &label, label_count, next_id)", "commit_from_2_points_and_height(kernel, &params, &name, next_id)"),
 ("commit_command_finish(kernel, &params, context, label_count, next_id)", "commit_command_finish(kernel, &params, context, &name, next_id)"),
 ('bar_object(kernel, &typology, format!("{label} {}", label_count + 1), p0, p1, thickness, next_id)', "bar_object(kernel, &typology, name.clone(), p0, p1, thickness, next_id)"),
 ("polyline_objects(kernel, &typology, &label, &points, closed, label_count, &next_id)", "polyline_objects(kernel, &typology, &name, &points, closed, &next_id)"),
]
EXTRA_MODE = [
 ("use semio_s_artifact_cad_cad_kit::editor::cad::terminology::CadLabels;\n", "use semio_s_artifact_cad_cad_kit::editor::cad::terminology::{object_name, CadLabels};\n"),
 ('("label".to_string(), DslValue::String(object.label.clone())),', '("label".to_string(), DslValue::String(object_name(object, view.axes))),'),
]
LABELS_USE = "use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};\n"
TERM = "semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En"

edit(SPEC + "📦️packages/🦀️rust/Cargo.toml", [("semio-framework-value-derive = { workspace = true }\n", "semio-framework-ui-locale = { workspace = true }\nsemio-framework-value-derive = { workspace = true }\n")], "cad-spec-rust-0.patch")

edit(SPEC + "🦀️.rs", [
 ("use semio_framework_value_derive::{FromValue, ToValue};\n", "use semio_framework_ui_locale::LocalizedLabel;\nuse semio_framework_value_derive::{FromValue, ToValue};\n"),
 ("    pub key: Option<String>,\n    #[value(default)]\n    pub label: Option<String>,\n    #[value(default)]\n    pub effects: Vec<Effect>,", "    pub key: Option<String>,\n    #[value(default)]\n    pub label: Option<LocalizedLabel>,\n    #[value(default)]\n    pub effects: Vec<Effect>,"),
 ("    pub prompt: Option<String>,", "    pub prompt: Option<LocalizedLabel>,"),
 ("        text: String,\n        position: Expr,", "        text: LocalizedLabel,\n        position: Expr,"),
 ("    pub version: String,\n    #[value(default)]\n    pub label: Option<String>,", "    pub version: String,\n    #[value(default)]\n    pub label: Option<LocalizedLabel>,"),
 ("impl InteractionSpec {\n", """impl InteractionSpec {
    /// 🌐️ Every text the viewer reads in this spec (its label, transition labels, selection prompts and display label texts), each a complete locale matrix; see [`LocalizedLabel`].
    pub fn localized_labels(&self) -> Vec<&LocalizedLabel> {
        let transitions = self.machine.states.iter().flat_map(|state| state.on.iter()).flat_map(|handler| handler.transitions.iter()).filter_map(|transition| transition.label.as_ref());
        let prompts = self.machine.states.iter().filter_map(|state| state.selection.as_ref()).filter_map(|selection| selection.prompt.as_ref());
        let texts = self.display.states.iter().flat_map(|state| state.items.iter()).filter_map(|item| match item {
            DisplayItemSpec::Label { text, .. } => Some(text),
            _ => None,
        });
        self.label.iter().chain(transitions).chain(prompts).chain(texts).collect()
    }

"""),
], "cad-spec-rust-1.patch")

edit(ENG + "📦️packages/🦀️rust/Cargo.toml", [("semio-framework-value = { workspace = true }\n", "semio-framework-ui-locale = { workspace = true }\nsemio-framework-value = { workspace = true }\n")], "cad-engine-0.patch")

edit(ENG + "🦀️.rs", EXTRA_ENG + [
 ("use semio_framework_3d_brep_engine::{Brep, BrepKernel};\n", "use semio_framework_3d_brep_engine::{Brep, BrepKernel};\nuse semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};\n"),
 ("pub struct KeyedTransition {\n    pub key: String,\n    pub label: String,", "pub struct KeyedTransition {\n    pub key: String,\n    pub label: LocalizedLabel,"),
 ("pub struct InteractionCatalogEntry {\n    pub id: String,\n    pub label: String,", "pub struct InteractionCatalogEntry {\n    pub id: String,\n    pub label: LocalizedLabel,"),
 ("                label: spec.label.clone().unwrap_or_else(|| spec.id.clone()),", "                label: spec.label.clone().unwrap_or_else(|| LocalizedLabel::data(spec.id.clone())),"),
 ("label: transition.label.clone().unwrap_or_else(|| handler.event.clone()), event_kind", "label: transition.label.clone().unwrap_or_else(|| LocalizedLabel::data(handler.event.clone())), event_kind"),
 ("    session.last_response = Some(\"OK\".into());\n", ""),
 ("pub fn state_prompt(session: &CadEngagementScratch) -> String {", "pub fn state_prompt(session: &CadEngagementScratch, terminology: Terminology, locale: Locale) -> String {"),
 (".and_then(|selection| selection.prompt.clone()) {\n            return prompt;", ".and_then(|selection| selection.prompt.as_ref()) {\n            return prompt.resolve(terminology, locale).to_string();"),
 ("if role.as_deref() == Some(\"prompt\") => Some(text.clone()),", "if role.as_deref() == Some(\"prompt\") => Some(text.resolve(terminology, locale).to_string()),"),
 ("fn display_item_to_json(item: &DisplayItemSpec, env: &ExprEnv<'_>, vars: &HashMap<String, DslValue>) -> Option<DslValue> {", "fn display_item_to_json(item: &DisplayItemSpec, env: &ExprEnv<'_>, vars: &HashMap<String, DslValue>, terminology: Terminology, locale: Locale) -> Option<DslValue> {"),
 ("(\"text\".to_string(), DslValue::String(text.clone()))", "(\"text\".to_string(), DslValue::String(text.resolve(terminology, locale).to_string()))"),
 ("pub fn preview_display_items(session: &CadEngagementScratch) -> Vec<DslValue> {", "pub fn preview_display_items(session: &CadEngagementScratch, terminology: Terminology, locale: Locale) -> Vec<DslValue> {"),
 ("display_item_to_json(item, &env, &empty_vars)).collect()", "display_item_to_json(item, &env, &empty_vars, terminology, locale)).collect()"),
], "cad-engine-1.patch")

edit(ENG + "🧪️tests/🔬️unit/🦀️.rs", [
 ("let items = preview_display_items(&session);", "let items = preview_display_items(&session, Terminology::Native, Locale::En);", 4),
 ('assert_eq!(state_prompt(&session), "Place Column: click opposite footprint corner");', 'assert_eq!(state_prompt(&session, Terminology::Native, Locale::En), "Place Column: click opposite footprint corner");\n    assert_eq!(state_prompt(&session, Terminology::Native, Locale::De), "Stütze platzieren: Gegenüberliegenden Grundrisseckpunkt anklicken");'),
 ('assert_eq!(state_prompt(&session), "Radius", "the display prompt label, not the raw state name");', 'assert_eq!(state_prompt(&session, Terminology::Native, Locale::En), "Radius", "the display prompt label, not the raw state name");\n    assert_eq!(state_prompt(&session, Terminology::Native, Locale::De), "Radius");'),
 ("#[semio_framework_async_macros::async_test]\nasync fn catalog_lists_every_model_definition_asset() {", """#[semio_framework_async_macros::async_test]
async fn every_interaction_text_carries_both_languages_and_no_blank_cell() {
    let (mut texts, mut translated) = (0usize, 0usize);
    for (model_definition, spec) in parsed_specs() {
        assert!(spec.label.is_some(), "{model_definition}/{} has a label", spec.id);
        for label in spec.localized_labels() {
            assert!(!label.has_blank_cell(), "{model_definition}/{}: a blank language cell", spec.id);
            texts += 1;
            translated += usize::from(label.resolve(Terminology::Native, Locale::En) != label.resolve(Terminology::Native, Locale::De));
        }
    }
    assert!(texts >= 600, "{texts} localized texts");
    assert!(translated * 100 >= texts * 85, "{translated} of {texts} texts differ between en and de");
}

#[semio_framework_async_macros::async_test]
async fn catalog_lists_every_model_definition_asset() {"""),
], "cad-engine-2.patch")

for w, short in [("📐️shape", "shape"), ("🏢️building", "building"), ("🔥️energy", "energy"), ("🏛️structure-classic", "structure")]:
    p = MODE + "🪟️windows/" + w + "/🦀️.rs"
    edit(p, [
     ("use semio_s_artifact_cad_cad_kit::editor::cad::terminology::CadLabels;\n", "use semio_framework_ui_locale::{Locale, Terminology};\nuse semio_s_artifact_cad_cad_kit::editor::cad::terminology::CadLabels;\n"),
     ("pub fn render(view: &CadPlayView, active_utility: Option<&str>, options: CadDislocateOptions) -> UiAssemblyResult<BuiltNode> {\n    edit::build_world_scene_for_pane(view, PANE, SURFACE_ID, active_utility, options)", "pub fn render(view: &CadPlayView, active_utility: Option<&str>, options: CadDislocateOptions, terminology: Terminology, locale: Locale) -> UiAssemblyResult<BuiltNode> {\n    edit::build_world_scene_for_pane(view, PANE, SURFACE_ID, active_utility, options, terminology, locale)"),
     ("pub fn engagement(view: &CadPlayView, labels: &CadLabels) -> WindowEngagement {\n    edit::cad_window_engagement(view, PANE, labels)", "pub fn engagement(view: &CadPlayView, labels: &CadLabels, terminology: Terminology, locale: Locale) -> WindowEngagement {\n    edit::cad_window_engagement(view, PANE, labels, terminology, locale)"),
    ], f"cad-mode-window-{short}.patch")

edit(MODE + "🦀️.rs", EXTRA_MODE + [
 ("use semio_framework_ui_locale::LocalizedLabel;\n", LABELS_USE),
 ("active_utility: Option<&str>, options: CadDislocateOptions) -> UiAssemblyResult<BuiltNode> {\n    let composed", "active_utility: Option<&str>, options: CadDislocateOptions, terminology: Terminology, locale: Locale) -> UiAssemblyResult<BuiltNode> {\n    let composed"),
 ("let mut items = preview_display_items(session);", "let mut items = preview_display_items(session, terminology, locale);"),
 ("pub fn cad_window_engagement(envelope: &CadPlayView, pane: CadPaneId, labels: &CadLabels) -> WindowEngagement {", "pub fn cad_window_engagement(envelope: &CadPlayView, pane: CadPaneId, labels: &CadLabels, terminology: Terminology, locale: Locale) -> WindowEngagement {"),
 ("                label: transition.label,\n", "                label: transition.label.resolve(terminology, locale).to_string(),\n"),
 ("                label: entry.label.clone(),\n", "                label: entry.label.resolve(terminology, locale).to_string(),\n"),
 ("engagement_step_text(labels, &envelope.runtime.engagement_step), state_prompt);", "engagement_step_text(labels, &envelope.runtime.engagement_step), |session| state_prompt(session, terminology, locale));"),
], "cad-mode-edit.patch")

K = ED + "🦀️.rs"
edit(K, [("let view = CadPlayView::of(doc.snapshot, &doc.children, runtime_of(cfg, transient), interaction);", "let view = CadPlayView::of(doc.snapshot, &doc.children, runtime_of(cfg, transient), interaction).with_axes(view_state.terminology, view_state.locale);", 2)] + [(f"{w}::engagement(&view, labels)", f"{w}::engagement(&view, labels, view_state.terminology, view_state.locale)") for w in ["shape", "building", "energy", "structure_classic"]] +
        [(f"{w}::render(&view, active_utility, options)", f"{w}::render(&view, active_utility, options, view_state.terminology, view_state.locale)") for w in ["shape", "building", "energy", "structure_classic"]], "cad-editor-callers.patch")

T = ED + "🧪️tests/🔬️unit/"
edit(T + "🖼️rendered-scene-and-manifest/🦀️.rs", [
 ("shape::SURFACE_ID, None, CadDislocateOptions::default())", f"shape::SURFACE_ID, None, CadDislocateOptions::default(), {TERM})"),
 ("shape::engagement(&view, labels)", f"shape::engagement(&view, labels, {TERM})")], "cad-tests-1.patch")
edit(T + "⏪️history-and-convergence/🦀️.rs", [('"cad.play.scene3d/shape", None, CadDislocateOptions::default())', f'"cad.play.scene3d/shape", None, CadDislocateOptions::default(), {TERM})')], "cad-tests-2.patch")
edit(T + "🎚️engagement-commit-and-render-probes/🦀️.rs", [('"cad.play.scene3d/building", None, options)', f'"cad.play.scene3d/building", None, options, {TERM})', 2)], "cad-tests-3.patch")
edit(T + "🌲️forest-mesh-and-schema/🦀️.rs", [('"cad.play.scene3d/building", None, Default::default())', f'"cad.play.scene3d/building", None, Default::default(), {TERM})')], "cad-tests-4.patch")


//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.
use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, FlowExtensionDeclaration, FlowExtensionExecutableIdentity, FlowExtensionManifest, HostMediaHandlerDeclaration, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the procedural 2D and 3D surfaces.
#[allow(unused_doc_comments, unused_qualifications)]
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum ProceduralApps: PluginApp {
        Generation2dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::generation2d::Generation2dPlayApp>>),
        Generation2dViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::generation2d::Generation2dViewer>>),
        Generation3dEditor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::generation3d::Generation3dPlayApp>>),
        Generation3dViewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::generation3d::Generation3dViewer>>),
    }
}
//#endregion 🗃️Apps

//#region 🖼️SemanticUi
fn ui_assembly_error(code: &'static str) -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new(code, "fixed UI admission failed")
}

pub(crate) fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_assembly_error("ui.text"))
}

pub(crate) fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| ui_assembly_error("ui.label"))
}

fn ui_id<B: HasBase>(builder: B, id: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_id(id).map_err(|_| ui_assembly_error("ui.node.id"))
}

fn ui_child<B: HasChildren>(builder: B, child: impl Into<semio_framework_plugin::BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_child(child).map_err(|_| ui_assembly_error("ui.node.child"))
}

fn ui_build<B: Buildable>(builder: B) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    builder.try_build().map_err(|_| ui_assembly_error("ui.node.build"))
}

pub(crate) fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| ui_assembly_error("ui.value.text"))
}

pub(crate) fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| ui_assembly_error("ui.value.list"))?;
    for value in values {
        builder.push(value).map_err(|_| ui_assembly_error("ui.value.list.item"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

pub(crate) fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("ui.value.map"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| ui_assembly_error("ui.value.map.entry"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

pub(crate) fn ui_node_list(
    values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| ui_assembly_error("ui.node-list.item"))?;
    }
    Ok(nodes)
}

/// 🖼️ Encodes one typed scene into the renderer-neutral semantic surface contract.
pub(crate) fn scene_surface<T: ui_wgpu::wgpu::SceneDoc>(id: impl Into<String>, kind: SurfaceKind, scene: &T) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let id = id.into();
    semio_framework_plugin::scene_surface(&id, kind, scene)
}

/// 📖 Renders the shared generation list without routing through Flow's legacy renderer node.
pub(crate) fn generation_tree(
    controller_id: &'static str,
    surface_prefix: &str,
    generation: &flow::playbook::GenerationPlayState,
    locale: ui_wgpu::wgpu::Locale,
    terminology: ui_wgpu::wgpu::Terminology,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let _ = terminology;
    let label = |key: &str| {
        match (key, locale) {
            ("remove", ui_wgpu::wgpu::Locale::De) => "Entfernen",
            ("remove", _) => "Remove",
            ("rename", ui_wgpu::wgpu::Locale::De) => "Umbenennen",
            ("rename", _) => "Rename",
            ("generations", ui_wgpu::wgpu::Locale::De) => "Generierungen",
            ("generations", _) => "Generations",
            ("add", ui_wgpu::wgpu::Locale::De) => "Generierung hinzufügen",
            ("add", _) => "Add Generation",
            ("empty", ui_wgpu::wgpu::Locale::De) => "(keine Generierungen)",
            ("empty", _) => "(no generations)",
            ("actions", ui_wgpu::wgpu::Locale::De) => "Aktionen",
            ("actions", _) => "Actions",
            _ => key,
        }
        .to_string()
    };
    let factory = ActionFactory::new(controller_id);
    let mut items = semio_framework_plugin::UiFixedList::default();
    for entry in &generation.generations {
        let args = ui_value_map([("id", ui_value_text(&entry.id)?)])?;
        let mut item = tree_item_with_action(format!("{surface_prefix}.generation.{}", entry.id), entry.name.clone(), Some(format!("{} values", entry.values.len())), factory.action("selectGeneration", Some(args))?)?;
        if let Component::TreeItem(props) = &mut item.component {
            props.icon = Some(ui_text("layers")?);
            let mut row_actions = semio_framework_plugin::UiFixedList::default();
            let rename_args = ui_value_map([("id", ui_value_text(&entry.id)?), ("name", ui_value_text(format!("{} copy", entry.name))?)])?;
            let (rename_action, rename_args) = factory.action("renameGeneration", Some(rename_args))?;
            row_actions
                .try_push(RowAction {
                    icon: ui_text("pencil")?,
                    label: Some(ui_label(label("rename"))?),
                    action: ActionBinding { trigger: Trigger::Activate, action: rename_action, args: rename_args, capability: None },
                    placement: RowActionPlacement::Menu,
                })
                .map_err(|_| ui_assembly_error("ui.generation.row-actions"))?;
            let remove_args = ui_value_map([("id", ui_value_text(&entry.id)?)])?;
            let (remove_action, remove_args) = factory.action("removeGeneration", Some(remove_args))?;
            row_actions
                .try_push(RowAction {
                    icon: ui_text("trash-2")?,
                    label: Some(ui_label(label("remove"))?),
                    action: ActionBinding { trigger: Trigger::Activate, action: remove_action, args: remove_args, capability: None },
                    placement: RowActionPlacement::Menu,
                })
                .map_err(|_| ui_assembly_error("ui.generation.row-actions"))?;
            props.row_actions = row_actions;
        }
        items.try_push(item).map_err(|_| ui_assembly_error("ui.generation.items"))?;
    }
    PanelTreeBuilder::new(surface_prefix)?
        .section_or_placeholder(format!("{surface_prefix}.generations"), Some(ui_label(label("generations"))?), true, items, label("empty"))?
        .section(format!("{surface_prefix}.actions"), Some(ui_label(label("actions"))?), true, ui_node_list([tree_item_with_action(format!("{surface_prefix}.add-generation"), label("add"), None, factory.action("addGeneration", None)?)])?)?
        .build()
}

fn generation_control_action<B: HasBase>(builder: B, controller_id: &'static str, action: &str, args: semio_framework_plugin::UiValue) -> semio_framework_plugin::UiAssemblyResult<B> {
    let (action, args) = ActionFactory::new(controller_id).action(action, Some(args))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| ui_assembly_error("ui.control.binding")),
        None => builder.try_on(Trigger::Change, action).map_err(|_| ui_assembly_error("ui.control.binding")),
    }
}

fn generation_control_args(generation_id: &str, question_id: &str, field_index: Option<usize>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut values = vec![("generationId", ui_value_text(generation_id)?), ("questionId", ui_value_text(question_id)?)];
    if let Some(field_index) = field_index {
        values.push(("fieldIndex", semio_framework_plugin::UiValue::Number(field_index as f64)));
    }
    ui_value_map(values)
}

/// 📝 Renders generation questions as semantic controls with typed change bindings.
pub(crate) fn generation_form(
    spec: &flow::playbook::PlaybookSpec,
    values: &flow::playbook::PlaybookValues,
    controller_id: &'static str,
    action: &str,
    generation_id: &str,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut root = ui_id(column(), "generate.form")?;
    let mut has_children = false;
    for step in &spec.steps {
        if !step.blocks.is_empty() {
            let heading = ui_build(ui_id(text(ui_label(&step.title)?), format!("generate.step.{}", step.id))?)?;
            root = ui_child(root, heading)?;
            has_children = true;
        }
        for question in &step.blocks {
            if !flow::playbook::is_block_visible(question, values) {
                continue;
            }
            let value = values.get(&question.id).cloned().unwrap_or_else(|| flow::playbook::default_value_for_block(question));
            let field_id = format!("generate.form.{}", question.id);
            let args = || generation_control_args(generation_id, &question.id, None);
            let control = match question.kind.as_str() {
                "text" | "longText" => {
                    let input = input(if question.kind == "longText" { InputKind::LongText } else { InputKind::Text }).value(ui_text(value.as_str().unwrap_or_default())?);
                    ui_build(generation_control_action(ui_id(input, format!("{field_id}.input"))?, controller_id, action, args()?)?)?
                }
                "number" => {
                    let value = value.as_f64().map(|number| number.to_string()).unwrap_or_default();
                    let input = input(InputKind::Number).value(ui_text(value)?);
                    ui_build(generation_control_action(ui_id(input, format!("{field_id}.input"))?, controller_id, action, args()?)?)?
                }
                "slider" => {
                    let slider = slider(value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0))).min(question.min.unwrap_or(0.0)).max(question.max.unwrap_or(100.0)).step(question.step.unwrap_or(1.0));
                    ui_build(generation_control_action(ui_id(slider, format!("{field_id}.slider"))?, controller_id, action, args()?)?)?
                }
                "boolean" => {
                    let toggle = toggle(value.as_bool().unwrap_or(false)).icon(ui_text("toggle-left")?).text(ui_label(&question.label)?);
                    ui_build(generation_control_action(ui_id(toggle, format!("{field_id}.toggle"))?, controller_id, action, args()?)?)?
                }
                "single" => {
                    let mut select = select(ui_text(value.as_str().unwrap_or_default())?);
                    for option in question.options.as_deref().unwrap_or_default() {
                        select = select.try_item(ui_text(&option.value)?, ui_label(&option.label)?).map_err(|_| ui_assembly_error("ui.select.item"))?;
                    }
                    ui_build(generation_control_action(ui_id(select, format!("{field_id}.select"))?, controller_id, action, args()?)?)?
                }
                "vector" => {
                    let numbers = value.as_array().map(<[dsl::DslValue]>::to_vec).unwrap_or_else(|| question.fields.as_deref().unwrap_or_default().iter().map(|field| dsl::DslValue::float(field.value.unwrap_or(0.0))).collect());
                    let labels: Vec<String> = question
                        .fields
                        .as_deref()
                        .map(|fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect())
                        .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());
                    let mut vector = ui_id(column(), format!("{field_id}.vector"))?;
                    for (index, number) in numbers.iter().enumerate() {
                        let input = input(InputKind::Number).value(ui_text(number.as_f64().map(|entry| entry.to_string()).unwrap_or_default())?);
                        let input = ui_id(input, format!("{field_id}.vector.{index}.input"))?;
                        let input = ui_build(generation_control_action(input, controller_id, action, generation_control_args(generation_id, &question.id, Some(index))?)?)?;
                        let label = labels.get(index).cloned().unwrap_or_else(|| format!("Field {}", index + 1));
                        let field = ui_id(field(ui_label(label)?), format!("{field_id}.vector.{index}"))?;
                        vector = ui_child(vector, ui_build(ui_child(field, input)?)?)?;
                    }
                    ui_build(vector)?
                }
                "note" => ui_build(ui_id(text(ui_label(question.text.clone().unwrap_or_default())?), format!("{field_id}.note"))?)?,
                "image" => ui_build(ui_id(text(ui_label(question.src.clone().unwrap_or_else(|| "(no image)".into()))?), format!("{field_id}.image"))?)?,
                _ => {
                    let input = input(InputKind::Text).value(ui_text(serde_json::Value::from(&value).to_string())?);
                    ui_build(generation_control_action(ui_id(input, format!("{field_id}.input"))?, controller_id, action, args()?)?)?
                }
            };
            let field = ui_id(field(ui_label(&question.label)?), field_id)?;
            root = ui_child(root, ui_build(ui_child(field, control)?)?)?;
            has_children = true;
        }
    }
    if !has_children {
        return ui_build(text(ui_label("No input widgets to generate from.")?));
    }
    ui_build(root)
}
//#endregion 🖼️SemanticUi

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin<ProceduralApps>, semio_framework_plugin::PluginAssemblyError> {
    crate::artifacts::assembly::standards::v1::subsets::any::schema::inferences::register_assembly_inference_factory(&semio_framework::ActionBus::production())
        .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("assembly-inference-factory", error.to_string()))?;
    Plugin::<ProceduralApps>::builder("procedural")
        .label("Procedural")
        .version("0.1.0")
        .routed_inference(crate::artifacts::assembly::standards::v1::subsets::any::schema::inferences::assembly_inference_metadata())
        .artifact(crate::artifacts::generation2d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::generation3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::mesh_dwg_bridge(
            "s.procedural.host-media.mesh-dwg",
            crate::artifacts::generation3d::artifact_kind(),
            crate::artifacts::generation3d::GENERATION_3D_SCHEMA,
            crate::editor::generation3d::generation3d_document_from_mesh,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.brep",
            FlowExtensionManifest::new("brep", "Brep", "0.3.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.brep", "semio.s.plugin.flow.extension.brep", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.math",
            FlowExtensionManifest::new("math", "Math", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.math", "semio.s.plugin.flow.extension.math", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.primitive",
            FlowExtensionManifest::new("core", "Core", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.primitive", "semio.s.plugin.flow.extension.primitive", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.logic",
            FlowExtensionManifest::new("logic", "Logic", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.logic", "semio.s.plugin.flow.extension.logic", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.dictionary",
            FlowExtensionManifest::new("dictionary", "Dictionary", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.dictionary", "semio.s.plugin.flow.extension.dictionary", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.list",
            FlowExtensionManifest::new("list", "List", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.list", "semio.s.plugin.flow.extension.list", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.text",
            FlowExtensionManifest::new("text", "Text", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.text", "semio.s.plugin.flow.extension.text", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.draw",
            FlowExtensionManifest::new("draw", "Draw", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.draw", "semio.s.plugin.flow.extension.draw", "register")?,
        )?)
        .flow_extension(FlowExtensionDeclaration::new(
            "s.procedural.flow-extension.bim",
            FlowExtensionManifest::new("bim", "Bim", "0.1.0")?,
            FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.bim", "semio.s.plugin.flow.extension.bim", "register")?,
        )?)
        .editor::<crate::editor::generation2d::Generation2dPlayApp>(crate::editor::generation2d::create_generation2d_app())
        .editor_mutation_roster::<crate::editor::generation2d::Generation2dPlayApp>()
        .viewer::<crate::viewer::generation2d::Generation2dViewer>(crate::viewer::generation2d::create_generation2d_viewer())
        .viewer_mutation_roster::<crate::viewer::generation2d::Generation2dViewer>()
        .editor::<crate::editor::generation3d::Generation3dPlayApp>(crate::editor::generation3d::create_generation3d_app())
        .editor_mutation_roster::<crate::editor::generation3d::Generation3dPlayApp>()
        .viewer::<crate::viewer::generation3d::Generation3dViewer>(crate::viewer::generation3d::create_generation3d_viewer())
        .viewer_mutation_roster::<crate::viewer::generation3d::Generation3dViewer>()
        // 🚧️ assembly's editor/viewer are authored (`🗿️artifacts/🧩️assembly/…/{✏️editor,👁️viewer}/`) but
        // not yet mounted in `🦀️.rs` or registered here: `ArtifactEditor`/`ArtifactViewer`'s own
        // trait bounds (`Snapshot: ArtifactDsl + ArtifactPack`, `Mutation`/`Command`: `OpText`/`OpBinary`)
        // are unsatisfied until assembly's schema gains its missing artifact-facet descriptor + leaf
        // set — see `📓️w2-p5-assembly-notes.md`. Wire once that lands.
        //
        // 🧬️ Assembly's editor remains unmounted, but its schema-owned `semio.infer` WFC factory
        // is registered above on the production action bus and needs no artifact surface.
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::generation2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::generation3d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("documents.write".into()),
            scope: "plugin".into(),
            reason: "persist generation2d/generation3d editor edits (flow graph parameter/node changes) to the open document".into(),
            optional: false,
        })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use crate::editor::generation2d::Generation2dPlayApp;
    use crate::editor::generation3d::Generation3dPlayApp;
    use crate::viewer::generation2d::Generation2dViewer;
    use crate::viewer::generation3d::Generation3dViewer;

    #[test]
    fn plugin_manifest_builds_synchronously() {
        super::plugin().expect("procedural plugin manifest should build synchronously");
    }

    /// 👁️ A viewer instance never mutates the document store, even when dispatched.
    #[test]
    fn generation2d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<Generation2dViewer>();
    }
    #[test]
    fn generation3d_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<Generation3dViewer>();
    }

    /// 🤝️ Editor and viewer surfaces agree on the artifact dialect they address.
    #[test]
    fn generation2d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<Generation2dPlayApp, Generation2dViewer>();
    }
    #[test]
    fn generation3d_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<Generation3dPlayApp, Generation3dViewer>();
    }
}
//#endregion 🧪️SurfaceTests

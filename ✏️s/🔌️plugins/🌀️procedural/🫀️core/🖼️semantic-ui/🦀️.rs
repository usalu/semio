//! 🖼️ Shared semantic UI builders for procedural generation surfaces.

use semio_framework_plugin::plugin_app_close_prelude::*;

fn ui_assembly_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "fixed UI admission failed")
}

pub(crate) fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_assembly_error("ui.text"))
}

pub(crate) fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> {
    Label::try_from(value.as_ref()).map_err(|_| ui_assembly_error("ui.label"))
}

fn ui_id<B: HasBase>(builder: B, id: impl AsRef<str>) -> UiAssemblyResult<B> {
    builder.try_id(id).map_err(|_| ui_assembly_error("ui.node.id"))
}

fn ui_child<B: HasChildren>(builder: B, child: impl Into<BuiltNode>) -> UiAssemblyResult<B> {
    builder.try_child(child).map_err(|_| ui_assembly_error("ui.node.child"))
}

fn ui_build<B: Buildable>(builder: B) -> UiAssemblyResult<BuiltNode> {
    builder.try_build().map_err(|_| ui_assembly_error("ui.node.build"))
}

pub(crate) fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value.as_ref()).map(UiValue::Text).ok_or_else(|| ui_assembly_error("ui.value.text"))
}

pub(crate) fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut builder = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("ui.value.map"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| ui_assembly_error("ui.value.map.entry"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

pub(crate) fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut nodes = UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| ui_assembly_error("ui.node-list.item"))?;
    }
    Ok(nodes)
}

/// 🖼️ Encodes one typed scene into the renderer-neutral semantic surface contract.
pub(crate) fn scene_surface<T: semio_framework_ui::wgpu::SceneDoc>(id: impl Into<String>, kind: SurfaceKind, scene: &T) -> UiAssemblyResult<BuiltNode> {
    let id = id.into();
    semio_framework_plugin::scene_surface(&id, kind, scene)
}

/// 📖 Renders the shared generation list without routing through Flow's legacy renderer node.
pub(crate) fn generation_tree(controller_id: &'static str, surface_prefix: &str, generation: &semio_framework_artifact_playbook_playbook::GenerationPlayState, locale: Locale, terminology: Terminology) -> UiAssemblyResult<BuiltNode> {
    let _ = terminology;
    let label = |key: &str| {
        match (key, locale) {
            ("remove", Locale::De) => "Entfernen",
            ("remove", _) => "Remove",
            ("rename", Locale::De) => "Umbenennen",
            ("rename", _) => "Rename",
            ("generations", Locale::De) => "Generierungen",
            ("generations", _) => "Generations",
            ("add", Locale::De) => "Generierung hinzufügen",
            ("add", _) => "Add Generation",
            ("empty", Locale::De) => "(keine Generierungen)",
            ("empty", _) => "(no generations)",
            ("actions", Locale::De) => "Aktionen",
            ("actions", _) => "Actions",
            _ => key,
        }
        .to_string()
    };
    let factory = ActionFactory::new(controller_id);
    let mut items = UiFixedList::default();
    for entry in &generation.generations {
        let args = ui_value_map([("id", ui_value_text(&entry.id)?)])?;
        let mut item = tree_item_with_action(format!("{surface_prefix}.generation.{}", entry.id), entry.name.clone(), Some(format!("{} values", entry.values.len())), factory.action("selectGeneration", Some(args))?)?;
        if let Component::TreeItem(props) = &mut item.component {
            props.icon = Some(ui_text("layers")?);
            let mut row_actions = UiFixedList::default();
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

fn generation_control_action<B: HasBase>(builder: B, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    let (action, args) = ActionFactory::new(controller_id).action(action, Some(args))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| ui_assembly_error("ui.control.binding")),
        None => builder.try_on(Trigger::Change, action).map_err(|_| ui_assembly_error("ui.control.binding")),
    }
}

fn generation_control_args(generation_id: &str, question_id: &str, field_index: Option<usize>) -> UiAssemblyResult<UiValue> {
    let mut values = vec![("generationId", ui_value_text(generation_id)?), ("questionId", ui_value_text(question_id)?)];
    if let Some(field_index) = field_index {
        values.push(("fieldIndex", UiValue::Number(field_index as f64)));
    }
    ui_value_map(values)
}

/// 📝 Renders generation questions as semantic controls with typed change bindings.
pub(crate) fn generation_form(
    spec: &semio_framework_artifact_playbook_playbook::PlaybookSpec,
    values: &semio_framework_artifact_playbook_playbook::PlaybookValues,
    controller_id: &'static str,
    action: &str,
    generation_id: &str,
) -> UiAssemblyResult<BuiltNode> {
    let mut root = ui_id(column(), "generate.form")?;
    let mut has_children = false;
    for step in &spec.steps {
        if !step.blocks.is_empty() {
            let heading = ui_build(ui_id(text(ui_label(&step.title)?), format!("generate.step.{}", step.id))?)?;
            root = ui_child(root, heading)?;
            has_children = true;
        }
        for question in &step.blocks {
            if !semio_framework_artifact_playbook_playbook::is_block_visible(question, values) {
                continue;
            }
            let value = values.get(&question.id).cloned().unwrap_or_else(|| semio_framework_artifact_playbook_playbook::default_value_for_block(question));
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
                    let numbers = value.as_array().map_or_else(|| question.fields.as_deref().unwrap_or_default().iter().map(|field| DslValue::float(field.value.unwrap_or(0.0))).collect(), <[DslValue]>::to_vec);
                    let labels: Vec<String> = question
                        .fields
                        .as_deref().map_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect(), |fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect());
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
                    let input = input(InputKind::Text).value(ui_text(Value::from(&value).to_string())?);
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

//#region 🔖️WindowActionLaw
/// 📇️ Collects every action id a projected `UiNode` tree emits — the law shared by generation3d's and
/// generation2d's `every_emitted_action_is_declared_on_its_window_kind` tests.
///
/// Walks the whole projection rather than only `BuiltNode.bindings`, because a tree row's menu
/// actions (`Component::TreeItem.row_actions`, `generation_tree` above) live inside the serialized
/// `component` and never reach `bindings`. Any object carrying `scope`/`name`/`version` is an
/// [`semio_framework_plugin::plugin_app_close_prelude::ActionId`] wherever it appears.
///
/// The gate this feeds is `ShellHost`'s `declaredAction` check
/// (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:5691`).
/// Shared by BOTH artifact crates through this file's `#[path]` mount, so each app uses only the half
/// its own law needs — `#[allow(dead_code)]` is that asymmetry, not an unused helper.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn emitted_action_ids(projection: &str) -> std::collections::BTreeSet<String> {
    fn walk(value: &serde_json::Value, found: &mut std::collections::BTreeSet<String>) {
        match value {
            serde_json::Value::Object(entries) => {
                if entries.contains_key("scope") && entries.contains_key("version") {
                    if let Some(name) = entries.get("name").and_then(serde_json::Value::as_str) {
                        found.insert(name.to_owned());
                    }
                }
                for entry in entries.values() {
                    walk(entry, found);
                }
            }
            serde_json::Value::Array(entries) => {
                for entry in entries {
                    walk(entry, found);
                }
            }
            _ => {}
        }
    }
    let mut found = std::collections::BTreeSet::new();
    walk(&serde_json::from_str(projection).expect("projection json"), &mut found);
    found
}

/// 🗂️ Seeds a two-entry generation roster with the first selected — the only state under which
/// `generation_tree` emits `selectGeneration`/`renameGeneration`/`removeGeneration` and
/// `generation_form` emits `updateGenerationValues`, so every window-action law test renders against
/// it rather than against the bundled examples' empty roster.
///
/// Mutates the root in place through `cold_builder_mut`: a `GenerationPlayRoot` is an `Arc` newtype
/// whose `Drop` panics on a nonempty unretired root, so a law test must never assign a fresh one over
/// an existing one.
/// Shared by BOTH artifact crates through this file's `#[path]` mount, so each app uses only the half
/// its own law needs — `#[allow(dead_code)]` is that asymmetry, not an unused helper.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn seed_law_generations(root: &mut semio_framework_artifact_playbook_playbook::GenerationPlayRoot) {
    let entry = |id: &str, name: &str| semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() };
    let state = root.cold_builder_mut().expect("unique cold generation owner");
    state.generations.clear();
    state.generations.push(entry("generation-1", "Generation 1"));
    state.generations.push(entry("generation-2", "Generation 2"));
    state.selected_generation_id = Some("generation-1".into());
}

/// 🎛️ Collects every action id a window kind's `WindowMeasure` chrome dispatches — the measure half of
/// the same law. `ActionDescriptor.action` is a plain `action` string field, so the walk keys on that
/// rather than on the `scope`/`name`/`version` triple `emitted_action_ids` looks for.
/// Shared by BOTH artifact crates through this file's `#[path]` mount, so each app uses only the half
/// its own law needs — `#[allow(dead_code)]` is that asymmetry, not an unused helper.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn measure_action_ids(measures: &[semio_framework_plugin::WindowMeasure]) -> std::collections::BTreeSet<String> {
    fn walk(value: &serde_json::Value, found: &mut std::collections::BTreeSet<String>) {
        match value {
            serde_json::Value::Object(entries) => {
                if let Some(name) = entries.get("action").and_then(serde_json::Value::as_str) {
                    found.insert(name.to_owned());
                }
                for entry in entries.values() {
                    walk(entry, found);
                }
            }
            serde_json::Value::Array(entries) => {
                for entry in entries {
                    walk(entry, found);
                }
            }
            _ => {}
        }
    }
    let mut found = std::collections::BTreeSet::new();
    walk(&serde_json::to_value(measures).expect("measures json"), &mut found);
    found
}
//#endregion 🔖️WindowActionLaw

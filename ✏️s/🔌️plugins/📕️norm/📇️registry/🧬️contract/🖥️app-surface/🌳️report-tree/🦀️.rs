//! 🌳️ Text helpers of the norm app surface: the roster example and the UI admission helpers. The check list lives in the checks window. See [`super`].

use super::*;

//#region 🔖️Examples
/// 🎨️ Resolves `setActiveExample` through the editor's OWN example roster — the list its navbar picker
/// offers — so every example the picker shows loads, and nothing outside it does. An empty id is the
/// empty document, an id outside the roster is `None` (the no-op a picker's stale selection needs),
/// and an example body that does not parse is a named fault, never a silent fallback.
pub fn roster_example_snapshot<D: pack::snapshot::ArtifactDsl + Default>(examples: Vec<ExampleSource>, example_id: &str) -> Result<Option<D>, Fault> {
    let id = example_id.trim();
    if id.is_empty() {
        return Ok(Some(D::default()));
    }
    let Some(example) = examples.into_iter().find(|example| example.id() == id) else {
        return Ok(None);
    };
    D::parse_dsl(&example.document())
        .map(Some)
        .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.set-active-example-invalid"), format!("example '{id}' does not parse: {error:?}")))
}
//#endregion 🔖️Examples

//#region 🔖️Render
pub fn render_text(value: impl Into<String>) -> UiAssemblyResult<BuiltNode> {
    let label = ui::Label::try_from(value.into()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI label admission failed"))?;
    ui::text(label).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text build failed"))
}

pub fn norm_ui_label(value: impl Into<String>) -> UiAssemblyResult<ui::Label> {
    ui::Label::try_from(value.into()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI label admission failed"))
}

pub fn render_text_chunks(value: &str) -> UiAssemblyResult<BuiltNode> {
    if value.len() <= ui::UI_TEXT_MAX_BYTES {
        return render_text(value);
    }
    let mut rest = value;
    let mut children = Vec::new();
    while !rest.is_empty() {
        let mut end = rest.len().min(ui::UI_TEXT_MAX_BYTES);
        while !rest.is_char_boundary(end) {
            end -= 1;
        }
        let label = ui::Label::try_from(&rest[..end]).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk admission failed"))?;
        let builder = ui::text(label).try_id(format!("norm-text-chunk-{}", children.len())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk id admission failed"))?;
        children.push(builder.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm UI text chunk build failed"))?);
        rest = &rest[end..];
    }
    ui::column().try_children(children).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm text chunk admission failed"))?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "norm text chunk build failed"))
}

pub fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "norm UI admission failed / Zulassung der Norm-Oberfläche fehlgeschlagen")
}

pub fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_error("ui.text"))
}

pub fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    ui_text(value).map(UiValue::Text)
}

pub fn action_args_map(entries: Vec<(&str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    for (key, value) in entries {
        args.push(key.into(), value).map_err(|_| ui_error("ui.value.map.entry"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

pub fn norm_action(controller_id: &'static str, action: &str, args: Option<UiValue>) -> UiAssemblyResult<(semio_framework_plugin::ActionId, Option<UiValue>)> {
    ActionFactory::new(controller_id).action(action, args)
}

pub fn bind_change<B: HasBase>(builder: B, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    bind_on(builder, Trigger::Change, controller_id, action, args)
}

pub fn bind_on<B: HasBase>(builder: B, trigger: Trigger, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    let (action, args) = norm_action(controller_id, action, Some(args))?;
    match args {
        Some(args) => builder.try_on_with(trigger, action, args).map_err(|_| ui_error("ui.control.binding")),
        None => builder.try_on(trigger, action).map_err(|_| ui_error("ui.control.binding")),
    }
}

pub fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let row = ui::tree_item(norm_ui_label(label)?).try_id(row_id).map_err(|_| ui_error("ui.node.id"))?;
    row.try_child(control).map_err(|_| ui_error("ui.node.child"))?.try_build().map_err(|_| ui_error("ui.node.build"))
}

//#endregion 🔖️Render

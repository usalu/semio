//! 🖼️ Shared semantic UI builders for procedural generation surfaces.
#![allow(clippy::result_large_err)]

use semio_plugin::Axis;
use semio_plugin::plugin_app_close_prelude::{form, *};
use semio_ui_locale::{Locale, Terminology};

fn ui_assembly_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "fixed UI admission failed")
}

pub fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_assembly_error("ui.text"))
}

pub fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> {
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

pub fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value.as_ref()).map(UiValue::Text).ok_or_else(|| ui_assembly_error("ui.value.text"))
}

pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut builder = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("ui.value.map"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| ui_assembly_error("ui.value.map.entry"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

/// 🖲️ Carries a retained graph hover target without embedding it in JSON text.
pub fn graph_hover_args(target_id: &str, granularity: &str) -> UiAssemblyResult<UiValue> {
    ui_value_map([
        ("channel", ui_value_text("pointer")?),
        ("domainId", ui_value_text("graph")?),
        ("targetId", ui_value_text(target_id)?),
        ("granularityId", ui_value_text(granularity)?),
    ])
}

/// 🖼️ Encodes one typed scene into the renderer-neutral semantic surface contract under its required localized accessible `name`.
pub fn scene_surface<T: semio_ui_declaration::ui::SceneDoc>(id: impl Into<String>, kind: SurfaceKind, scene: &T, name: &str, description: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let id = id.into();
    semio_plugin::scene_surface(&id, kind, scene, name, description)
}

/// ♿️ [`scene_surface`] plus the accessible name, description and liveness a canvas owes a reader.
///
/// A scene surface paints into a texture with no accessible children, so the `AccessibilitySpec` on
/// its own record is the ONLY thing an assistive technology ever learns about it — without this the
/// canvas is an anonymous `application` region that nothing can name, reach or hear. `live` makes the
/// surface a live region so an evaluation advancing `Computing → Evaluated` is announced without the
/// reader having to go find it.
///
/// The description is [`UiText::clipped`] display copy: a description that failed admission would
/// fail the whole refresh, and its full text is always reachable in the window's own outline.
///
/// @see ../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs
pub fn accessible_scene_surface<T: semio_ui_declaration::ui::SceneDoc>(
    id: impl Into<String>,
    kind: SurfaceKind,
    scene: &T,
    label: &str,
    description: &str,
    live: Liveness,
) -> UiAssemblyResult<BuiltNode> {
    let mut node = scene_surface(id, kind, scene, label, None)?;
    node.accessibility.description = Some(Label(UiText::clipped(description)));
    node.accessibility.live = live;
    Ok(node)
}

/// ⏎️ Gives a scene surface the `Trigger::Activate` binding its `role="application"` promises.
///
/// A canvas that announces itself as an application is telling assistive technology "I handle my own
/// Enter/Space" — and the React interpreter's `SurfaceAccessibilityShell` only installs its
/// `onKeyDown` when the record actually carries an activate binding, so without this the promise is
/// unkept and Enter does nothing. `shortcut` is what reaches the reader as `aria-keyshortcuts`, which
/// is the only way a user who cannot see the canvas learns the chord exists.
///
/// @see ../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx
pub fn activatable_scene_surface(mut node: BuiltNode, controller_id: &'static str, action: &str, shortcut: &str) -> UiAssemblyResult<BuiltNode> {
    let (action_id, _) = ActionFactory::new(controller_id).action(action, None)?;
    node.bindings
        .try_push(ActionBinding { trigger: Trigger::Activate, action: action_id, args: None, capability: None })
        .map_err(|_| ui_assembly_error("ui.surface.activate-binding"))?;
    node.accessibility.shortcut = UiText::try_from_str(shortcut);
    Ok(node)
}

/// 🖊️ The SELECTED generation's inline name editor: a text `input` seeded with the current name
/// whose `Trigger::Commit` (Enter / blur — `InputProps::commit == "blur"`) dispatches
/// `renameGeneration{id}`, the typed text arriving as the framework's scalar `value` argument
/// (`uiIntentPayload`,
/// `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`).
///
/// 🐛️ Why an editor and not the old row action: rename used to be a `RowActionPlacement::Menu`
/// affordance — right-click only — dispatching a HARDCODED `"{name} copy"`, so a user could reach the
/// verb but never choose a name (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #7).
fn generation_rename_field(factory: &ActionFactory, surface_prefix: &str, id: &str, name: &str, placeholder: &str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = factory.action("renameGeneration", Some(ui_value_map([("id", ui_value_text(id)?)])?))?;
    let editor = input(InputKind::Text).value(ui_text(name)?).placeholder(ui_label(placeholder)?).commit(ui_text("blur")?);
    let editor = ui_id(editor, format!("{surface_prefix}.generation.{id}.rename"))?.try_label(placeholder).map_err(|_| ui_assembly_error("ui.generation.rename.label"))?;
    let editor = match args {
        Some(args) => editor.try_on_with(Trigger::Commit, action, args).map_err(|_| ui_assembly_error("ui.generation.rename.binding"))?,
        None => editor.try_on(Trigger::Commit, action).map_err(|_| ui_assembly_error("ui.generation.rename.binding"))?,
    };
    ui_build(editor)
}

/// 📖 Renders the shared generation list without routing through Flow's legacy renderer node.
///
/// 🖱️ The three row verbs a user meets, in reach order: the row itself activates
/// `selectGeneration{id}`; the selected row carries [`generation_rename_field`]'s inline name editor
/// (`renameGeneration`); every row carries a `RowActionPlacement::Row` delete button
/// (`removeGeneration{id}`) painted ON the row rather than folded into its right-click menu, which is
/// what `RowActionPlacement::Menu` meant (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`'s
/// `mergeTreeRowContextMenu`).
///
/// 🪟️ The roster is WINDOWED, not paged: the generations container states its full `total` and
/// materialises only the rows the host's viewport asked for, so a document with hundreds of saved
/// generations scrolls instead of refusing at the fixed-list ceiling — and, because every row costs
/// three `UiValue` argument maps (select + rename + remove), the window is also what keeps the shared
/// argument arena affordable (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
pub fn generation_tree(
    controller_id: &'static str,
    surface_prefix: &str,
    generation: &semio_os_playbook_contract::GenerationPlayState,
    selected_id: Option<&str>,
    locale: Locale,
    terminology: Terminology,
    windows: &TreeWindows<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let _ = terminology;
    let label = |key: &str| {
        match (key, locale) {
            ("remove", Locale::De) => "Entfernen",
            ("remove", _) => "Remove",
            ("rename", Locale::De) => "Generierung umbenennen",
            ("rename", _) => "Rename generation",
            ("generations", Locale::De) => "Generierungen",
            ("generations", _) => "Generations",
            ("add", Locale::De) => "Generierung hinzufügen",
            ("add", _) => "Add Generation",
            ("empty", Locale::De) => "(keine Generierungen)",
            ("empty", _) => "(no generations)",
            ("actions", Locale::De) => "Aktionen",
            ("actions", _) => "Actions",
            ("values", Locale::De) => "Werte",
            ("values", _) => "values",
            _ => key,
        }
        .to_string()
    };
    let factory = ActionFactory::new(controller_id);
    let generation_row = |entry: &semio_os_playbook_contract::FormGeneration| -> UiAssemblyResult<BuiltNode> {
        let args = ui_value_map([("id", ui_value_text(&entry.id)?)])?;
        let mut item = tree_item_with_action(format!("{surface_prefix}.generation.{}", entry.id), entry.name.clone(), Some(format!("{} {}", entry.values.len(), label("values"))), factory.action("selectGeneration", Some(args))?)?;
        if let Component::TreeItem(props) = &mut item.component {
            props.icon = Some(ui_text("layers")?);
            let mut row_actions = UiFixedList::default();
            row_actions.try_push(row_action("trash-2", &label("remove"), "removeGeneration", RowActionPlacement::Row)?).map_err(|_| ui_assembly_error("ui.generation.row-actions"))?;
            props.row_actions = row_actions;
        }
        if selected_id == Some(entry.id.as_str()) {
            item = item.try_with_children([generation_rename_field(&factory, surface_prefix, &entry.id, &entry.name, &label("rename"))?]).map_err(|_| ui_assembly_error("ui.generation.rename"))?;
        }
        Ok(item)
    };
    let generations_id = format!("{surface_prefix}.generations");
    PanelTreeBuilder::new(surface_prefix)?
        .window_section_or_placeholder(windows, &generations_id, Some(ui_label(label("generations"))?), true, &generation.generations, generation_row, ui_label(label("empty"))?)?
        .section(format!("{surface_prefix}.actions"), Some(ui_label(label("actions"))?), true, ui_node_list([tree_item_with_action(format!("{surface_prefix}.add-generation"), label("add"), None, factory.action("addGeneration", None)?)])?)?
        .build()
}

fn generation_control_action<B: HasBase>(builder: B, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    generation_control_binding(builder, Trigger::Change, controller_id, action, args)
}

/// ⌨️ A typed answer commits once, on blur or Enter (`commit("blur")` + `Trigger::Commit`): one edit per answer, never one
/// per keystroke (design §13.2, short single-line and long text fields).
fn generation_text_action<B: HasBase>(builder: B, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    generation_control_binding(builder, Trigger::Commit, controller_id, action, args)
}

fn generation_control_binding<B: HasBase>(builder: B, trigger: Trigger, controller_id: &'static str, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    let (action, args) = ActionFactory::new(controller_id).action(action, Some(args))?;
    match args {
        Some(args) => builder.try_on_with(trigger, action, args).map_err(|_| ui_assembly_error("ui.control.binding")),
        None => builder.try_on(trigger, action).map_err(|_| ui_assembly_error("ui.control.binding")),
    }
}

fn generation_control_args(generation_id: &str, question_id: &str, field_index: Option<usize>) -> UiAssemblyResult<UiValue> {
    let mut values = vec![("generationId", ui_value_text(generation_id)?), ("questionId", ui_value_text(question_id)?)];
    if let Some(field_index) = field_index {
        values.push(("fieldIndex", UiValue::Number(field_index as f64)));
    }
    ui_value_map(values)
}

/// ⌨️ Publishes an editable text draft with its exact retained body when it outgrows an inline field.
pub fn retained_text_control(
    id: &str,
    label: &str,
    kind: InputKind,
    value: &str,
    controller: &'static str,
    action: &str,
    args: UiValue,
) -> UiAssemblyResult<BuiltNode> {
    let inline = UiText::try_from_str(value);
    let key = inline.is_none().then(|| semio_plugin::input_value_body_key(value));
    let authored = match &key { Some(key) => ui_text(key)?, None => inline.unwrap_or_default() };
    let control = input(kind).value(authored).commit(ui_text(if key.is_some() { "blurRetainedBody" } else { "blurRetained" })?);
    let control = ui_id(control, id)?.try_label(label).map_err(|_| ui_assembly_error("ui.input.label"))?;
    let control = ui_build(generation_text_action(control, controller, action, args)?)?;
    if let Some(key) = key {
        let body = semio_plugin::paged_text_carrier(&key, value)?;
        control.try_with_children([body]).map_err(|_| ui_assembly_error("ui.input.value"))
    } else {
        Ok(control)
    }
}

/// 🏷️ The generated form and its control labels, resolved by the calling app.
pub struct GenerationFormLabels<'a> {
    pub title: &'a str,
    pub no_inputs: &'a str,
    pub field: &'a str,
    pub no_image: &'a str,
}

fn generation_vector_axes(question: &semio_os_playbook_contract::PlaybookBlock, value: Option<&DslValue>) -> usize {
    question.fields.as_deref().filter(|axes| !axes.is_empty()).map_or_else(
        || value.and_then(DslValue::as_array).map_or(0, |axes| axes.len()),
        |axes| axes.len(),
    )
}

fn generation_question(
    question: &semio_os_playbook_contract::PlaybookBlock,
    values: &semio_os_playbook_contract::PlaybookValues,
    controller_id: &'static str,
    action: &str,
    generation_id: &str,
    words: &GenerationFormLabels<'_>,
    resources: &semio_plugin::WindowResourceRequests<'_>,
) -> UiAssemblyResult<BuiltNode> {
    let value = values.get(&question.id).cloned().unwrap_or_else(|| semio_os_playbook_contract::default_value_for_block(question));
    let field_id = format!("generate.form.{}", question.id);
    let control_id = format!("{field_id}.control");
    let args = || generation_control_args(generation_id, &question.id, None);
    let mut control = match question.kind.as_str() {
        "text" | "longText" => {
            let value = value.as_str().unwrap_or_default();
            retained_text_control(&control_id, &question.label, if question.kind == "longText" { InputKind::LongText } else { InputKind::Text }, value, controller_id, action, args()?)?
        }
        "number" => {
            let input = input(InputKind::Number).number(value.as_f64().unwrap_or(0.0)).commit(ui_text("blur")?);
            ui_build(generation_text_action(ui_id(input, &control_id)?, controller_id, action, args()?)?)?
        }
        "slider" => {
            let slider = slider(value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0))).min(question.min.unwrap_or(0.0)).max(question.max.unwrap_or(100.0)).step(question.step.unwrap_or(1.0));
            ui_build(generation_control_action(ui_id(slider, &control_id)?, controller_id, action, args()?)?)?
        }
        "boolean" => {
            let toggle = toggle(value.as_bool().unwrap_or(false)).icon(ui_text("toggle-left")?).text(ui_label(&question.label)?);
            ui_build(generation_control_action(ui_id(toggle, &control_id)?, controller_id, action, args()?)?)?
        }
        "single" => {
            let mut select = select(ui_text(value.as_str().unwrap_or_default())?);
            for option in question.options.as_deref().unwrap_or_default() {
                select = select.try_item(ui_text(&option.value)?, ui_label(&option.label)?).map_err(|_| ui_assembly_error("ui.select.item"))?;
            }
            ui_build(generation_control_action(ui_id(select, &control_id)?, controller_id, action, args()?)?)?
        }
        "vector" => {
            let mut vector = vector_input(ui_label(&question.label)?)
                .commit(ui_text("blur")?).try_id(&field_id).map_err(|_| ui_assembly_error("ui.vector.id"))?;
            let axis_values = value.as_array().unwrap_or_default();
            let axes = question.fields.as_deref().unwrap_or_default();
            for index in 0..generation_vector_axes(question, Some(&value)) {
                let field = axes.get(index);
                let axis = field.map(|entry| entry.label.as_deref().unwrap_or(&entry.key).to_owned())
                    .unwrap_or_else(|| format!("{} {}", words.field, index + 1));
                let number = axis_values.get(index).and_then(DslValue::as_f64).or_else(|| field.and_then(|entry| entry.value)).unwrap_or(0.0);
                let (action, args) = ActionFactory::new(controller_id).action(action, Some(generation_control_args(generation_id, &question.id, Some(index))?))?;
                let args = args.ok_or_else(|| ui_assembly_error("ui.vector.args"))?;
                vector = vector.try_axis(&format!("{field_id}.vector.{index}.input"), ui_label(axis)?, number, |input| input.try_on_with(Trigger::Commit, action, args).map_err(|(input, _)| input))
                    .map_err(|_| ui_assembly_error("ui.vector.axis"))?;
            }
            return ui_build(vector);
        }
        "note" => ui_build(ui_id(text(ui_label(question.text.clone().unwrap_or_default())?), format!("{field_id}.note"))?)?,
        "image" => {
            let id = format!("{field_id}.image");
            match question.src.as_deref().map(str::trim).filter(|src| !src.is_empty()) {
                Some(src) => {
                    let alt = question.description.as_deref().map(str::trim).filter(|alt| !alt.is_empty()).unwrap_or(&question.label);
                    resources.image(&id, src, ui_label(alt)?)?
                }
                None => ui_build(ui_id(text(ui_label(words.no_image)?), id)?)?,
            }
        }
        _ => {
            let input = input(InputKind::Text).value(ui_text(Value::from(&value).to_string())?).commit(ui_text("blur")?);
            ui_build(generation_text_action(ui_id(input, &control_id)?, controller_id, action, args()?)?)?
        }
    };
    let label = ui_label(&question.label)?;
    if control.key.as_str() == control_id {
        control.accessibility.label = Some(label.clone());
    }
    let field = ui_id(field(label), field_id)?;
    ui_build(ui_child(field, control)?)
}

/// 📝️ The visible semantic form and its bounded location in the logical question sequence.
pub struct GenerationFormPage {
    pub body: BuiltNode,
    pub index: usize,
    pub total: usize,
}

impl GenerationFormPage {
    /// 📬️ Reads the explicit request for one page from the addressed window's view context.
    pub fn requested(view: &ViewModel, body_key: &str, window_id: &str) -> usize {
        view.tree_windows.iter()
            .find(|request| request.body_key == body_key && request.node_key == window_id && request.open.is_none() && request.rows == 1)
            .map_or(0, |request| request.offset as usize)
    }

    /// 🧭️ Presents named pagination controls and a separate viewport for the current fields.
    pub fn present(self, window_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
        let mut body = self.body;
        if self.total == 1 {
            return Ok(body);
        }
        let current = self.index + 1;
        let (previous, next, position) = match locale {
            Locale::En => ("Previous page", "Next page", format!("Page {current} of {}", self.total)),
            Locale::De => ("Vorherige Seite", "Nächste Seite", format!("Seite {current} von {}", self.total)),
        };
        let button = |key, label, icon, page: Option<usize>| -> UiAssemblyResult<BuiltNode> {
            let button = ui_id(button(ui_label(label)?).icon(ui_text(icon)?).disabled(page.is_none()), key)?;
            if let Some(page) = page {
                let args = ui_value_map([("windowId", ui_value_text(window_id)?), ("page", UiValue::Number(page as f64))])?;
                ui_build(generation_control_binding(button, Trigger::Activate, "framework", "setFormPage", args)?)
            } else {
                ui_build(button)
            }
        };
        let mut position = ui_build(ui_id(text(ui_label(position)?), "generate.form.page.position")?)?;
        position.accessibility.live = Liveness::Polite;
        let mut navigation = ui_id(row(), "generate.form.navigation")?;
        navigation = ui_child(navigation, button("generate.form.previous", previous, "arrow-left", self.index.checked_sub(1))?)?;
        navigation = ui_child(navigation, position)?;
        navigation = ui_child(navigation, button("generate.form.next", next, "arrow-right", (current < self.total).then_some(current))?)?;
        let mut viewport = ui_build(ui_id(column(), format!("generate.form.viewport.{}", self.index))?)?;
        viewport.layout = std::mem::replace(&mut body.layout, LayoutSpec::Stack(StackLayout { axis: Axis::Vertical, grow: true, ..Default::default() }));
        let viewport = viewport.try_with_children(std::mem::take(&mut body.children)).map_err(|_| ui_assembly_error("ui.generation.form.viewport"))?;
        body.try_with_children([ui_build(navigation)?, viewport]).map_err(|_| ui_assembly_error("ui.generation.form.navigation"))
    }
}

/// 📋️ Renders one record-budgeted page of generation questions with typed change bindings.
pub fn generation_form(
    spec: &semio_os_playbook_contract::PlaybookSpec,
    values: &semio_os_playbook_contract::PlaybookValues,
    controller_id: &'static str,
    action: &str,
    generation_id: &str,
    words: &GenerationFormLabels<'_>,
    requested_page: usize,
    resources: &semio_plugin::WindowResourceRequests<'_>,
) -> UiAssemblyResult<GenerationFormPage> {
    let visible = || spec.steps.iter().enumerate().flat_map(|(step_index, step)| {
        step.blocks.iter().filter(move |question| semio_os_playbook_contract::is_block_visible(question, values))
            .map(move |question| (step_index, step, question))
    });
    let grouped = spec.steps.len() > 1;
    let costs = || visible().map(|(step_index, _, question)| {
        let nodes = if question.kind == "vector" {
            1 + 2 * generation_vector_axes(question, values.get(&question.id))
        } else if matches!(question.kind.as_str(), "text" | "longText") {
            let fallback = || semio_os_playbook_contract::default_value_for_block(question);
            let value = values.get(&question.id).cloned().unwrap_or_else(fallback);
            let value = value.as_str().unwrap_or_default();
            if UiText::try_from_str(value).is_some() { 2 } else { 2 + semio_plugin::paged_text_carrier_nodes(value) }
        } else if question.kind == "image" {
            question.src.as_deref().map(str::trim)
                .map_or(2, |source| 2 + semio_plugin::WindowResourceRequests::image_body_nodes(source))
        } else {
            2
        };
        form_pages::FieldCost { nodes, section: grouped.then_some(step_index) }
    });
    let over_budget = |ordinal: usize| {
        PluginAssemblyError::new("ui.generation.form.field-budget", format!("Form question {} exceeds the mounted record budget", ordinal + 1))
    };
    let complete = form_pages::form_page(costs(), UI_DOCUMENT_NODES - 2, 0).map_err(over_budget)?;
    let page = if complete.total == 1 {
        complete
    } else {
        form_pages::form_page(costs(), TREE_WINDOW_BODY_NODE_BUDGET - 1, requested_page).map_err(over_budget)?
    };
    let mut fields = ui_id(column(), "generate.form.fields")?;
    let mut section = None;
    for (step_index, step, question) in visible().skip(page.fields.start).take(page.fields.len()) {
        if grouped && section != Some(step_index) {
            let heading = ui_build(ui_id(text(ui_label(&step.title)?), format!("generate.step.{}", step.id))?)?;
            fields = ui_child(fields, heading)?;
            section = Some(step_index);
        }
        fields = ui_child(fields, generation_question(question, values, controller_id, action, generation_id, words, resources)?)?;
    }
    if page.nodes == 0 {
        fields = ui_child(fields, ui_build(text(ui_label(words.no_inputs)?))?)?;
    }
    let mut body = ui_build(ui_child(ui_id(form(ui_label(words.title)?), "generate.form")?, ui_build(fields)?)?)?;
    body.layout = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, sizing: Sizing::Fill, ..Default::default() });
    Ok(GenerationFormPage { body, index: page.index, total: page.total })
}

#[path = "📝️form/pages.rs"]
mod form_pages;

#[path = "📊️channel-values/🦀️.rs"]
mod channel_values;
pub use channel_values::{scene_with_channel_values, windowed_read_only_text, ChannelValueLabels, ChannelValueRow};

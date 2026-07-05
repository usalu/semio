//! 🧩 Maps framework UiNode trees to ui_wgpu widget nodes.

use crate::scenes::render_component_scene;
use semio_framework_core::{CommandDescriptor, UiControlNode, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeSectionNode};
use ui_wgpu::{
    ControlNode, KeyValueEntry, Rect, SelectItem, Theme, TreeItem, TreeItemAction, TreeSection, WidgetContext,
    WidgetInteractionMaps, WidgetNode, measure_widget, render_widget,
};

pub type FrameworkWidgetContext<'a> = WidgetContext<'a, CommandDescriptor>;

pub fn measure_ui_node(atlas: &mut ui_wgpu::FontAtlas, theme: &Theme, node: &UiNode) -> (f32, f32) {
    match node {
        UiNode::ComponentScene(_) => (320.0, 240.0),
        other => measure_widget(atlas, theme, &ui_node_to_widget(other)),
    }
}

pub fn render_ui_node(
    node: &UiNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    gpu: &mut ui_wgpu::GpuContext,
    world3d_states: &mut std::collections::HashMap<String, crate::world3d::World3dState>,
) {
    match node {
        UiNode::ComponentScene(scene) => render_component_scene(scene, bounds, ctx, gpu, world3d_states),
        other => render_widget(&ui_node_to_widget(other), bounds, ctx),
    }
}

pub fn ui_node_to_widget(node: &UiNode) -> WidgetNode<CommandDescriptor> {
    match node {
        UiNode::Stack(stack) => WidgetNode::Stack {
            direction: stack.direction.clone(),
            gap: stack.gap.clone(),
            padding: stack.padding.clone(),
            children: stack.children.iter().map(ui_node_to_widget).collect(),
        },
        UiNode::Text(text) => WidgetNode::Text {
            value: text.value.clone(),
            emphasize: text.emphasize.unwrap_or(false),
        },
        UiNode::Separator(_) => WidgetNode::Separator,
        UiNode::Button(button) => WidgetNode::Button {
            id: button.id.clone(),
            icon_id: Some(button.icon_id.clone()),
            label: button.label.clone(),
            event: Some(button.command.clone()),
        },
        UiNode::Input(input) => WidgetNode::Input {
            id: input.id.clone(),
            input_kind: input.input_kind.clone(),
            value: input.value.clone(),
            placeholder: input.placeholder.clone(),
            commit: input.commit.clone(),
            on_change: Some(input.on_change.clone()),
        },
        UiNode::Select(select) => WidgetNode::Select {
            id: select.id.clone(),
            value: select.value.clone(),
            items: select.items.iter().map(|i| SelectItem { value: i.value.clone(), label: i.label.clone() }).collect(),
            placeholder: select.placeholder.clone(),
            on_change: Some(select.on_change.clone()),
        },
        UiNode::Toggle(toggle) => WidgetNode::Toggle {
            id: toggle.id.clone(),
            icon_id: toggle.icon_id.clone(),
            pressed: toggle.pressed,
            text: toggle.text.clone(),
            on_change: Some(toggle.on_change.clone()),
        },
        UiNode::Vec3(vec3) => WidgetNode::Vec3 {
            id: vec3.id.clone(),
            value: vec3.value,
            on_change: Some(vec3.on_change.clone()),
        },
        UiNode::KeyValue(kv) => WidgetNode::KeyValue {
            entries: kv.entries.iter().map(|e| KeyValueEntry { label: e.label.clone(), value: e.value.clone() }).collect(),
        },
        UiNode::Slider(slider) => WidgetNode::Slider {
            id: slider.id.clone(),
            value: slider.value,
            min: slider.min,
            max: slider.max,
            step: slider.step,
            on_change: Some(slider.on_change.clone()),
        },
        UiNode::NumberStepper(stepper) => WidgetNode::NumberStepper {
            id: stepper.id.clone(),
            value: stepper.value,
            step: stepper.step,
            uniform: stepper.uniform,
            on_absolute: Some(stepper.on_absolute.clone()),
            on_delta: Some(stepper.on_delta.clone()),
        },
        UiNode::Ring(ring) => WidgetNode::Ring {
            id: ring.id.clone(),
            t: ring.t,
            disabled: ring.disabled.unwrap_or(false),
            on_change: Some(ring.on_change.clone()),
        },
        UiNode::IconSelect(icon) => WidgetNode::IconSelect {
            id: icon.id.clone(),
            value: icon.value.clone(),
            uniform: icon.uniform,
            classifier_kind: icon.classifier_kind.clone(),
            on_change: Some(icon.on_change.clone()),
        },
        UiNode::Field(field) => WidgetNode::Field {
            id: field.id.clone(),
            label: field.label.clone(),
            child: control_to_widget(&field.child),
        },
        UiNode::Section(section) => WidgetNode::Section {
            id: section.id.clone(),
            label: section.label.clone(),
            default_open: section.default_open.unwrap_or(true),
            children: section.children.iter().map(ui_node_to_widget).collect(),
        },
        UiNode::Tree(tree) => WidgetNode::Tree {
            sections: tree.sections.iter().map(tree_section_to_widget).collect(),
            selected_ids: tree.selected_ids.clone().unwrap_or_default(),
            highlighted_ids: tree.highlighted_ids.clone().unwrap_or_default(),
            selection_change: tree.selection_change.clone(),
        },
        UiNode::ComponentScene(_) => WidgetNode::Text {
            value: String::new(),
            emphasize: false,
        },
    }
}

fn control_to_widget(control: &UiControlNode) -> ControlNode<CommandDescriptor> {
    match control {
        UiControlNode::Button(n) => ControlNode::Button {
            id: n.id.clone(),
            icon_id: Some(n.icon_id.clone()),
            label: n.label.clone(),
            event: Some(n.command.clone()),
        },
        UiControlNode::Input(n) => ControlNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone(),
            commit: n.commit.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => ControlNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|i| SelectItem { value: i.value.clone(), label: i.label.clone() }).collect(),
            placeholder: n.placeholder.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => ControlNode::Toggle {
            id: n.id.clone(),
            icon_id: n.icon_id.clone(),
            pressed: n.pressed,
            text: n.text.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Vec3(n) => ControlNode::Vec3 {
            id: n.id.clone(),
            value: n.value,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::KeyValue(n) => ControlNode::KeyValue {
            entries: n.entries.iter().map(|e| KeyValueEntry { label: e.label.clone(), value: e.value.clone() }).collect(),
        },
        UiControlNode::Slider(n) => ControlNode::Slider {
            id: n.id.clone(),
            value: n.value,
            min: n.min,
            max: n.max,
            step: n.step,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::NumberStepper(n) => ControlNode::NumberStepper {
            id: n.id.clone(),
            value: n.value,
            step: n.step,
            uniform: n.uniform,
            on_absolute: Some(n.on_absolute.clone()),
            on_delta: Some(n.on_delta.clone()),
        },
        UiControlNode::Ring(n) => ControlNode::Ring {
            id: n.id.clone(),
            t: n.t,
            disabled: n.disabled.unwrap_or(false),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::IconSelect(n) => ControlNode::IconSelect {
            id: n.id.clone(),
            value: n.value.clone(),
            uniform: n.uniform,
            classifier_kind: n.classifier_kind.clone(),
            on_change: Some(n.on_change.clone()),
        },
    }
}

fn tree_action_to_widget(action: &UiTreeItemAction) -> TreeItemAction<CommandDescriptor> {
    TreeItemAction {
        icon_id: action.icon_id.clone(),
        label: action.label.clone(),
        event: action.command.clone(),
        reveal_on_hover: action.reveal_on_hover.unwrap_or(false),
    }
}

fn tree_section_to_widget(section: &UiTreeSectionNode) -> TreeSection<CommandDescriptor> {
    TreeSection {
        id: section.id.clone(),
        label: section.label.clone(),
        default_open: section.default_open.unwrap_or(true),
        items: section.items.iter().map(tree_item_to_widget).collect(),
    }
}

fn tree_item_to_widget(item: &UiTreeItemNode) -> TreeItem<CommandDescriptor> {
    TreeItem {
        id: item.id.clone(),
        label: item.label.clone(),
        description: item.description.clone(),
        icon_id: item.icon_id.clone(),
        selected: item.selected.unwrap_or(false),
        highlighted: false,
        default_open: item.default_open.unwrap_or(false),
        is_hidden: item.is_hidden.unwrap_or(false),
        event: item.command.clone(),
        hover_event: item.hover_command.clone(),
        unhover_event: item.unhover_command.clone(),
        actions: item
            .actions
            .as_ref()
            .map(|actions| actions.iter().map(tree_action_to_widget).collect())
            .unwrap_or_default(),
        draggable: item.draggable.unwrap_or(false),
        drag_data: item.drag_data.clone().unwrap_or_default(),
        control: item
            .control
            .as_ref()
            .map(|control| Box::new(control_to_widget_node(control))),
        children: item
            .items
            .as_ref()
            .map(|items| items.iter().map(tree_item_to_widget).collect())
            .unwrap_or_default(),
    }
}

fn control_to_widget_node(control: &UiControlNode) -> WidgetNode<CommandDescriptor> {
    match control {
        UiControlNode::Button(n) => WidgetNode::Button {
            id: n.id.clone(),
            icon_id: Some(n.icon_id.clone()),
            label: n.label.clone(),
            event: Some(n.command.clone()),
        },
        UiControlNode::Input(n) => WidgetNode::Input {
            id: n.id.clone(),
            input_kind: n.input_kind.clone(),
            value: n.value.clone(),
            placeholder: n.placeholder.clone(),
            commit: n.commit.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Select(n) => WidgetNode::Select {
            id: n.id.clone(),
            value: n.value.clone(),
            items: n.items.iter().map(|i| SelectItem { value: i.value.clone(), label: i.label.clone() }).collect(),
            placeholder: n.placeholder.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Toggle(n) => WidgetNode::Toggle {
            id: n.id.clone(),
            icon_id: n.icon_id.clone(),
            pressed: n.pressed,
            text: n.text.clone(),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::Vec3(n) => WidgetNode::Vec3 {
            id: n.id.clone(),
            value: n.value,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::KeyValue(n) => WidgetNode::KeyValue {
            entries: n.entries.iter().map(|e| KeyValueEntry { label: e.label.clone(), value: e.value.clone() }).collect(),
        },
        UiControlNode::Slider(n) => WidgetNode::Slider {
            id: n.id.clone(),
            value: n.value,
            min: n.min,
            max: n.max,
            step: n.step,
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::NumberStepper(n) => WidgetNode::NumberStepper {
            id: n.id.clone(),
            value: n.value,
            step: n.step,
            uniform: n.uniform,
            on_absolute: Some(n.on_absolute.clone()),
            on_delta: Some(n.on_delta.clone()),
        },
        UiControlNode::Ring(n) => WidgetNode::Ring {
            id: n.id.clone(),
            t: n.t,
            disabled: n.disabled.unwrap_or(false),
            on_change: Some(n.on_change.clone()),
        },
        UiControlNode::IconSelect(n) => WidgetNode::IconSelect {
            id: n.id.clone(),
            value: n.value.clone(),
            uniform: n.uniform,
            classifier_kind: n.classifier_kind.clone(),
            on_change: Some(n.on_change.clone()),
        },
    }
}

pub fn framework_widget_context<'a>(
    draw: &'a mut ui_wgpu::DrawList,
    overlay: Option<&'a mut ui_wgpu::DrawList>,
    atlas: &'a mut ui_wgpu::FontAtlas,
    icons: Option<&'a ui_wgpu::IconAtlas>,
    input: &'a mut ui_wgpu::InputState<CommandDescriptor>,
    theme: &'a Theme,
    scroll_offsets: &'a mut std::collections::HashMap<String, f32>,
    collapsed_sections: &'a mut std::collections::HashMap<String, bool>,
    open_selects: &'a mut std::collections::HashMap<String, bool>,
    interaction_maps: Option<&'a mut WidgetInteractionMaps<CommandDescriptor>>,
) -> FrameworkWidgetContext<'a> {
    WidgetContext {
        draw,
        overlay,
        atlas,
        icons,
        input,
        theme,
        scroll_offsets,
        collapsed_sections,
        open_selects,
        interaction_maps,
    }
}

import json, sys
V = "️"
F = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🎨️node-painting-with-driver/🦀️.rs"
def child(e, name, text):
    return {"dir": e + V + name, "doc": "//! %s%s %s See [`super`]." % (e, V, text)}
c1 = child("📝", "text-and-structure-nodes", "Text and structure nodes: the retained paint steps of the text, stack, separator, key value, field, section and group nodes.")
c2 = child("🎛", "control-nodes", "Control nodes: the retained paint steps of the button, input, select, toggle and icon select nodes.")
c3 = child("🎚", "slider-stepper-and-progress-nodes", "Slider, stepper and progress nodes: the retained paint steps of the slider, number stepper, ring and progress nodes.")
c4 = child("🖼", "media-and-slot-nodes", "Media and slot nodes: the retained paint steps of the image, component scene and external slot nodes.")
N = "&semio_framework_ui_declaration::ui::"
def arm(line, name, emoji, text, bind, node, ch):
    d = {"line": line, "name": name, "doc": "%s %s" % (emoji, text), "binds": {bind: N + node} if bind else {}, "child": ch, "allow_returns": True}
    return d
spec = {
    "file": F, "function": "paint_node_step_with_driver", "stmt": 60, "match_line": 61,
    "types": {
        "node": "&Node", "unscoped_body_font_size": "f32", "bounds": "Rect", "presence": "&UiPresence", "flags": "NodeFlags", "theme": "&Theme",
    },
    "arms": [
        arm(62, "paint_text_node_step", "📝", "Paints one step of a text node: the weighted body or emphasized run.", "text", "UiTextNode", c1),
        arm(87, "paint_stack_node_step", "🗃", "Paints one step of a stack node: its chrome frame and tree item decoration.", "stack", "UiStackNode", c1),
        arm(105, "paint_separator_node_step", "➖", "Paints one step of a separator node: the hairline.", None, None, c1),
        arm(118, "paint_button_node_step", "🔘", "Paints one step of a button node: chrome, border and label.", "button", "UiButtonNode", c2),
        arm(143, "paint_input_node_step", "⌨", "Paints one step of an input node: chrome, text, caret and selection.", "input_node", "UiInputNode", c2),
        arm(196, "paint_select_node_step", "🔽", "Paints one step of a select node: the closed control and the open popup.", "select", "UiSelectNode", c2),
        arm(347, "paint_toggle_node_step", "🔀", "Paints one step of a toggle node: the switch or checkbox and its label.", "toggle", "UiToggleNode", c2),
        arm(384, "paint_key_value_node_step", "🔑", "Paints one step of a key value node: the key and its value cell.", "key_value", "UiKeyValueNode", c1),
        arm(418, "paint_slider_node_step", "🎚", "Paints one step of a slider node: track, thumb, value and unit label.", "slider", "UiSliderNode", c3),
        arm(489, "paint_number_stepper_node_step", "🔢", "Paints one step of a number stepper node: segments, chrome, icons and value text.", "stepper", "UiNumberStepperNode", c3),
        arm(582, "paint_ring_node_step", "⭕", "Paints one step of a ring node: the progress ring and its label.", "ring", "UiRingNode", c3),
        arm(615, "paint_progress_node_step", "📶", "Paints one step of a progress node: the bar track and fill.", "progress", "UiProgressNode", c3),
        arm(628, "paint_icon_select_node_step", "🧩", "Paints one step of an icon select node: the icon grid and its selection.", "select", "UiIconSelectNode", c2),
        arm(663, "paint_field_node_step", "🏷", "Paints one step of a field node: label, required marker and help text.", "field", "UiFieldNode", c1),
        arm(698, "paint_section_node_step", "📑", "Paints one step of a section node: the header band and title.", "section", "UiSectionNode", c1),
        arm(717, "paint_group_node_step", "🧱", "Paints one step of a group node: the group frame and title.", "group", "UiGroupNode", c1),
        arm(747, "paint_image_node_step", "🖼", "Paints one step of an image node: the decoded raster, the host raster or the placeholder chrome.", "image", "UiImageNode", c4),
        arm(791, "paint_component_scene_node_step", "🎬", "Paints one step of a component scene node: the scene host placeholder.", "scene", "UiComponentSceneNode", c4),
        arm(814, "paint_external_slot_node_step", "🔌", "Paints one step of an external slot node: the slot placeholder chrome.", "slot", "UiExternalSlotNode", c4),
    ],
}
for a in spec["arms"]:
    if a["name"] == "paint_slider_node_step":
        a["derive"] = {"inline_suffix_width": {"type": "f32", "expr": "layout.inline_suffix_width", "from": "layout"}}
json.dump(spec, open(sys.argv[1], "w", encoding="utf8"), ensure_ascii=False, indent=1)

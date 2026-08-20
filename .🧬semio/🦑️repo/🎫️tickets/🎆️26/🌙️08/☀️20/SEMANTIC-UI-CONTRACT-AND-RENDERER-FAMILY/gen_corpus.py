#!/usr/bin/env python3
"""Generates the semio-framework-ui-contract conformance corpus.

Mirrors the exact serde wire shape of ui_contract's Rust types (read directly from
document.rs / component.rs / layout.rs / style.rs / accessibility.rs / action.rs /
surface.rs / limits.rs on 2026-08-20) so every emitted fixture is what the real
builders + serde would actually produce. This script only uses python3 (no cargo),
per ruling U4 in ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY.
"""
import json
import os

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance"

written = []


def emit(group: str, slug: str, obj, kind: str):
    path = os.path.join(ROOT, group, f"{slug}.{kind}.json")
    with open(path, "w", encoding="utf-8") as f:
        json.dump(obj, f, indent=2, ensure_ascii=False, sort_keys=False)
        f.write("\n")
    written.append(path)


#region wire helpers (mirror serde exactly)
def style_spec(variant=None, size=None, density=None, tone=None, emphasis=None):
    out = {}
    if variant is not None:
        out["variant"] = variant
    if size is not None:
        out["size"] = size
    if density is not None:
        out["density"] = density
    if tone is not None:
        out["tone"] = tone
    if emphasis is not None:
        out["emphasis"] = emphasis
    return out


def accessibility_spec(label=None, description=None, live=None, shortcut=None, hidden=None):
    out = {}
    if label is not None:
        out["label"] = label
    if description is not None:
        out["description"] = description
    if live is not None:
        out["live"] = live
    if shortcut is not None:
        out["shortcut"] = shortcut
    if hidden:
        out["hidden"] = True
    return out


def leaf_layout(width="hug", height="hug"):
    return {"kind": "leaf", "width": width, "height": height}


def stack_layout(axis="vertical", gap="none", padding=None, align="stretch", justify="start", grow=False, wrap=False):
    return {"kind": "stack", "axis": axis, "gap": gap, "padding": padding or {"all": "none"}, "align": align, "justify": justify, "grow": grow, "wrap": wrap}


def grid_layout(columns, rows, column_gap="none", row_gap="none", padding=None, align="stretch", justify="start"):
    return {"kind": "grid", "columns": columns, "rows": rows, "columnGap": column_gap, "rowGap": row_gap, "padding": padding or {"all": "none"}, "align": align, "justify": justify}


def overlay_layout(anchor="center", inset=None, dismissible=False):
    return {"kind": "overlay", "anchor": anchor, "inset": inset or {"all": "none"}, "dismissible": dismissible}


def scroll_layout(axes="vertical", padding=None, sizing="fill"):
    return {"kind": "scroll", "axes": axes, "padding": padding or {"all": "none"}, "sizing": sizing}


def absolute_layout(sizing_width="hug", sizing_height="hug"):
    return {"kind": "absolute", "sizingWidth": sizing_width, "sizingHeight": sizing_height}


def action_id(scope, name, version=1):
    return {"scope": scope, "name": name, "version": version}


def action_binding(trigger, scope, name, version=1, args=None, capability=None):
    out = {"trigger": trigger, "action": action_id(scope, name, version)}
    if args is not None:
        out["args"] = args
    if capability is not None:
        out["capability"] = capability
    return out


def menu_ref(mid, args=None):
    out = {"id": mid}
    if args is not None:
        out["args"] = args
    return out


#region Component variants
def c_container(role=None, label=None, description=None, required=None, error=None, default_open=None, drop_overlay=None):
    out = {}
    if role is not None and role != "plain":
        out["role"] = role
    if label is not None:
        out["label"] = label
    if description is not None:
        out["description"] = description
    if required is not None:
        out["required"] = required
    if error is not None:
        out["error"] = error
    if default_open is not None:
        out["defaultOpen"] = default_open
    if drop_overlay is not None:
        out["dropOverlay"] = drop_overlay
    return {"type": "container", **out}


def c_text(value, emphasize=None, data_attributes=None):
    out = {"value": value}
    if emphasize is not None:
        out["emphasize"] = emphasize
    if data_attributes is not None:
        out["dataAttributes"] = data_attributes
    return {"type": "text", **out}


def c_button(label, icon=""):
    return {"type": "button", "icon": icon, "label": label}


def c_separator():
    return {"type": "separator"}


def c_input(kind=None, value="", placeholder=None, commit=None, min=None, max=None, step=None, accept=None):
    out = {}
    if kind is not None and kind != "text":
        out["kind"] = kind
    out["value"] = value
    if placeholder is not None:
        out["placeholder"] = placeholder
    if commit is not None:
        out["commit"] = commit
    if min is not None:
        out["min"] = min
    if max is not None:
        out["max"] = max
    if step is not None:
        out["step"] = step
    if accept is not None:
        out["accept"] = accept
    return {"type": "input", **out}


def c_select(value, items, placeholder=None):
    out = {"value": value, "items": items}
    if placeholder is not None:
        out["placeholder"] = placeholder
    return {"type": "select", **out}


def c_toggle(on, icon="", text=None):
    out = {"on": on, "icon": icon}
    if text is not None:
        out["text"] = text
    return {"type": "toggle", **out}


def c_key_value_list(entries):
    return {"type": "keyValueList", "entries": entries}


def c_slider(value, min=0.0, max=1.0, step=0.1, unit=None):
    out = {"value": value, "min": min, "max": max, "step": step}
    if unit is not None:
        out["unit"] = unit
    return {"type": "slider", **out}


def c_number_stepper(value, step, uniform):
    return {"type": "numberStepper", "value": value, "step": step, "uniform": uniform}


def c_ring(orb_id, t):
    return {"type": "ring", "orbId": orb_id, "t": t}


def c_icon_select(value, uniform, classifier_kind):
    return {"type": "iconSelect", "value": value, "uniform": uniform, "classifierKind": classifier_kind}


def c_tree(interaction_domain=None):
    out = {}
    if interaction_domain is not None:
        out["interactionDomain"] = interaction_domain
    return {"type": "tree", **out}


def c_tree_section(label=None, default_open=None):
    out = {}
    if label is not None:
        out["label"] = label
    if default_open is not None:
        out["defaultOpen"] = default_open
    return {"type": "treeSection", **out}


def c_tree_item(label, description=None, icon=None, default_open=None, draggable=None, drag_data=None, dimmed=None, row_actions=None):
    out = {"label": label}
    if description is not None:
        out["description"] = description
    if icon is not None:
        out["icon"] = icon
    if default_open is not None:
        out["defaultOpen"] = default_open
    if draggable is not None:
        out["draggable"] = draggable
    if drag_data is not None:
        out["dragData"] = drag_data
    if dimmed is not None:
        out["dimmed"] = dimmed
    if row_actions:
        out["rowActions"] = row_actions
    return {"type": "treeItem", **out}


def c_image(src, alt=None):
    out = {"src": src}
    if alt is not None:
        out["alt"] = alt
    return {"type": "image", **out}


def c_surface(surface_id, controller_id, kind, doc_schema, doc_bytes=None, pane_id=None, binding_id=None, domain_id=None, domain_granularity_id=None):
    out = {"surfaceId": surface_id, "controllerId": controller_id, "kind": kind}
    if pane_id is not None:
        out["paneId"] = pane_id
    if binding_id is not None:
        out["bindingId"] = binding_id
    out["docSchema"] = doc_schema
    out["doc"] = {"bytes": doc_bytes or []}
    if domain_id is not None:
        out["domainId"] = domain_id
    if domain_granularity_id is not None:
        out["domainGranularityId"] = domain_granularity_id
    return {"type": "surface", **out}


def c_extension(extension, props=None):
    return {"type": "extension", "extension": extension, "props": props if props is not None else None}


def row_action(icon, action, label=None, placement=None):
    out = {"icon": icon}
    if label is not None:
        out["label"] = label
    out["action"] = action
    if placement is not None and placement != "row":
        out["placement"] = placement
    return out
#endregion Component variants


def node(node_id, key, component, layout=None, style=None, activity=None, disabled=False, transition=None, accessibility=None, bindings=None, menu=None, children=None):
    out = {
        "id": node_id,
        "key": key,
        "component": component,
        "layout": layout if layout is not None else leaf_layout(),
        "style": style if style is not None else style_spec(),
        "activity": activity if activity is not None else "idle",
    }
    if disabled:
        out["disabled"] = True
    if transition is not None:
        out["transition"] = transition
    out["accessibility"] = accessibility if accessibility is not None else accessibility_spec()
    if bindings:
        out["bindings"] = bindings
    if menu is not None:
        out["menu"] = menu
    if children:
        out["children"] = children
    return out


def snapshot(surface, revision, root, nodes, layout_epoch=0):
    return {"surface": surface, "revision": revision, "root": root, "nodes": nodes, "layoutEpoch": layout_epoch}


def patch(surface, base_revision, revision, ops):
    return {"surface": surface, "baseRevision": base_revision, "revision": revision, "ops": ops}


#region UiPatchOp constructors
def op_upsert(rec):
    return {"type": "upsert", **rec}


def op_set_component(node_id, component):
    return {"type": "setComponent", "id": node_id, "component": component}


def op_set_layout(node_id, layout):
    return {"type": "setLayout", "id": node_id, "layout": layout}


def op_set_activity(node_id, activity, disabled):
    return {"type": "setActivity", "id": node_id, "activity": activity, "disabled": disabled}


def op_set_children(node_id, children):
    return {"type": "setChildren", "id": node_id, "children": children}


def op_set_style(node_id, style):
    return {"type": "setStyle", "id": node_id, "style": style}


def op_set_accessibility(node_id, accessibility):
    return {"type": "setAccessibility", "id": node_id, "accessibility": accessibility}


def op_set_bindings(node_id, bindings):
    return {"type": "setBindings", "id": node_id, "bindings": bindings}


def op_set_menu(node_id, menu):
    return {"type": "setMenu", "id": node_id, "menu": menu}


def op_remove(node_id):
    return {"type": "remove", "id": node_id}


def op_set_root(node_id):
    return {"type": "setRoot", "id": node_id}
#endregion UiPatchOp constructors


#region UiContractViolation / PatchRejection constructors
def v_cycle(n):
    return {"type": "cycle", "node": n}


def v_orphan(parent, child):
    return {"type": "orphanChild", "parent": parent, "child": child}


def v_dup_key(parent, key):
    return {"type": "duplicateSiblingKey", "parent": parent, "key": key}


def v_node_quota(count, max_):
    return {"type": "nodeQuota", "count": count, "max": max_}


def v_depth_quota(n, depth, max_):
    return {"type": "depthQuota", "node": n, "depth": depth, "max": max_}


def v_dangling(n):
    return {"type": "danglingRoot", "node": n}


def v_section_nested(n):
    return {"type": "sectionNested", "node": n}


def v_non_finite(n):
    return {"type": "nonFiniteNumber", "node": n}


def r_revision_mismatch(expected, actual):
    return {"type": "revisionMismatch", "expected": expected, "actual": actual}


def r_unknown_node(n):
    return {"type": "unknownNode", "id": n}


def r_quota_exceeded(quota, actual, max_):
    return {"type": "quotaExceeded", "quota": quota, "actual": actual, "max": max_}


def r_invariant_violated(violations):
    return {"type": "invariantViolated", "violations": violations}


def limits(max_nodes=20000, max_depth=128, max_children=4096, max_text_bytes=65536, max_patch_ops=4096, max_patch_bytes=1048576):
    return {"maxNodes": max_nodes, "maxDepth": max_depth, "maxChildren": max_children, "maxTextBytes": max_text_bytes, "maxPatchOps": max_patch_ops, "maxPatchBytes": max_patch_bytes}
#endregion

#endregion wire helpers


def expect(group, slug, kind, description, outcome, **extra):
    obj = {"case": slug, "kind": kind, "description": description, "outcome": outcome}
    obj.update(extra)
    emit(group, slug, obj, "expect")


def tree_shape(nodes):
    return [{"id": n["id"], "key": n["key"], "type": n["component"]["type"], "children": n.get("children", [])} for n in nodes]


def accessibility_shape(nodes):
    out = []
    for n in nodes:
        a = n.get("accessibility", {})
        out.append({"id": n["id"], "label": a.get("label"), "description": a.get("description"), "live": a.get("live", "off"), "shortcut": a.get("shortcut"), "hidden": a.get("hidden", False)})
    return out


def action_ids_of(nodes):
    ids = []
    for n in nodes:
        for b in n.get("bindings", []):
            aid = b["action"]
            ids.append(f"{aid['scope']}.{aid['name']}@{aid['version']}")
    return ids


#region 🧩️component fixtures
def gen_component():
    G = "🧩️component"

    def one_node_case(slug, description, comp, **node_kwargs):
        n = node(0, node_kwargs.pop("key", "#0"), comp, **node_kwargs)
        snap = snapshot(f"conformance.component.{slug}", 0, 0, [n])
        emit(G, slug, snap, "snapshot")
        expect(G, slug, "component", description, "accept", limits=None, tree={"root": 0, "nodeCount": 1, "shape": tree_shape([n])}, accessibility=accessibility_shape([n]), actionIds=action_ids_of([n]))

    one_node_case("container", "A labelled, collapsible Container(role=section) with description.", c_container(role="section", label="Settings", description="Adjust preferences", default_open=True), accessibility=accessibility_spec(label="Settings"))
    one_node_case("text", "Plain display Text, emphasized.", c_text("Hello", emphasize=True))
    one_node_case("button", "A clickable Button bound to an Activate action.", c_button("Save", icon="save"), accessibility=accessibility_spec(label="Save"), bindings=[action_binding("activate", "app", "save")])
    one_node_case("separator", "A Separator, which carries no props of its own.", c_separator())
    one_node_case("input", "A text Input with a placeholder and a draft value.", c_input(value="draft", placeholder="Title"))
    one_node_case("select", "A single-choice Select with two options.", c_select("a", [{"value": "a", "label": "Option A"}, {"value": "b", "label": "Option B"}]))
    one_node_case("toggle", "A Toggle currently on.", c_toggle(True, icon="toggle", text="Enabled"))
    one_node_case("key-value-list", "A KeyValueList with one row.", c_key_value_list([{"label": "Author", "value": "Ada"}]))
    one_node_case("slider", "A Slider at 0.4 with a percent unit.", c_slider(0.4, unit="%"))
    one_node_case("number-stepper", "A NumberStepper at 2, step 1.", c_number_stepper(2.0, 1.0, False))
    one_node_case("ring", "A Ring bound to orb 'orb-1' at t=0.6.", c_ring("orb-1", 0.6))
    one_node_case("icon-select", "An IconSelect over the 'shape' classifier.", c_icon_select("circle", True, "shape"))
    one_node_case("image", "An Image with real alt text.", c_image("atlas://logo", alt="Company logo"), accessibility=accessibility_spec(label="Company logo"))
    one_node_case("surface", "An embedded Surface of kind world-3d.", c_surface("surf-1", "ctrl", "world-3d", "world3d@1", doc_bytes=[1, 2, 3]))
    one_node_case("extension", "An Extension slot carrying a Map-shaped UiValue payload.", c_extension("plugin.app.slot", props={"id": "widget"}))

    # Tree / TreeSection / TreeItem only make sense nested together — one fixture covers all three
    # Component variants, since a bare TreeSection/TreeItem outside a Tree has no realistic use.
    item = node(2, "item-1", c_tree_item("First item", icon="file", row_actions=[row_action("trash", action_binding("activate", "cad-play", "deleteItem"), label="Delete", placement="menu")]), accessibility=accessibility_spec(label="First item"))
    section = node(1, "section-1", c_tree_section("Section", default_open=True), layout=stack_layout(), children=[2])
    root = node(0, "#0", c_tree(interaction_domain="selection"), layout=stack_layout(), children=[1])
    snap = snapshot("conformance.component.tree", 0, 0, [root, section, item])
    emit(G, "tree", snap, "snapshot")
    expect(G, "tree", "component", "A Tree containing one TreeSection with one TreeItem — covers Tree, TreeSection and TreeItem together.", "accept", limits=None, tree={"root": 0, "nodeCount": 3, "shape": tree_shape([root, section, item])}, accessibility=accessibility_shape([root, section, item]), actionIds=action_ids_of([root, section, item]))

    #region interesting states
    def state_case(slug, description, comp, **node_kwargs):
        n = node(0, "#0", comp, **node_kwargs)
        snap = snapshot(f"conformance.component.{slug}", 0, 0, [n])
        emit(G, slug, snap, "snapshot")
        expect(G, slug, "component", description, "accept", limits=None, tree={"root": 0, "nodeCount": 1, "shape": tree_shape([n])}, accessibility=accessibility_shape([n]), actionIds=action_ids_of([n]))

    state_case("state-disabled", "A disabled Button — non-interactive without leaving the tree.", c_button("Delete", icon="trash"), accessibility=accessibility_spec(label="Delete"), disabled=True)
    state_case("state-activity-waiting", "A Toggle in Activity::Waiting.", c_toggle(False, icon="toggle"), activity="waiting")
    state_case("state-activity-loading", "An Input in Activity::Loading.", c_input(value="fetching..."), activity="loading")
    state_case("state-activity-finished", "A Slider in Activity::Finished.", c_slider(1.0), activity="finished")
    state_case("state-transition-introducing", "A Text node entering with TransitionHint::Introducing.", c_text("Just added"), transition="introducing")
    state_case("state-transition-celebrating", "A Button entering with TransitionHint::Celebrating.", c_button("Done", icon="check"), accessibility=accessibility_spec(label="Done"), transition="celebrating")
    state_case("state-with-menu", "A TreeItem carrying a resolved context MenuRef.", c_tree_item("Row with menu"), layout=stack_layout(), accessibility=accessibility_spec(label="Row with menu"), menu=menu_ref("context.tree-item", args={"row": 1}))
    #endregion interesting states
#endregion 🧩️component fixtures


#region 🖥️composite fixtures
def gen_composite():
    G = "🖥️composite"

    # composite-form-with-validation: Form containing two Fields, one carrying a validation error.
    name_input = node(2, "name-input", c_input(value="Ada"))
    name_field = node(1, "name-field", c_container(role="field", label="Name", required=True), layout=stack_layout(), children=[2])
    email_input = node(4, "email-input", c_input(value="not-an-email"))
    email_field = node(3, "email-field", c_container(role="field", label="Email", required=True, error="Enter a valid email address"), layout=stack_layout(), accessibility=accessibility_spec(description="Enter a valid email address"), children=[4])
    form_root = node(0, "#0", c_container(role="form", label="Contact"), layout=stack_layout(), children=[1, 3])
    nodes = [form_root, name_field, name_input, email_field, email_input]
    snap = snapshot("conformance.composite.form", 0, 0, nodes)
    emit(G, "form-with-validation", snap, "snapshot")
    expect(G, "form-with-validation", "composite", "A Form with two Fields; the email field carries a validation error and matching accessibility description.", "accept", limits=None, tree={"root": 0, "nodeCount": 5, "shape": tree_shape(nodes)}, accessibility=accessibility_shape(nodes), actionIds=action_ids_of(nodes))

    # composite-tree-nested-sections: Tree > TreeSection > [TreeItem (row action), TreeItem > TreeItem (nested)]
    grandchild = node(5, "#0", c_tree_item("Nested child", icon="file"), accessibility=accessibility_spec(label="Nested child"))
    parent_item = node(4, "parent-item", c_tree_item("Parent row", icon="folder", default_open=True), layout=stack_layout(), accessibility=accessibility_spec(label="Parent row"), children=[5])
    leaf_item = node(3, "leaf-item", c_tree_item("Leaf row", icon="file", row_actions=[row_action("trash", action_binding("activate", "cad-play", "deleteItem"), label="Delete", placement="row"), row_action("more", action_binding("activate", "cad-play", "rowMenu"), placement="menu")]), accessibility=accessibility_spec(label="Leaf row"))
    section = node(1, "section-1", c_tree_section("Objects", default_open=True), layout=stack_layout(), children=[3, 4])
    root = node(0, "#0", c_tree(interaction_domain="scene-objects"), layout=stack_layout(), children=[1])
    nodes = [root, section, leaf_item, parent_item, grandchild]
    snap = snapshot("conformance.composite.tree", 0, 0, nodes)
    emit(G, "tree-nested-sections", snap, "snapshot")
    expect(G, "tree-nested-sections", "composite", "A Tree with one TreeSection holding a row-action item and a nested parent/child item pair.", "accept", limits=None, tree={"root": 0, "nodeCount": 5, "shape": tree_shape(nodes)}, accessibility=accessibility_shape(nodes), actionIds=action_ids_of(nodes))

    # composite-toolbar: Container(role=toolbar) with Button, Separator, Toggle
    b1 = node(1, "#0", c_button("New", icon="plus"), accessibility=accessibility_spec(label="New"), bindings=[action_binding("activate", "app", "new")])
    sep = node(2, "#1", c_separator())
    t1 = node(3, "#2", c_toggle(False, icon="grid", text="Grid"))
    root = node(0, "#0", c_container(role="toolbar"), layout=stack_layout(axis="horizontal"), children=[1, 2, 3])
    nodes = [root, b1, sep, t1]
    snap = snapshot("conformance.composite.toolbar", 0, 0, nodes)
    emit(G, "toolbar", snap, "snapshot")
    expect(G, "toolbar", "composite", "A Container(role=toolbar) with a Button, a Separator and a Toggle in a horizontal Stack.", "accept", limits=None, tree={"root": 0, "nodeCount": 4, "shape": tree_shape(nodes)}, accessibility=accessibility_shape(nodes), actionIds=action_ids_of(nodes))

    # composite-dialog: Overlay-laid-out container with title/body Text and a toolbar of Confirm/Cancel buttons.
    title = node(1, "#0", c_text("Discard changes?", emphasize=True))
    body = node(2, "#1", c_text("Unsaved changes will be lost."))
    confirm = node(4, "#0", c_button("Discard", icon="trash"), accessibility=accessibility_spec(label="Discard"), style=style_spec(tone="danger"), bindings=[action_binding("activate", "app", "discard")])
    cancel = node(5, "#1", c_button("Cancel", icon="close"), accessibility=accessibility_spec(label="Cancel"), bindings=[action_binding("activate", "app", "cancel")])
    actions = node(3, "#2", c_container(role="toolbar"), layout=stack_layout(axis="horizontal", justify="end"), children=[4, 5])
    root = node(0, "#0", c_container(role="plain"), layout=overlay_layout(anchor="center", dismissible=True), accessibility=accessibility_spec(label="Discard changes?", description="Unsaved changes will be lost."), children=[1, 2, 3])
    nodes = [root, title, body, actions, confirm, cancel]
    snap = snapshot("conformance.composite.dialog", 0, 0, nodes)
    emit(G, "dialog", snap, "snapshot")
    expect(G, "dialog", "composite", "A dismissible Overlay-laid-out dialog with title/body Text and a Confirm/Cancel toolbar.", "accept", limits=None, tree={"root": 0, "nodeCount": 6, "shape": tree_shape(nodes)}, accessibility=accessibility_shape(nodes), actionIds=action_ids_of(nodes))

    # composite-surface-embedded: a Surface sits beside ordinary widgets in one Stack.
    heading = node(1, "#0", c_text("Scene", emphasize=True))
    surf = node(2, "#1", c_surface("surf-scene", "cad-play", "world-3d", "world3d@1", doc_bytes=[9, 9, 9], pane_id="viewport-1"), layout=leaf_layout(width="fill", height="fill"))
    status = node(3, "#2", c_text("3 objects selected"))
    root = node(0, "#0", c_container(role="plain"), layout=stack_layout(), children=[1, 2, 3])
    nodes = [root, heading, surf, status]
    snap = snapshot("conformance.composite.surface-embedded", 0, 0, nodes)
    emit(G, "surface-embedded", snap, "snapshot")
    expect(G, "surface-embedded", "composite", "A Surface(world-3d) embedded beside ordinary Text widgets inside one vertical Stack.", "accept", limits=None, tree={"root": 0, "nodeCount": 4, "shape": tree_shape(nodes)}, accessibility=accessibility_shape(nodes), actionIds=action_ids_of(nodes))
#endregion 🖥️composite fixtures


#region 📐️layout fixtures
def gen_layout():
    G = "📐️layout"

    def layout_case(slug, description, layout_spec):
        child = node(1, "#0", c_text("Content"))
        root = node(0, "#0", c_container(role="plain"), layout=layout_spec, children=[1])
        nodes = [root, child]
        snap = snapshot(f"conformance.layout.{slug}", 0, 0, nodes)
        emit(G, slug, snap, "snapshot")
        expect(G, slug, "layout", description, "accept", limits=None, tree={"root": 0, "nodeCount": 2, "shape": tree_shape(nodes)}, accessibility=accessibility_shape(nodes), actionIds=action_ids_of(nodes))

    layout_case("leaf", "Root using LayoutSpec::Leaf — a terminal box imposing no layout of its own.", leaf_layout(width="fill", height="hug"))
    layout_case("stack", "Root using LayoutSpec::Stack — one-axis flex-like arrangement.", stack_layout(axis="vertical", gap="md", padding={"all": "sm"}, align="center", justify="spaceBetween", grow=True))
    layout_case("grid", "Root using LayoutSpec::Grid — a two-dimensional track arrangement.", grid_layout(columns=[{"fraction": 1}, {"fixed": "lg"}], rows=["auto"], column_gap="sm", row_gap="none", padding={"each": {"top": "xs", "right": "sm", "bottom": "xs", "left": "sm"}}, align="stretch", justify="start"))
    layout_case("overlay", "Root using LayoutSpec::Overlay — anchored, dismissible positioning context.", overlay_layout(anchor="bottomEnd", inset={"symmetric": {"vertical": "md", "horizontal": "lg"}}, dismissible=True))
    layout_case("scroll", "Root using LayoutSpec::Scroll — vertical overflow scrolling.", scroll_layout(axes="vertical", padding={"all": "none"}, sizing="fill"))
    layout_case("absolute", "Root using LayoutSpec::Absolute — freeform positioning outside normal flow.", absolute_layout(sizing_width={"fixed": "xl"}, sizing_height="hug"))

    # nesting: Stack > Grid > Scroll > Leaf, four levels deep, one child chain.
    leaf = node(3, "#0", c_text("Deepest"), layout=leaf_layout())
    scroll = node(2, "#0", c_container(role="plain"), layout=scroll_layout(axes="both"), children=[3])
    grid = node(1, "#0", c_container(role="plain"), layout=grid_layout(columns=["auto"], rows=["auto"]), children=[2])
    root = node(0, "#0", c_container(role="plain"), layout=stack_layout(), children=[1])
    nodes = [root, grid, scroll, leaf]
    snap = snapshot("conformance.layout.nesting", 0, 0, nodes)
    emit(G, "nesting", snap, "snapshot")
    expect(G, "nesting", "layout", "Four-level LayoutSpec nesting: Stack > Grid > Scroll > Leaf.", "accept", limits=None, tree={"root": 0, "nodeCount": 4, "shape": tree_shape(nodes)}, accessibility=accessibility_shape(nodes), actionIds=action_ids_of(nodes))
#endregion 📐️layout fixtures


#region ♿️accessibility fixtures
def gen_accessibility():
    G = "♿️accessibility"

    def a11y_case(slug, description, comp, accessibility, layout=None):
        n = node(0, "#0", comp, layout=layout, accessibility=accessibility)
        snap = snapshot(f"conformance.a11y.{slug}", 0, 0, [n])
        emit(G, slug, snap, "snapshot")
        expect(G, slug, "accessibility", description, "accept", limits=None, tree={"root": 0, "nodeCount": 1, "shape": tree_shape([n])}, accessibility=accessibility_shape([n]), actionIds=action_ids_of([n]))

    a11y_case("labelled", "A Toggle whose accessible name is set explicitly, distinct from its visible icon-only chrome.", c_toggle(True, icon="bell"), accessibility_spec(label="Notifications enabled"))
    a11y_case("described", "A Slider carrying an accessible description in addition to its (absent) label.", c_slider(0.75, unit="%"), accessibility_spec(description="Playback volume, zero to one hundred percent."))
    a11y_case("live-region", "A Text node in an assertive ARIA live region — updates are announced immediately.", c_text("3 errors found"), accessibility_spec(live="assertive"))
    a11y_case("shortcut", "A Button carrying a keyboard shortcut hint.", c_button("Save", icon="save"), accessibility_spec(label="Save", shortcut="Ctrl+S"))
    a11y_case("decorative-image", "An Image marked hidden/decorative — built via ImageBuilder::decorative(), so it carries no alt text and is hidden from the accessibility tree.", c_image("atlas://divider"), accessibility_spec(hidden=True))
#endregion ♿️accessibility fixtures


#region 🩹️patch fixtures
def gen_patch():
    G = "🩹️patch"
    SURFACE = "conformance.patch"

    def base_two_node():
        root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1])
        a = node(1, "a", c_text("A"))
        return [root, a]

    def patch_case(slug, description, base_nodes, ops, result_nodes, root_id=0, extra_limits=None):
        base_snap = snapshot(SURFACE, 0, 0, base_nodes)
        emit(G, slug, base_snap, "snapshot")
        p = patch(SURFACE, 0, 1, ops)
        emit(G, slug, p, "patch")
        expect(G, slug, "patch", description, "accept", limits=extra_limits, baseRevision=0, resultRevision=1, tree={"root": root_id, "nodeCount": len(result_nodes), "shape": tree_shape(result_nodes)}, accessibility=accessibility_shape(result_nodes), actionIds=action_ids_of(result_nodes))

    # 1. Upsert — replaces an EXISTING node's whole record (not merely its component, which is what
    # distinguishes it from SetComponent below): key, layout and style all change in one op, and
    # since id 1 stays a child of root the tree stays reachable with no companion op needed. A
    # same-id Upsert never fails UnknownNode — it inserts OR overwrites unconditionally.
    base = base_two_node()
    replaced = node(1, "a-replaced", c_button("Now upserted", icon="star"), accessibility=accessibility_spec(label="Now upserted"))
    patch_case("upsert", "UiPatchOp::Upsert overwrites node 1's entire record in place — key, component, layout and style all change together, unlike SetComponent's single-field update.", base, [op_upsert(replaced)], [base[0], replaced])

    # 2. SetComponent — touches ONLY .component; accessibility/layout/style/bindings on node 1 stay
    # exactly as the base snapshot left them (default {}), which is the point of contrast with Upsert
    # above (whole-record replace) and with the button builder (which would auto-derive a label the
    # SetComponent op itself never sets).
    base = base_two_node()
    changed = dict(base[1], component=c_button("Now a button", icon="star"))
    patch_case("set-component", "UiPatchOp::SetComponent swaps node 1's Component from Text to Button and touches nothing else — its accessibility stays the base snapshot's default {}, unlike the button() builder's auto-derived label.", base, [op_set_component(1, c_button("Now a button", icon="star"))], [base[0], changed])

    # 3. SetLayout
    base = base_two_node()
    changed = dict(base[1], layout=leaf_layout(width="fill", height="fill"))
    patch_case("set-layout", "UiPatchOp::SetLayout replaces node 1's LayoutSpec.", base, [op_set_layout(1, leaf_layout(width="fill", height="fill"))], [base[0], changed])

    # 4. SetActivity
    base = base_two_node()
    changed = dict(base[1], activity="loading", disabled=True)
    patch_case("set-activity", "UiPatchOp::SetActivity moves node 1 to Activity::Loading and disables it in one op.", base, [op_set_activity(1, "loading", True)], [base[0], changed])

    # 5. SetChildren — adds a second child under root.
    base = base_two_node()
    extra = node(2, "b", c_text("B"))
    new_root = dict(base[0], children=[1, 2])
    patch_case("set-children", "UiPatchOp::SetChildren adds node 2 as root's second child (node 2 itself must already exist, so this patch also Upserts it).", base, [op_upsert(extra), op_set_children(0, [1, 2])], [new_root, base[1], extra])

    # 6. SetStyle
    base = base_two_node()
    changed = dict(base[1], style=style_spec(tone="danger", variant="outline"))
    patch_case("set-style", "UiPatchOp::SetStyle replaces node 1's StyleSpec wholesale.", base, [op_set_style(1, style_spec(tone="danger", variant="outline"))], [base[0], changed])

    # 7. SetAccessibility
    base = base_two_node()
    changed = dict(base[1], accessibility=accessibility_spec(shortcut="Ctrl+S", live="polite"))
    patch_case("set-accessibility", "UiPatchOp::SetAccessibility replaces node 1's AccessibilitySpec wholesale.", base, [op_set_accessibility(1, accessibility_spec(shortcut="Ctrl+S", live="polite"))], [base[0], changed])

    # 8. SetBindings
    base = base_two_node()
    bindings = [action_binding("activate", "scope", "name")]
    changed = dict(base[1], bindings=bindings)
    patch_case("set-bindings", "UiPatchOp::SetBindings replaces node 1's binding list wholesale.", base, [op_set_bindings(1, bindings)], [base[0], changed])

    # 9. SetMenu — attaches a menu; the op's payload is itself an Option, so `menu: None` (detach)
    # is exercised separately by document.rs's own `every_patch_op_variant_round_trips` test, not
    # re-demonstrated here.
    base = base_two_node()
    changed = dict(base[1], menu=menu_ref("menu"))
    patch_case("set-menu", "UiPatchOp::SetMenu attaches a resolved MenuRef to node 1.", base, [op_set_menu(1, menu_ref("menu"))], [base[0], changed])

    # 10. Remove — subtree removal, deletes a whole orphaned branch.
    root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1])
    mid = node(1, "mid", c_container(role="group"), layout=stack_layout(), children=[2, 3])
    leaf_a = node(2, "a", c_text("A"))
    leaf_b = node(3, "b", c_text("B"))
    base = [root, mid, leaf_a, leaf_b]
    new_root = dict(root, children=[])
    patch_case("remove-subtree", "UiPatchOp::Remove deletes node 1 and its whole orphaned subtree (nodes 2 and 3 disappear too).", base, [op_remove(1), op_set_children(0, [])], [new_root])

    # 11. SetRoot — root swap. SetRoot alone would leave the old root present-but-unreachable, which
    # validate_state's own DanglingRoot check forbids (see rejection/quota-depth's sibling case for
    # that same walk) — so a *valid* root swap detaches the new root from the old one first and
    # discards the old root in the same patch: SetChildren clears node 0's children, SetRoot repoints
    # to node 1, Remove deletes the now-childless (so non-cascading) node 0.
    root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1])
    alt_root = node(1, "alt-root", c_container(role="plain"), layout=stack_layout(), children=[])
    base = [root, alt_root]
    patch_case("set-root", "UiPatchOp::SetRoot repoints the document root from node 0 to the already-existing node 1, paired with SetChildren+Remove to detach and discard the old root in the same patch — SetRoot alone would leave node 0 dangling, which validate_state forbids.", base, [op_set_children(0, []), op_set_root(1), op_remove(0)], [alt_root], root_id=1)

    # 12. Reorder — SetChildren with the same three children, order reversed (distinct from set-children's add case).
    a = node(1, "a", c_text("A"))
    b = node(2, "b", c_text("B"))
    c = node(3, "c", c_text("C"))
    root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1, 2, 3])
    base = [root, a, b, c]
    new_root = dict(root, children=[3, 2, 1])
    patch_case("reorder-children", "UiPatchOp::SetChildren reverses root's existing three children — every id is preserved, only order changes.", base, [op_set_children(0, [3, 2, 1])], [new_root, a, b, c])
#endregion 🩹️patch fixtures


#region 🚫️rejection fixtures
def gen_rejection():
    G = "🚫️rejection"
    SURFACE = "conformance.rejection"

    def base_two_node():
        root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1])
        a = node(1, "a", c_text("A"))
        return [root, a]

    # 1. Stale base_revision — patch names a base_revision that does not match the receiver's current revision (0).
    base = base_two_node()
    base_snap = snapshot(SURFACE, 0, 0, base)
    emit(G, "stale-base-revision", base_snap, "snapshot")
    p = patch(SURFACE, 99, 100, [])
    emit(G, "stale-base-revision", p, "patch")
    expect(G, "stale-base-revision", "rejection", "A patch whose base_revision (99) does not match the receiver's actual revision (0) is rejected whole before any op runs.", "reject", limits=None, baseRevision=0, patchRejection=r_revision_mismatch(0, 99))

    # 2. Dangling child reference — Upsert names a child id that has no record (OrphanChild).
    base = base_two_node()
    emit(G, "dangling-child", snapshot(SURFACE, 0, 0, base), "snapshot")
    bad_root = dict(base[0], children=[1, 404])
    p = patch(SURFACE, 0, 1, [op_upsert(bad_root)])
    emit(G, "dangling-child", p, "patch")
    expect(G, "dangling-child", "rejection", "Root's children names id 404, which has no record — rejected as OrphanChild; the receiver's state is left byte-for-byte unchanged.", "reject", limits=None, baseRevision=0, patchRejection=r_invariant_violated([v_orphan(0, 404)]))

    # 3. Cycle — a non-root node's children is set to include an ancestor.
    root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1])
    a = node(1, "a", c_container(role="plain"), layout=stack_layout(), children=[])
    base = [root, a]
    emit(G, "cycle", snapshot(SURFACE, 0, 0, base), "snapshot")
    p = patch(SURFACE, 0, 1, [op_set_children(1, [0])])
    emit(G, "cycle", p, "patch")
    expect(G, "cycle", "rejection", "Node 1's children is set to [0], its own ancestor — node 0 (or 1) is reachable from itself, rejected as Cycle.", "reject", limits=None, baseRevision=0, patchRejection=r_invariant_violated([v_cycle(0)]))

    # 4. Duplicate sibling key — Upsert reuses a key already held by a sibling.
    root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1, 2])
    a = node(1, "same", c_text("A"))
    b = node(2, "other", c_text("B"))
    base = [root, a, b]
    emit(G, "duplicate-sibling-key", snapshot(SURFACE, 0, 0, base), "snapshot")
    dup = node(2, "same", c_text("B'"))
    p = patch(SURFACE, 0, 1, [op_upsert(dup)])
    emit(G, "duplicate-sibling-key", p, "patch")
    expect(G, "duplicate-sibling-key", "rejection", "Upserting node 2 with key 'same' collides with sibling node 1's key — rejected as DuplicateSiblingKey.", "reject", limits=None, baseRevision=0, patchRejection=r_invariant_violated([v_dup_key(0, "same")]))

    # 5. Quota: max_nodes — single-node base so the arithmetic is unambiguous: 1 existing + 1
    # Upserted = 2, exceeding max_nodes=1. validate_state's node-count check short-circuits before
    # the reachability walk, so NodeQuota is the ONLY violation even though the new node is never
    # attached as anyone's child.
    base = [node(0, "root", c_container(role="plain"))]
    emit(G, "quota-nodes", snapshot(SURFACE, 0, 0, base), "snapshot")
    extra = node(1, "a", c_text("A"))
    p = patch(SURFACE, 0, 1, [op_upsert(extra)])
    emit(G, "quota-nodes", p, "patch")
    lim = limits(max_nodes=1)
    expect(G, "quota-nodes", "rejection", "Upserting a second node exceeds UiDocumentLimits::max_nodes (set to 1 for this fixture) — rejected as NodeQuota; the check short-circuits before the reachability walk, so this is the only violation reported even though the new node is unattached.", "reject", limits=lim, baseRevision=0, patchRejection=r_invariant_violated([v_node_quota(2, 1)]))

    # 6. Quota: max_depth
    root = node(0, "root", c_container(role="plain"), layout=stack_layout(), children=[1])
    mid = node(1, "mid", c_container(role="plain"), layout=stack_layout(), children=[])
    base = [root, mid]
    emit(G, "quota-depth", snapshot(SURFACE, 0, 0, base), "snapshot")
    p = patch(SURFACE, 0, 1, [op_set_activity(1, "idle", False)])
    emit(G, "quota-depth", p, "patch")
    lim = limits(max_depth=0)
    expect(G, "quota-depth", "rejection", "Node 1 sits one edge below root, exceeding UiDocumentLimits::max_depth (set to 0 for this fixture) — rejected as DepthQuota.", "reject", limits=lim, baseRevision=0, patchRejection=r_invariant_violated([v_depth_quota(1, 1, 0)]))

    # 7. Quota: max_children (per-patch quota, enforced directly by apply_patch, not via validate).
    base = base_two_node()
    emit(G, "quota-children", snapshot(SURFACE, 0, 0, base), "snapshot")
    p = patch(SURFACE, 0, 1, [op_set_children(0, [1, 2])])
    emit(G, "quota-children", p, "patch")
    lim = limits(max_children=1)
    expect(G, "quota-children", "rejection", "SetChildren names 2 children, exceeding UiDocumentLimits::max_children (set to 1 for this fixture) — rejected directly by apply_patch as QuotaExceeded(Children), before the shadow draft is even validated.", "reject", limits=lim, baseRevision=0, patchRejection=r_quota_exceeded("children", 2, 1))

    # 8. Quota: max_text_bytes
    base = base_two_node()
    emit(G, "quota-text-bytes", snapshot(SURFACE, 0, 0, base), "snapshot")
    big_text = c_text("way too long")
    p = patch(SURFACE, 0, 1, [op_set_component(0, big_text)])
    emit(G, "quota-text-bytes", p, "patch")
    lim = limits(max_text_bytes=4)
    expect(G, "quota-text-bytes", "rejection", "SetComponent's new Text value is 12 bytes, exceeding UiDocumentLimits::max_text_bytes (set to 4 for this fixture) — rejected as QuotaExceeded(TextBytes).", "reject", limits=lim, baseRevision=0, patchRejection=r_quota_exceeded("textBytes", 12, 4))

    # 9. Quota: max_patch_ops
    base = base_two_node()
    emit(G, "quota-patch-ops", snapshot(SURFACE, 0, 0, base), "snapshot")
    extra = node(2, "b", c_text("B"))
    extra2 = node(3, "c", c_text("C"))
    p = patch(SURFACE, 0, 1, [op_upsert(extra), op_upsert(extra2)])
    emit(G, "quota-patch-ops", p, "patch")
    lim = limits(max_patch_ops=1)
    expect(G, "quota-patch-ops", "rejection", "The patch carries 2 ops, exceeding UiDocumentLimits::max_patch_ops (set to 1 for this fixture) — rejected as QuotaExceeded(PatchOps) before any op is applied.", "reject", limits=lim, baseRevision=0, patchRejection=r_quota_exceeded("patchOps", 2, 1))

    # 10. Quota: max_patch_bytes
    base = base_two_node()
    emit(G, "quota-patch-bytes", snapshot(SURFACE, 0, 0, base), "snapshot")
    big = node(2, "b", c_text("a fairly long piece of text"))
    p = patch(SURFACE, 0, 1, [op_upsert(big)])
    emit(G, "quota-patch-bytes", p, "patch")
    lim = limits(max_patch_bytes=4)
    expect(G, "quota-patch-bytes", "rejection", "The Upsert's estimated wire cost exceeds UiDocumentLimits::max_patch_bytes (set to 4 for this fixture) — rejected as QuotaExceeded(PatchBytes) before any op is applied.", "reject", limits=lim, baseRevision=0, patchRejection=r_quota_exceeded("patchBytes", 44, 4))
#endregion 🚫️rejection fixtures


if __name__ == "__main__":
    gen_component()
    gen_composite()
    gen_layout()
    gen_accessibility()
    gen_patch()
    gen_rejection()
    print(f"wrote {len(written)} files")

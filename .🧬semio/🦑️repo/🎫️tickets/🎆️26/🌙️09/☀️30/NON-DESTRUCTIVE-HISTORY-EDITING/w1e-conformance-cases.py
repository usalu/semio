"""🧪️ Writes the six W1-E conformance corpus cases (snapshot + expectation) and registers them in the catalog.

Run from the repo root: `python3 .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/w1e-conformance-cases.py`.
"""
import json
import os

ROOT = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance"


def stack(axis, gap="none", justify="start", wrap=False):
    return {"kind": "stack", "axis": axis, "gap": gap, "padding": {"all": "none"}, "align": "stretch", "justify": justify, "grow": False, "wrap": wrap}


LEAF = {"kind": "leaf", "width": "hug", "height": "hug"}


def action(name, trigger="activate", args=None):
    binding = {"trigger": trigger, "action": {"scope": "app", "name": name, "version": 1}}
    if args is not None:
        binding["args"] = args
    return binding


def node(id, key, component, layout=LEAF, style=None, accessibility=None, bindings=None, children=None):
    record = {"id": id, "key": key, "component": component, "layout": layout, "style": style or {}, "activity": "idle", "accessibility": accessibility or {}}
    if bindings:
        record["bindings"] = bindings
    if children:
        record["children"] = children
    return record


CASES = {
    ("🧩️component", "slider-with-snaps", "🧲️slider-with-snaps"): (
        "A Slider with three detents (snaps) and a millimetre unit — every renderer paints one tick per snap.",
        [node(0, "#0", {"type": "slider", "value": 5.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "mm", "snaps": [2.5, 5.0, 7.5]}, accessibility={"label": "Offset"}, bindings=[action("setOffset", "change")])],
    ),
    ("🧩️component", "stepper-precision", "🎯️stepper-precision"): (
        "A bounded NumberStepper stepping by a quarter and showing two fraction digits, with absolute and relative bindings.",
        [node(0, "#0", {"type": "numberStepper", "value": 2.5, "step": 0.25, "uniform": True, "min": 0.0, "max": 10.0, "precision": 2}, accessibility={"label": "Scale"}, bindings=[action("setScale", "change"), action("nudgeScale", "delta")])],
    ),
    ("🖥️composite", "vector-input", "🧭️vector-input"): (
        "The `vector_input` recipe: a labelled group of keyed number fields sharing unit, step, precision, detents and commit convention.",
        [
            node(0, "#0", {"type": "container", "role": "group", "label": "Offset"}, layout=stack("horizontal", gap="sm", wrap=True), children=[1, 3]),
            node(1, "dx.axis", {"type": "container", "role": "field", "label": "X", "description": "mm"}, layout=stack("vertical"), children=[2]),
            node(2, "dx", {"type": "input", "kind": "number", "value": "1.3", "commit": "blur", "step": 0.5, "precision": 1, "snaps": [0.0]}, accessibility={"label": "X"}, bindings=[action("setDx", "commit")]),
            node(3, "dy.axis", {"type": "container", "role": "field", "label": "Y", "description": "mm"}, layout=stack("vertical"), children=[4]),
            node(4, "dy", {"type": "input", "kind": "number", "value": "-3.0", "commit": "blur", "step": 0.5, "precision": 1, "snaps": [0.0]}, accessibility={"label": "Y"}, bindings=[action("setDy", "commit")]),
        ],
    ),
    ("🖥️composite", "color-input", "🎨️color-input"): (
        "The `color_input` recipe: a labelled group holding the swatch, the hex field and the alpha slider of an sRGB colour with alpha.",
        [
            node(0, "#0", {"type": "container", "role": "group", "label": "Tint"}, layout=stack("horizontal", gap="sm", wrap=True), children=[1, 2, 3]),
            node(1, "swatch", {"type": "input", "kind": "color", "value": "#ff8000"}, accessibility={"label": "Tint"}, bindings=[action("setTint", "change")]),
            node(2, "hex", {"type": "input", "value": "#ff800080", "commit": "blur"}, accessibility={"label": "Hex"}, bindings=[action("setTint", "change")]),
            node(3, "alpha", {"type": "slider", "value": 0.5, "min": 0.0, "max": 1.0, "step": 0.01}, accessibility={"label": "Alpha"}, bindings=[action("setTintAlpha", "change")]),
        ],
    ),
    ("🖥️composite", "reference-list", "🧷️reference-list"): (
        "The `reference_list` recipe: reference chips that remove on activation, a use-current-selection button and a candidate list.",
        [
            node(0, "#0", {"type": "container", "role": "group", "label": "Targets"}, layout=stack("vertical", gap="xs"), children=[1, 4]),
            node(1, "chips", {"type": "container", "role": "toolbar", "label": "Targets"}, layout=stack("horizontal", gap="xs", wrap=True), children=[2, 3]),
            node(2, "piece-3", {"type": "button", "icon": "x", "label": "Piece 3"}, style={"variant": "outline"}, accessibility={"label": "Remove Piece 3"}, bindings=[action("removeTarget")]),
            node(3, "piece-7", {"type": "button", "icon": "x", "label": "Piece 7"}, style={"variant": "outline"}, accessibility={"label": "Remove Piece 7"}, bindings=[action("removeTarget")]),
            node(4, "actions", {"type": "container"}, layout=stack("horizontal", gap="xs", wrap=True), children=[5, 6]),
            node(5, "useSelection", {"type": "button", "icon": "crosshair", "label": "Use selection"}, accessibility={"label": "Use selection"}, bindings=[action("useSelection")]),
            node(6, "candidates", {"type": "select", "value": "", "items": [{"value": "piece-9", "label": "Piece 9"}], "placeholder": "Add target"}, accessibility={"label": "Add target"}, bindings=[action("addTarget", "change")]),
        ],
    ),
    ("🖥️composite", "dialog-choices", "🔀️dialog-choices"): (
        "A finalize prompt with a staged name field, a destructive choice described by its consequence, and the primary submit.",
        [
            node(0, "#0", {"type": "container"}, layout={"kind": "overlay", "anchor": "center", "inset": {"all": "none"}, "dismissible": True}, accessibility={"label": "Finish editing history", "description": "Keep the edit as a new alternative or overwrite the existing history."}, children=[1, 2, 3, 5, 6]),
            node(1, "title", {"type": "text", "value": "Finish editing history", "emphasize": True}),
            node(2, "body", {"type": "text", "value": "Keep the edit as a new alternative or overwrite the existing history."}),
            node(3, "name", {"type": "container", "role": "field", "label": "Alternative name", "required": True}, layout=stack("vertical"), children=[4]),
            node(4, "name.input", {"type": "input", "value": "Variant B"}, accessibility={"label": "Alternative name"}),
            node(5, "overwrite.description", {"type": "text", "value": "Replaces the edited mutations in every alternative that contains them."}),
            node(6, "actions", {"type": "container", "role": "toolbar"}, layout=stack("horizontal", justify="spaceBetween"), children=[7, 8, 9]),
            node(7, "cancel", {"type": "button", "icon": "x", "label": "Back"}, style={"variant": "ghost"}, accessibility={"label": "Back"}, bindings=[action("historyEditBack")]),
            node(8, "overwrite", {"type": "button", "icon": "triangle-alert", "label": "Overwrite"}, style={"tone": "danger"}, accessibility={"label": "Overwrite", "description": "Replaces the edited mutations in every alternative that contains them."}, bindings=[action("historyEditCommit", args={"choice": "overwrite"})]),
            node(9, "submit", {"type": "button", "icon": "check", "label": "New alternative"}, style={"tone": "primary"}, accessibility={"label": "New alternative"}, bindings=[action("historyEditCommit")]),
        ],
    ),
}


def expectation(case, kind, description, nodes):
    return {
        "case": case,
        "kind": kind,
        "description": description,
        "outcome": "accept",
        "limits": None,
        "tree": {"root": 0, "nodeCount": len(nodes), "shape": [{"id": n["id"], "key": n["key"], "type": n["component"]["type"], "children": n.get("children", [])} for n in nodes]},
        "accessibility": [{"id": n["id"], "label": n["accessibility"].get("label"), "description": n["accessibility"].get("description"), "live": "off", "shortcut": None, "hidden": False} for n in nodes],
        "actionIds": [f'{b["action"]["scope"]}.{b["action"]["name"]}@{b["action"]["version"]}' for n in nodes for b in n.get("bindings", [])],
    }


def write(path, value):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def main():
    catalog_path = os.path.join(ROOT, "📇️catalog.json")
    with open(catalog_path, encoding="utf-8") as handle:
        catalog = json.load(handle)
    for (group, case, directory), (description, nodes) in CASES.items():
        folder = os.path.join(ROOT, group, directory)
        os.makedirs(folder, exist_ok=True)
        kind = "component" if group == "🧩️component" else "composite"
        write(os.path.join(folder, "📸️snapshot.json"), {"surface": f"conformance.{kind}.{case}", "revision": 0, "root": 0, "nodes": nodes, "layoutEpoch": 0})
        write(os.path.join(folder, "🎯️expect.json"), expectation(case, kind, description, nodes))
        if case in catalog["groups"][group]["cases"]:
            continue
        with open(catalog_path, encoding="utf-8") as handle:
            text = handle.read()
        entries = sorted([*catalog["groups"][group]["cases"].items(), (case, directory)])
        position = entries.index((case, directory))
        if position == 0:
            following = entries[1]
            anchor = f'        "{following[0]}": "{following[1]}"'
            assert text.count(anchor) == 1, anchor
            text = text.replace(anchor, f'        "{case}": "{directory}",\n{anchor}', 1)
        else:
            previous = entries[position - 1]
            anchor = f'        "{previous[0]}": "{previous[1]}"'
            assert text.count(anchor) == 1, anchor
            text = text.replace(anchor, f'{anchor},\n        "{case}": "{directory}"', 1)
        with open(catalog_path, "w", encoding="utf-8") as handle:
            handle.write(text)
        catalog["groups"][group]["cases"][case] = directory


main()

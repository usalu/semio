"""💬️ S3-W1E / S4-UI: writes the conformance case `🖥️composite/💬️row-semantics` (coordinator decisions on tree rows, both renderers).

A property row's description is always exposed and names its control (`aria-describedby` on React, the control's
description in the wgpu ARIA mirror); a disabled row action stays focusable (`aria-disabled`), never runs, names its
reason as its description and shows that reason as visible text on hover, keyboard focus and press (audit W1E-1); an
option row of a single-choice list exposes its choice state as the row's selected state (audit W1E-2). Expectations under
`rowSemantics`; registers the case in the catalog once. Run from the repo root.
"""
import json
import os

ROOT = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance"
GROUP, CASE, DIRECTORY = "🖥️composite", "row-semantics", "💬️row-semantics"
STACK = {"kind": "stack", "axis": "vertical", "gap": "none", "padding": {"all": "none"}, "align": "stretch", "justify": "start", "grow": False, "wrap": False}
LEAF = {"kind": "leaf", "width": "hug", "height": "hug"}
REFUSAL = "Must be greater than 0"
REASON = "Not possible right now"


def node(id, key, component, layout=LEAF, accessibility=None, bindings=None, children=None):
    record = {"id": id, "key": key, "component": component, "layout": layout, "style": {}, "activity": "idle", "accessibility": accessibility or {}}
    if bindings:
        record["bindings"] = bindings
    if children:
        record["children"] = children
    return record


NODES = [
    node(0, "#0", {"type": "tree"}, layout=STACK, children=[1]),
    node(1, "inputs", {"type": "treeSection", "label": "Inputs", "defaultOpen": True}, layout=STACK, children=[2, 4, 5, 6]),
    node(2, "scale.row", {"type": "treeItem", "label": "Scale", "description": REFUSAL}, layout=STACK, accessibility={"label": "Scale"}, children=[3]),
    node(3, "scale", {"type": "slider", "value": 1.0, "min": 0.1, "max": 10.0, "step": 0.01}, accessibility={"label": "Scale"}, bindings=[{"trigger": "change", "action": {"scope": "app", "name": "setScale", "version": 1}}]),
    node(4, "entry.7", {"type": "treeItem", "label": "Move", "icon": "move", "rowActions": [{"icon": "pencil", "label": "Edit", "verb": "historyEditBegin", "disabled": True, "reason": REASON}], "target": {"scope": "app", "version": 1}}, layout=STACK, accessibility={"label": "Move"}),
    node(5, "interpolation.option.0.row", {"type": "treeItem", "label": "Linear", "icon": "check", "selected": True}, layout=STACK, accessibility={"label": "Linear"}),
    node(6, "interpolation.option.1.row", {"type": "treeItem", "label": "Smooth", "icon": "circle", "selected": False}, layout=STACK, accessibility={"label": "Smooth"}),
]

EXPECT = {
    "case": CASE,
    "kind": "composite",
    "description": "Tree row semantics every renderer shares: a property row's description (here a refused value's reason) is always shown and named by its control (React `aria-describedby`, the wgpu ARIA mirror's control description); a disabled row action stays focusable with `aria-disabled`, never runs, names its reason as its description, and shows that same reason as visible text anchored to the action while it is hovered, keyboard-focused or pressed (`revealReason.on`) until the pointer leaves, focus moves away or Escape (`revealReason.off`); an option row of a single-choice list exposes its choice (`selected`) as the row's selected state (React `aria-selected`, the wgpu ARIA mirror's `selected`) and paints the chosen row selected, never by its icon alone.",
    "outcome": "accept",
    "limits": None,
    "tree": {"root": 0, "nodeCount": len(NODES), "shape": [{"id": n["id"], "key": n["key"], "type": n["component"]["type"], "children": n.get("children", [])} for n in NODES]},
    "accessibility": [{"id": n["id"], "label": n["accessibility"].get("label"), "description": None, "live": "off", "shortcut": None, "hidden": False} for n in NODES],
    "actionIds": ["app.setScale@1", "app.historyEditBegin@1"],
    "rowSemantics": {
        "describedControls": [{"row": 2, "control": 3, "description": REFUSAL}],
        "disabledRowActions": [{"row": 4, "index": 0, "label": "Edit: Move", "reason": REASON, "revealReason": {"on": ["hover", "focus", "press"], "off": ["leave", "blur", "escape"]}}],
        "selectedRows": [{"row": 5, "selected": True}, {"row": 6, "selected": False}],
    },
}


def write(path, value):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def main():
    folder = os.path.join(ROOT, GROUP, DIRECTORY)
    os.makedirs(folder, exist_ok=True)
    write(os.path.join(folder, "📸️snapshot.json"), {"surface": f"conformance.composite.{CASE}", "revision": 0, "root": 0, "nodes": NODES, "layoutEpoch": 0})
    write(os.path.join(folder, "🎯️expect.json"), EXPECT)
    catalog_path = os.path.join(ROOT, "📇️catalog.json")
    with open(catalog_path, encoding="utf-8") as handle:
        text = handle.read()
    catalog = json.loads(text)
    cases = catalog["groups"][GROUP]["cases"]
    if CASE in cases:
        return
    entries = sorted([*cases.items(), (CASE, DIRECTORY)])
    position = entries.index((CASE, DIRECTORY))
    if position == 0:
        following = entries[1]
        anchor = f'        "{following[0]}": "{following[1]}"'
        assert text.count(anchor) == 1, anchor
        text = text.replace(anchor, f'        "{CASE}": "{DIRECTORY}",\n{anchor}', 1)
    else:
        previous = entries[position - 1]
        anchor = f'        "{previous[0]}": "{previous[1]}"'
        assert text.count(anchor) == 1, anchor
        text = text.replace(anchor, f'{anchor},\n        "{CASE}": "{DIRECTORY}"', 1)
    with open(catalog_path, "w", encoding="utf-8") as handle:
        handle.write(text)


main()
print("[row-semantics] written")

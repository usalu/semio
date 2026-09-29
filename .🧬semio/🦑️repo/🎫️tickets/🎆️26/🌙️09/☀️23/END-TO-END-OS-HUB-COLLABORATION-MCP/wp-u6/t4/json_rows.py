#!/usr/bin/env python3
"""🎯️ U6 T4 helper: migrates JSON UI documents to the row-target contract — a TreeItem/TableRow's row actions name only
a `verb`, their shared scope/version/args become the row's ONE `target`, and a record `Trigger::Activate` binding becomes
the target's `activation` verb. Refuses (reports) a row whose verbs do not share one target.

Usage: json_rows.py [--write] <file.json>..."""
import json
import sys


def migrate_node(node, conflicts, where):
    component = node.get("component")
    if not isinstance(component, dict) or component.get("type") not in ("treeItem", "tableRow"):
        return False
    actions = component.get("rowActions") or []
    activations = [binding for binding in node.get("bindings", []) if binding.get("trigger") == "activate"]
    if not actions and not activations:
        return False
    bindings = [action["action"] for action in actions] + activations
    targets = {json.dumps({"scope": b["action"]["scope"], "version": b["action"]["version"], "args": b.get("args")}, sort_keys=True) for b in bindings}
    if len(targets) != 1 or any(b.get("capability") for b in bindings) or len(activations) > 1:
        conflicts.append(f"{where}: {node.get('key')} verbs do not share one target: {sorted(targets)}")
        return False
    first = bindings[0]
    target = {"scope": first["action"]["scope"], "version": first["action"]["version"]}
    if first.get("args") is not None:
        target["args"] = first["args"]
    migrated = {}
    for key, value in component.items():
        if key == "rowActions":
            migrated[key] = [{**{k: v for k, v in action.items() if k in ("icon", "label")}, "verb": action["action"]["action"]["name"], **({"placement": action["placement"]} if "placement" in action else {})} for action in actions]
        else:
            migrated[key] = value
    if activations:
        target["activation"] = activations[0]["action"]["name"]
        node["bindings"] = [binding for binding in node["bindings"] if binding.get("trigger") != "activate"]
        if not node["bindings"]:
            del node["bindings"]
    migrated["target"] = target
    component.clear()
    component.update(migrated)
    return True


def walk(value, conflicts, where, changed):
    if isinstance(value, dict):
        if migrate_node(value, conflicts, where):
            changed.append(value.get("key"))
        for child in value.values():
            walk(child, conflicts, where, changed)
    elif isinstance(value, list):
        for child in value:
            walk(child, conflicts, where, changed)


def main():
    write = "--write" in sys.argv
    status = 0
    for path in [arg for arg in sys.argv[1:] if arg != "--write"]:
        raw = open(path, encoding="utf-8").read()
        document = json.loads(raw)
        indent = 2 if json.dumps(document, indent=2, ensure_ascii=False) + "\n" == raw else None
        conflicts, changed = [], []
        walk(document, conflicts, path, changed)
        for conflict in conflicts:
            print("CONFLICT", conflict)
            status = 1
        if changed and indent is None:
            print("FORMAT", path, "is not indent-2 JSON; migrate by hand")
            status = 1
            continue
        if changed:
            print(("WRITE " if write else "DRY ") + path, changed)
            if write:
                open(path, "w", encoding="utf-8").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
    return status


if __name__ == "__main__":
    sys.exit(main())

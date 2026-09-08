#!/usr/bin/env python3
"""🗣️ WP4b helper — normalise retained-command lane/disposition/execution spellings in plugin data.

`framework.ui` (`🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json`) publishes one kebab vocabulary per
axis (`retainedCommandLane`, `retainedCommandDisposition`, `retainedCommandExecution`). Plugin data
still spells the same axes in the Rust variant's PascalCase (or a camelCase halfway house). This
rewrites the *values* of the retained-command row keys only, never any other `status`/`lane` key:
a node qualifies when it carries a retained-command row shape.

Usage: wp4b-retained-vocabulary.py plan|apply [<root>…]
"""
import json
import os
import sys
from collections import Counter, OrderedDict

LANE = {"Artifact": "artifact", "Config": "config", "HostOnly": "host-only", "hostOnly": "host-only",
        "Transient": "transient", "Child": "child", "Draft": "draft", "Presence": "presence",
        "Document": "document", "hostonly": "host-only"}
DISPOSITION = {"Migrated": "migrated", "BatchOnlyPendingRewrite": "batch-only-pending-rewrite",
               "batchOnly": "batch-only-pending-rewrite", "BatchOnly": "batch-only-pending-rewrite",
               "batch-only": "batch-only-pending-rewrite", "batchOnlyPendingRewrite": "batch-only-pending-rewrite",
               "FailClosed": "fail-closed", "failClosed": "fail-closed",
               "Unclassified": "unclassified", "ForbiddenFromUi": "forbidden-from-ui", "Deleted": "deleted"}
EXECUTION = {"BoundedFirstStep": "bounded-first-step", "boundedFirstStep": "bounded-first-step",
             "Bounded": "bounded", "Batch": "batch", "Resumable": "resumable"}
LANE_KEYS = ("lane", "lanes", "publicationLane", "publicationLanes")
DISPOSITION_KEYS = ("disposition", "status", "admission", "classification")
EXECUTION_KEYS = ("execution",)


def is_row(node):
    """🧷 A retained-command row: an `id` beside a lane/disposition/execution axis, or a lane group."""
    if not isinstance(node, dict):
        return False
    axes = any(key in node for key in LANE_KEYS + EXECUTION_KEYS) or ("disposition" in node)
    if ("id" in node or "toolId" in node) and axes:
        return True
    return "lanes" in node and ("status" in node or "routes" in node)


def rewrite(node, counts):
    if isinstance(node, list):
        return [rewrite(item, counts) for item in node]
    if not isinstance(node, dict):
        return node
    row = is_row(node)
    out = OrderedDict()
    for key, value in node.items():
        table = None
        if row and key in LANE_KEYS:
            table = LANE
        elif row and key in DISPOSITION_KEYS:
            table = DISPOSITION
        elif row and key in EXECUTION_KEYS:
            table = EXECUTION
        if table is not None:
            if isinstance(value, str) and value in table:
                counts[(key, value, table[value])] += 1
                out[key] = table[value]
                continue
            if isinstance(value, list) and all(isinstance(item, str) for item in value):
                new = []
                for item in value:
                    if item in table:
                        counts[(key, item, table[item])] += 1
                        new.append(table[item])
                    else:
                        new.append(item)
                out[key] = new
                continue
        out[key] = rewrite(value, counts)
    return out


KEY_TABLES = {}


def schema_values(node, table, counts, key):
    """🔤 Rewrites the `const`/`enum` vocabulary a retained-command row property narrows with."""
    if not isinstance(node, dict):
        return node
    out = OrderedDict()
    for name, value in node.items():
        if name == "const" and isinstance(value, str) and value in table:
            counts[(key, value, table[value])] += 1
            out[name] = table[value]
        elif name == "const" and isinstance(value, list):
            new = []
            for item in value:
                if isinstance(item, str) and item in table:
                    counts[(key, item, table[item])] += 1
                    new.append(table[item])
                else:
                    new.append(item)
            out[name] = new
        elif name == "enum" and isinstance(value, list):
            new = []
            for item in value:
                if isinstance(item, str) and item in table:
                    counts[(key, item, table[item])] += 1
                    new.append(table[item])
                else:
                    new.append(item)
            out[name] = new
        elif name in ("items", "anyOf", "oneOf", "allOf", "contains", "additionalItems"):
            if isinstance(value, list):
                out[name] = [schema_values(item, table, counts, key) for item in value]
            else:
                out[name] = schema_values(value, table, counts, key)
        else:
            out[name] = value
    return out


def rewrite_schema(node, counts):
    if isinstance(node, list):
        return [rewrite_schema(item, counts) for item in node]
    if not isinstance(node, dict):
        return node
    out = OrderedDict()
    for key, value in node.items():
        if key == "properties" and isinstance(value, dict) and is_row(value):
            props = OrderedDict()
            for name, child in value.items():
                table = LANE if name in LANE_KEYS else DISPOSITION if name in DISPOSITION_KEYS else EXECUTION if name in EXECUTION_KEYS else None
                props[name] = schema_values(rewrite_schema(child, counts), table, counts, name) if table else rewrite_schema(child, counts)
            out[key] = props
            continue
        out[key] = rewrite_schema(value, counts)
    return out


def schema_document(value):
    return isinstance(value, dict) and any(key in value for key in ("$defs", "$schema", "properties", "definitions"))


def main():
    action = sys.argv[1] if len(sys.argv) > 1 else "plan"
    roots = sys.argv[2:] or ["✏️s"]
    counts = Counter()
    touched = []
    for root in roots:
        for directory, names, files in os.walk(root):
            names[:] = [name for name in names if name not in ("target", "node_modules", ".venv", "dist")]
            if "🧬️mutations" in directory.split(os.sep):
                continue
            for name in files:
                if not name.endswith(".json"):
                    continue
                path = os.path.join(directory, name)
                try:
                    with open(path, encoding="utf-8") as handle:
                        text = handle.read()
                    value = json.loads(text, object_pairs_hook=OrderedDict)
                except Exception:
                    continue
                document = schema_document(value)
                if document and action not in ("schema", "schema-apply"):
                    continue
                if not document and action in ("schema", "schema-apply"):
                    continue
                local = Counter()
                rewritten = rewrite_schema(value, local) if document else rewrite(value, local)
                if not local:
                    continue
                counts.update(local)
                touched.append((path, sum(local.values())))
                if action in ("apply", "schema-apply"):
                    with open(path, "w", encoding="utf-8") as handle:
                        json.dump(rewritten, handle, ensure_ascii=False, indent=2)
                        handle.write("\n")
    for (key, old, new), count in sorted(counts.items()):
        print(f"  {key}: {old!r} -> {new!r}  ×{count}")
    print(f"files: {len(touched)}  rewrites: {sum(counts.values())}  action: {action}")
    for path, count in touched:
        print(f"    {count:4d}  {path}")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""🎁️ W2-S stdio: the svg / xml / json `phase`/`value` leaf enums carry `#[mutation_leaf(payload = Apply)]` (W1-D). Each such
leaf schema keeps its flat root (the `Apply` content, the editable payload) and gains `$defs/<LeafEnum>` — the whole leaf wire
`{phase: apply, value: <root>} | {phase: restore, value: <artifact diff>}` — and its adjacently tagged aggregate branch
references that definition (`<leaf $id>#/$defs/<LeafEnum>`), exactly the union the `schema mutation-payloads` lint derives from
the Rust layout (glTF precedent). Leaves without a wrapper keep the plain leaf `$ref`. Idempotent; files are re-read before
they are written; leaf edits are span-surgical (`🧪️w2-s-stdio-snapshots.py`).

    python3 🧪️w2-s-stdio-wrapped.py [--dry-run]
"""
import importlib.util
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("snapshots", os.path.join(HERE, "🧪️w2-s-stdio-snapshots.py"))
snapshots = importlib.util.module_from_spec(spec)
spec.loader.exec_module(snapshots)
ART = snapshots.ART
OWNERS = ["🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base", "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base", "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base"]
WRAPPER = re.compile(r'#\[mutation_leaf\([^\]]*payload = Apply\)\]\s*#\[value\(tag = "phase", content = "value", rename_all = "camelCase"\)\]\s*pub enum (\w+)\s*\{')


def camel(name):
    return name[:1].lower() + name[1:]


def main():
    for owner in OWNERS:
        root = "%s/%s/🧬️schema" % (ART, owner)
        diff_id = json.load(open(root + "/🔺️diff/🔣️.json", encoding="utf-8"))["$id"]
        aggregate_rs = open(root + "/🧬️mutations/🦀️.rs", encoding="utf-8").read()
        body = aggregate_rs[aggregate_rs.index('#[value(tag = "mutation", content = "payload", rename_all = "camelCase")]'):]
        variants = re.findall(r"^\s{4}([A-Z]\w*)\(([\w:]+)\),", body[: body.index("\n}")], re.M)
        leaves = {}
        for name in sorted(os.listdir(root + "/🧬️mutations")):
            directory = "%s/🧬️mutations/%s" % (root, name)
            if not os.path.exists(directory + "/🔣️.json") or not os.path.exists(directory + "/🧬️schema/🔣️.json"):
                continue
            descriptor = json.load(open(directory + "/🔣️.json", encoding="utf-8"))
            source = open(directory + "/🦀️.rs", encoding="utf-8").read()
            wrapper = WRAPPER.search(source)
            leaves[descriptor["aggregateVariant"]] = (directory + "/🧬️schema/🔣️.json", wrapper.group(1) if wrapper else None)
        branches = []
        for variant, _ in variants:
            path, wrapper = leaves[variant]
            leaf_id = json.load(open(path, encoding="utf-8"))["$id"]
            if wrapper is not None:
                definition = {
                    "description": "The whole wire of %s: the editable apply payload (this document's root) or the inert restore." % wrapper,
                    "oneOf": [
                        {"type": "object", "additionalProperties": False, "required": ["phase", "value"], "properties": {"phase": {"const": "apply"}, "value": {"$ref": "#"}}},
                        {"type": "object", "additionalProperties": False, "required": ["phase", "value"], "properties": {"phase": {"const": "restore"}, "value": {"$ref": diff_id}}},
                    ],
                }

                def add(text, definition=definition, wrapper=wrapper):
                    if "$defs" not in json.loads(text):
                        return snapshots.add_member(text, [], "$defs", {wrapper: definition})
                    return snapshots.add_member(text, ["$defs"], wrapper, definition)

                snapshots.edit(path, add)
            reference = leaf_id if wrapper is None else "%s#/$defs/%s" % (leaf_id, wrapper)
            branches.append({"type": "object", "additionalProperties": False, "required": ["mutation", "payload"], "properties": {"mutation": {"const": camel(variant)}, "payload": {"$ref": reference}}})
        aggregate = root + "/🧬️mutations/🔣️.json"

        def rebuild(text, branches=branches):
            document = json.loads(text)
            if document.get("oneOf") == branches:
                return text
            return snapshots.replace_value(text, ["oneOf"], branches)

        snapshots.edit(aggregate, rebuild)


if __name__ == "__main__":
    main()

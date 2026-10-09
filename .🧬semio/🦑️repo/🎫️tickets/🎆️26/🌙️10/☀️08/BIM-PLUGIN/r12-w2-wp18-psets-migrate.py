"""🗂️ WP-18: hand-migrates the committed JSON documents of `s.bim.model@1` from one classification per element (`{system, code, title}`) to the keyed classifications per (element, system) with the
library of classification systems: every snapshot gets `classification_systems` for the system names it uses (id `cs-<slug>`, the entries are the (code, title) pairs it uses) and
`classifications` as `element -> {system id: code}`; every diff entry of `classifications` is converted the same way. Run once from the repo root: `python r12-w2-wp18-psets-migrate.py <dir>...`.
Idempotent: a document that already has the new shape is left alone.
"""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")


def slug(name):
    return "cs-" + re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")


def old_shape(value):
    return isinstance(value, dict) and "system" in value and "code" in value


def migrate_snapshot(doc):
    classifications = doc.get("classifications")
    if not classifications or not any(old_shape(value) for value in classifications.values()):
        return False
    systems = doc.get("classification_systems", {})
    migrated = {}
    for element, old in classifications.items():
        if not old_shape(old):
            migrated[element] = old
            continue
        identity = slug(old["system"])
        system = systems.setdefault(identity, {"name": old["system"], "edition": "", "entries": []})
        if not any(entry["code"] == old["code"] for entry in system["entries"]):
            system["entries"].append({"code": old["code"], "title": old.get("title", "")})
        migrated[element] = {identity: old["code"]}
    doc["classifications"] = migrated
    doc["classification_systems"] = systems
    return True


def migrate_diff(doc):
    entries = doc.get("classifications")
    if not entries or not any(old_shape(value) for value in entries.values()):
        return False
    for element, entry in list(entries.items()):
        if old_shape(entry):
            kind = entry.get("entry")
            entries[element] = {"entry": kind, slug(entry["system"]): entry["code"]}
    return True


def migrate(path):
    with open(path, encoding="utf8") as handle:
        text = handle.read()
    try:
        doc = json.loads(text)
    except ValueError:
        return False
    changed = False
    nodes = []

    def collect(node):
        if isinstance(node, dict):
            nodes.append(node)
            for value in node.values():
                collect(value)
        elif isinstance(node, list):
            for value in node:
                collect(value)

    collect(doc)
    for node in nodes:
        is_snapshot = "project" in node or node.get("schema") == "s.bim.model@1"
        if "classifications" in node and isinstance(node["classifications"], dict):
            changed |= migrate_snapshot(node) if is_snapshot else migrate_diff(node)
    if changed:
        os.remove(path)
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
    return changed


total = 0
for root in sys.argv[1:]:
    for directory, _, names in os.walk(root):
        for name in names:
            if name.endswith(".json") and migrate(os.path.join(directory, name)):
                total += 1
print(f"migrated {total} documents")


def pair(root):
    fixed = 0
    for directory, _, _ in os.walk(root):
        before = os.path.join(directory, "⬅️", "🔣️.json")
        if not os.path.basename(directory).startswith("⬅"):
            continue
        parent = os.path.dirname(directory)
        after_dir = next((os.path.join(parent, name) for name in os.listdir(parent) if name.startswith("➡")), None)
        if after_dir is None:
            continue
        before_path = next((os.path.join(directory, name) for name in os.listdir(directory) if name.endswith(".json")), None)
        after_path = next((os.path.join(after_dir, name) for name in os.listdir(after_dir) if name.endswith(".json")), None)
        if not before_path or not after_path:
            continue
        try:
            before_doc = json.load(open(before_path, encoding="utf8"))
            after_doc = json.load(open(after_path, encoding="utf8"))
        except ValueError:
            continue
        systems = before_doc.get("classification_systems")
        if systems and "project" in after_doc and not all(key in after_doc.get("classification_systems", {}) for key in systems):
            merged = dict(after_doc.get("classification_systems", {}))
            for key, value in systems.items():
                merged.setdefault(key, value)
            after_doc["classification_systems"] = merged
            os.remove(after_path)
            with open(after_path, "w", encoding="utf8", newline="") as handle:
                handle.write(json.dumps(after_doc, indent=2, ensure_ascii=False) + "\n")
            fixed += 1
    return fixed


print("completed", sum(pair(root) for root in sys.argv[1:]), "after snapshots")

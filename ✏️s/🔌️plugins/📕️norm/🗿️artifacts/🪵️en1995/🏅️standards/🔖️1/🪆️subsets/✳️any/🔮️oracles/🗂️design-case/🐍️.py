#!/usr/bin/env python3
"""🗂️ Independent oracle of the EN 1995 design-case bridge: reads the catalogue sources and the typed input vocabulary and verifies that every catalogue entry is reachable as a design case.
Checks: the shared vocabulary oracle accepts `🧬️schema/⚖️kernel/📇️catalogue/📚️inputs/🔣️.json`; every input key declared by an `Entry { ... inputs: &[...] }` has a vocabulary row that applies to the entry (an unscoped row or a row scoped to its id); every scope names a real entry; every strength class choice is a row of the strength table.
Usage: python3 -I 🐍️.py  (prints one JSON line)."""
import importlib.util
import json
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
SUBSET = HERE.parent.parent
KERNEL = SUBSET / "🧬️schema" / "⚖️kernel"
VOCABULARY = KERNEL / "📇️catalogue" / "📚️inputs" / "🔣️.json"
SHARED = HERE
while SHARED.name != "📕️norm":
    SHARED = SHARED.parent
SHARED_ORACLE = SHARED / "⚖️compliance" / "🧰️design-case" / "🔮️oracles" / "🐍️.py"


def entries():
    found = {}
    for path in KERNEL.rglob("*.rs"):
        text = path.read_text(encoding="utf-8")
        for match in re.finditer(r'Entry \{ id: "([^"]+)".*?inputs: &\[([^\]]*)\]', text):
            found[match.group(1)] = re.findall(r'"([a-z0-9_]+)"', match.group(2))
    return found


def strength_classes():
    text = (KERNEL / "📚️strength-tables" / "🦀️.rs").read_text(encoding="utf-8")
    return set(re.findall(r'row\("([A-Za-z0-9]+)"', text))


def main():
    spec = importlib.util.spec_from_file_location("shared_oracle", SHARED_ORACLE)
    shared = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(shared)
    rows = shared.check_vocabulary(VOCABULARY)
    doc = json.loads(VOCABULARY.read_text(encoding="utf-8"))["inputs"]
    declared = entries()
    assert len(declared) >= 67, len(declared)
    for entry_id, keys in declared.items():
        for key in keys:
            assert any(r["key"] == key and (not r["scope"] or entry_id in r["scope"].split(",")) for r in doc), (entry_id, key)
    for row in doc:
        for scoped in filter(None, row["scope"].split(",")):
            assert scoped in declared, ("scope names no entry", row["key"], scoped)
    classes = {c["value"] for r in doc if r["key"] == "class" for c in r["choices"]}
    assert classes <= strength_classes() and len(classes) >= 30, classes - strength_classes()
    return {"entries": len(declared), "rows": rows, "strengthClasses": len(classes)}


if __name__ == "__main__":
    print(json.dumps(main()))

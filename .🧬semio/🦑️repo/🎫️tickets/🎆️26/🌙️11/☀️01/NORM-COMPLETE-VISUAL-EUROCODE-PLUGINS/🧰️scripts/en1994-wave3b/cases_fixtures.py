"""Adds `cases` to every en1994 snapshot (empty list) and diff (empty case delta) JSON and the empty DSL table to the assets. Idempotent. Run: /home/deck/semio/.venv/bin/python -I cases_fixtures.py [--apply]"""
import json, os, re, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "field-adder")); sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "schema-evolve"))
import jsonspan as js
from core import walk_files
from ctx import Layout
ANYDIR = Layout("en1994").any
EMPTY = {"removed": [], "inserted": [], "moved": [], "modified": []}
apply = "--apply" in sys.argv
count = {"snapshot": 0, "diff": 0, "dsl": 0}
for path in walk_files(ANYDIR, (".json",)):
    rel = path[len(ANYDIR) + 1:]
    if rel.startswith("🧬️schema/") and "🧫️fixtures" not in rel and rel.count("/") <= 2:
        continue
    try:
        text = open(path, encoding="utf8").read()
    except UnicodeDecodeError:
        continue
    if '"reliabilityClass"' not in text or '"cases"' in text or '"beams"' not in text:
        continue
    try:
        root = js.parse(text)
    except (AssertionError, IndexError, ValueError):
        continue
    hits = js.find_objects(root, lambda o: o.get("reliabilityClass") is not None and o.get("beams") is not None and o.get("properties") is None)
    edits = []
    for _, obj in hits:
        anchor = obj.get("reliabilityClass")
        beams = obj.get("beams").value
        snapshot = beams.kind == "array"
        value = "[]" if snapshot else json.dumps(EMPTY)
        colon = text[anchor.key_end:anchor.value.start]
        separator = js._separator(text, obj, obj.members.index(anchor))
        edits.append((anchor.value.end, separator + json.dumps("cases") + colon + value))
        count["snapshot" if snapshot else "diff"] += 1
    if edits and apply:
        open(path, "w", encoding="utf8").write(js.apply_edits(text, edits))
for path in walk_files(ANYDIR, (".dsl.semio",)):
    text = open(path, encoding="utf8").read()
    if re.search(r"^cases \[", text, re.M) or not re.search(r"^placements \[", text, re.M):
        continue
    count["dsl"] += 1
    if apply:
        open(path, "w", encoding="utf8").write(text.rstrip("\n") + "\ncases [id:TEXT label-en:TEXT label-de:TEXT entry-id:TEXT numbers:TABLE texts:TABLE] {\n}\n")
print(count)

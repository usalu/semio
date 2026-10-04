#!/usr/bin/env python3
"""🧮️ Runs writers of the quiz vector generator without writing any fixture and prints every hint list that differs from the committed fixture (before → after).

Run from the repository root: ``.venv/Scripts/python.exe <this file> [writer …]`` (default: challenge lifecycle).
"""

import glob
import importlib.util
import json
import os
import sys

sys.dont_write_bytecode = True
GENERATOR = glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "🌙️09", "☀️28", "*", "generate_quiz_vectors.py"))[0]
spec = importlib.util.spec_from_file_location("generate_quiz_vectors", GENERATOR)
generator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)
captured = {}
generator.write = lambda directory, document: captured.__setitem__(directory, document)


def hint_lists(node, path=""):
    """🧭️ Every ``hints`` member and every hint vector's ``expected`` below ``node``, by path."""
    found = {}
    if isinstance(node, dict):
        for key, value in node.items():
            here = "%s/%s" % (path, node.get("id", key) if key in ("expected",) else key)
            if key == "hints" and isinstance(value, dict) and all(isinstance(entry, list) for entry in value.values()):
                found[path + "/hints"] = value
            elif key == "expected" and isinstance(value, list) and all(isinstance(entry, dict) and "kind" in entry for entry in value):
                found["%s/%s" % (path, node.get("id"))] = value
            else:
                found.update(hint_lists(value, here))
    elif isinstance(node, list):
        for index, value in enumerate(node):
            found.update(hint_lists(value, "%s[%s]" % (path, value.get("id", index) if isinstance(value, dict) else index)))
    return found


for writer in sys.argv[1:] or ["challenge", "lifecycle"]:
    getattr(generator, writer)()
changed = 0
for directory, document in captured.items():
    committed = json.load(open(os.path.join(generator.QUIZ, "🧫️fixtures", directory, "🔣️.json"), encoding="utf-8"))
    before, after = hint_lists(committed), hint_lists(document)
    for path in sorted(set(before) | set(after)):
        if before.get(path) != after.get(path):
            changed += 1
            print("[DEBUG] %s%s\n  before %s\n  after  %s" % (directory, path, json.dumps(before.get(path), ensure_ascii=False), json.dumps(after.get(path), ensure_ascii=False)))
    print("[DEBUG] %s: whole document %s" % (directory, "unchanged" if committed == document else "changed"))
print("[DEBUG] %d hint lists changed" % changed)

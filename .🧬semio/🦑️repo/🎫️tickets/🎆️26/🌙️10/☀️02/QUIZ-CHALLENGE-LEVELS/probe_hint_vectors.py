#!/usr/bin/env python3
"""🔎️ Runs writers of the quiz vector generator without writing any fixture and prints the hint vectors (or a named group) they would commit, one line per vector.

Run from the repository root: ``.venv/Scripts/python.exe <this file> [writer] [group …]``.
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
writer = sys.argv[1] if len(sys.argv) > 1 else "challenge"
getattr(generator, writer)()
for directory, document in captured.items():
    for group in sys.argv[2:] or ["hints", "classificationHints", "quizHints"]:
        for vector in document.get(group, []):
            if group == "learners":
                for run, view in vector.get("views", {}).get("expected", {}).get("runs", {}).items():
                    if "hints" in view:
                        print("[DEBUG] learners/%s/%s %s" % (vector["id"], run[:8], json.dumps(view["hints"], ensure_ascii=False)))
            else:
                print("[DEBUG] %s/%s %s" % (group, vector["id"], json.dumps(vector.get("expected"), ensure_ascii=False)))

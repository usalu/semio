#!/usr/bin/env python3
"""🪞️ Lists docstrings whose leading emoji repeats inside one Python file of the quiz cases or the vector generator.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``.
"""

import ast
import glob
import os
from collections import Counter

FILES = sorted(glob.glob(os.path.join("🧰️framework", "🛍️products", "❓️quiz", "🧪️tests", "*", "🐍️.py"))) + [os.path.join(".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️28", "QUIZ-PRODUCT-AND-TEACHING-PROCTOR", "generate_quiz_vectors.py")]

for path in FILES:
    with open(path, encoding="utf-8") as handle:
        tree = ast.parse(handle.read())
    leads = []
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.ClassDef, ast.Module)):
            text = ast.get_docstring(node)
            if text:
                leads.append(text.split(" ", 1)[0])
    repeated = {lead: count for lead, count in Counter(leads).items() if count > 1}
    if repeated:
        print("[DEBUG] %s %r" % (path, repeated))

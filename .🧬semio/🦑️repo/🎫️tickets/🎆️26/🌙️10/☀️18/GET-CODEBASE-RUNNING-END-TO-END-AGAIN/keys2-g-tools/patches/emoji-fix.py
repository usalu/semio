"""python3 emoji-fix.py <precision-dir> : gives the docstrings added by keys2-g unique emojis (the engine file already holds a 🛤️ and a 🧭️ docstring of its own). Anchored, idempotent."""
import sys
path = sys.argv[1] + "/🟦️.ts"
s = open(path, encoding="utf-8").read()
for old, new in (("/** 🧭️ The words after the last script token", "/** 🚏️ The words after the last script token"), ("/** 🛤️ The index just after", "/** 🧲️ The index just after"), ("/** 🛤️ The name of the class", "/** 🪪️ The name of the class"), ("/** 🛤️ The text of a class declaration", "/** 📃️ The text of a class declaration"), ("/** 🛤️ The `if (segments[0]", "/** 🌿️ The `if (segments[0]"), ("/** 🛤️ The text of the top-level", "/** 🍱️ The text of the top-level")):
    if new in s:
        continue
    if s.count(old) != 1:
        sys.exit(f"anchor not unique ({s.count(old)}): {old}")
    s = s.replace(old, new)
open(path, "w", encoding="utf-8").write(s)
print("docstring emojis made unique in", sys.argv[1])

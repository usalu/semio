import os
import re

root = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
field = re.compile(r"pub (\w+): Option<(\w+Delta)>")
for dirpath, _, files in os.walk(root):
    for name in files:
        if not name.endswith(".rs"):
            continue
        path = os.path.join(dirpath, name)
        if "diff" not in path:
            continue
        text = open(path, encoding="utf-8", errors="replace").read()
        hits = field.findall(text)
        if not hits:
            continue
        bad = []
        for fname, _typ in hits:
            if f"self.{fname}.absorb(" in text or f"self.{fname}.commit_onto(" in text or f"self.{fname}.inverse(" in text:
                bad.append(fname)
        if bad:
            print(path)
            print(" ", ", ".join(bad))

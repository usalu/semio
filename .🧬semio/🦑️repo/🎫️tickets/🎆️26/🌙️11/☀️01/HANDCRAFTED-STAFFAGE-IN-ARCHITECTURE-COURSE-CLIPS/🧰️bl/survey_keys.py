"""🔎 Survey localized and text key paths across the staffage catalog to plan the bilingual audit."""
import glob
import json
import os
from collections import Counter

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog"
paths = Counter()
examples = {}


def walk(node, prefix):
    if isinstance(node, dict):
        if set(node.keys()) >= {"en", "de"}:
            paths[prefix + "{en,de}"] += 1
            return
        for k, v in node.items():
            walk(v, f"{prefix}.{k}")
    elif isinstance(node, list):
        for v in node:
            walk(v, f"{prefix}[]")
    elif isinstance(node, str):
        paths[prefix + " (str)"] += 1
        examples.setdefault(prefix + " (str)", node[:80])


for f in glob.glob(os.path.join(ROOT, "**", "🔣️.json"), recursive=True):
    with open(f, encoding="utf-8") as fh:
        walk(json.load(fh), "")

for k, v in sorted(paths.items()):
    print(v, k, "|", examples.get(k, ""))

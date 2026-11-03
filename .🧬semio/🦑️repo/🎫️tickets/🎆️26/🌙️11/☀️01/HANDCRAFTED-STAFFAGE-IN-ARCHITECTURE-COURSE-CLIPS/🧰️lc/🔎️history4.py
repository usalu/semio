import glob
import json
import os
import sys

NEEDLES = ["Beat2", "Sorptionsrad", "Plattentauscher", "Verdunstungskühlung", "def construct"]
ROOT = r"C:\Users\Ueli\.claude\projects\C--git-semio"
files = glob.glob(ROOT + "/*.jsonl") + glob.glob(ROOT + "/*/subagents/*.jsonl")
hits = []
for path in files:
    try:
        lines = open(path, encoding="utf-8").read().split("\n")
    except Exception:
        continue
    for ln, line in enumerate(lines):
        if not line.strip() or "Plattentauscher" not in line or "Verdunstungskühlung" not in line:
            continue
        try:
            rec = json.loads(line)
        except Exception:
            continue
        content = rec.get("message", {}).get("content")
        if not isinstance(content, list):
            continue
        for part in content:
            if isinstance(part, dict) and part.get("type") == "tool_result":
                c = part.get("content")
                text = c if isinstance(c, str) else "".join(x.get("text", "") for x in c if isinstance(x, dict))
                if all(n in text for n in NEEDLES):
                    hits.append((rec.get("timestamp"), os.path.basename(path)[:28], ln, len(text), text[:80].replace("\n", " ")))
hits.sort()
for h in hits:
    print(*h)

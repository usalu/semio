import glob
import json
import os
import sys

NEEDLES = sys.argv[1:]
ROOT = r"C:\Users\Ueli\.claude\projects\C--git-semio"
files = glob.glob(ROOT + "/*.jsonl") + glob.glob(ROOT + "/*/subagents/*.jsonl")
for path in files:
    try:
        lines = open(path, encoding="utf-8").read().split("\n")
    except Exception:
        continue
    for ln, line in enumerate(lines):
        if not line.strip() or not all(n in line for n in NEEDLES):
            continue
        try:
            rec = json.loads(line)
        except Exception:
            continue
        content = rec.get("message", {}).get("content")
        if not isinstance(content, list):
            continue
        for part in content:
            if isinstance(part, dict) and part.get("type") == "tool_use":
                blob = json.dumps(part.get("input", {}), ensure_ascii=False)
                if all(n in blob for n in NEEDLES):
                    print(rec.get("timestamp"), os.path.basename(path)[:26], ln, part["name"], len(blob), blob[:100].replace("\n", " "))

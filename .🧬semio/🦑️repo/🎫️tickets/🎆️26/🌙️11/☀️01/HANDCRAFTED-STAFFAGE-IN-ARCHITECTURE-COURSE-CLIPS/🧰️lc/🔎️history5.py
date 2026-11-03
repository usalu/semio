import glob
import json
import os
import re

ROOT = r"C:\Users\Ueli\.claude\projects\C--git-semio"
files = glob.glob(ROOT + "/*.jsonl") + glob.glob(ROOT + "/*/subagents/*.jsonl")
write_words = re.compile(r"sed -i|write_text|write_bytes|open\(|\.write\(|perl -|cp |mv |git checkout|git restore|> *\"?\$F")
out = []
for path in files:
    try:
        lines = open(path, encoding="utf-8").read().split("\n")
    except Exception:
        continue
    for ln, line in enumerate(lines):
        if "summer-str" not in line or not line.strip():
            continue
        try:
            rec = json.loads(line)
        except Exception:
            continue
        content = rec.get("message", {}).get("content")
        if not isinstance(content, list):
            continue
        for part in content:
            if isinstance(part, dict) and part.get("type") == "tool_use" and part.get("name") == "Bash":
                cmd = part["input"].get("command", "")
                if "summer-str" in cmd and write_words.search(cmd):
                    out.append((rec.get("timestamp") or "", os.path.basename(path)[:28], ln, cmd))
out.sort()
for ts, p, ln, cmd in out:
    print("=====", ts, p, ln)
    print(cmd[:700])

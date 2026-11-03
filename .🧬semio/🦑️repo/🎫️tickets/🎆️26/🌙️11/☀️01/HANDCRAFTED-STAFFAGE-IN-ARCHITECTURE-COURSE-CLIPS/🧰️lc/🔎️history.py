import glob
import json
import os
import sys

KEY = "summer-strategy"
ROOT = r"C:\Users\Ueli\.claude\projects\C--git-semio"
files = glob.glob(ROOT + "/*.jsonl") + glob.glob(ROOT + "/*/subagents/*.jsonl")
events = []
for path in files:
    uses = {}
    try:
        lines = open(path, encoding="utf-8").read().split("\n")
    except Exception:
        continue
    for ln, line in enumerate(lines):
        if KEY not in line or not line.strip():
            continue
        try:
            rec = json.loads(line)
        except Exception:
            continue
        msg = rec.get("message", {})
        content = msg.get("content")
        if not isinstance(content, list):
            continue
        ts = rec.get("timestamp", "")
        for part in content:
            if not isinstance(part, dict):
                continue
            if part.get("type") == "tool_use":
                inp = part.get("input", {})
                fp = inp.get("file_path", "") or ""
                if KEY in fp and fp.endswith(".py") and "tickets" not in fp:
                    uses[part["id"]] = (part["name"], inp)
                    if part["name"] in ("Edit", "Write"):
                        events.append((ts, path, ln, part["name"], inp))
            elif part.get("type") == "tool_result":
                tid = part.get("tool_use_id")
                if tid in uses and uses[tid][0] == "Read":
                    c = part.get("content")
                    text = c if isinstance(c, str) else "".join(x.get("text", "") for x in c if isinstance(x, dict))
                    events.append((ts, path, ln, "ReadResult", {"input": uses[tid][1], "text": text}))
events.sort(key=lambda e: e[0])
out = sys.argv[1]
json.dump(events, open(out, "w", encoding="utf-8"), ensure_ascii=False)
for e in events:
    kind = e[3]
    extra = ""
    if kind == "ReadResult":
        extra = f"len={len(e[4]['text'])} input={e[4]['input']}"
    elif kind == "Edit":
        extra = f"old={len(e[4]['old_string'])} new={len(e[4]['new_string'])}"
    else:
        extra = f"write len={len(e[4]['content'])}"
    print(e[0], os.path.basename(e[1])[:20], e[2], kind, extra[:120])

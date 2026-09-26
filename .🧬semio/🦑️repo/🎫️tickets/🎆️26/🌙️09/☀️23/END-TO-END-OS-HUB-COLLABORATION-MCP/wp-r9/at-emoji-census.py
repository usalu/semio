#!/usr/bin/env python3
"""🔢️ R9 item 5: census of docstring openings (`@emoji <emoji>` vs bare `<emoji>` vs other) over tracked-source Rust/TS files."""
import os, re, sys, unicodedata, json
ROOT = "/Users/ueli/Documents/semio"
SKIP = {"node_modules", "dist", "target", ".git", "🎫️tickets", "storybook-static", ".🧬semio", "🤖️generated", "🗑️generated"}
def is_emoji(ch):
    cp = ord(ch)
    return cp >= 0x2190 and (unicodedata.category(ch) == "So" or 0x1F000 <= cp <= 0x1FAFF)
def classify(text):
    t = text.strip()
    if t.startswith("@emoji"):
        return "at-emoji"
    return "emoji" if t and is_emoji(t[0]) else ("empty" if not t else "other")
counts = {"rs": {}, "ts": {}}
per_file = {}
for dirpath, dirnames, filenames in os.walk(ROOT):
    dirnames[:] = [d for d in dirnames if d not in SKIP and not d.startswith(".tmp-ticket")]
    for name in filenames:
        ext = "rs" if name.endswith(".rs") else "ts" if name.endswith((".ts", ".tsx")) and not name.endswith(".d.ts") else None
        if not ext:
            continue
        path = os.path.join(dirpath, name)
        try:
            lines = open(path, encoding="utf-8").read().split("\n")
        except Exception:
            continue
        local = {}
        if ext == "rs":
            previous_doc = False
            for line in lines:
                s = line.lstrip()
                doc = s.startswith("///") and not s.startswith("////")
                if doc and not previous_doc:
                    k = classify(s[3:])
                    local[k] = local.get(k, 0) + 1
                previous_doc = doc
        else:
            for m in re.finditer(r"/\*\*(?!/)(.*?)(?:\*/|$)", "\n".join(lines), re.S):
                body = m.group(1).lstrip("*").strip()
                first = body.split("\n", 1)[0].lstrip("* ").strip() if body else ""
                if not first and "\n" in body:
                    first = next((l.strip().lstrip("*").strip() for l in body.split("\n") if l.strip().lstrip("*").strip()), "")
                k = classify(first)
                local[k] = local.get(k, 0) + 1
        for k, v in local.items():
            counts[ext][k] = counts[ext].get(k, 0) + v
        if local.get("at-emoji"):
            per_file[os.path.relpath(path, ROOT)] = local
print(json.dumps(counts, indent=1))
mixed = sum(1 for v in per_file.values() if v.get("emoji"))
print("files with @emoji docstrings:", len(per_file), "of which also bare-emoji docstrings:", mixed)
json.dump(per_file, open(sys.argv[1], "w"), ensure_ascii=False, indent=0) if len(sys.argv) > 1 else None

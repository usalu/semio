#!/usr/bin/env python3
"""📋️ Lists the errors (or warnings) of one `--message-format=json` cargo log: file, line, message, primary text.

usage: 📋️errors.py <tag> [error|warning] [path-substring]
"""
import json, sys
tag = sys.argv[1]; level = sys.argv[2] if len(sys.argv) > 2 else "error"; sub = sys.argv[3] if len(sys.argv) > 3 else ""
seen = set()
for line in open(f"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/p5/logs/{tag}.json.log", encoding="utf-8", errors="replace"):
    if not line.startswith("{"): continue
    m = json.loads(line)
    if m.get("reason") != "compiler-message": continue
    d = m["message"]
    if d["level"] != level or not d.get("spans"): continue
    sp = [s for s in d["spans"] if s["is_primary"]][0]
    if sub not in sp["file_name"]: continue
    key = (sp["file_name"], sp["line_start"], d["message"])
    if key in seen: continue
    seen.add(key)
    short = sp["file_name"].split("/📦️packages/🦀️rust/../../")[-1] if "📦️packages" in sp["file_name"] else sp["file_name"]
    pkg = sp["file_name"].split("/📦️packages")[0].split("/")[-1]
    print(f'{pkg}:{short}:{sp["line_start"]} | {d["message"][:110]} | {(sp["text"][0]["text"].strip()[:130] if sp["text"] else "")}')

#!/bin/zsh
C=$(ls -d /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/*/*/*/GET-CODEBASE-RUNNING-END-TO-END-AGAIN | tail -1)
rm -f "$C/🗑️generated/vit2-out/one.jsonl"
print -r -- "$1" > "$C/🗑️generated/vit2-out/one.list"
cd /Users/ueli/Documents/semio
python3 "$C/vit2-tools/runvt.py" "$C/🗑️generated/vit2-out/one.list" "$C/🗑️generated/vit2-out/one.jsonl" 900 ${2:-300} >/dev/null
python3 - "$C" <<'P'
import json,sys,re
r=json.loads(open(sys.argv[1]+"/🗑️generated/vit2-out/one.jsonl",encoding="utf-8").readline())
print(r["rc"],r["secs"],r["files"],"|",r["tests"])
if r.get("log"):
    t=open(r["log"],encoding="utf-8").read()
    seen=set()
    for m in re.finditer(r"(FAIL .*|(?:\w*Error)[^\n]*|Failed to (?:resolve|load)[^\n]*|Cannot find [^\n]*)",t):
        s=m.group(1)[:300]
        if s in seen: continue
        seen.add(s); print("  ",s)
        if len(seen)>25: break
P

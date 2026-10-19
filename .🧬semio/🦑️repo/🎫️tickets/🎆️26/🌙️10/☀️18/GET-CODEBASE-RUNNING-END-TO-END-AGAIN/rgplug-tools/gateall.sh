#!/bin/zsh
# usage: gateall.sh <spec.json>... prints canonical gate for old+new crates of specs
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
dirs=$(python3 - "$@" <<'P'
import json,sys
for f in sys.argv[1:]:
    s=json.load(open(f))
    out=[]
    if "old_pkg" in s: out.append(s["old_pkg"])
    elif "old_dir" in s: out.append(s["old_dir"]+"/📦️packages/🦀️rust")
    if "new_dir" in s: out.append(s["new_dir"]+"/📦️packages/🦀️rust")
    for n in s.get("new",[]): out.append(n["dir"]+"/📦️packages/🦀️rust")
    print("\n".join(out))
P
)
echo "$dirs" | while IFS= read -r d; do [ -n "$d" ] && bun "$C/rgplug-tools/gate.ts" "$d"; done

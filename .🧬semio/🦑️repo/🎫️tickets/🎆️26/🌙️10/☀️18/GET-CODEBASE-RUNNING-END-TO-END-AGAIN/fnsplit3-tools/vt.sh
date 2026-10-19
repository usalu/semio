#!/bin/zsh
# usage: vt.sh <configRel> <filter...>   runs one vitest config restricted to files matching the filters
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
G=$C/🗑️generated/fnsplit3-x/vt
VTID="${RANDOM}${RANDOM}"; cfg="$1"; shift
mkdir -p $G
rm -f $G/one-$VTID.jsonl
print -r -- "$cfg" > $G/one-$VTID.list
export VT_EXTRA=$(python3 -c 'import json,sys; print(json.dumps(["--reporter=json","--outputFile.json="+sys.argv[1]]+sys.argv[2:]))' "${VT_RES:-$G/res.json}" "$@")
cd /Users/ueli/Documents/semio
python3 $C/fnsplit3-tools/vt.py $G/one-$VTID.list $G/one-$VTID.jsonl 900 ${VT_CAP:-900} >/dev/null
python3 - $G $VTID <<'P'
import json,sys,re
r=json.loads(open(sys.argv[1]+"/one-"+sys.argv[2]+".jsonl",encoding="utf-8").readline())
print(r["rc"],r["secs"],r["files"],"|",r["tests"])
if r.get("log"):
    t=open(r["log"],encoding="utf-8").read()
    seen=set()
    for m in re.finditer(r"(FAIL .*|(?:\w*Error)[^\n]*|Failed to (?:resolve|load)[^\n]*|Cannot find [^\n]*|AssertionError[^\n]*)",t):
        s=m.group(1)[:300]
        if s in seen: continue
        seen.add(s); print("  ",s)
        if len(seen)>25: break
P

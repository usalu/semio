#!/bin/zsh
# usage: t1.sh <repo-relative test file> [maxlines]  (hoist env; prints summary and log tail on red)
C=/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/C
T=/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad/one-$$
mkdir -p $T; print -r -- "$1" > $T/l.txt
cd /Users/ueli/Documents/semio
TST3C_HOIST=${HOIST:-1} python3 $C/tst3c-tools/run.py $T/l.txt $T/o.jsonl one$$ 3000 >/dev/null
python3 - <<PY
import json
r=json.loads(open("$T/o.jsonl",encoding="utf-8").read().splitlines()[0])
print("RESULT rc=%d pass=%d fail=%d secs=%s"%(r["rc"],r["pass_"],r["fail"],r["secs"]))
if r["rc"]!=0:
    t=open(r["log"],encoding="utf-8").read().splitlines()
    t=[l for l in t if l.strip()]
    print("\n".join(l[:${3:-260}] for l in t[-${2:-40}:]))
PY
rm -rf $T

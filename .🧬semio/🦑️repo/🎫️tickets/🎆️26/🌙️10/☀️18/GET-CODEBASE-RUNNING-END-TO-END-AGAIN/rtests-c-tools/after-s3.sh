#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
while kill -0 60035 2>/dev/null; do sleep 15; done
python3 - <<'PY'
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
rows=[l.rstrip("\n").split("\t")[:6] for l in open(C+"/🗑️generated/rtests-c/s3/summary.tsv")]
green={r[5] for r in rows if r[1]=="0"}
allr=[l.strip() for l in open(C+"/rtests-c-tools/reds2.txt") if l.strip()]
open(C+"/rtests-c-tools/reds3.txt","w").write("\n".join(f for f in allr if f not in green)+"\n")
PY
python3 "$C/rtests-c-tools/drive.py" "$C/rtests-c-tools/reds3.txt" "$C/🗑️generated/rtests-c/s6" 240
exec python3 "$C/rtests-c-tools/drive.py" "$C/rtests-c-tools/greens0.txt" "$C/🗑️generated/rtests-c/s4" 240

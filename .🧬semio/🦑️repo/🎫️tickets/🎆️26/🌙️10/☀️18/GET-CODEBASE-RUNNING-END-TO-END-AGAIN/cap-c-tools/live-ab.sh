#!/bin/sh
# lane-g proof: two consecutive builds of the same component; unit-cache events of lane g are counted before, between and after
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/x/g.jsonl"
count() { python3 - "$G" <<'P'
import json,sys,collections
c=collections.Counter()
for l in open(sys.argv[1],errors='ignore'):
    try:c[json.loads(l)['kind']]+=1
    except Exception: pass
print(dict(c))
P
}
echo "before $(count)"
sh "$C/cap-c-tools/live-a.sh" > "$C/🗑️generated/cap-c/build-1.log" 2>&1; echo "after-1 rc=$? $(count)"
sh "$C/cap-c-tools/live-a.sh" > "$C/🗑️generated/cap-c/build-2.log" 2>&1; echo "after-2 rc=$? $(count)"

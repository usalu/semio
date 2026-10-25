#!/bin/sh
# usage: lockroot.sh  (adds the `color` oracle to the root devDependencies and regenerates the root lock through the official command; restores package.json on failure)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/cui-d"
export SEMIO_LANGUAGE=en
unset SEMIO_SCRIPT_STALL_MS
python3 - <<'PY'
import hashlib
p="/Users/ueli/Documents/semio/package.json"
s=open(p,encoding="utf8").read()
old='    "color-convert": "2.0.1",\n    "color-string": "1.9.1",\n    "colord": "2.10.0",'
assert s.count(old)==1
open(p,"w",encoding="utf8").write(s.replace(old,'    "color": "4.2.3",\n'+old))
PY
date +%s > "$O/lockroot.start"
nice -n 10 bun repo dependencies lock --scope . > "$O/lockroot.log" 2>&1
rc=$?
if [ $rc -ne 0 ]; then cp "$O/package.orig.json" package.json; fi
echo $rc > "$O/lockroot.rc"

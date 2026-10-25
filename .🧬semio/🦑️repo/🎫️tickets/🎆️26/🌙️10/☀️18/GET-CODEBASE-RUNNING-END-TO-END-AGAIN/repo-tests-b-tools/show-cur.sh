#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/🗑️generated/repo-tests-b"
nice -n 10 bun repo show projects --json --graph-policy=current > "$S/cur2.json" 2> "$S/cur2.err"
echo "rc=$?" >> "$S/cur2.err"
python3 -c "import json;d=json.load(open('$S/cur2.json'));print('valid json list',len(d))" > "$S/cur2.verdict" 2>&1

#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/🗑️generated/repo-tests-b"
nice -n 10 bun repo show projects --json > "$S/projects.json" 2> "$S/projects.err"
echo "rc=$?" > "$S/projects.rc"
python3 -c "import json,sys;d=json.load(open('$S/projects.json'));print('valid json', type(d).__name__, len(d))" > "$S/projects.verdict" 2>&1

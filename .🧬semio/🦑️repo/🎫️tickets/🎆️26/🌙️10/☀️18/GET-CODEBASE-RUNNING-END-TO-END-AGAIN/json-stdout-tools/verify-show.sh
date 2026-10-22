#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/json-stdout"
nice -n 10 bun repo show projects --json >"$G/projects.json" 2>"$G/projects.err"
echo "rc=$?" >"$G/projects.rc"
python3 -I -c 'import json,sys; d=json.load(open(sys.argv[1])); print("projects json ok", len(d))' "$G/projects.json" >"$G/projects.verdict" 2>&1

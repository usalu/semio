#!/bin/bash
# usage: job.sh NAME args...   (args passed to bun in cwd S); prints the output when done
D="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-H1/jobs"
N="$1"; shift
python3 -I -c 'import json,sys;json.dump({"argv":sys.argv[1:]},open(sys.argv[0] if False else "'"$D/$N"'.job.json","w"))' "$@"
for i in $(seq 1 1100); do [ -f "$D/$N.done" ] && break; sleep 1; done
cat "$D/$N.out.txt"; rm -f "$D/$N".*

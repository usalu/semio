#!/bin/bash
# usage: waitfix.sh <substring>  — waits <=9 min for a FIXED line in 1105b-claims.md containing the substring
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
for i in $(seq 1 18); do python3 -c 'import sys;sys.exit(0 if any("FIXED" in l and sys.argv[1] in l for l in open(sys.argv[2])) else 1)' "$1" "$C/1105b-claims.md" && { echo FOUND; exit 0; }; sleep 30; done; echo NONE

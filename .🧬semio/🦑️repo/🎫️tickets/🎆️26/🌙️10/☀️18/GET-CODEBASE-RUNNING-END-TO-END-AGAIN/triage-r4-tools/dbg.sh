#!/bin/sh
# usage: dbg.sh <rel test> -- runs a debug copy (failure printer) through the genuine-birth harness
C=$(cd "$(dirname "$0")/.." && pwd); R=/Users/ueli/Documents/semio
python3 "$C/triage-r4-tools/run-copy.py" "$1" "$C/triage-r4-tools/staged/run2/owner.ts" "$2" "$3"
echo "${C#$R/}/triage-r4-tools/staged/run2/owner.ts" > "$C/triage-r4-tools/run2.txt"
rm -f "$C/🗑️generated/triage-r4/r2.jsonl"
nice -n 10 python3 -I "$C/testfix-defect-tools/run.py" "$C/triage-r4-tools/run2.txt" "$C/🗑️generated/triage-r4/r2.jsonl" --jobs 1 --cap 200 2>&1 | tail -1
L=$(ls -t "$C"/🗑️generated/tfd/logs/* | head -1); grep -a "DEBUG" -A8 "$L" | head -14 | cut -c1-500

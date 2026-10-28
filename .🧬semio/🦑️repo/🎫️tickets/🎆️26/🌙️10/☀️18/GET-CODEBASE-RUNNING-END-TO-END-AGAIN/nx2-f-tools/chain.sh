#!/bin/sh
# chain.sh <suffix> <target>...: runs each `repo run <target>` (default tool) one after the other via drive.py
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
s=$1; shift
for t in "$@"; do
  n=$(echo "$t" | sed 's/[^A-Za-z0-9]/_/g' | sed 's/^_*//')-$s
  python3 "$C/nx2-f-tools/drive.py" "$n" run "$t"
  echo "$n $(date +%H:%M:%S) $(tail -n 1 "$C/🗑️generated/nx2-f/$n.raw")" >> "$C/🗑️generated/nx2-f/chain-summary.txt"
done

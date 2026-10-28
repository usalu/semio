#!/bin/sh
# 🎲️ Runs the flake hunt scenarios one after the other inside a mirror (see flake.py). usage: sh flake-all.sh <mirror> <tag>   (output in $TMPDIR/s2g-flake, copied to the ticket by hand)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 2
MM="python3 -I $C/mirror-tools/mirror.py"; M=$1; TAG=$2
OUT="${TMPDIR:-/tmp}/s2g-flake"; mkdir -p "$OUT"
for spec in "60 plain" "300 edit promote" "100 freeze" "30 emfile=40" "30 emfile=24"; do
  set -- $spec; n=$1; shift
  name=$(echo "$*" | tr ' =' '__')
  $MM run $M --peers-ok -- python3 -I "$PWD/$C/seal2-g-tools/flake.py" "$n" "$@" --out "$OUT/$TAG-$name.jsonl" > "$OUT/$TAG-$name.log" 2>&1
done
echo ALLDONE > "$OUT/$TAG-done.txt"

#!/bin/bash
# one round: author mixer + heat pump (python), then ONE queued bun process (all.ts) -> output file
cd /c/git/semio || exit 1
export PYTHONUTF8=1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
OUT="$T/🗑️generated/QA12"
mkdir -p "$OUT"
for s in $AUTH; do
  python "$T/🧰️M1/$s.py" || echo "AUTHOR FAILED $s"
done > "$OUT/round-author.txt" 2>&1
repo tools exec bun -- "$T/🧰️QA12/all.ts" "$@" > "$OUT/round-all.txt" 2>&1
echo "round finished" >> "$OUT/round-all.txt"

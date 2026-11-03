#!/bin/bash
# one round: author all eight assets (python, local), then ONE queued bun process (all.ts) -> output file
cd /c/git/semio || exit 1
export PYTHONUTF8=1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE"
M="$T/🧰️M1"
OUT="$T/🗑️generated/M1"
mkdir -p "$OUT"
for s in a_expansion_valve a_evaporative_humidifier a_finned_heat_exchanger a_heat_pump a_forklift a_diesel a_excavator a_mixer; do
  python "$M/$s.py" || echo "AUTHOR FAILED $s"
done > "$OUT/round-author.txt" 2>&1
repo tools exec bun -- "$M/all.ts" "$@" > "$OUT/round-all.txt" 2>&1
echo "round finished" >> "$OUT/round-all.txt"

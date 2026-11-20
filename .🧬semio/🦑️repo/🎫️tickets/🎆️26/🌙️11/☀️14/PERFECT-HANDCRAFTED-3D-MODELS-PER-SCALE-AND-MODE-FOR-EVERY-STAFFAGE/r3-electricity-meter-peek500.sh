#!/bin/sh
# 🔭 Renders the 1:500 lists through r3-sheet.ts, which needs a 2D representation of the scale: the 1:500 volume is placed in the 1:200 slot of the source for the render, then the source is rebuilt. Usage: sh r3-electricity-meter-peek500.sh [r3-sheet options]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 - <<'P'
import json
p = "volumes/appliances/electricity-meter.json"
d = json.load(open(p))
d["volumes"]["200"] = {**d["volumes"]["500"], "denominator": 200}
json.dump(d, open(p, "w"), ensure_ascii=False, indent=1)
P
OUT=$($BUN r3-sheet.ts appliances/electricity-meter --scale 200 "$@" 2>&1 | tail -1)
mkdir -p "🗑️generated/r3-electricity-meter"
NAME="🗑️generated/r3-electricity-meter/s500$(echo "$@" | tr -d ' ' | tr -c 'A-Za-z0-9\n' '-').png"
mv "$OUT" "$NAME"
python3 r3-electricity-meter-model.py > /dev/null
echo "$NAME"

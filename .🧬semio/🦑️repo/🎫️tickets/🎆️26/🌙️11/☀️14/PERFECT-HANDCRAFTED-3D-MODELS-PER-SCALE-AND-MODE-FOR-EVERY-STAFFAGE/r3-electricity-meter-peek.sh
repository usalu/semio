#!/bin/sh
# 👀 Renders the body of one scale without the window pane (r3-sheet draws glazing opaque), then restores the source. Usage: sh r3-electricity-meter-peek.sh <scale> [r3-sheet options]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
SCALE=$1; shift
python3 - "$SCALE" <<'P'
import json, sys
p = "volumes/appliances/electricity-meter.json"
d = json.load(open(p))
v = d["volumes"][sys.argv[1]]
v["body"] = [s for s in v["body"] if s["id"] != "window"]
v["thermal"] = [s for s in v["thermal"] if s.get("of") != "window"] + [{**v["body"][0], "id": "th-peek", "region": "window"}]
json.dump(d, open(p, "w"), ensure_ascii=False, indent=1)
P
OUT=$($BUN r3-sheet.ts appliances/electricity-meter --scale "$SCALE" "$@" 2>&1 | tail -1)
mkdir -p "🗑️generated/r3-electricity-meter"
NAME="🗑️generated/r3-electricity-meter/peek.s$SCALE$(echo "$@" | tr -d ' ' | tr -c 'A-Za-z0-9\n' '-').png"
cp "$OUT" "$NAME"
python3 r3-electricity-meter-model.py > /dev/null
echo "$NAME"

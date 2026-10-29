#!/usr/bin/env bash
# [scratch] regenerates scratch-emitted fragments, assembles, checks and renders previews: build.sh [preview args]
set -euo pipefail
A="$(cd "$(dirname "$0")" && pwd)"; T="$A/../.."; OUT="$T/🗑️generated/animals"; S="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
python3 -I "$A/pigeon-rig.py" "$A" >/dev/null; python3 -I "$A/pigeon-geom.py" "$A" >/dev/null
bash "$A/category.sh" "$OUT/pigeon.json" pigeon
python3 -I "$A/pigeon-emit.py" "$A" "$OUT/pigeon.json"
bash "$A/category.sh" "$OUT/pigeon.json" pigeon

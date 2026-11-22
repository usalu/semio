#!/bin/sh
# 🩺️ PW-08 inner loop: build one asset candidate and validate it with the r3 tools of the 3D-model ticket; extra arguments go to r3-validate.ts (for example --volumes --scale 500 --rules silhouette --no-print).
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
T3="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
ID="$1"; shift
OUT="$HERE/../../🗑️generated/PW-08"
mkdir -p "$OUT"
cd "$HERE" && repo tools exec bun -- build.ts "$ID" 2>&1 | grep -v -e "\[capacity\]" -e "still working" -e "waits for machine" > "$OUT/$ID.build.txt"
tail -3 "$OUT/$ID.build.txt"
cd "$T3" && repo tools exec bun -- r3-validate.ts --file "$OUT/$ID/🔣️.json" "$@" 2>&1 | grep -v -e "\[capacity\]" -e "still working" -e "waits for machine" > "$OUT/$ID.validate.txt" || true
cut -c1-400 "$OUT/$ID.validate.txt" | tail -60

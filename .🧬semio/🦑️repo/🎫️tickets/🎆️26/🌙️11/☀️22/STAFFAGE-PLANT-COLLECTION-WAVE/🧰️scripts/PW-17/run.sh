#!/bin/sh
# 🏃️ PW-17: build one asset (dry candidate) and validate it: run.sh <id> [validate flags]
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/../../🗑️generated/PW-17"
T14="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
ID="$1"; shift
mkdir -p "$G"
(cd "$HERE" && repo tools exec bun -- build.ts "$ID" --dry) 2>&1 | grep -v "^\[capacity\]\|^\[tools\]\|still working" > "$G/build-$ID.txt"
(cd "$T14" && repo tools exec bun -- r3-validate.ts --file "$G/$ID/🔣️.json" "$@") 2>&1 | grep -v "^\[capacity\]\|^\[tools\]\|still working" > "$G/val-$ID.txt"
cat "$G/build-$ID.txt"; cat "$G/val-$ID.txt" | cut -c1-700

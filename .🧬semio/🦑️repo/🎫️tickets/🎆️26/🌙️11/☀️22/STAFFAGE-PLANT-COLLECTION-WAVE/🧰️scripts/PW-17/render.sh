#!/bin/sh
# 🖼️ PW-17: render the dry candidate of an asset: render.sh <id> <scale> [r3-2d|r3-sheet] [extra flags]
HERE="$(cd "$(dirname "$0")" && pwd)"
G="$HERE/../../🗑️generated/PW-17"
T14="$HERE/../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
ID="$1"; S="$2"; TOOL="${3:-r3-2d}"; shift 3 2>/dev/null || shift $#
(cd "$T14" && repo tools exec bun -- "$TOOL.ts" --file "$G/$ID/🔣️.json" --scale "$S" "$@") 2>&1 | grep -v "^\[capacity\]\|^\[tools\]\|still working" | tail -5

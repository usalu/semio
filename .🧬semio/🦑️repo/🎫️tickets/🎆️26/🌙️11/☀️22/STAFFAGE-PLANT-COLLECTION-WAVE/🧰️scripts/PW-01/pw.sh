#!/bin/sh
# 🌳 PW-01 helper: `pw.sh build <ids>` | `pw.sh validate <id> [r3-validate flags]` | `pw.sh preview <id> <scales> <viewpoints> [view] [pose]` | `pw.sh check <id>` (product check CLI on the candidate)
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
T="$HERE/../.."
G="$T/🗑️generated/PW-01"
S="$T/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
R3="$T/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
quiet() { grep -v "^\[capacity\]\|^\[repo\] still\|^\[tools\]" || true; }
case "$1" in
  build) cd "$HERE" && repo tools exec bun -- run.ts "$2" $3 2>&1 | quiet ;;
  validate) id="$2"; shift 2; cd "$R3" && repo tools exec bun -- r3-validate.ts --file "$G/$id/🔣️.json" "$@" 2>&1 | quiet | cut -c1-600 ;;
  check) cd "$S" && repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$G/$2/🔣️.json" --samples ${3:-5} 2>&1 | quiet | cut -c1-600 ;;
  preview)
    id="$2"; mkdir -p "$G/prev"
    for s in $(echo "$3" | tr ',' ' '); do for v in $(echo "$4" | tr ',' ' '); do
      (cd "$S" && repo tools exec bun -- 🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset "$id" --file "$G/$id/🔣️.json" --scale "$s" --view "${5:-light-solid}" ${6:+--pose $6} --viewpoint "$v" --no-png --out "$G/prev" 2>&1 | quiet | grep -v "^$" | tail -2) || true
    done; done
    DPI=${DPI:-150} repo tools exec bun -- "$HERE/png.ts" "$G"/prev/"$id"*.svg 2>&1 | quiet | tail -1 ;;
esac

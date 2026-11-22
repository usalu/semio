#!/bin/sh
# 🧹 Finisher A proof sweep: for each given vegetation id the full whole-asset validation of the catalog file (schema, validateAsset, print, checkVolumes) and the staffage check CLI, one tool process at a time; results in 🗑️generated/finish-a/<id>.{validate,check}.txt.
# usage: sweep.sh <id>...
HERE="$(cd "$(dirname "$0")" && pwd)"
K="$HERE/../.."
P="$K/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
S="$K/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
G="$K/🗑️generated/finish-a"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
mkdir -p "$G"
for id in "$@"; do
  (cd "$P" && "$BUN" r3-validate.ts "vegetation/$id" --catalog-only --volumes > "$G/$id.validate.txt" 2>&1)
  (cd "$S" && "$BUN" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$id" > "$G/$id.check.txt" 2>&1)
  echo "[DEBUG] $id | $(tail -1 "$G/$id.validate.txt" | cut -c1-160) | $(grep -m1 -e '^✓' -e '^✗' "$G/$id.check.txt" | cut -c1-80)"
done

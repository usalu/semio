#!/bin/zsh
# 🧬️ Refits solid shapes of a person from the volume into the work ops (`solidfit.ts`), printing one short line per shape: sf.sh <slug> <scale> <spec>…   (SRC=<asset.json>: the document the ops apply to instead of the catalog file, e.g. the backup of before)
source "${0:A:h}/env.sh"
a="$1"; sc="$2"; shift 2; mkdir -p "$GEN/ops"; o="$GEN/ops/$a.json"; [[ -f "$o" ]] || echo "[]" > "$o"
cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$W/solidfit.ts" "${SRC:-$a}" "$sc" "$o" "$@" --base "$o" 2>&1 | grep -v "centres\|the view runs" | sed -E 's/ about the axis.*radii/ r/; s/, fit across the view//' | cut -c1-150 | grep -E "bend ([4-9]|[1-9][0-9])|round ([4-9]|[1-9][0-9])|clamped [1-9]|error|rror|outlines|extruded" 

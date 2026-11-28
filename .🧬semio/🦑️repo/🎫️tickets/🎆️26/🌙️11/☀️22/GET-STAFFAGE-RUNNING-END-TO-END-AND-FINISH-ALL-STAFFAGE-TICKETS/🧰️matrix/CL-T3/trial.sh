#!/bin/zsh
# 🧫️ The trial copies of assets of lane CL-T3 with their size, inside the grant of its caller: `try.sh` (the repair into `$GEN/try/<id>.json`, the geometry findings of the copy), then the pose GLBs of the copy with their scenes (`glb.ts --scenes`). trial.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do zsh "$W/try.sh" "$a"; (cd "$S" && "$B" "$W/glb.ts" --scenes "$GEN/try/$a.json") 2>&1 | cut -c1-1500; done

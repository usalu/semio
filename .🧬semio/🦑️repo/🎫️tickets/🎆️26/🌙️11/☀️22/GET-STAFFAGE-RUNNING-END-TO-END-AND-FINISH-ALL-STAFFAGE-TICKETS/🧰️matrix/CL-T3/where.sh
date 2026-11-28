#!/bin/zsh
# 🔬️ Where the bytes and triangles of an asset (id or trial copy) sit, inside the grant of its caller: the pose GLBs with their scenes and owners (`glb.ts --scenes`), then every solid of every scene with its triangles (`geo.ts --solids`). where.sh <asset id | file.json>…
source "${0:A:h}/env.sh"
for a in "$@"; do (cd "$S" && "$B" "$W/glb.ts" --scenes "$a"; "$B" "$W/geo.ts" --solids "$a") 2>&1; done

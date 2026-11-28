#!/bin/zsh
# ⚖️ Runs `glb.ts` with the pinned bun from the staffage root: geo.sh <args of glb.ts>
source "${0:A:h}/env.sh"
cd "$S" && "$B" "$W/glb.ts" "$@"

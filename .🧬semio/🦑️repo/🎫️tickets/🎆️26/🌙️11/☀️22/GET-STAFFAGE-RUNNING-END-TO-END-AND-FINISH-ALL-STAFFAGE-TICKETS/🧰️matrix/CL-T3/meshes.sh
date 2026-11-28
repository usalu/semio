#!/bin/zsh
# 🧱️ Runs `meshes.ts` with the pinned bun from the staffage root: meshes.sh <args of meshes.ts>
source "${0:A:h}/env.sh"
cd "$S" && "$B" "$W/meshes.ts" "$@"

#!/bin/zsh
# 🧿️ Runs `geo.ts` with the pinned bun from the staffage root: geo.sh <args of geo.ts>
source "${0:A:h}/env.sh"
cd "$S" && "$B" "$W/geo.ts" "$@"

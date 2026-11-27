#!/bin/zsh
# 🩻️ Runs `invalid.ts` with the pinned bun from the staffage root: invalid.sh <asset id | file.json>…
source "${0:A:h}/env.sh"
cd "$S" && "$B" "$W/invalid.ts" "$@"

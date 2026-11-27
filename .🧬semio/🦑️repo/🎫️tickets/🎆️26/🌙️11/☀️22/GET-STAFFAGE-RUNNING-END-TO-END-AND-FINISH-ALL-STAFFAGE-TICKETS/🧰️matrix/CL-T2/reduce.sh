#!/bin/zsh
# 🔬️ Runs `reduce.ts` with the pinned bun from the staffage root: reduce.sh
source "${0:A:h}/env.sh"
cd "$S" && "$B" "$W/reduce.ts"

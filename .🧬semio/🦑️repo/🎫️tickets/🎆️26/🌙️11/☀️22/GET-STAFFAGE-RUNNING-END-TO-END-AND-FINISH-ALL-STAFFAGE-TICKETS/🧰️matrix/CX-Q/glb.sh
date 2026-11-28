#!/bin/zsh
# 🔺️ Runs `glb.ts` with the pinned bun through the memory queue (OLD=1 restores the clamped chord): glb.sh <args of glb.ts>
D="${0:A:h}"; source "$D/../MR/env.sh"; pre=(); [ -n "$OLD" ] && pre=(--preload "$D/old-quality.ts")
cd "$R" && repo capacity run --memory=3000 -- timeout 3000 "$B" "${pre[@]}" "$D/glb.ts" "$@"

#!/bin/zsh
# 🔺️ Runs `tri.ts` with the pinned bun through the memory queue (OLD=1 restores the clamped chord): tri.sh <args of tri.ts>
D="${0:A:h}"; source "$D/../MR/env.sh"; pre=(); [ -n "$OLD" ] && pre=(--preload "$D/old-quality.ts")
cd "$R" && repo capacity run --memory=1500 -- timeout 3000 "$B" "${pre[@]}" "$D/tri.ts" "$@"

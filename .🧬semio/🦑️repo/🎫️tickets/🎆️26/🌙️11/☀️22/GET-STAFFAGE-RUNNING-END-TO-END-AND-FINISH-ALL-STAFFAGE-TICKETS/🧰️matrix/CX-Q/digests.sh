#!/bin/zsh
# 🧬️ Runs `digests.ts` through the memory queue (OLD=1 restores the clamped chord): digests.sh <out.json>
D="${0:A:h}"; source "$D/../MR/env.sh"; pre=(); [ -n "$OLD" ] && pre=(--preload "$D/old-quality.ts")
cd "$R" && repo capacity run --memory=3000 -- timeout 3000 "$B" "${pre[@]}" "$D/digests.ts" "$@"

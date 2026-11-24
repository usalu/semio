#!/bin/zsh
# 🔍️ rings.ts in the package directory: r.sh <id> <scale> <view> [substring|-] [--at x y r] [--solids]
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript" && timeout 300 "$B" "$W/rings.ts" "$@" 2>&1 | cut -c1-230 | head -${HEAD:-40}

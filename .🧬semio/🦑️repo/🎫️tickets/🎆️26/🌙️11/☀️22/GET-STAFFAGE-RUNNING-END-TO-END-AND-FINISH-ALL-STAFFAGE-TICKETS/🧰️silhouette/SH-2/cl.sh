#!/bin/zsh
# 🧭️ The worst mismatch clusters of one person, scale and view with the nearest drawn shapes and volume solids: cl.sh <slug> <scale> <view|all> [count] [trial-ops.json]
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$W/clusters.ts" "$1" "$2" "${3:-all}" "${5:--}" "${4:-6}"

#!/bin/zsh
# 📦️ The projected boxes of the solids and drawn shapes of a view whose id matches a pattern: box.sh <id|file> <scale> <view> <grep pattern> [ops|-]
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript" && timeout 600 "$B" "$W/near.ts" "$1" "$2" "$3" 0 0 1000 "${5:--}" | grep -E "$4"

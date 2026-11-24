#!/bin/zsh
# 🧭️ Runs where.ts for triples "<id> <scale> <view>" given as arguments in threes, one process after the other: w.sh <id> <scale> <view> [<id> <scale> <view>]…
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript"
while (( $# >= 3 )); do timeout 600 "$B" "$W/where.ts" "$1" "$2" "$3" 2>&1 | cut -c1-600; shift 3; done

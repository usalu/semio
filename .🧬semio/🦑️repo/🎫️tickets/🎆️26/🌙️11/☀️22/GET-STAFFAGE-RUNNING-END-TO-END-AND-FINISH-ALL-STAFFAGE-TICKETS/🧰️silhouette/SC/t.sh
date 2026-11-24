#!/bin/zsh
# 🧱️ tiles.ts in the package directory, triples as arguments: t.sh <id> <scale> <view> […]
source "${0:A:h}/env.sh"
cd "$S/📦️packages/🟦️typescript"
while (( $# >= 3 )); do timeout 600 "$B" "$W/tiles.ts" "$1" "$2" "$3" 2>&1 | cut -c1-260 | head -${HEAD:-14}; shift 3; done

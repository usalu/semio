#!/bin/zsh
# 📦️ `boxes.ts` inside the grant of its caller: boxes.sh <id|file.json> <scale> <shape id> <view> [pose]
source "${0:A:h}/env.sh"
cd "$T" && timeout 900 "$B" "$W/boxes.ts" "$@"

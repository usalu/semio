#!/bin/zsh
# 🗒️ Lists the drawn shapes of a catalog asset: d.sh <id> [scale] [view]   (FULL=1 prints whole shapes)
export BATCH=MF; source "${0:A:h}/../MR/env.sh"
a="$1"; shift; python3 "$W/dump.py" "$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)" "$@"

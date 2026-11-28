#!/bin/zsh
# 👑️ `crown.ts` inside the grant of its caller: crown.sh <name of the output under $GEN/crown> <id | file.json> [crown options…] writes `$GEN/crown/<name>.json` (ops) and `$GEN/crown/<name>.txt`.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/crown"; n="$1"; a="$2"; shift 2
zsh "$W/run.sh" crown.ts "$a" --out "$GEN/crown/$n.json" "$@" > "$GEN/crown/$n.txt" 2>&1; echo "$n exit $?"; grep "^1:" "$GEN/crown/$n.txt" | cut -c1-520

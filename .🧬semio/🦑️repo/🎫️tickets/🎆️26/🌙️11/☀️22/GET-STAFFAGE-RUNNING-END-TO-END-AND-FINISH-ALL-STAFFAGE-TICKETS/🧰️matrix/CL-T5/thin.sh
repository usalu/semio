#!/bin/zsh
# 🪒️ `thin.ts` inside the grant of its caller: thin.sh <name of the output under $GEN/thin> <id | file.json> [thin options…] writes `$GEN/thin/<name>.json` (ops) and `$GEN/thin/<name>.txt`.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/thin"; n="$1"; a="$2"; shift 2
zsh "$W/run.sh" thin.ts "$a" --out "$GEN/thin/$n.json" "$@" > "$GEN/thin/$n.txt" 2>&1; echo "$n exit $?"; grep "^1:" "$GEN/thin/$n.txt" | cut -c1-520; grep -v "^1:" "$GEN/thin/$n.txt" | head -5 | cut -c1-300

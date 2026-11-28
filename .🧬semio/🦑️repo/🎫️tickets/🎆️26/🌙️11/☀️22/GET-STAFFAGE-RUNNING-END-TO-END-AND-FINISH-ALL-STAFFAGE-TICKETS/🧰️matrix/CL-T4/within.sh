#!/bin/zsh
# 🫥️ `within.ts` inside the grant of its caller: within.sh <name under $GEN/within> <id | file.json> [options…] writes `$GEN/within/<name>.json` (ops) and `.txt`.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/within"; n="$1"; a="$2"; shift 2
zsh "$W/run.sh" within.ts "$a" --out "$GEN/within/$n.json" "$@" > "$GEN/within/$n.txt" 2>&1; echo "$n exit $? $(grep -c '^  enclosed' "$GEN/within/$n.txt") enclosed, $(grep -c '^  free' "$GEN/within/$n.txt") free"

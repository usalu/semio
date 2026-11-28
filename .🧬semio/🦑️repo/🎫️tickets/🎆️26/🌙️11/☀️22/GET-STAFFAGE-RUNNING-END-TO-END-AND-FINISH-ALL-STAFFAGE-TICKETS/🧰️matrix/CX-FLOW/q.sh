#!/bin/zsh
# 🚦️ q.sh <MB> <timeout s> <name> <tool.ts> <args…>: one tool through the capacity queue, output to $GEN/<name>.txt
source "${0:A:h}/env.sh"
mb="$1"; t="$2"; n="$3"; shift 3; mkdir -p "$GEN"
cd "$R" && repo capacity run --memory="$mb" -- timeout "$t" zsh "$W/run.sh" "$@" > "$GEN/$n.txt" 2>&1
echo "$n exit $?"

#!/bin/zsh
# 🔎️ One diagnosis (see diag.ts): d.sh <slug> <scale> <view> [png-name] [x y radius]
source "${0:A:h}/env.sh"
mkdir -p "$GEN/png"
a="$1"; s="$2"; v="$3"; shift 3
png="-"; [[ -n "$1" ]] && png="$GEN/png/$1.png"; [[ $# -gt 0 ]] && shift
cd "$S/📦️packages/🟦️typescript" && timeout ${TMO:-900} "$B" "$W/diag.ts" "$a" "$s" "$v" "$png" "$@" 2>&1

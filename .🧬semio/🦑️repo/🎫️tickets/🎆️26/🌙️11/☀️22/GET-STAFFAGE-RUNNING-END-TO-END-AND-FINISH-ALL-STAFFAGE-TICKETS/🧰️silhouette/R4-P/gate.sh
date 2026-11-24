#!/bin/zsh
# 🌓️ The silhouette gate of plants, one process after the other, one log per plant in 🗑️generated/silhouette/R4-P/<dir>: gate.sh <dir> <id>…
source "${0:A:h}/env.sh"
d="$GEN/$1"; shift; mkdir -p "$d"
for a in "$@"; do
  (cd "$W" && timeout 900 "$B" measure.ts ${=MARGS} "$a" > "$d/$a.log" 2>&1)
  cat "$d/$a.log"
done

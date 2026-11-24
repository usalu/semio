#!/bin/zsh
# 🚢️ Ships the candidate ops of assets of batch SAV: 🗑️generated/silhouette/SAV/ops/<id>.json becomes `repair/<id>.json` and `apply.ts` applies it to the catalog file (re-read right before the write): promote.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do cp "$GEN/ops/$a.json" "$W/repair/$a.json" && (cd "$W" && timeout 300 "$B" apply.ts "$a"); done

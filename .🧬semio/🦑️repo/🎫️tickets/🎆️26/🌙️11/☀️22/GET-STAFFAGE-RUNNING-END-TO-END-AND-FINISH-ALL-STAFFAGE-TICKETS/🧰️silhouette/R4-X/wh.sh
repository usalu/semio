#!/bin/zsh
# 🧭️ where.ts for scales and views into logs (🗑️generated/silhouette/R4-X/where/<tag>-<scale>-<view>.log), one after the other: wh.sh <id|file> <tag> <scales,…> <views,…> <ops|-> [count]
source "${0:A:h}/env.sh"
mkdir -p "$GEN/where"; cd "$S/📦️packages/🟦️typescript" || exit 1
for s in ${(s:,:)3}; do for v in ${(s:,:)4}; do
  timeout 3000 "$B" "$W/where.ts" "$1" "$s" "$v" "${5:--}" "${6:-14}" > "$GEN/where/$2-$s-$v.log" 2>&1
  grep "^==" "$GEN/where/$2-$s-$v.log"
done; done

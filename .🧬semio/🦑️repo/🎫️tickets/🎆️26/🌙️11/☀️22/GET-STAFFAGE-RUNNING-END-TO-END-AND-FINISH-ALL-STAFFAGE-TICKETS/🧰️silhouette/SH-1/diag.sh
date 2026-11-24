#!/bin/zsh
# 🔎️ The worst mismatch clusters of failing views, one process after the other: diag.sh < lines "<slug> <scale> <view>"  (output 🗑️generated/silhouette/SH-1/diag/<slug>-<scale>-<view>.txt)
source "${0:A:h}/env.sh"
mkdir -p "$GEN/diag"
while read -r a s v; do (cd "$W" && timeout 300 "$B" clusters.ts "$a" "$s" "$v" - ${COUNT:-6}) > "$GEN/diag/$a-$s-$v.txt" 2>&1; echo "$a $s $v $(grep -c . "$GEN/diag/$a-$s-$v.txt")"; done

#!/bin/zsh
# 🔎️ TS1: type-checks package tsconfigs one at a time (nice, load-gated < 24) and records error counts.
# usage: zsh ts1-tsc-packages.zsh <round> <slug=tsconfig-path…>
round="$1"; shift
repo=/Users/ueli/Documents/semio
out="$repo/.🧬semio/🌐hub/s14-ts1-logs/$round"
mkdir -p "$out"
for pair in "$@"; do
  slug="${pair%%=*}"; config="${pair#*=}"
  while true; do load=$(sysctl -n vm.loadavg | awk '{print int($2)}'); [ "$load" -lt 24 ] && break; sleep 20; done
  start=$(date +%s)
  (cd "$repo/$(dirname "$config")" && NX_DAEMON=false nice -n 15 bunx tsc --noEmit -p "$(basename "$config")" > "$out/$slug.txt" 2>&1)
  rc=$?
  echo "$slug rc=$rc errors=$(/usr/bin/grep -c 'error TS' "$out/$slug.txt") wall=$(( $(date +%s) - start ))s" | tee -a "$out/summary.txt"
done

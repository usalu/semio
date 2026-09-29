#!/bin/zsh
# 🔎️ TS1: every tracked package tsconfig (ticket configs excluded), non-incremental, sequential, nice 15, load-gated < 24.
# usage: zsh ts1-tsc-all.zsh <round>
round="$1"
repo=/Users/ueli/Documents/semio
out="$repo/.🧬semio/🌐hub/s14-ts1-logs/$round"
mkdir -p "$out"; : > "$out/summary.txt"
cd "$repo" || exit 2
git ls-files -- '*tsconfig*.json' | /usr/bin/grep -v '^\.🧬semio/' | /usr/bin/grep -v node_modules | while IFS= read -r config; do
  [ "$config" = "tsconfig.json" ] && continue
  slug=$(dirname "$config" | tr '/' '\n' | sed -E 's/^[^A-Za-z0-9]+//' | /usr/bin/grep -v '^$' | /usr/bin/grep -v '^typescript$\|^packages$' | tail -2 | tr '\n' '-' | sed 's/-$//')
  while true; do load=$(sysctl -n vm.loadavg | awk '{print int($2)}'); [ "$load" -lt 24 ] && break; sleep 20; done
  start=$(date +%s)
  (cd "$repo/$(dirname "$config")" && NX_DAEMON=false nice -n 15 bunx tsc --noEmit --incremental false -p "$(basename "$config")" > "$out/$slug.txt" 2>&1)
  rc=$?
  echo "$config rc=$rc errors=$(/usr/bin/grep -c 'error TS' "$out/$slug.txt") wall=$(( $(date +%s) - start ))s" | tee -a "$out/summary.txt"
done

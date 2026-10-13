#!/bin/bash
# 🔁️ Repeats the wave-2 fix pass; waits while only foreign (upstream) errors remain; stops on a clean pass or on
# semio-owned errors it cannot fix. usage: 🔁️fix-loop.sh <passes> <cargo args...>
cd /Users/ueli/Documents/semio || exit 2
H=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/lane-c-semio-split"
N="$1"; shift
for i in $(seq 1 "$N"); do
  r=$(python3 "$H/w2_fix.py" "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml" "$@" 2>&1)
  head=$(echo "$r" | head -1); semio=$(grep -c '^error' "$H/../🗑️generated/lane-c/semio-errors.txt")
  echo "pass $i $(date +%H:%M:%S) $head semio-errors=$semio"
  echo "$head" | grep -q 'errors=0 ' && exit 0
  echo "$head" | grep -q 'pub=[1-9]\|deps={[^}]' && continue
  if [ "$semio" = "0" ] && ! echo "$r" | grep -q 'UNPROMOTED\|UNFIXED'; then sleep 45; continue; fi
  echo "$r" | tail -n +2 | head -40; exit 1
done

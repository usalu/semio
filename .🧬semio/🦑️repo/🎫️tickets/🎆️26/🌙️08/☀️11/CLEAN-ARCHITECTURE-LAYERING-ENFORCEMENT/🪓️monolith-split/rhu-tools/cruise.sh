#!/bin/sh
# Usage: cruise.sh <label> <full|scoped>   (rhu: renderer-hosts-ui). Output: 🗑️generated/rhu-<label>-{json,status}
L="$1"; MODE="$2"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
CFG="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs"
R="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
n=0
while [ $n -lt 8 ]; do
  n=$((n+1))
  if [ "$MODE" = "full" ]; then
    perl -e 'alarm 3000; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config "$CFG" --output-type err > "$G/rhu-$L-err.txt" 2> "$G/rhu-$L-err.stderr.txt"
  else
    perl -e 'alarm 1800; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$R/🎯️targets/⚛️react" "$R/🧱️elements" --do-not-follow '^(?!🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/(🎯️targets/⚛️react|🧱️elements)/)' --config "$CFG" --output-type json --output-to "$G/rhu-$L.json" 2> "$G/rhu-$L.stderr.txt"
  fi
  echo $? > "$G/rhu-$L-status.txt"
  if /usr/bin/grep -q -E "ENOENT|Dependency policy|ERROR" "$G/rhu-$L.stderr.txt" "$G/rhu-$L-err.stderr.txt" 2>/dev/null; then sleep 45; continue; fi
  break
done
date > "$G/rhu-$L-done.txt"

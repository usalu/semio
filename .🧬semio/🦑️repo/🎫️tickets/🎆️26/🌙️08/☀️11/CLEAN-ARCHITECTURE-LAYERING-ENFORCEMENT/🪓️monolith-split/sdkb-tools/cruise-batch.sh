#!/bin/sh
# Cruises the given roots with the audit-2 config; retries when a peer renames or deletes files mid-run.
# Usage: cruise-batch.sh <label> <root>...
L="$1"; shift
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
CFG="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs"
n=0
while [ $n -lt 12 ]; do
  n=$((n+1))
  date +%s > "$G/sdkb-$L-start.txt"
  perl -e 'alarm 2400; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" 🧰️framework/📦️packages/🟦️typescript --config "$CFG" --output-type err > "$G/sdkb-$L-err.txt" 2> "$G/sdkb-$L-err.stderr.txt"
  echo $? > "$G/sdkb-$L-status.txt"
  date +%s > "$G/sdkb-$L-end.txt"
  echo $n > "$G/sdkb-$L-attempts.txt"
  if /usr/bin/grep -q -E "ENOENT|Dependency policy|ERROR" "$G/sdkb-$L-err.stderr.txt"; then sleep 45; continue; fi
  break
done

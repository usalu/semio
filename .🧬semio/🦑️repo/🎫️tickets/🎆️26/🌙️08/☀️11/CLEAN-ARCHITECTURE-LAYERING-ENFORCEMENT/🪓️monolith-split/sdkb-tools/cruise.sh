#!/bin/sh
# Runs dependency-cruiser with the audit-2 section 1 config and retries when a peer deletes a file mid-run (ENOENT).
# Usage: cruise.sh <label> <full|scoped>
# full   = the exact audit command (roots: framework, s, hub, mit-bestand)
# scoped = roots plugins + the SDK package only (same config), for fast iteration
L="$1"; MODE="$2"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
CFG="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs"
n=0
while [ $n -lt 12 ]; do
  n=$((n+1))
  date +%s > "$G/sdkb-$L-start.txt"
  if [ "$MODE" = "full" ]; then
    perl -e 'alarm 2400; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config "$CFG" --output-type err > "$G/sdkb-$L-err.txt" 2> "$G/sdkb-$L-err.stderr.txt"
  else
    perl -e 'alarm 1800; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s/🔌️plugins 🧰️framework/📦️packages/🟦️typescript --config "$CFG" --output-type err > "$G/sdkb-$L-err.txt" 2> "$G/sdkb-$L-err.stderr.txt"
  fi
  echo $? > "$G/sdkb-$L-status.txt"
  date +%s > "$G/sdkb-$L-end.txt"
  echo $n > "$G/sdkb-$L-attempts.txt"
  if /usr/bin/grep -q -E "ENOENT|Dependency policy|ERROR" "$G/sdkb-$L-err.stderr.txt"; then sleep 45; continue; fi
  break
done

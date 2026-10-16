#!/bin/sh
# Runs dependency-cruiser with the audit-2 section 1 config; retries on peer churn (ENOENT, Dependency policy, ENFILE). Usage: cruise.sh <label> <full|scoped>


# full   = the exact audit command (roots: framework, s, hub, mit-bestand)
# scoped = roots plugins + the SDK package only (same config), for fast iteration
L="$1"; MODE="$2"
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cd /Users/ueli/Documents/semio || exit 2
CFG="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs"
n=0
while [ $n -lt 12 ]; do
  n=$((n+1))
  date +%s > "$G/psc2-$L-start.txt"
  if [ "$MODE" = "full" ]; then
    perl -e 'alarm 7200; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config "$CFG" --output-type err > "$G/psc2-$L-err.txt" 2> "$G/psc2-$L-err.stderr.txt"
  else
    perl -e 'alarm 7200; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s/🔌️plugins 🧰️framework/📦️packages/🟦️typescript --config "$CFG" --output-type err > "$G/psc2-$L-err.txt" 2> "$G/psc2-$L-err.stderr.txt"
  fi
  echo $? > "$G/psc2-$L-status.txt"
  date +%s > "$G/psc2-$L-end.txt"
  echo $n > "$G/psc2-$L-attempts.txt"
  st=$(cat "$G/psc2-$L-status.txt")
  if [ "$st" = "143" ] || [ "$st" = "137" ]; then sleep 30; continue; fi
  if /usr/bin/grep -q -E "ENOENT|Dependency policy|ERROR" "$G/psc2-$L-err.stderr.txt"; then sleep 45; continue; fi
  break
done

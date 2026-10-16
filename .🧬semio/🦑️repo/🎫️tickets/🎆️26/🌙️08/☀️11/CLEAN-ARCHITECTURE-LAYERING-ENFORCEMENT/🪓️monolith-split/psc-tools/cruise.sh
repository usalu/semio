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
  date +%s > "$G/psc-$L-start.txt"
  if [ "$MODE" = "full" ]; then
    perl -e 'alarm 2400; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config "$CFG" --output-type err > "$G/psc-$L-err.txt" 2> "$G/psc-$L-err.stderr.txt"
  else
    perl -e 'alarm 1800; exec @ARGV' -- node node_modules/dependency-cruiser/bin/dependency-cruise.mjs ✏️s/🔌️plugins 🧰️framework/📦️packages/🟦️typescript --config "$CFG" --output-type err > "$G/psc-$L-err.txt" 2> "$G/psc-$L-err.stderr.txt"
  fi
  echo $? > "$G/psc-$L-status.txt"
  date +%s > "$G/psc-$L-end.txt"
  echo $n > "$G/psc-$L-attempts.txt"
  if /usr/bin/grep -q -E "ENOENT|Dependency policy|ERROR" "$G/psc-$L-err.stderr.txt"; then sleep 45; continue; fi
  break
done

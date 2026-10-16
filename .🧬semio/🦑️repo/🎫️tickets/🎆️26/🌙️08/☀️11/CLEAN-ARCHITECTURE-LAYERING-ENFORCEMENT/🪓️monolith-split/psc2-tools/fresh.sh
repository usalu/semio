#!/bin/sh
# Re-dumps the policy snapshot and re-scans the edges. Usage: fresh.sh <label>
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
cd "$F" || exit 2
perl -e 'alarm 900; exec @ARGV' -- node psc2-tools/dump-policy.cjs "$G/psc2-policy-$1.json" > "$G/psc2-fresh-$1.log" 2>&1
perl -e 'alarm 1800; exec @ARGV' -- bun psc2-tools/scan.ts "$G/psc2-policy-$1.json" "$G/psc2-scan-$1.json" >> "$G/psc2-fresh-$1.log" 2>&1
tail -n 3 "$G/psc2-fresh-$1.log" | head -c 600

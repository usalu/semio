#!/bin/sh
# 🧪 FOR gate-g: `sh red-g-tools/verify.sh [mirror-name]` builds a mirror (mirror.py, sealed, no live write), applies the red-g batches in ORDER, runs the laws below
# and the whole-tree runtime-name law simulation. Expected: every line `0 fail` except the two known mirror artifacts (caching suites 'every test file' needs the os module; door startup 'sealed-mode-leaf' is the door lane's own row).
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/red-g-tools"; N="${1:-redg-v}"
cd /Users/ueli/Documents/semio || exit 1
sh "$T/mm.sh" create "$N" --only library process launcher "🧰️framework/🔨️modules/⚙️runtime" "🧰️framework/📦️packages/🟦️typescript/⚙️runtime" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" --batch "sh '$T/apply-all.sh' {root} tests tests2 runtimelaw leasesentry witnesshost gaterace inferfix startupbudget" || exit 1
R=$(sh "$T/mm.sh" list | awk -v n="$N" '$1==n{print $NF}'); M="🧰️framework/🔨️modules/⚙️runtime"
for e in "$M"/*; do [ -e "$R/$M/$(basename "$e")" ] || cp -cR "$e" "$R/$M/$(basename "$e")"; done
for e in "$M/🧪️tests"/*; do [ -e "$R/$M/🧪️tests/$(basename "$e")" ] || cp -cR "$e" "$R/$M/🧪️tests/$(basename "$e")"; done
OUT="$C/🗑️generated/red-g/verify-$N.log"; rm -f "$OUT"
sh "$T/run-list.sh" "$N" "$T/suite-list.txt" "$OUT"
cat "$OUT"
bun "$T/law-sim.ts" "$R" $(ls "$T"/staged/*/edits.json)

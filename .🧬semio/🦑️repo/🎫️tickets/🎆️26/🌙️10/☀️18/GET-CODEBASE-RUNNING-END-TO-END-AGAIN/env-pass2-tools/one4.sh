#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
F="$1"; N="$2"; TAG="${RUN_TAG:-r4}"; OUT="$C/🗑️generated/env-pass2-$TAG"
mkdir -p "$OUT"
LOG="$OUT/$N.log"
EP2_OVERLAY="${EP2_OVERLAY:-/Users/ueli/Documents/semio/$C/🗑️generated/ep2-overlay/map.json}" ENV_PASS2_TAG="$N" perl -e 'alarm 420; exec @ARGV' -- bun "$C/env-pass2-tools/wrap2.ts" --timeout 120000 "./$F" > "$LOG" 2>&1
RC=$?
P=$(command grep -a -E '^ *[0-9]+ pass$' "$LOG" | tail -1 | tr -dc 0-9)
X=$(command grep -a -E '^ *[0-9]+ fail$' "$LOG" | tail -1 | tr -dc 0-9)
echo "$N	rc=$RC	pass=${P:-?}	fail=${X:-?}	$F" >> "$C/env-pass2-tools/summary-$TAG.tsv"

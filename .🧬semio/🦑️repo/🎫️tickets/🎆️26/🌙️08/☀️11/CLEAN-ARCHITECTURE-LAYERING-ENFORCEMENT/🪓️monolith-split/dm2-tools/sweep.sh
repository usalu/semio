#!/bin/zsh
# 🧹 Reads every touched crate (lib, bins and tests) through the compile-ingress source reader; prints the failures only.
cd /Users/ueli/Documents/semio || exit 1
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
LOG="$F/🗑️generated/dm2-sweep.log"
: > "$LOG"
/usr/bin/grep "Cargo.toml$" "$F/dm2-tools/touched.txt" | sort -u | while IFS= read -r m; do
  [ -f "$m" ] || continue
  /usr/bin/grep -q '^\[package\]' "$m" || continue
  out=$(perl -e 'alarm 240; exec @ARGV' -- bun "$F/dm2-tools/closure.ts" "$m" --tests --summary 2>&1)
  if echo "$out" | /usr/bin/grep -q "^error"; then echo "FAIL $m" >> "$LOG"; echo "$out" | /usr/bin/grep -m3 "^error" | cut -c1-260 >> "$LOG"; else echo "ok   $m $(echo "$out" | /usr/bin/grep -c '^##')" >> "$LOG"; fi
done
echo done >> "$LOG"

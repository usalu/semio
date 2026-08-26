#!/usr/bin/env bash
# 🧪️ Sequential per-case parity for the four blockers of w21. Never piped: each exit code is the tool's own.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
OUT="$1"
shift
for c in "$@"; do
  echo "===== $c  start $(date +%T)" >> "$OUT"
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$c" >> "$OUT" 2>&1
  echo "===== $c  exit=$? end $(date +%T)" >> "$OUT"
done
echo "ALLDONE $(date +%T)" >> "$OUT"

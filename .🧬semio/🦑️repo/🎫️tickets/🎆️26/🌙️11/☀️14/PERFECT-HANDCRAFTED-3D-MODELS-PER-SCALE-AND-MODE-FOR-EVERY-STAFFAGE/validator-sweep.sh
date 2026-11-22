#!/bin/sh
# 🧹 Validates every catalog asset as published (`r3-validate.ts --catalog-only --json`, one process at a time) and keeps the issues: one JSON line per asset in 🗑️generated/validator/sweep-<name>.jsonl. Usage: validator-sweep.sh <name> [extra r3-validate flags, e.g. --semantics to leave out the print checks]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
NAME=$1; shift
OUT="🗑️generated/validator/sweep-$NAME.jsonl"
mkdir -p 🗑️generated/validator; : > "$OUT"
for p in /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/*/🔣️.json; do
  slug=$(basename "$(dirname "$p")"); cat=$(basename "$(dirname "$(dirname "$p")")" | LC_ALL=C sed 's/[^a-z-]//g')
  line=$(nice -n 10 "$BUN" r3-validate.ts "$cat/$slug" --catalog-only --json "$@" 2>/dev/null | tail -1)
  case "$line" in '{'*) printf '%s\n' "$line" >> "$OUT" ;; *) printf '{"asset":"%s/%s","crashed":true}\n' "$cat" "$slug" >> "$OUT" ;; esac
done
echo "$OUT $(wc -l < "$OUT")"

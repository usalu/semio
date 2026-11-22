#!/bin/sh
# 🧪️ Runs staffage suites natively, one after the other; one summary line per suite on stdout, full output in 🗑️generated/validator/suite-<name>.out. Usage: validator-suites.sh <suite folder name> …
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S="🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests"
OUT="🗑️generated/validator"
mkdir -p "$OUT"
for t in "$@"; do
  name=$(echo "$t" | LC_ALL=C tr -dc 'a-z-')
  VALIDATOR_TEST="$S/$t/**/*.ts" nice -n 5 "$BUN" /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --configLoader native --config "$PWD/validator-vitest.config.ts" --maxWorkers=1 > "$OUT/suite-$name.out" 2>&1
  echo "$name: $(grep -E '^ +(Tests|Test Files) ' "$OUT/suite-$name.out" | tr -s ' ' | tr '\n' ';')"
done

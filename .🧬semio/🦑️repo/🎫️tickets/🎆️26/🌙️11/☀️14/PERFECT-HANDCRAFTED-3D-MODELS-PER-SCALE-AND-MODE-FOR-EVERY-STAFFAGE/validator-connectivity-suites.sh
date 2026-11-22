#!/bin/sh
# 🧪️ Runs the staffage suites that the validator-connectivity work touches or that pin sizes, one after the other, natively; one summary line per suite on stdout, full output in 🗑️generated/validator-connectivity/suite-<name>.out. Usage: validator-connectivity-suites.sh [suite folder name …]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S="🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests"
OUT="🗑️generated/validator-connectivity"
mkdir -p "$OUT"
for t in ${@:-🧊️volumes-connected 🧊️volumes-rules 🧊️volumes-flood 🧊️volumes-thermal 🧿️exposed-normals 🧊️model-silhouette 🪤️volume-traps 🧊️model-scales 🧊️model-skeleton 🧊️model-bounds 🧊️model-looks 🧊️catalog-silhouette ✅️asset-validation 🧊️volume-schema}; do
  name=$(echo "$t" | tr -dc 'a-z-')
  VALIDATOR_TEST="$S/$t/**/*.ts" nice -n 5 "$BUN" /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --configLoader native --config "$PWD/validator-vitest.config.ts" --maxWorkers=1 > "$OUT/suite-$name.out" 2>&1
  echo "$name: $(grep -E '^ +Tests ' "$OUT/suite-$name.out" | tr -s ' ')"
done

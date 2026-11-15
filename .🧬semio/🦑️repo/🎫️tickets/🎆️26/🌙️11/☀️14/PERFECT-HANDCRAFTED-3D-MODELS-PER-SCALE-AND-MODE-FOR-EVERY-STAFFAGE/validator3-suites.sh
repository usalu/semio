#!/bin/sh
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
S="🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests"
for t in 🧊️volumes-thermal 🧿️exposed-normals 🧊️volumes-rules 🧊️model-silhouette 🪤️volume-traps ✅️asset-validation 🧊️volume-schema; do
  VALIDATOR_TEST="$S/$t/**/*.ts" nice -n 10 "$BUN" /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --configLoader native --config "$PWD/validator3-vitest.config.ts" --maxWorkers=2 > "🗑️generated/validator3/suite-$(echo $t | tr -dc 'a-z-').out" 2>&1
done
echo done > 🗑️generated/validator3/suites.done

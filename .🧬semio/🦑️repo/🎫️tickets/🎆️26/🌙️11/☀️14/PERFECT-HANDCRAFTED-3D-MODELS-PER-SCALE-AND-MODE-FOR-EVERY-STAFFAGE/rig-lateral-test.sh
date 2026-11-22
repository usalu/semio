#!/bin/sh
# Runs staffage test folders natively (no repo graph): rig-lateral-test.sh <test folder name> [...]; output in 🗑️generated/rig/test-<name>.out
T="$(cd "$(dirname "$0")" && pwd)"
S="🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "$T/🗑️generated/rig"
cd /Users/ueli/Documents/semio || exit 1
for t in "$@"; do
  out="$T/🗑️generated/rig/test-$(echo "$t" | tr -dc 'a-z0-9-').out"
  VALIDATOR_TEST="$S/$t/🟦️.ts" nice -n 10 "$BUN" node_modules/vitest/vitest.mjs run --configLoader native --config "$T/validator-vitest.config.ts" --maxWorkers=2 > "$out" 2>&1
  echo "$t: $(grep -E '^ *(Tests|Test Files) ' "$out" | tr -s ' ' | tr '\n' ';')"
done

#!/bin/zsh
# 🧪️ The other users of the weld oracle after its ring-loft clause, inside the grant of its caller: the suites `🌊️flow-model` and `🧊️model-scale-files` whole, then `🧊️model-catalog` for three other assets with lofts (`modelcat.sh`); one line each into `$GEN/proof/suites.txt`. suites.sh <id,id,id>
source "${0:A:h}/env.sh"
o="$GEN/proof/suites.txt"; mkdir -p "$GEN/vt" "$GEN/proof"
for suite in "🌊️flow-model" "🧊️model-scale-files"; do
  (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts") > "$GEN/vt/$suite.log" 2>&1
  echo "$(date +%H:%M) $suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/$suite.log" | tr -s ' ' | tr '\n' ';')" >> "$o"
done
zsh "$W/modelcat.sh" others "$1"

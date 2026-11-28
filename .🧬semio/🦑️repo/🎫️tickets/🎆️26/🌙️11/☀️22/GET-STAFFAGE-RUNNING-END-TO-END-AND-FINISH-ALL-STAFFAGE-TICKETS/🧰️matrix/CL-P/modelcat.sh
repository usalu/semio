#!/bin/zsh
# 🧊️ The real `🧊️model-catalog` with the given assets as the only selected ones (the command of the closing brief), inside the grant of its caller; log `$GEN/vt/<name>-model-catalog.log`, one line into `$GEN/proof/model-catalog.txt`. modelcat.sh <name> <id[,id…]>
source "${0:A:h}/env.sh"
mkdir -p "$GEN/vt" "$GEN/proof"; log="$GEN/vt/$1-model-catalog.log"
(cd "$R" && SEMIO_STAFFAGE_MODEL_ASSETS="$2" timeout 3600 "$B" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000) > "$log" 2>&1
echo "$(date +%H:%M) 🧊️model-catalog $2 exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$GEN/proof/model-catalog.txt"

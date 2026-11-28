#!/bin/zsh
# 🧪️ Runs the suites of lane CX-Q in one vitest process through the memory queue: suites.sh <log>
source "${0:A:h}/../MR/env.sh"
cd "$R" && repo capacity run --memory=3000 -- timeout 3400 "$B" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-digests/ /🧊️model-exports/ /🧊️cast-lod/ /🧊️model-scale-files/ /🧊️model-glyph-edges/ /🧊️model-glyph-size/ /🧊️model-silhouette/ /🧊️model-skeleton/ /🧊️model-batch/ /🧊️model-looks/ /🧊️catalog-silhouette/ /🧊️volumes-thermal/ /🧊️volumes-flood/ /🧊️volumes-connected/ /🧊️three-views/ /🧊️volume-schema/ --testTimeout=3000000 --hookTimeout=3000000 > "$1" 2>&1
echo "exit $?" >> "$1"

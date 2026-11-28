#!/bin/zsh
# 🔁️ The single-asset run of `🧊️model-catalog` again for written assets (after a lane changed the shared writer or the suite): its findings go to `$GEN/proof/<id>-modelcat.txt` and one more `model-catalog exit …` line to `$GEN/proof/<id>.txt`. recat.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do d="$GEN/proof"; o="$d/$a.txt"
s=$(date +%s); (cd "$R" && SEMIO_STAFFAGE_MODEL_ASSETS="$a" timeout 3600 "$B" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000) > "$d/$a-modelcat.log" 2>&1; r=$?
sed -E 's/\x1b\[[0-9;]*m//g' "$d/$a-modelcat.log" | grep -E '^[[:space:]]*\+[[:space:]]+"' | sed -E 's/^[[:space:]]*\+[[:space:]]+//' | sort -u > "$d/$a-modelcat.txt"
echo "model-catalog exit $r ($(( $(date +%s)-s )) s, run again $(date +%H:%M)): $(grep -cE 'triangles|bounds' "$d/$a-modelcat.txt") findings of this lane (triangles, bounds), $(grep -c 'GLB of' "$d/$a-modelcat.txt") GLB size, $(grep -cvE 'triangles|bounds|GLB of' "$d/$a-modelcat.txt") other" >> "$o"; tail -1 "$o"
done

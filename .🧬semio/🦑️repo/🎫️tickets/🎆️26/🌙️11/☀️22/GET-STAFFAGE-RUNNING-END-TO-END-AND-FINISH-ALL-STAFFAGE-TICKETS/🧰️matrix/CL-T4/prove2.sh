#!/bin/zsh
# 🩺️ Write and proof of one asset of lane CL-T4, one process after the other inside the grant of its caller. It refuses unless `$GEN/light/<id>.txt` ends in PASS for the very trial copy `$GEN/try/<id>.json` and the catalog file still is the one the copy was made from (`$GEN/try/<id>.base`). Then it copies the trial copy over the catalog file (its digest goes to `$GEN/written/<id>.sha` for `make.sh`), appends `CL-T4 rewrote <id> at <time>` to `requests.md` and writes one line per check into `$GEN/proof/<id>.txt`: the gate over every subject (`../MR/gate.ts --soft`), the official scan, the silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`), the check CLI, the matrix of the asset to its end and the single-asset run of `🧊️model-catalog` (its findings, one per line, in `$GEN/proof/<id>-modelcat.txt`; the lines of this lane are those with `triangles` or `bounds`). prove2.sh <id>
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d"; o="$d/$a.txt"; f="$(ls "$C"/*/"$a"/🔣️.json)"
grep -q "LIGHT PROOF $a: PASS" "$GEN/light/$a.txt" && grep -q "copy sha-256 $(shasum -a 256 < "$GEN/try/$a.json" | cut -c1-16)" "$GEN/light/$a.txt" || { echo "$a: no passed light proof of this trial copy, nothing written"; exit 1; }
now="$(shasum -a 256 < "$f" | cut -c1-16)"
if [[ "$now" != "$(shasum -a 256 < "$GEN/try/$a.json" | cut -c1-16)" ]]; then
  [[ "$now" == "$(cat "$GEN/try/$a.base")" ]] || { echo "$a: the catalog file changed since the trial copy was made, nothing written"; exit 1; }
  cat "$GEN/try/$a.json" > "$f"; mkdir -p "$GEN/written"; shasum -a 256 < "$f" | cut -c1-16 > "$GEN/written/$a.sha"
  printf '\nCL-T4 rewrote %s at %s\n' "$a" "$(date '+%F %H:%M:%S')" >> "$U/requests.md"
fi
echo "proof of $(date +%F\ %H:%M), written $now -> $(shasum -a 256 < "$f" | cut -c1-16)" > "$o"
cd "$T"
timeout 3000 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log" | cut -c1-200)" >> "$o"
timeout 3000 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"
(cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$a") > "$d/$a-measure.log" 2>&1; echo "measure exit $?" >> "$o"; grep -E "silhouette:|validate:|sweep:|bbox" "$d/$a-measure.log" | cut -c1-300 >> "$o"
(cd "$S" && timeout 3000 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)" >> "$o"
rm -rf "$GEN/matrix-$a"; s=$(date +%s); timeout ${MATRIX_TIMEOUT:-7200} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1
echo "matrix exit $? $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1 | cut -c1-300) ($(( $(date +%s)-s )) s) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" >> "$o"
s=$(date +%s); (cd "$R" && SEMIO_STAFFAGE_MODEL_ASSETS="$a" timeout 3600 "$B" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" /🧊️model-catalog/ --testTimeout=3600000 --hookTimeout=3600000) > "$d/$a-modelcat.log" 2>&1; r=$?
sed -E 's/\x1b\[[0-9;]*m//g' "$d/$a-modelcat.log" | grep -E '^[[:space:]]*\+[[:space:]]+"' | sed -E 's/^[[:space:]]*\+[[:space:]]+//' | sort -u > "$d/$a-modelcat.txt"
echo "model-catalog exit $r ($(( $(date +%s)-s )) s): $(grep -cE 'triangles|bounds' "$d/$a-modelcat.txt") findings of this lane (triangles, bounds), $(grep -c 'GLB of' "$d/$a-modelcat.txt") GLB size, $(grep -c 'KHR_mesh_quantization' "$d/$a-modelcat.txt") of the quantized writer of lane CX-GLB, $(grep -cvE 'triangles|bounds|GLB of|KHR_mesh_quantization' "$d/$a-modelcat.txt") other; $(grep -E 'Tests ' "$d/$a-modelcat.log" | tail -1 | sed -E 's/\x1b\[[0-9;]*m//g' | tr -s ' ')" >> "$o"
grep -vE 'KHR_mesh_quantization' "$d/$a-modelcat.txt" | cut -c1-240 >> "$o"
echo "sha-256 $(shasum -a 256 < "$f" | cut -c1-16), $(wc -c < "$f" | tr -d ' ') bytes; done $(date +%H:%M)" >> "$o"
touch "$d/$a.proved"

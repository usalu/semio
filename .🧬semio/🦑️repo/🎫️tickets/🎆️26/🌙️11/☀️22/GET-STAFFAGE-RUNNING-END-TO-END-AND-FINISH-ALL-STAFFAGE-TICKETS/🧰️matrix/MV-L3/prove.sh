#!/bin/zsh
# 🩺️ Write and full proof of one tree of round MV-L3, one process after the other inside the grant of its caller. It refuses unless `$GEN/light/<id>.txt` ends in PASS for the very trial copy `$GEN/try/<id>.json` (digest in its first line). Then, one line per check into `$GEN/proof/<id>.txt`: `make.sh` twice (the second run must leave the digest), the catalog file against the trial copy, the gate over every subject (`../MR/gate.ts --soft`), the official scan, print suitability at the key and both ends of every band (`../MC/prints.ts`), the silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`), the check CLI, and the matrix of the asset to its end; `$GEN/proof/<id>.proved` marks the end. The six real test files follow in `suites.sh`. prove.sh <id>
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d"; o="$d/$a.txt"; f="$(ls "$C"/*/"$a"/🔣️.json)"
grep -q "LIGHT PROOF $a: PASS" "$GEN/light/$a.txt" && grep -q "copy sha-256 $(shasum -a 256 < "$GEN/try/$a.json" | cut -c1-16)" "$GEN/light/$a.txt" || { echo "$a: no passed light proof of this trial copy, nothing written"; exit 1; }
echo "proof of $(date +%F\ %H:%M)" > "$o"
zsh "$W/make.sh" "$a" >> "$o" 2>&1; zsh "$W/make.sh" "$a" 2>&1 | tail -1 | sed "s/^/second run: /" >> "$o"
echo "same as the trial copy: $(python3 "$W3/same.py" "$GEN/try/$a.json" "$f")" >> "$o"
cd "$T"
timeout 3000 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$o"
timeout 3000 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"
timeout 3000 "$B" "$U/🧰️matrix/MC/prints.ts" "$a" > "$d/$a-prints.log" 2>&1; echo "prints exit $? $(head -1 "$d/$a-prints.log" | cut -c1-200)" >> "$o"
(cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$a") > "$d/$a-measure.log" 2>&1; echo "measure exit $?" >> "$o"; grep -E "silhouette:|validate:|sweep:|bbox" "$d/$a-measure.log" | cut -c1-300 >> "$o"
(cd "$S" && timeout 3000 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" >> "$o"
rm -rf "$GEN/matrix-$a"; s=$(date +%s); timeout ${MATRIX_TIMEOUT:-14400} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1
echo "matrix exit $? $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" >> "$o"
echo "sha-256 $(shasum -a 256 < "$f" | cut -c1-16), $(wc -c < "$f" | tr -d ' ') bytes; done $(date +%H:%M)" >> "$o"
touch "$d/$a.proved"

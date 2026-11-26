#!/bin/zsh
# 🩺️ The light proofs of one asset of round MV-L2, one process after the other inside the grant of the worker; one line per check into `$GEN/proof/<id>.txt`: `make.sh` twice (the second run must leave the digest), the catalog file against the trial copy that was looked at, the gate over every subject (`../MR/gate.ts --soft`), the official scan, print suitability at the key and both ends of every band (`../MC/prints.ts`), the silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`, backup first), the check CLI, and the matrix of the asset to its end; `$GEN/proof/<id>.proved` marks the end for the suites job. prove.sh <id>
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d"; o="$d/$a.txt"; f="$(ls "$C"/*/"$a"/🔣️.json)"
rm -f "$d/$a.proved"; echo "proof of $(date +%F\ %H:%M)" > "$o"
zsh "$W/make.sh" "$a" >> "$o" 2>&1; zsh "$W/make.sh" "$a" 2>&1 | tail -1 | sed "s/^/second run: /" >> "$o"
echo "same as the trial copy: $(python3 "$U/🧰️matrix/MV-3/same.py" "$GEN/try/$a.json" "$f")" >> "$o"
cd "$T"
timeout 3000 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$o"
timeout 3000 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"
timeout 3000 "$B" "$U/🧰️matrix/MC/prints.ts" "$a" > "$d/$a-prints.log" 2>&1; echo "prints exit $? $(head -1 "$d/$a-prints.log" | cut -c1-200)" >> "$o"
(cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$a") > "$d/$a-measure.log" 2>&1; echo "measure exit $?" >> "$o"; grep -E "silhouette:|validate:|sweep:|bbox" "$d/$a-measure.log" | cut -c1-260 >> "$o"
(cd "$S" && timeout 3000 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" >> "$o"
rm -rf "$GEN/matrix-$a"; s=$(date +%s); timeout ${MATRIX_TIMEOUT:-14400} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1
echo "matrix exit $? $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" >> "$o"
echo "sha-256 $(shasum -a 256 < "$f" | cut -c1-16), $(wc -c < "$f" | tr -d ' ') bytes; done $(date +%H:%M)" >> "$o"
touch "$d/$a.proved"

#!/bin/zsh
# 🩺️ Write and full proof of one asset of closing lane CL-A, one process after the other inside the grant of its caller. It refuses unless `$GEN/light/<id>.txt` ends in PASS for the very trial copy `$GEN/try/<id>.json` and the catalog file still is the backup the copy was made from. Then, one line per check into `$GEN/proof/<id>.txt`: the repair tool on the catalog file twice (the second run must leave the digest; a file this lane wrote before is first replaced by the trial copy, which is the tool on the backup), the catalog file against the trial copy (bytes) and its drawings against the backup (`../MV-3/same.py`), a line for lane MZ in `requests.md`, the pins (`pins.ts`), the gate over every subject (`../MR/gate.ts --soft`), the official scan, print suitability (`../MC/prints.ts`), the silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`), the check CLI and the matrix of the asset to its end; `$GEN/proof/<id>.proved` marks the end. The real test files follow in `suites.sh`. prove.sh <id> <closing section title pattern of requests.md>
source "${0:A:h}/env.sh"
a="$1"; d="$GEN/proof"; mkdir -p "$d"; o="$d/$a.txt"; f="$(ls "$C"/*/"$a"/🔣️.json)"
grep -q "LIGHT PROOF $a: PASS" "$GEN/light/$a.txt" && grep -q "copy sha-256 $(shasum -a 256 < "$GEN/try/$a.json" | cut -c1-16)" "$GEN/light/$a.txt" || { echo "$a: no passed light proof of this trial copy, nothing written"; exit 1; }
cmp -s "$f" "$GEN/backup/$a.json" || cmp -s "$f" "$GEN/try/$a.json" || cmp -s "$f" "$GEN/written/$a.json" || { echo "$a: the catalog file is neither the backup, the trial copy nor this lane's last write (somebody wrote it), nothing written"; exit 1; }
echo "proof of $(date +%F\ %H:%M)" > "$o"
echo "before $(shasum -a 256 < "$f" | cut -c1-16)" >> "$o"
cmp -s "$f" "$GEN/backup/$a.json" || { cat "$GEN/try/$a.json" > "$f"; echo "the catalog file was this lane's earlier write: replaced by the trial copy" >> "$o"; }
zsh "$W/tool.sh" "$a" "$f" "$f" 2>&1 | tail -1 | sed "s/^/first run: /" >> "$o"; one="$(shasum -a 256 < "$f" | cut -c1-16)"
zsh "$W/tool.sh" "$a" "$f" "$f" 2>&1 | tail -1 | sed "s/^/second run: /" >> "$o"
echo "after $one, after the second run $(shasum -a 256 < "$f" | cut -c1-16); $(cmp -s "$f" "$GEN/try/$a.json" && echo "equal to the trial copy (bytes)" || echo "DIFFERS from the trial copy")" >> "$o"
mkdir -p "$GEN/written"; cp "$f" "$GEN/written/$a.json"; echo "drawings against the backup: $(python3 "$W3/same.py" "$GEN/backup/$a.json" "$f")" >> "$o"
python3 "$W/tell.py" "$a" "$2" >> "$o" 2>&1
(cd "$S/📦️packages/🟦️typescript" && "$B" "$W/pins.ts" "$a" "$f") > "$GEN/pins/$a-after.txt" 2>&1; echo "pins: $(grep -E 'failures|RED' "$GEN/pins/$a-after.txt" | tr '\n' ';' | cut -c1-400)" >> "$o"
cd "$T"
timeout 3000 "$B" "$M/gate.ts" "$a" --soft > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log")" >> "$o"
timeout 3000 "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)" >> "$o"
timeout 3000 "$B" "$U/🧰️matrix/MC/prints.ts" "$a" > "$d/$a-prints.log" 2>&1; echo "prints exit $? $(head -1 "$d/$a-prints.log" | cut -c1-200)" >> "$o"
(cd "$S/📦️packages/🟦️typescript" && timeout 3000 "$B" "$M/measure.ts" "$a") > "$d/$a-measure.log" 2>&1; echo "measure exit $?" >> "$o"; grep -E "silhouette:|validate:|sweep:|bbox" "$d/$a-measure.log" | cut -c1-300 >> "$o"
(cd "$S" && timeout 3000 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$d/$a-check.log" 2>&1; echo "check: exit $? $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160); $(grep -ciE 'prints|suitable' "$d/$a-check.log") suitability lines" >> "$o"
rm -rf "$GEN/matrix-$a"; s=$(date +%s); timeout ${MATRIX_TIMEOUT:-7200} "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1
echo "matrix exit $? $(grep '\[matrix\].*cells:' "$d/$a-matrix.log" | tail -1) ($(( $(date +%s)-s )) s) $(python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | tail -3 | tr '\n' ' ' | cut -c1-600)" >> "$o"
echo "sha-256 $(shasum -a 256 < "$f" | cut -c1-16), $(wc -c < "$f" | tr -d ' ') bytes; done $(date +%H:%M)" >> "$o"
touch "$d/$a.proved"

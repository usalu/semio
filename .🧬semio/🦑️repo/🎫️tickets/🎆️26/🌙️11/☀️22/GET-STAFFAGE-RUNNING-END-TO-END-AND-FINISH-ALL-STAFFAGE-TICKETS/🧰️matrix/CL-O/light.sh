#!/bin/zsh
# 🪶️ The light proof of TRIAL COPIES of lane CL-O (`$GEN/try/<id>.json`), which must pass BEFORE the catalog file is written, one process after the other inside the grant of its caller (the proof of `../MV-L3/light.sh` plus the pins of this lane): the gate over every subject (`../MR/gate.ts --soft`), the gate over the poses alone as the scan line, the silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`), the check CLI with `--file`, the detail pins (`pins.ts`) and for the gas hob the drawn dimension (`depth.ts`). One line per check into `$GEN/light/<id>.txt`, ending in `LIGHT PROOF <id>: PASS|FAIL` with the digest of the copy. light.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
f="$GEN/try/$a.json"; d="$GEN/light"; mkdir -p "$d"; ok=1; o="$d/$a.txt"; echo "light proof of $(date +%F\ %H:%M), copy sha-256 $(shasum -a 256 < "$f" | cut -c1-16)" > "$o"
(cd "$T" && timeout 1200 "$B" "$M/gate.ts" "$f" --soft) > "$d/$a-gate.log" 2>&1; r=$?; echo "gate exit $r $(tail -1 "$d/$a-gate.log" | cut -c1-200)" >> "$o"; [[ $r -eq 0 ]] || ok=0
(cd "$T" && timeout 1200 "$B" "$M/gate.ts" "$f" --poses) > "$d/$a-poses.log" 2>&1; r=$?; echo "scan (gate over the poses) exit $r $(tail -1 "$d/$a-poses.log" | cut -c1-200)" >> "$o"; [[ $r -eq 0 ]] || ok=0
(cd "$S/📦️packages/🟦️typescript" && timeout 1800 "$B" "$M/measure.ts" "$f") > "$d/$a-measure.log" 2>&1; r=$?; echo "measure exit $r" >> "$o"; grep -E "silhouette:|validate:|sweep:|bbox" "$d/$a-measure.log" | cut -c1-300 >> "$o"; grep '!' "$d/$a-measure.log" | cut -c1-300 >> "$o"
[[ $r -eq 0 ]] && grep -q "silhouette: OK" "$d/$a-measure.log" && grep -q "validate: 0 issues" "$d/$a-measure.log" && grep -q "sweep: 0 findings" "$d/$a-measure.log" || ok=0
(cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$f") > "$d/$a-check.log" 2>&1; r=$?; echo "check: exit $r $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)" >> "$o"; [[ $r -eq 0 ]] || ok=0
if [[ "$a" == gas-hob ]]; then
  (cd "$S/📦️packages/🟦️typescript" && timeout 300 "$B" "$W/depth.ts" "$f") > "$d/$a-depth.log" 2>&1; r=$?; cut -c1-200 "$d/$a-depth.log" >> "$o"; [[ $r -eq 0 ]] && grep -q "DEPTH $a: OK" "$d/$a-depth.log" || ok=0
else
  (timeout 300 "$B" "$W/pins.ts" "$f") > "$d/$a-pins.log" 2>&1; r=$?; cut -c1-200 "$d/$a-pins.log" >> "$o"; [[ $r -eq 0 ]] && grep -q "PINS $a: OK" "$d/$a-pins.log" || ok=0
fi
[[ $ok -eq 1 ]] && echo "LIGHT PROOF $a: PASS" >> "$o" || echo "LIGHT PROOF $a: FAIL" >> "$o"
done

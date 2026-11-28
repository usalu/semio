#!/bin/zsh
# 🪶️ The light proof of TRIAL COPIES of lane CL-T3 (`$GEN/try/<id>.json`), which must pass BEFORE the catalog file is written, one process after the other inside the grant of its caller: the gate over every subject (`../MR/gate.ts --soft`), the gate over the poses alone as the scan line, the silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`), the check CLI with `--file` the geometry findings of `🧊️model-catalog` (`geo.ts`: none) and the pose GLBs (`glb.ts`: every pose at least `MARGIN` MB, default 0.2, below the budget of 8 MB). One line per check into `$GEN/light/<id>.txt`, ending in `LIGHT PROOF <id>: PASS|FAIL` with the digest of the copy. light.sh <id>…
source "${0:A:h}/env.sh"
for a in "$@"; do
f="$GEN/try/$a.json"; d="$GEN/light"; mkdir -p "$d"; ok=1; o="$d/$a.txt"; echo "light proof of $(date +%F\ %H:%M), copy sha-256 $(shasum -a 256 < "$f" | cut -c1-16)" > "$o"
(cd "$T" && timeout 1200 "$B" "$M/gate.ts" "$f" --soft) > "$d/$a-gate.log" 2>&1; r=$?; echo "gate exit $r $(tail -1 "$d/$a-gate.log" | cut -c1-200)" >> "$o"; [[ $r -eq 0 ]] || ok=0
(cd "$T" && timeout 1200 "$B" "$M/gate.ts" "$f" --poses) > "$d/$a-poses.log" 2>&1; r=$?; echo "scan (gate over the poses) exit $r $(tail -1 "$d/$a-poses.log" | cut -c1-200)" >> "$o"; [[ $r -eq 0 ]] || ok=0
(cd "$S/📦️packages/🟦️typescript" && timeout 1800 "$B" "$M/measure.ts" "$f") > "$d/$a-measure.log" 2>&1; r=$?; echo "measure exit $r" >> "$o"; grep -E "silhouette:|validate:|sweep:|bbox" "$d/$a-measure.log" | cut -c1-300 >> "$o"; grep '!' "$d/$a-measure.log" | cut -c1-300 >> "$o"
[[ $r -eq 0 ]] && grep -q "silhouette: OK" "$d/$a-measure.log" && grep -q "validate: 0 issues" "$d/$a-measure.log" && grep -q "sweep: 0 findings" "$d/$a-measure.log" || ok=0
(cd "$S" && timeout 1800 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$f") > "$d/$a-check.log" 2>&1; r=$?; echo "check: exit $r $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)" >> "$o"; [[ $r -eq 0 ]] || ok=0
(cd "$S" && timeout 900 "$B" "$W/geo.ts" "$f") > "$d/$a-geo.log" 2>&1; r=$?; cut -c1-400 "$d/$a-geo.log" >> "$o"; [[ $r -eq 0 ]] && grep -q "^$a findings 0:" "$d/$a-geo.log" || ok=0
(cd "$S" && timeout 1800 "$B" "$W/glb.ts" "$f") > "$d/$a-glb.log" 2>&1; r=$?; cut -c1-200 "$d/$a-glb.log" >> "$o"; [[ $r -eq 0 ]] && awk -v m="${MARGIN:-0.2}" '/ pose:/{n++; if ($0 ~ /OVER/ || $(NF-1) + 0 < m) bad++} END{exit !(n > 0 && bad == 0)}' "$d/$a-glb.log" || ok=0
[[ $ok -eq 1 ]] && echo "LIGHT PROOF $a: PASS" >> "$o" || echo "LIGHT PROOF $a: FAIL" >> "$o"
done

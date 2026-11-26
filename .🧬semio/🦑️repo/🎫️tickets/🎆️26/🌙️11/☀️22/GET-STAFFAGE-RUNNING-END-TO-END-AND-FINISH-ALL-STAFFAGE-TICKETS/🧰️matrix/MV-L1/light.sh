#!/bin/zsh
# 🪶️ The light proof of a TRIAL COPY of a plant of batch MV-L1 (`$GEN/try/<id>.json`), which must pass before the catalog file is written (binding since 2026-11-26 03:38: `black-alder` was written on a green gate alone and failed the silhouette guard by 1207 mm). One process after the other inside the reservation of its caller (1100 MB: `measure.ts` peaks at 833 MB): gate over every subject (it holds the checks of the official scan, which reads the catalog only and runs after the write), the gate over the poses alone as the scan line, silhouette guard with `validateAsset` and the three-views sweep (`../MR/measure.ts`), check CLI with `--file`. Prints one line per check and `LIGHT PROOF <id>: PASS|FAIL`. light.sh <id>
source "${0:A:h}/env.sh"
a="$1"; f="$GEN/try/$a.json"; d="$GEN/light"; mkdir -p "$d"; ok=1
"$W/q.sh" 0 "$T" 3000 "$B" "$M/gate.ts" "$f" --soft > "$d/$a-gate.log" 2>&1; r=$?; echo "gate exit $r $(tail -1 "$d/$a-gate.log")"; [[ $r -eq 0 ]] || ok=0
"$W/q.sh" 0 "$T" 3000 "$B" "$M/gate.ts" "$f" --poses > "$d/$a-poses.log" 2>&1; r=$?; echo "scan (gate over the poses) exit $r $(tail -1 "$d/$a-poses.log")"; [[ $r -eq 0 ]] || ok=0
"$W/q.sh" 0 "$S/📦️packages/🟦️typescript" 2400 "$B" "$M/measure.ts" "$f" > "$d/$a-measure.log" 2>&1; r=$?; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log"; grep -q '!' "$d/$a-measure.log" && grep '!' "$d/$a-measure.log" | cut -c1-200
[[ $r -eq 0 ]] && grep -q "silhouette: OK" "$d/$a-measure.log" && grep -q "validate: 0 issues" "$d/$a-measure.log" && grep -q "sweep: 0 findings" "$d/$a-measure.log" || ok=0
"$W/q.sh" 0 "$S" 2400 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$f" > "$d/$a-check.log" 2>&1; r=$?; echo "check: exit $r $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"; [[ $r -eq 0 ]] || ok=0
[[ $ok -eq 1 ]] && echo "LIGHT PROOF $a: PASS" || echo "LIGHT PROOF $a: FAIL"

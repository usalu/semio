#!/bin/zsh
# 🛬️ Lands the repair of one person: land.sh check <id> applies ops/<id>.json to a copy ($GEN/try/<id>.json), applies it a second time (must change nothing), and runs the model gate (mgate.ts; REAL_GATE=1 runs the real gate of MR over every pose, clip instant and transition instead, which `prove` runs on the catalog file anyway) measure.ts (silhouette guard, validateAsset, three-views sweep) on the backup and the copy, the check CLI on the copy (`--file`) and the anchor and whole-figure findings of the real gate over the poses. land.sh write <id> applies the ops to the backup (the asset as it was before this batch) and puts the result in the catalog, refused unless the catalog file is still the backup or what this fixer landed last; then the ops are applied to the catalog file once more (must report 0 changed). land.sh prove <id> runs the proof of MR (`prove.sh`: gate, scan, measure, check CLI, the six tests) into $GEN/proof; land.sh matrix <id> runs the matrix of the asset (the slow confirmation, run for all assets after their proofs). land.sh all <id> does check, stops when the gate of the copy has a hard finding (ALLOW_HARD=1 lands an asset whose remaining hard pairs are known and stated as open), the guard fails, validateAsset or the sweep differ from the backup or the check CLI fails on the copy, else writes and proves. Heavy steps hold the lock of q.sh.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
step="$1"; a="$2"; ops="$W/ops/$a.json"; out="$GEN/try/$a.json"; mkdir -p "$GEN/try" "$GEN/land"
case "$step" in
  check)
    (cd "$T" && "$W/b.sh" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$ops" --out "$out" | tail -1 && "$W/b.sh" "$W/fit.ts" "$out" --ops "$ops" | tail -1)
    if [[ -n "$REAL_GATE" ]]; then
      (cd "$T" && "$W/q.sh" timeout 2400 "$B" "$M/gate.ts" "$out" --twins) > "$GEN/land/$a.gate.log" 2>&1; echo "gate: $(tail -1 "$GEN/land/$a.gate.log"); $(grep -c ' HARD ' "$GEN/land/$a.gate.log") hard lines"
      grep -A3 " HARD " "$GEN/land/$a.gate.log" | cut -c1-260
    else
      mkdir -p "$GEN/mgate"; (cd "$T" && "$W/b.sh" "$W/mgate.ts" "$out") > "$GEN/mgate/$a.after.log" 2>&1; grep "1:[0-9]* HARD " "$GEN/mgate/$a.after.log" | cut -c1-220
      echo "gate: hard findings: $(grep -c '1:[0-9]* HARD ' "$GEN/mgate/$a.after.log"); model gate ($(tail -1 "$GEN/mgate/$a.after.log"))"
    fi
    (cd "$S/📦️packages/🟦️typescript" && "$W/q.sh" timeout 1800 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$out") > "$GEN/land/$a.measure.log" 2>&1; grep -v "^ *$" "$GEN/land/$a.measure.log" | cut -c1-250
    echo "checkfile: $(cd "$S" && "$W/q.sh" timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$out" 2>&1 | grep -E '^(✓|✗)|suitability-|^  [a-z-]+ ' | grep -v authored | tr '\n' ' ' | cut -c1-240)"
    "$W/anchors.sh" "$out" | sed 's/^/poses: /' ;;
  write)
    file="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; mkdir -p "$GEN/landed"
    cmp -s "$file" "$GEN/backup/$a.json" || cmp -s "$file" "$GEN/landed/$a.json" || { echo "REFUSED $a: the catalog file is neither the backup nor what this fixer landed last (somebody else changed it): nothing written"; exit 1; }
    (cd "$T" && "$W/apply.sh" "$GEN/backup/$a.json" "$ops" "$out" | grep -E "REFUSED|something|views changed")
    cmp -s "$file" "$GEN/backup/$a.json" || cmp -s "$file" "$GEN/landed/$a.json" || { echo "REFUSED $a: the catalog file changed meanwhile"; exit 1; }
    cp "$out" "$file" && cp "$out" "$GEN/landed/$a.json" && echo "written: $file"
    (cd "$T" && "$W/apply.sh" "$file" "$ops" | grep -E "REFUSED|something|views changed") ;;
  prove)
    "$W/q.sh" env BATCH=MH-5 MATRIX= "$M/prove.sh" "$a" 2>&1 | cut -c1-260 ;;
  matrix)
    rm -rf "$GEN/matrix-$a"; (cd "$T" && "$W/q.sh" timeout 9000 "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$GEN/proof/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$GEN/proof/$a-matrix.log")"
    python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-260 ;;
  all)
    "$0" check "$a" > "$GEN/land/$a.check.out" 2>&1; cat "$GEN/land/$a.check.out"
    [[ -n "$ALLOW_HARD" ]] || grep -q "^gate: hard findings: 0;" "$GEN/land/$a.check.out" || { echo "STOP $a: the gate of the copy is not clean"; exit 1; }
    [[ "$(grep -c 'silhouette: OK' "$GEN/land/$a.check.out")" == 2 ]] || { echo "STOP $a: silhouette guard"; exit 1; }
    [[ "$(grep 'validate:' "$GEN/land/$a.check.out" | sort -u | wc -l | tr -d ' ')" == 1 && "$(grep 'three-views sweep:' "$GEN/land/$a.check.out" | sort -u | wc -l | tr -d ' ')" == 1 ]] || { echo "STOP $a: validate or sweep differ"; exit 1; }
    grep -q "^checkfile: ✓" "$GEN/land/$a.check.out" || { echo "STOP $a: the check CLI fails on the copy"; exit 1; }
    "$0" write "$a" && "$0" prove "$a" ;;
esac

#!/bin/zsh
# 🛬️ Lands the repair of one person: land.sh check <id> applies ops/<id>.json to a copy ($GEN/try/<id>.json), applies it a second time (must change nothing), and runs the real gate of MR (every pose, clip instant and transition, twins) and measure.ts (silhouette guard, validateAsset, three-views sweep) on the backup and the copy. land.sh write <id> applies the ops to the catalog file (refused when the file changed on disk meanwhile) and repeats the fit (must report 0 changed). land.sh prove <id> runs the whole proof of MR (`prove.sh`, with the matrix of the asset) into $GEN/proof. land.sh all <id> does check, stops when the gate of the copy has a hard finding, the guard fails or validateAsset or the sweep differ from the backup, else writes and proves. Heavy steps hold the lock of q.sh.
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
step="$1"; a="$2"; ops="$W/ops/$a.json"; out="$GEN/try/$a.json"; mkdir -p "$GEN/try" "$GEN/land"
case "$step" in
  check)
    (cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$ops" --out "$out" | tail -1 && "$B" "$W/fit.ts" "$out" --ops "$ops" | tail -1)
    (cd "$T" && "$W/q.sh" timeout 2400 "$B" "$M/gate.ts" "$out" --twins) > "$GEN/land/$a.gate.log" 2>&1; echo "gate: $(tail -1 "$GEN/land/$a.gate.log"); $(grep -c ' HARD ' "$GEN/land/$a.gate.log") hard lines"
    grep -A3 " HARD " "$GEN/land/$a.gate.log" | cut -c1-260
    (cd "$S/📦️packages/🟦️typescript" && "$W/q.sh" timeout 1800 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$out") > "$GEN/land/$a.measure.log" 2>&1; grep -v "^ *$" "$GEN/land/$a.measure.log" | cut -c1-250 ;;
  write)
    (cd "$T" && "$B" "$W/fit.ts" "$a" --ops "$ops" --write | tail -2 && "$B" "$W/fit.ts" "$a" --ops "$ops" | tail -1) ;;
  prove)
    "$W/q.sh" env BATCH=MH-5 MATRIX=1 MATRIX_TIMEOUT=7200 "$M/prove.sh" "$a" 2>&1 | cut -c1-260
    python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-260 ;;
  all)
    "$0" check "$a" > "$GEN/land/$a.check.out" 2>&1; cat "$GEN/land/$a.check.out"
    grep -q "^gate: hard findings: 0;" "$GEN/land/$a.check.out" || { echo "STOP $a: the gate of the copy is not clean"; exit 1; }
    [[ "$(grep -c 'silhouette: OK' "$GEN/land/$a.check.out")" == 2 ]] || { echo "STOP $a: silhouette guard"; exit 1; }
    [[ "$(grep 'validate:' "$GEN/land/$a.check.out" | sort -u | wc -l | tr -d ' ')" == 1 && "$(grep 'three-views sweep:' "$GEN/land/$a.check.out" | sort -u | wc -l | tr -d ' ')" == 1 ]] || { echo "STOP $a: validate or sweep differ"; exit 1; }
    "$0" write "$a" && "$0" prove "$a" ;;
esac

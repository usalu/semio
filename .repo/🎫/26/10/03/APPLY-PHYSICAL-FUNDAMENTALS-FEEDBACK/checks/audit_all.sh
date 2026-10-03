#!/usr/bin/env bash
# Run collision_check.py for every PF beat, three at a time, one log per beat.
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../.." && pwd)"
OUT="${PF_MEDIA:-/tmp/pf_audit}"
mkdir -p "$HERE/audit"
cd "$ROOT"
run() {
  "$ROOT/.venv/bin/python" -u "$HERE/collision_check.py" "$OUT/$1" "$1" 2>&1 \
    | grep -v "SyntaxWarning\|re.match\|elif re" > "$HERE/audit/$1.log"
}
BEATS="${*:-Beat1_EnergieImAlltag Beat2_Leistung Beat3_Energieerhaltung Beat4_Waermepumpe Beat5_Strahlung Beat6_ThermischeMasse Beat7_SensibelLatent Beat8_Venturi Beat9_Kraft}"
n=0
for b in $BEATS; do
  run "$b" &
  n=$((n + 1))
  if [ $((n % 3)) -eq 0 ]; then wait; fi
done
wait
grep -h "===" "$HERE"/audit/*.log

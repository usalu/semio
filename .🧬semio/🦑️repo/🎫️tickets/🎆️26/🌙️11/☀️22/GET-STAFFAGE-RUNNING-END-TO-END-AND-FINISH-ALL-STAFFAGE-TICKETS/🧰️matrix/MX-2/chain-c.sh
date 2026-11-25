#!/bin/zsh
# 🔗️ The pocket trial and the tie scan in ONE queue grant (2500 MB when the sample holds `bamboo-clump`, a 20 MB document; ONLY="<ids>" measures those and appends), one process after the other: chain-c.sh <step>… with the steps `pocket-test` (the synthetic cases on the trial copy, then on the module of the catalog, where the repaired cases must fail), `pocket-measure` (the twenty sampled assets, 🗑️generated/matrix/MX-2/pocket-measure.txt) and `tie-scan` (D55 over the whole catalog, 🗑️generated/matrix/MX-2/tie-scan.txt).
self="${0:A}"; export BATCH=MX-2; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=2500 -- "$self" --inside "$@"; fi
shift
P="$S/📦️packages/🟦️typescript"
EDGE=(person-with-white-cane miscanthus-grass japanese-cherry ivy-on-trellis pampas-grass used-baby-grand-piano used-bicycle-shelter used-dog-waste-station used-viola bamboo-clump)
PLAIN=(steel-column glulam-beam three-seat-sofa city-bicycle laptop scroll-compressor kneeling-gardener tour-guide horse electric-fan-heater)
for step in "$@"; do
  echo "[DEBUG] step $step $(date +%H:%M:%S)"
  case "$step" in
    pocket-test)
      python3 "$W/pocket/patch.py" --trial
      (cd "$W/pocket" && timeout 600 "$B" test ./test.ts) > "$GEN/pocket-test-trial.txt" 2>&1; echo "trial copy: exit $? $(grep -E '^ *[0-9]+ (pass|fail)' "$GEN/pocket-test-trial.txt" | tr -s ' ' | tr '\n' ';')"
      (cd "$W/pocket" && MODULE=real timeout 600 "$B" test ./test.ts) > "$GEN/pocket-test-real.txt" 2>&1; echo "catalog module: exit $? $(grep -E '^ *[0-9]+ (pass|fail)' "$GEN/pocket-test-real.txt" | tr -s ' ' | tr '\n' ';')"; grep -E '^\(fail\)|✗' "$GEN/pocket-test-real.txt" | cut -c1-200 ;;
    pocket-measure)
      [[ -n "$ONLY" ]] && sample=(${=ONLY}) || { sample=($PLAIN $EDGE); : > "$GEN/pocket-measure.txt"; }
      for a in $sample; do (cd "$P" && /usr/bin/time -l timeout 1800 "$B" "$W/pocket/measure.ts" "$a") >> "$GEN/pocket-measure.txt" 2>&1; echo "[DEBUG] $a $(date +%H:%M:%S) $(grep -E 'maximum resident' "$GEN/pocket-measure.txt" | tail -1 | awk '{printf "%d MB", $1/1048576}')"; done
      grep -E 'CHANGES SIDES|^views' "$GEN/pocket-measure.txt" | cut -c1-260 ;;
    tie-scan) (cd "$T" && timeout 5400 "$B" "$W/tie-scan.ts") > "$GEN/tie-scan.txt" 2>&1; tail -1 "$GEN/tie-scan.txt"; grep -c 'VERDICT CHANGES' "$GEN/tie-scan.txt" ;;
  esac
done
echo "[DEBUG] chain done $(date +%H:%M:%S)"

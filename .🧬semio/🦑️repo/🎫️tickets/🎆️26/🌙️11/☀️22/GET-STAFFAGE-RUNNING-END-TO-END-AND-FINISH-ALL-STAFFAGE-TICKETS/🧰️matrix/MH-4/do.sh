#!/bin/zsh
# 🚂️ The whole chain of one or more assets, unattended, each step only when the one before is clean: do.sh <id>…   dry run on a copy with the gate over the worst subjects (try.sh), the gate over every subject on the copy, the silhouette guard of the copy, then the write into the catalog (apply.sh), the sheets before and after (look.sh, the default pose at every authored scale and POSES="a b" at 1:50 and 1:500) and the proof with the matrix (prove.sh). NOWRITE=1 stops before the write. Everything it prints goes to $GEN/do/<id>.out; an asset that stops is left untouched from the step that failed on.
source "${0:A:h}/env.sh"
mkdir -p "$GEN/devq"; touch "$GEN/devq/$$"
source "$W/lock.sh"
mkdir -p "$GEN/do"
for a in "$@"; do
  {
    echo "== $a $(date +%H:%M)"
    NOMEASURE=1 "$W/try.sh" "$a" | cut -c1-300
    if ! tail -1 "$GEN/try/$a.gate.log" | grep -q "^hard findings: 0$" || grep -q REFUSED "$GEN/try/$a.fit.log"; then echo "STOP $a: the worst subjects are not clean or an op was refused"; continue; fi
    "$W/try.sh" "$a" full | cut -c1-300
    if ! tail -1 "$GEN/try/$a.gate-full.log" | grep -q "^hard findings: 0$"; then echo "STOP $a: the full gate is not clean"; continue; fi
    if [[ "$(grep -c "silhouette: OK\|validate: 0 issues\|three-views sweep: 0 findings" <<< "$("$W/cap.sh" 2500 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$GEN/try/$a.json" 2>&1 | tee "$GEN/try/$a.measure.log")")" != 3 ]]; then echo "STOP $a: silhouette, validate or sweep"; grep -E "silhouette|validate|sweep|!" "$GEN/try/$a.measure.log" | cut -c1-260; continue; fi
    if ! grep -q ": 0 hard$" "$GEN/try/$a.scan.log"; then echo "STOP $a: the scan of the copy has a hard finding"; grep -v "sync-bone\|anchor-landmark" "$GEN/try/$a.scan.log" | cut -c1-260; continue; fi
    if [[ -n "$NOWRITE" ]]; then echo "READY $a: the copy is clean (NOWRITE)"; continue; fi
    "$W/apply.sh" "$a"
    scales=($(python3 -c "import json,sys; print(' '.join(k for k,r in json.load(open(sys.argv[1]))['asset']['representations'].items() if r.get('status')=='authored'))" "$GEN/backup/$a.json"))
    TAG=before "$W/look.sh" "$a" $scales >/dev/null; "$W/look.sh" "$a" $scales | sed 's/.*sheets\//sheet /'
    for p in ${=POSES}; do POSE=$p TAG=before "$W/look.sh" "$a" 50 500 >/dev/null; POSE=$p "$W/look.sh" "$a" 50 500 | sed 's/.*sheets\//sheet /'; done
    MATRIX=1 "$W/prove.sh" "$a"
    echo "== $a done $(date +%H:%M)"
  } 2>&1 | tee "$GEN/do/$a.out"
done

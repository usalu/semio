#!/bin/zsh
# 🏁️ Writes and proves assets whose dry run is clean (it waits while a dry run of do.sh is queued, so the long matrix never holds up the short runs; call it once per asset): finish.sh <id>…   checks that the copy $GEN/try/<id>.json is what the ops give today, that its full gate log is newer than the copy and clean (ALLOW_OPEN=1: an asset whose remaining findings are stated in the report is written in its best proven state) that silhouette, validateAsset and the three-views sweep of the copy are clean and that the authors' scan of the copy has no paper, print-profile, derived-view, dimension or cue finding, then writes the catalog file (apply.sh), renders the sheets before and after (the default pose at every authored scale, POSES="a b" at 1:50 and 1:500) and runs the proof with the matrix (prove.sh). Output in $GEN/do/<id>.finish.out; a proven document is kept as $GEN/done/<id>.json to notice a later overwrite.
source "${0:A:h}/env.sh"
until [[ -z "$(ls -A "$GEN/devq" 2>/dev/null)" ]]; do sleep 10; done
source "$W/lock.sh"
mkdir -p "$GEN/do" "$GEN/done"
for a in "$@"; do
  {
    echo "== finish $a $(date +%H:%M)"
    "$W/cap.sh" 800 300 "$T" "$B" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$W/ops/$a.json" --out "$GEN/try/$a.check.json" | tail -1
    if ! cmp -s "$GEN/try/$a.check.json" "$GEN/try/$a.json" || [[ "$GEN/try/$a.gate-full.log" -ot "$GEN/try/$a.json" ]] || { [[ -z "$ALLOW_OPEN" ]] && ! tail -1 "$GEN/try/$a.gate-full.log" | grep -q "^hard findings: 0$" }; then echo "STOP $a: the dry run is not the proven one"; rm -f "$GEN/try/$a.check.json"; continue; fi
    rm -f "$GEN/try/$a.check.json"
    if [[ "$("$W/cap.sh" 1000 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$GEN/try/$a.json" 2>&1 | tee "$GEN/try/$a.measure.log" | grep -c "silhouette: OK\|validate: 0 issues\|three-views sweep: 0 findings")" != 3 ]]; then echo "STOP $a: silhouette, validate or sweep of the dry run"; continue; fi
    "$W/cap.sh" 800 900 "$T" "$B" "$W/scan-file.ts" "$GEN/try/$a.json" > "$GEN/try/$a.scan.log" 2>&1
    if grep -q "unavailable-\|paper-\|derived-view\|dimension-\|-empty\|-bounds\|analysis-cue\|evaluate" "$GEN/try/$a.scan.log" || ! grep -q " hard$" "$GEN/try/$a.scan.log"; then echo "STOP $a: the scan of the dry run has a finding the repair must not leave or bring"; grep -v "sync-bone\|anchor-landmark\|   sync-" "$GEN/try/$a.scan.log" | cut -c1-240; continue; fi
    echo "open after the dry run: $(tail -1 "$GEN/try/$a.gate-full.log")"
    "$W/apply.sh" "$a"
    scales=($(python3 -c "import json,sys; print(' '.join(k for k,r in json.load(open(sys.argv[1]))['asset']['representations'].items() if r.get('status')=='authored'))" "$GEN/backup/$a.json"))
    TAG=before "$W/look.sh" "$a" $scales >/dev/null; "$W/look.sh" "$a" $scales | sed 's/.*sheets\//sheet /'
    for p in ${=POSES}; do POSE=$p TAG=before "$W/look.sh" "$a" 50 500 >/dev/null; POSE=$p "$W/look.sh" "$a" 50 500 | sed 's/.*sheets\//sheet /'; done
    MATRIX=1 "$W/prove.sh" "$a"
    cp "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/done/$a.json"
    echo "== $a done $(date +%H:%M)"
  } 2>&1 | tee "$GEN/do/$a.finish.out"
done

#!/bin/zsh
# 🏁️ Writes and proves assets whose dry run is clean: finish.sh <id>…   checks that the copy $GEN/try/<id>.json is what the ops give today and that its full gate log is clean and newer than the copy, then writes the catalog file (apply.sh), renders the sheets before and after (the default pose at every authored scale, POSES="a b" at 1:50 and 1:500) and runs the proof with the matrix (prove.sh). Output in $GEN/do/<id>.finish.out; a proven document is kept as $GEN/done/<id>.json to notice a later overwrite.
source "${0:A:h}/env.sh"
source "$W/lock.sh"
mkdir -p "$GEN/do" "$GEN/done"
for a in "$@"; do
  {
    echo "== finish $a $(date +%H:%M)"
    (cd "$T" && "$B" "$W/fit.ts" "$GEN/backup/$a.json" --ops "$W/ops/$a.json" --out "$GEN/try/$a.check.json") | tail -1
    if ! cmp -s "$GEN/try/$a.check.json" "$GEN/try/$a.json" || [[ "$GEN/try/$a.gate-full.log" -ot "$GEN/try/$a.json" ]] || ! tail -1 "$GEN/try/$a.gate-full.log" | grep -q "^hard findings: 0$"; then echo "STOP $a: the dry run is not the proven one"; rm -f "$GEN/try/$a.check.json"; continue; fi
    rm -f "$GEN/try/$a.check.json"
    "$W/apply.sh" "$a"
    scales=($(python3 -c "import json,sys; print(' '.join(k for k,r in json.load(open(sys.argv[1]))['asset']['representations'].items() if r.get('status')=='authored'))" "$GEN/backup/$a.json"))
    TAG=before BATCH=MH-4 "$M/look.sh" "$a" $scales >/dev/null; BATCH=MH-4 "$M/look.sh" "$a" $scales | sed 's/.*sheets\//sheet /'
    for p in ${=POSES}; do POSE=$p TAG=before BATCH=MH-4 "$M/look.sh" "$a" 50 500 >/dev/null; POSE=$p BATCH=MH-4 "$M/look.sh" "$a" 50 500 | sed 's/.*sheets\//sheet /'; done
    MATRIX=1 "$W/prove.sh" "$a"
    cp "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/done/$a.json"
    echo "== $a done $(date +%H:%M)"
  } 2>&1 | tee "$GEN/do/$a.finish.out"
done

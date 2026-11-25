#!/bin/zsh
# 🏁️ The closing proofs of the takeover, split over the two requests a fixer may hold in the memory queue (run each part through g.sh): final.sh write <id>… | light <id>… | matrix <id>… | suites <id>…
#   write   (grant 1200) applies the ops of the asset to the backup, measures the copy (gate over every subject, silhouette, validate, three-views sweep, scan) and, when nothing hard is left, writes the catalog file and renders the sheets of the default pose
#   light   (grant 1200) gate, authors' scan, silhouette measure and check CLI of the catalog file, then the matrix of the asset unless the other part has claimed it
#   matrix  (grant 1200) the matrix of every asset named that no part has claimed yet
#   suites  (grant 2500) the six test suites with the name filter (vts.sh)
# Lines go to $GEN/proof/<id>.<part>.out, logs to $GEN/proof; a matrix is claimed with the folder $GEN/claim/<id>.
source "${0:A:h}/env.sh"
export MH4_LOCKED=1
part="$1"; shift; d="$GEN/proof"; mkdir -p "$d" "$GEN/claim" "$GEN/done"
matrix() {
  local a="$1"
  mkdir "$GEN/claim/$a" 2>/dev/null || return 0
  rm -rf "$GEN/matrix-$a"
  { "$W/cap.sh" 800 10800 "$T" "$B" "$T/📊️matrix.ts" --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a" > "$d/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$d/$a-matrix.log") ($(date +%H:%M))"; python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-300; } > "$d/$a.matrix.out" 2>&1
}
for a in "$@"; do
  case "$part" in
    write)
      {
        echo "== write $a $(date +%H:%M)"
        NOMEASURE=1 NOSCAN=1 "$W/try.sh" "$a" full | cut -c1-260
        "$W/cap.sh" 1000 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$GEN/try/$a.json" > "$GEN/try/$a.measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$GEN/try/$a.measure.log"
        "$W/cap.sh" 800 900 "$T" "$B" "$W/scan-file.ts" "$GEN/try/$a.json" > "$GEN/try/$a.scan.log" 2>&1; tail -1 "$GEN/try/$a.scan.log" | cut -c1-200
        if ! tail -1 "$GEN/try/$a.gate-full.log" | grep -q "^hard findings: 0$" || [[ "$(grep -c "silhouette: OK\|validate: 0 issues\|three-views sweep: 0 findings" "$GEN/try/$a.measure.log")" != 3 ]] || ! grep -q ": 0 hard$" "$GEN/try/$a.scan.log"; then echo "STOP $a: the copy is not clean, the catalog file stays"; continue; fi
        "$W/apply.sh" "$a"
        scales=($(python3 -c "import json,sys; print(' '.join(k for k,r in json.load(open(sys.argv[1]))['asset']['representations'].items() if r.get('status')=='authored'))" "$GEN/backup/$a.json"))
        "$W/look.sh" "$a" $scales | sed 's/.*sheets\//sheet /'
        echo "== $a written $(date +%H:%M)"
      } > "$d/$a.write.out" 2>&1 ;;
    light)
      {
        "$W/cap.sh" 800 3600 "$T" "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$d/$a-gate.log") ($(date +%H:%M))"
        "$W/cap.sh" 800 1800 "$T" "$B" "$T/🔎️scan.ts" --language en "$a" > "$d/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-200)"
        "$W/cap.sh" 1000 1800 "$S/📦️packages/🟦️typescript" "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$d/$a-measure.log"
        "$W/cap.sh" 1000 1800 "$S" "$B" "$S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a" > "$d/$a-check.log" 2>&1; echo "check: $(grep -m1 -E '^(✓|✗)' "$d/$a-check.log" | cut -c1-160)"
        md5 -q "$S/🗂️catalog/👤️humans/$a/🔣️.json"
      } > "$d/$a.light.out" 2>&1
      cp "$S/🗂️catalog/👤️humans/$a/🔣️.json" "$GEN/done/$a.json"
      matrix "$a" ;;
    matrix) matrix "$a" ;;
    suites) "$W/vts.sh" "$a" > "$d/$a.suites.out" 2>&1 ;;
  esac
  echo "$(date +%H:%M) $part $a" >> "$GEN/final.log"
done
echo "$(date +%H:%M) $part END" >> "$GEN/final.log"

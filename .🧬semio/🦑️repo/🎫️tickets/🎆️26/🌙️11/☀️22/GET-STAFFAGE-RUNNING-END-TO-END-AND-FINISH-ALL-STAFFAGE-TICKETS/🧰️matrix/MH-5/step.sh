#!/bin/zsh
# (Reservations follow the queue rule of 2026-11-25 09:20: 800 MB scans, gates, fits and measures, 1000 MB check CLI, 1200 MB matrix, 2500 MB the six tests.)
# 🪜️ One step of the landing of one person as its OWN small grant of the machine-wide queue (or, queued as a job of worker.sh, inside the one grant of the worker; a grant of 5000 MB for a whole chain waited 76 minutes without being admitted on 2026-11-25 because the session shares its budget with other fixers): step.sh <step> <id>…, the log of each in $GEN/land/<id>.<step>.log, its result lines on stdout. Steps, in the order of a landing:
#   fit      1000 MB  ops/<id>.json and ops/<id>.lofts.json applied (apply.sh) to the backup into $GEN/try/<id>.json, applied a second time (must change nothing), model gate of the copy
#   measure  2500 MB  measure.ts (silhouette guard, validateAsset, three-views sweep) of the backup and of the copy
#   gate     2500 MB  the real gate of MR over every pose, clip instant and transition, with twins, of the copy (FILE=catalog: of the catalog file)
#   write    1000 MB  land.sh write: the copy into the catalog (refused unless the catalog file is the backup or what this fixer landed last), ops applied once more (must report 0 changed)
#   back     none     the catalog file put back to the backup (refused unless it is what this fixer landed last)
#   scan     1000 MB  T/🔎️scan.ts of the catalog file → $GEN/proof/<id>-scan.log
#   pgate    2500 MB  the real gate of the catalog file → $GEN/proof/<id>-gate.log
#   pmeasure 2500 MB  measure.ts of the catalog file → $GEN/proof/<id>-measure.log
#   tests    2500 MB  the six real tests with the name filter of the asset, one after the other (vt.sh of MR) → $GEN/vt
#   check    5000 MB  the check CLI of the asset → $GEN/proof/<id>-check.log   (FILE=copy: of the copy, `--file`)
#   matrix   7000 MB  T/📊️matrix.ts --assets <id> → $GEN/matrix-<id>, $GEN/proof/<id>-matrix.log
export BATCH=MH-5
source "${0:A:h}/../MR/env.sh"
step="$1"; shift; mkdir -p "$GEN/land" "$GEN/try" "$GEN/proof" "$GEN/vt" "$GEN/landed"
grant() { local mem="$1"; shift; MEM=$mem "$W/q.sh" env MH5_GRANT=1 BATCH=MH-5 "$@"; }
for a in "$@"; do
  log="$GEN/land/$a.$step.log"; ops="$W/ops/$a.json"; out="$GEN/try/$a.json"; file="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"
  case "$step" in
    fit) (cd "$T" && grant 800 /bin/zsh -c '"$2/apply.sh" "$3" "$4" "$5" | grep -E "REFUSED|OVER|changed" | cut -c1-300; "$2/apply.sh" "$5" "$4" | grep -E "REFUSED|something|views changed" | cut -c1-200; "$1" "$2/mgate.ts" "$5" | grep -E "HARD|subjects" | cut -c1-230' fit "$B" "$W" "$GEN/backup/$a.json" "$ops" "$out") > "$log" 2>&1; grep -v '^\[capacity\]' "$log" ;;
    measure) (cd "$S/📦️packages/🟦️typescript" && grant 800 timeout 1800 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$out") > "$log" 2>&1; grep -E " 1:|silhouette:|validate:|sweep:" "$log" | cut -c1-250 ;;
    gate) src="$out"; [[ "$FILE" == catalog ]] && src="$a"; (cd "$T" && grant 800 timeout 2400 "$B" "$M/gate.ts" "$src" --twins) > "$log" 2>&1; echo "gate $a: $(tail -1 "$log")"; grep -A${DETAIL:-3} " HARD " "$log" | cut -c1-250 ;;
    write) grant 800 "$W/land.sh" write "$a" > "$log" 2>&1; grep -v '^\[capacity\]' "$log" ;;
    back) cmp -s "$file" "$GEN/landed/$a.json" && cp "$GEN/backup/$a.json" "$file" && rm "$GEN/landed/$a.json" && echo "put back: $a" || echo "REFUSED $a: the catalog file is not what this fixer landed last" ;;
    scan) (cd "$T" && grant 800 timeout 1800 "$B" 🔎️scan.ts --language en "$a") > "$GEN/proof/$a-scan.log" 2>&1; echo "scan exit $? $(tail -1 "$GEN/proof/$a-scan.log" | cut -c1-200)" ;;
    pgate) (cd "$T" && grant 800 timeout 2400 "$B" "$M/gate.ts" "$a" --twins) > "$GEN/proof/$a-gate.log" 2>&1; echo "gate exit $? $(tail -1 "$GEN/proof/$a-gate.log")"; grep -A${DETAIL:-3} " HARD " "$GEN/proof/$a-gate.log" | cut -c1-250 ;;
    pmeasure) (cd "$S/📦️packages/🟦️typescript" && grant 800 timeout 900 "$B" "$M/measure.ts" "$a") > "$GEN/proof/$a-measure.log" 2>&1; grep -E "silhouette:|validate:|sweep:" "$GEN/proof/$a-measure.log" ;;
    tests) grant 2500 /bin/zsh -c '"$1/vt.sh" "$2-silhouette" "🧊️catalog-silhouette" "$2"; "$1/vt.sh" "$2-three-views" "🧊️three-views" "$2"; "$1/vt.sh" "$2-physics" "🌡️catalog-physics" "$2"; SUFFIX=":" "$1/vt.sh" "$2-breathing" "🫁️breathing-catalog" "$2"; SUFFIX="" "$1/vt.sh" "$2-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$2"; SUFFIX=":" "$1/vt.sh" "$2-power" "⚡️asset-power" "$2"' tests "$M" "$a" > "$log" 2>&1; grep -v '^\[capacity\]' "$log" | cut -c1-200 ;;
    check) if [[ "$FILE" == copy ]]; then (cd "$S" && grant 1000 timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --file "$out") > "$log" 2>&1; echo "checkfile: $(grep -E '^(✓|✗)|suitability-' "$log" | tr '\n' ' ' | cut -c1-240)"; else (cd "$S" && grant 1000 timeout 900 "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$a") > "$GEN/proof/$a-check.log" 2>&1; echo "check: $(grep -E '^(✓|✗)|suitability-' "$GEN/proof/$a-check.log" | tr '\n' ' ' | cut -c1-240)"; fi ;;
    matrix) rm -rf "$GEN/matrix-$a"; (cd "$T" && grant 800 timeout 9000 "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$a" --jobs 1 --fresh --assets "$a") > "$GEN/proof/$a-matrix.log" 2>&1; echo "matrix exit $? $(tail -1 "$GEN/proof/$a-matrix.log" | cut -c1-200)"; python3 "$M/cells.py" "$GEN/matrix-$a" 2>&1 | cut -c1-260 ;;
  esac
done

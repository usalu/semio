#!/bin/zsh
# 🧾️ The proof of assets as written in the catalog, in three grants of the memory queue per call instead of one per step: the 2500 MB class (gate, scan, measures, the six real tests with a name filter) of all given ids one after the other under ONE grant, then the check CLI of each under a 5000 MB grant, then with MATRIX=1 the matrix of each under a 7000 MB grant (MATRIX_MB states another reservation once the peak is measured: the largest process of the run is written by /usr/bin/time into $GEN/proof/<id>-matrix.time). Logs in $GEN/proof and $GEN/vt, one summary line per step with the md5 of the file, appended to $GEN/proof/summary.log: proof.sh <id>…   (STEPS="gate scan measure tests check" selects; inside a grant the script calls itself with INSIDE=<class>)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
d="$GEN/proof"; mkdir -p "$d" "$GEN/vt"; steps="${STEPS:-gate scan measure tests check}"; want() { [[ " $steps " == *" $1 "* ]] }
say() { echo "$(date +%d\ %H:%M) $*" | tee -a "$d/summary.log" }
sum() { md5 -q "$S/🗂️catalog/👤️humans/$1/🔣️.json" | cut -c1-8 }
vt() { local log="$GEN/vt/$1.log" suite="$2"; shift 2; (cd "$S/📦️packages/🟦️typescript" && "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$1" > "$log" 2>&1); echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" }
case "$INSIDE" in
  light)
    for a in "$@"; do
      want gate && { (cd "$T" && "$B" "$M/gate.ts" "$a" --twins > "$d/$a-gate.log" 2>&1); say "$a $(sum $a) gate exit $? $(tail -1 "$d/$a-gate.log")" }
      want scan && { (cd "$T" && "$B" 🔎️scan.ts --language en "$a" > "$d/$a-scan.log" 2>&1); say "$a $(sum $a) scan exit $? $(tail -1 "$d/$a-scan.log" | cut -c1-160)" }
      want measure && { (cd "$S/📦️packages/🟦️typescript" && "$B" "$M/measure.ts" "$a" > "$d/$a-measure.log" 2>&1); say "$a $(sum $a) $(grep -E 'silhouette:|validate:|sweep:' "$d/$a-measure.log" | cut -d' ' -f2- | tr '\n' ';')" }
      want tests && {
        say "$a $(sum $a) $(vt "$a-silhouette" "🧊️catalog-silhouette" "($a)( |$)")"
        say "$a $(sum $a) $(vt "$a-three-views" "🧊️three-views" "($a)( |$)")"
        say "$a $(sum $a) $(vt "$a-physics" "🌡️catalog-physics" "($a)( |$)")"
        say "$a $(sum $a) $(vt "$a-breathing" "🫁️breathing-catalog" "($a):")"
        say "$a $(sum $a) $(vt "$a-validation" "✅️asset-validation" "(accepts the sample figure|validates the manifest|$a)")"
        say "$a $(sum $a) $(vt "$a-power" "⚡️asset-power" "($a):")"
      }
    done ;;
  check) (cd "$S" && "$B" "🔨️modules/🗂️catalog/🩺️check/🟦️.ts" --asset "$1" > "$d/$1-check.log" 2>&1); say "$1 $(sum $1) check exit $?: $(grep -m1 -E '^(✓|✗)' "$d/$1-check.log" | cut -c1-160)" ;;
  matrix) rm -rf "$GEN/matrix-$1"; (cd "$T" && /usr/bin/time -l -o "$d/$1-matrix.time" "$B" 📊️matrix.ts --language en --out "$GEN/matrix-$1" --jobs 1 --fresh --assets "$1" > "$d/$1-matrix.log" 2>&1); say "$1 $(sum $1) matrix exit $? $(tail -1 "$d/$1-matrix.log" | cut -c1-200) | peak $(awk '/maximum resident/{printf "%d MB", $1/1048576}' "$d/$1-matrix.time") | $(python3 "$M/cells.py" "$GEN/matrix-$1" 2>&1 | tr '\n' ' ' | cut -c1-300)" ;;
  *)
    [[ "$steps" == *gate* || "$steps" == *scan* || "$steps" == *measure* || "$steps" == *tests* ]] && INSIDE=light "$W/q.sh" 2500 ${LIGHT_TIMEOUT:-14400} "$T" "${0:A}" "$@"
    want check && for a in "$@"; do INSIDE=check "$W/q.sh" 1000 5400 "$T" "${0:A}" "$a"; done
    [[ -n "$MATRIX" ]] && for a in "$@"; do INSIDE=matrix "$W/q.sh" ${MATRIX_MB:-800} ${MATRIX_TIMEOUT:-10800} "$T" "${0:A}" "$a"; done ;;
esac

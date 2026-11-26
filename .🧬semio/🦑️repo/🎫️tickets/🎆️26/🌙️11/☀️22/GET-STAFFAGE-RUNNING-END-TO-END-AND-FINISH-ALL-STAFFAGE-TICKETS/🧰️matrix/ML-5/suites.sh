# 🧫️ The six test suites of assets with the name filter, one after the other (sourced by q.sh, grant 2500 MB): suites.sh <tag> <id>…   one line per suite in $GEN/proof/<id>.suites.out, logs in $GEN/vt.
shift
vt() {
  local log="$GEN/vt/$1.log" suite="$2"; shift 2
  local pat="(${(j:|:)@})${SUFFIX-( |$)}"
  cap 1800 "$P" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat" > "$log" 2>&1
  echo "$suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')"
}
for a in "$@"; do
  {
    echo "== suites $a $(date +%H:%M) md5 $(md5 -q "$C/$a/🔣️.json")"
    vt "$a-silhouette" "🧊️catalog-silhouette" "$a"
    vt "$a-three-views" "🧊️three-views" "$a"
    vt "$a-physics" "🌡️catalog-physics" "$a"
    SUFFIX=":" vt "$a-breathing" "🫁️breathing-catalog" "$a"
    SUFFIX="" vt "$a-validation" "✅️asset-validation" "accepts the sample figure" "validates the manifest" "$a"
    SUFFIX=":" vt "$a-power" "⚡️asset-power" "$a"
    echo "== end $(date +%H:%M)"
  } > "$GEN/proof/$a.suites.out" 2>&1
  cat "$GEN/proof/$a.suites.out"
done

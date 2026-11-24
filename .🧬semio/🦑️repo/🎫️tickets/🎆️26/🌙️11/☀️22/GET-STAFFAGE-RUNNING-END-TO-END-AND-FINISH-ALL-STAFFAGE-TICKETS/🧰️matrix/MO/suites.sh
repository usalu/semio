#!/bin/zsh
# 🧪️ The real tests of the batch assets, one process per suite with a name filter over the given ids (all 17 when none is given), verbose so that every test line can be compared: suites.sh <tag> [<id>…] writes $GEN/vt/<tag>-<suite>.log and prints the totals; `python3 results.py <tag-a> <tag-b>` compares two runs.
source "${0:A:h}/env.sh"
tag="$1"; shift
ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($(cut -f1 "$U/🧰️matrix/batches/MO.tsv"))
mkdir -p "$GEN/vt"
run() {
  local name="$1" suite="$2" pat="$3"
  (cd "$P" && timeout ${VT_TIMEOUT:-3000} "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$GEN/vt/$tag-$name.log" 2>&1
  echo "$name exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/$tag-$name.log" | tr -s ' ' | tr '\n' ';')"
}
any="(${(j:|:)ids})"
run silhouette "🧊️catalog-silhouette" "$any( |\$)"
run three-views "🧊️three-views" "$any( |\$)"
run physics "🌡️catalog-physics" "$any( |\$)"
run breathing "🫁️breathing-catalog" "$any:"
run validation "✅️asset-validation" "(accepts the sample figure|validates the manifest|$any)"
run power "⚡️asset-power" "$any:"

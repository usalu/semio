#!/bin/zsh
# 🧪️ The real tests of the batch assets in one queue grant, one vitest process per suite with a name filter over the given ids (all 20 when none is given), verbose so that every test line can be compared: suites.sh <tag> [<id>…] writes $GEN/vt/<tag>-<suite>.log and prints the totals; `python3 results.py <tag-a> <tag-b>` compares two runs. The six suites of the brief (catalog-silhouette, three-views, catalog-physics, breathing-catalog, asset-validation, asset-power) and the ten other suites that name an asset of the batch (model-bounds, section-views, thermal-scenarios, thermal-multiview, structure-projection, multiview-sheet, model-skeleton, thermal-legend, plan-projection, model-roundness). SUITES=<name,…> limits the run to those short names.   (run it through q.sh with 2500 MB)
source "${0:h}/env.sh"
tag="$1"; shift
ids=("$@"); [[ ${#ids} -eq 0 ]] && ids=($IDS)
mkdir -p "$GEN/vt"
run() {
  local name="$1" suite="$2" pat="$3"
  [[ -n "$SUITES" && ",$SUITES," != *",$name,"* ]] && return
  (cd "$P" && timeout ${VT_TIMEOUT:-3000} "$B" x vitest run --reporter verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$GEN/vt/$tag-$name.log" 2>&1
  echo "$name exit $? $(grep -E '^ *(Test Files|Tests) ' "$GEN/vt/$tag-$name.log" | tr -s ' ' | tr '\n' ';')"
}
any="(${(j:|:)ids})"
wide="($any|city car|forklift|generator|garage|sliding|bicycle|bike|scooter|vintage)"
run silhouette "🧊️catalog-silhouette" "$any( |\$)"
run three-views "🧊️three-views" "$any( |\$)"
run physics "🌡️catalog-physics" "$any( |\$)"
run breathing "🫁️breathing-catalog" "$any:"
run validation "✅️asset-validation" "(accepts the sample figure|validates the manifest|$any)"
run power "⚡️asset-power" "$any"
run model-bounds "🧊️model-bounds" "$wide"
run section-views "✂️section-views" "$wide"
run thermal-scenarios "🧊️thermal-scenarios" "$wide"
run thermal-multiview "🌐️thermal-multiview" "$wide"
run structure-projection "🏗️structure-projection" "$wide"
run multiview-sheet "📑️multiview-sheet" "$wide"
run model-skeleton "🧊️model-skeleton" "$wide"
run thermal-legend "🎨️thermal-legend" "$wide"
run plan-projection "🧍️plan-projection" "$wide"
run model-roundness "🧊️model-roundness" "$wide"

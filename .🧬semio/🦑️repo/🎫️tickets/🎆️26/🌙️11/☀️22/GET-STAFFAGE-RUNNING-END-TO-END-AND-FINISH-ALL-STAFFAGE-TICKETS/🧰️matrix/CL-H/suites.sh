#!/bin/zsh
# 🧪️ The real test files for assets of lane CL-H, inside the grant of its caller: `🧊️catalog-silhouette` with each asset as name filter, then `🧊️model-skeleton` once with the two pinned cases (long bones, passengers) as filter; one line each into `$GEN/proof/suites.txt`. suites.sh <id>…
source "${0:A:h}/env.sh"
o="$GEN/proof/suites.txt"; mkdir -p "$GEN/vt" "$GEN/proof"
vt() { local log="$GEN/vt/$1.log" suite="$2" pat="$3"
  (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$suite/🟦️.ts" -t "$pat") > "$log" 2>&1
  echo "$(date +%H:%M) $1 $suite exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$o"; }
vt skeleton "🧊️model-skeleton" "(two forearm bones|passenger)"
for a in "$@"; do vt "silhouette-$a" "🧊️catalog-silhouette" "($a)( |$)"; done

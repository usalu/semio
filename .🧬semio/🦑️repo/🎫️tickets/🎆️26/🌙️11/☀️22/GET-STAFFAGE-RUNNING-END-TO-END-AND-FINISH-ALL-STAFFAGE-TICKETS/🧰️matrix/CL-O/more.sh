#!/bin/zsh
# 🧷️ The other real test files that name an asset of lane CL-O, inside the grant of its caller, on the catalog as written: `🔤️caption-fit` (gas-hob), `🏗️structure-projection` and `💨️transport-assets` (excavator); one line each into `$GEN/proof/more.txt`.
source "${0:A:h}/env.sh"
o="$GEN/proof/more.txt"; mkdir -p "$GEN/vt"; echo "named suites of $(date +%F\ %H:%M)" > "$o"
vt() { local log="$GEN/vt/more-$1.log"; local -a filter; [[ -n "$3" ]] && filter=(-t "$3")
  (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$2/🟦️.ts" "${filter[@]}") > "$log" 2>&1
  echo "$2 exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$o"; }
vt caption "🔤️caption-fit" ""
vt projection "🏗️structure-projection" ""
vt transport "💨️transport-assets" ""
echo "done $(date +%H:%M)" >> "$o"

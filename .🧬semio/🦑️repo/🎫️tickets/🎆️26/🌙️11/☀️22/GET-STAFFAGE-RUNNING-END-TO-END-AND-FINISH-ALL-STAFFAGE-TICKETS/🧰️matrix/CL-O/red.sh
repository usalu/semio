#!/bin/zsh
# 🔴️ The pinned cases of lane CL-O on the catalog as it is, inside the grant of its caller: the four detail cases of `🧊️model-skeleton` (bicycle nipples, scissor bores, idler pin) and `📐️plan-dimension`, one line each into `$GEN/proof/red-<tag>.txt`. red.sh <tag>
source "${0:A:h}/env.sh"
o="$GEN/proof/red-$1.txt"; mkdir -p "$GEN/vt" "$GEN/proof"; echo "red cases of $(date +%F\ %H:%M)" > "$o"
vt() { local log="$GEN/vt/red-$1-$2.log"; local -a filter; [[ -n "$4" ]] && filter=(-t "$4")
  (cd "$S/📦️packages/🟦️typescript" && timeout 2400 "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$3/🟦️.ts" "${filter[@]}") > "$log" 2>&1
  echo "$3 exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';')" >> "$o"; }
vt "$1" skeleton "🧊️model-skeleton" "(laces both bicycle wheels|bores every scissor arm|passes each idler pin)"
vt "$1" plan "📐️plan-dimension" ""
echo "done $(date +%H:%M)" >> "$o"

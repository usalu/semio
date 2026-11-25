#!/bin/zsh
# 🧪️ The six real suites of the proof list with one name filter over several assets, one after the other, each in its own grant of the size MM measured for it (takeover brief, "Vitest with a name filter", 2026-11-25 20:20: physics 700, silhouette 1600, power and validation 1900, breathing 1950, three-views 2500 MB), smallest first, with the peak of the worker from `/usr/bin/time -l`: vt-chain.sh <id>…   Every result line goes to $GEN/proof/vt-chain.out, the whole output to $GEN/vt/chain-<suite>.log
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN/vt" "$GEN/proof"
ids="${(j:|:)@}"; out="$GEN/proof/vt-chain.out"
echo "## $(date '+%d %H:%M') $ids" >> "$out"
run() {
  log="$GEN/vt/chain-$2.log"
  (cd "$R" && SEMIO_LANGUAGE=en repo capacity run --memory="$1" -- zsh -c 'cd "$1" && exec timeout 3000 /usr/bin/time -l "$2" x vitest run --reporter=verbose --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/$3/🟦️.ts" -t "$4"' - "$S/📦️packages/🟦️typescript" "$B" "$2" "$3" > "$log" 2>&1)
  echo "$2 exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';') peak $(awk '/maximum resident set size/{printf "%d MB", $1/1048576}' "$log") ($1 MB reserved, $(date '+%H:%M'))" >> "$out"
  grep -E '^ *(FAIL|×|✗) ' "$log" | sort -u | head -20 >> "$out"
}
run 700 "🌡️catalog-physics" "($ids)( |$)"
run 1600 "🧊️catalog-silhouette" "($ids)( |$)"
run 1900 "⚡️asset-power" "($ids):"
run 1900 "✅️asset-validation" "(accepts the sample figure|validates the manifest|$ids)"
run 1950 "🫁️breathing-catalog" "($ids):"
run 2500 "🧊️three-views" "($ids)( |$)"
echo "chain ended $(date '+%d %H:%M')" >> "$out"

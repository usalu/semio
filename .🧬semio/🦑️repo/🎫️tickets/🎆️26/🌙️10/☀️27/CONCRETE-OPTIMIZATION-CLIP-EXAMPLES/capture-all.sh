#!/usr/bin/env bash
# 📸 Capture every optimization scene in both languages in parallel; logs go to 🗑️generated/frames/<Scene>-<language>.log.
cd "$(dirname "$0")" || exit 1
P="$(git rev-parse --show-toplevel)/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
mkdir -p "🗑️generated/frames"
SCENES=("📉️extrema Beat1_LocalGlobalExtrema" "🎲️sampling Beat2_GridRandomSampling" "🧪️experiments Beat3_FactorialExperiments"
        "⚖️index Beat4_NormalizedIndex" "🏆️pareto Beat5_ParetoFront" "🧬️algorithms Beat6_EvolutionarySearch")
ONLY="${1:-}"
for entry in "${SCENES[@]}"; do
  set -- $entry
  [ -n "$ONLY" ] && [ "$ONLY" != "$2" ] && continue
  for language in en de; do
    "$P" capture.py "$1" "$2" "$language" > "🗑️generated/frames/$2-$language.log" 2>&1 &
  done
done
wait
grep -a -h -o "\[DEBUG\] captured.*\|[A-Za-z]*Error.*" 🗑️generated/frames/*.log

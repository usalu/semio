#!/usr/bin/env bash
# 📸 Capture every BIM scene in both languages in parallel; logs go to 🗑️generated/frames/<Scene>-<language>.log.
cd "$(dirname "$0")" || exit 1
P="$(git rev-parse --show-toplevel)/.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
mkdir -p "🗑️generated/frames"
SCENES=("📊️information Beat1_DataToInformation" "🧬️objects Beat2_ObjectOrientation" "👁️representations Beat3_SameObject"
        "🧩️models Beat4_PartialModels" "🔌️exchange Beat5_Interfaces" "🔗️consistency Beat6_Consistency")
ONLY="${1:-}"
LANGUAGES="${2:-en de}"
for entry in "${SCENES[@]}"; do
  set -- $entry
  [ -n "$ONLY" ] && [ "$ONLY" != "all" ] && [ "$ONLY" != "$2" ] && continue
  for language in $LANGUAGES; do
    "$P" capture.py "$1" "$2" "$language" > "🗑️generated/frames/$2-$language.log" 2>&1 &
  done
done
wait
grep -a -h -o "\[DEBUG\] captured.*\|[A-Za-z]*Error.*" 🗑️generated/frames/*.log

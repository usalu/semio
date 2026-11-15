#!/usr/bin/env bash
cd /c/git/semio
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$PWD/$TK/🗑️generated/vent-a-out/direct"
E="🎓️teaching/🏛️architecture/⚡️energy"
export SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_OUTPUT_ROOT="$PWD/$TK/🗑️generated/vent-a-out"
mkdir -p "$OUT"
run() {
  folder="$1"; shift
  for scene in "$@"; do
    echo "== $scene" >> "$OUT/log.txt"
    uv run --project "$E/🏗️authoring/📦️packages/🐍️python" --python 3.12 python -m manim render --resolution 854,480 --fps 15 \
      --media_dir "$OUT/media" --disable_caching "$E/🌬️ventilation/🎬️clip/$folder/🐍️.py" "$scene" >> "$OUT/log-$scene.txt" 2>&1
    echo "done $scene rc=$?" >> "$OUT/log.txt"
  done
}
run "❓️need" Beat1_Lueftungsbedarf Beat2_Lueftungsweg Beat3_Fahrplan
run "📦️volume" Beat1_Innenvolumen Beat2_Luftwechselrate Beat3_Volumenstrom
run "🌡️heat-flow" Beat1_Luftwaerme Beat2_SensibleWaerme Beat3_LatenteLast Beat4_WinterUndSommer
echo finished >> "$OUT/log.txt"

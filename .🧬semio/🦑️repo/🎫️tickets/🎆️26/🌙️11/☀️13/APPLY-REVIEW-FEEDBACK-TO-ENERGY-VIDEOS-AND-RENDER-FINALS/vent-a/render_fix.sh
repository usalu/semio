#!/usr/bin/env bash
cd /c/git/semio
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$PWD/$TK/🗑️generated/vent-a-out/direct"
E="🎓️teaching/🏛️architecture/⚡️energy"
export PYTHONIOENCODING=utf-8 SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_OUTPUT_ROOT="$PWD/$TK/🗑️generated/vent-a-out"
rm -f "$OUT/log2.txt"
run() {
  folder="$1"; shift
  for scene in "$@"; do
    uv run --project "$E/🏗️authoring/📦️packages/🐍️python" --python 3.12 python -m manim render --resolution 854,480 --fps 15 \
      --media_dir "$OUT/media" --disable_caching "$E/🌬️ventilation/🎬️clip/$folder/🐍️.py" "$scene" > "$OUT/log2-$scene.txt" 2>&1
    echo "done $scene rc=$?" >> "$OUT/log2.txt"
  done
}
run "🌡️heat-flow" Beat4_WinterUndSommer Beat1_Luftwaerme
run "📦️volume" Beat2_Luftwechselrate
echo finished >> "$OUT/log2.txt"

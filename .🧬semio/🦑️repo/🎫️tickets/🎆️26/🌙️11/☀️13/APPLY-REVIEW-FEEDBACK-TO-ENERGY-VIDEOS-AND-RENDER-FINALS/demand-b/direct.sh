#!/usr/bin/env bash
# usage: direct.sh <chapter-folder> <Scene>
cd /c/git/semio || exit 1
TK=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
O="$TK/🗑️generated/demand-b-out/direct"
mkdir -p "$O"
PYTHONIOENCODING=utf-8 SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_PROFILE=preview SEMIO_TEST_MANIM=1 SEMIO_TEACHING_OUTPUT_ROOT="$O" \
  uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 \
  python -m manim render -ql --fps 15 --media_dir "$O/media" \
  "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/$1/🐍️.py" "$2" > "$O/$2.log" 2>&1
echo "exit $?" >> "$O/$2.log"

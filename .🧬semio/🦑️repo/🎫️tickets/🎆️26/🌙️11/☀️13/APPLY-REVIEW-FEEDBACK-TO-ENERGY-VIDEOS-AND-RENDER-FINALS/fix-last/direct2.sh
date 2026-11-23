#!/usr/bin/env bash
# usage: direct.sh <clip-relative-folder-from-energy> <Scene>
cd /c/git/semio || exit 1
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
O="$TK/🗑️generated/fix-last/direct"
mkdir -p "$O"
SEMIO_TEACHING_OUTPUT_ROOT="$O" SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_AUDIENCE=speaker PYTHONIOENCODING=utf-8 \
  uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 \
  python -m manim --media_dir "$O/media-$2" --resolution 854,480 --fps 15 --disable_caching "🎓️teaching/🏛️architecture/⚡️energy/$1/🐍️.py" "$2" > "$O/$2.log" 2>&1
echo "exit $?" >> "$O/$2.log"
tail -2 "$O/$2.log" | tr '\r' '\n' | tail -2

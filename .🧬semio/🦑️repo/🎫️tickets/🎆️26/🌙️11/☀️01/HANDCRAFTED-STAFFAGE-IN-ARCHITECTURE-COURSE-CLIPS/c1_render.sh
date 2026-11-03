#!/bin/bash
# usage: c1_render.sh <Scene>   (renders one practice scene at -ql 15 fps into the ticket generated folder)
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
MEDIA="$T/🗑️generated/c1/media"
SCRIPT="$W/🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py"
cd "$W" && PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$W/.🧬semio/🎓️teaching/⚡️cache/python" \
  SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker \
  SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/c1/out" \
  SEMIO_TEACHING_CACHE_DIR=.🧬semio/🎓️teaching/⚡️cache/fonts \
  uv run --project "$P" --python 3.12 python -m manim -ql --fps 15 --disable_caching \
  --media_dir "$MEDIA" "$SCRIPT" "$1"

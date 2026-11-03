#!/usr/bin/env bash
# V1: render ONE scene at quality l, 15 fps: v1_render.sh <script relative to energy dir> <Scene>
set -e
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
MEDIA="$T/🗑️generated/v1/media"
SCRIPT="$W/🎓️teaching/🏛️architecture/⚡️energy/$1"
cd "$W"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker \
  SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" \
  SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  uv run --project "$P" --python 3.12 python -m manim -ql --fps 15 --disable_caching \
  --media_dir "$MEDIA" "$SCRIPT" "$2"
find "$MEDIA" -name "$2.mp4"

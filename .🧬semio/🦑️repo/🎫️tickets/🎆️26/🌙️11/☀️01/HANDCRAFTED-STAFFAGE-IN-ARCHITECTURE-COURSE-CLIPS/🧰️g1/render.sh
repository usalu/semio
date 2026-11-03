#!/usr/bin/env bash
# G1: render ONE scene at quality l, 15 fps, then a frame sheet: g1_render.sh <script rel to energy dir> <Scene> <tag> <t1,t2,...>
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
G="$T/🗑️generated/g1"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
PY="$CACHE/python/Scripts/python.exe"
cd "$W"
mkdir -p "$G"
MEDIA="$G/media-$3"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker \
  SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" \
  SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  uv run --project "$P" --python 3.12 python -m manim -ql --fps 15 --disable_caching \
  --media_dir "$MEDIA" "$W/🎓️teaching/🏛️architecture/⚡️energy/$1" "$2" > "$G/$3.log" 2>&1
mp4=$(find "$MEDIA" -name "$2.mp4" | head -1)
if [ -n "$mp4" ]; then
  mkdir -p "$G/mp4"; cp "$mp4" "$G/mp4/$3.mp4"
  PYTHONUTF8=1 "$PY" "$T/v1_pick.py" "$G/mp4/$3.mp4" "$G/$3-sheet.png" 3 "$4" > "$G/$3.pick.log" 2>&1
  rm -rf "$MEDIA"
  echo "done $3"
else
  echo "FAILED $3"; tail -15 "$G/$3.log"
fi

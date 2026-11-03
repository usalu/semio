#!/usr/bin/env bash
# sm2: render ONE scene at quality l, 15 fps, copy the mp4 into the generated folder: render.sh <script rel to energy dir> <Scene> <tag>
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/sm2"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
cd "$W"
mkdir -p "$G"
MEDIA="$G/media-$3"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker \
  SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" \
  SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  "$CACHE/python/Scripts/python.exe" -m manim -ql --fps 15 --disable_caching \
  --media_dir "$MEDIA" "$W/🎓️teaching/🏛️architecture/⚡️energy/$1" "$2" > "$G/$3.log" 2>&1
mp4=$(find "$MEDIA" -name "$2.mp4" | head -1)
if [ -n "$mp4" ]; then
  cp "$mp4" "$G/$3.mp4"; rm -rf "$MEDIA"; echo "done $3"
else
  echo "FAILED $3"; tail -15 "$G/$3.log"
fi

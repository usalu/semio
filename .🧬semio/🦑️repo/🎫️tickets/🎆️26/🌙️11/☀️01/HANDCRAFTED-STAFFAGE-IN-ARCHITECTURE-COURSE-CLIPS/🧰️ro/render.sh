#!/usr/bin/env bash
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
G="$T/🗑️generated/ro"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
cd "$W"
MEDIA="$G/media-$3"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=${LANGX:-en} SEMIO_TEACHING_AUDIENCE=speaker \
  SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  uv run --project "$P" --python 3.12 python -m manim -ql --fps 15 --disable_caching \
  --media_dir "$MEDIA" "$W/$1" "$2" > "$G/$3.log" 2>&1
mp4=$(find "$MEDIA" -name "$2.mp4" | head -1)
if [ -n "$mp4" ]; then mkdir -p "$G/mp4"; cp "$mp4" "$G/mp4/$3.mp4"; rm -rf "$MEDIA"; echo "done $3"; else echo FAILED; tail -15 "$G/$3.log"; fi

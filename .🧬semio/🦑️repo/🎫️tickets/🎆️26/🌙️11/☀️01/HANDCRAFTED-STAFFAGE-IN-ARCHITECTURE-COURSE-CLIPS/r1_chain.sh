#!/usr/bin/env bash
# R1: render a list of "<script relative to energy dir>|<Scene>" lines (file $1) one at a time, then extract frames.
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/r1"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
PY="$CACHE/python/Scripts/python.exe"
cd "$W"
mkdir -p "$G"
while IFS='|' read -r rel scene tag; do
  [ -z "$scene" ] && continue
  [ -f "$G/$tag-sheet.png" ] && continue
  echo "$(date +%T) start $tag" >> "$G/chain.log"
  MEDIA="$G/media-$tag"
  PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
    UV_PROJECT_ENVIRONMENT="$CACHE/python" \
    SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker \
    SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" \
    SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
    uv run --project "$P" --python 3.12 python -m manim -ql --fps 15 --disable_caching \
    --media_dir "$MEDIA" "$W/🎓️teaching/🏛️architecture/⚡️energy/$rel" "$scene" > "$G/$tag.log" 2>&1
  mp4=$(find "$MEDIA" -name "$scene.mp4" | head -1)
  if [ -n "$mp4" ]; then
    mkdir -p "$G/mp4"; cp "$mp4" "$G/mp4/$tag.mp4"
    PYTHONUTF8=1 "$PY" "$T/r1_frames.py" "$G/mp4/$tag.mp4" "$G" "$tag" >> "$G/chain.log" 2>&1
    rm -rf "$MEDIA"
    echo "$(date +%T) done $tag" >> "$G/chain.log"
  else
    echo "$(date +%T) FAILED $tag" >> "$G/chain.log"
  fi
done < "$1"
echo "$(date +%T) chain complete" >> "$G/chain.log"

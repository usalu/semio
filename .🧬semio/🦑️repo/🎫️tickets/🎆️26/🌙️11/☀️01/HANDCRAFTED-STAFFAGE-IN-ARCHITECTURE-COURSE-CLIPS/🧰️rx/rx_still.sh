#!/usr/bin/env bash
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
G="$T/🗑️generated/rx"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
cd "$W"; mkdir -p "$G"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=${LANGX:-en} SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  uv run --project "$P" --python 3.12 python -m manim ${QUAL:--ql} -s --fps 15 --disable_caching --media_dir "$G/media-$3" "$1" "$2" > "$G/$3.log" 2>&1
find "$G/media-$3" -name "*.png" | head -3
tail -5 "$G/$3.log"

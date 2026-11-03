#!/usr/bin/env bash
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
G="$T/🗑️generated/rb"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
cd "$W"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_LANGUAGE=${LANGUAGE_RB:-en} SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" SEMIO_TEACHING_CACHE_DIR="$CACHE/fonts" \
  uv run --project "$P" --python 3.12 python -m manim -ql --fps 15 --disable_caching --media_dir "$G/media-$3" "$W/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/$1/🐍️.py" "$2" > "$G/$3.log" 2>&1
echo "EXIT $?" >> "$G/$3.log"

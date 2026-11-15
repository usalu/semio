#!/usr/bin/env bash
# usage: direct.sh <clip-folder> <SceneClass>...
cd /c/git/semio || exit 1
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
OUT="$TK/🗑️generated/demand-a-out/direct"
mkdir -p "$OUT"
export SEMIO_TEACHING_LANGUAGE="${SEMIO_TEACHING_LANGUAGE:-de}" PYTHONUTF8=1 PYTHONIOENCODING=utf-8 SEMIO_TEACHING_OUTPUT_ROOT="$TK/🗑️generated/demand-a-out"
export SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_FPS=15 SEMIO_TEACHING_RESOLUTION=854,480
clip="$1"; shift
for scene in "$@"; do
  export SEMIO_TEACHING_JOB="preview-$scene-$(date +%s%N)"
  echo "=== $scene $(date +%T)"
  uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 python -m manim --resolution 854,480 --fps 15 --disable_caching --media_dir "$OUT/media" "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/$clip/🐍️.py" "$scene" > "$OUT/$scene.log" 2>&1
  echo "exit $? $(date +%T)"
  tail -3 "$OUT/$scene.log"
done

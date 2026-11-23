#!/bin/bash
# Usage: run-refactor-stage.sh <label> <module>...; runs modules sequentially with the final profile, logs under 🗑️generated/refactor-stage.
cd /c/git/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
G="$T/🗑️generated/refactor-stage"
mkdir -p "$G"
label="$1"; shift
export SEMIO_REPO_BINARY="C:/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe"
export PATH="/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79:$PATH"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1
export SEMIO_TEACHING_PROFILE=final SEMIO_TEACHING_LANGUAGE=en
for m in "$@"; do
  safe=$(echo "$m" | md5sum | cut -c1-8)
  export SEMIO_TEACHING_OUTPUT_ROOT="$(pwd)/$G/o-$safe"
  mkdir -p "$SEMIO_TEACHING_OUTPUT_ROOT"
  s=$(date +%s)
  timeout 1200 uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest -v "$m" > "$G/$label-$safe.log" 2>&1
  echo "$? $(( $(date +%s)-s ))s $m" >> "$G/$label.txt"
done

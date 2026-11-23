#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
G="$T/🗑️generated/tests-final"
m="$1"
safe=$(echo "$m" | md5sum | cut -c1-8)
n=$(echo "$m" | sed 's/^🎓️teaching\.🏛️architecture\.⚡️energy\.//;s/[^[:alnum:]_.-]//g' | cut -c1-60)
log="$G/logs/$n-$safe.log"
export SEMIO_REPO_BINARY="C:/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79/repo.exe"
export PATH="/c/Users/Ueli/AppData/Local/Semio/repo/aa03a414b2a2840b9756fa5684782a315a2713c64c27bd96434f67fdf9490e79:$PATH"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1
case "$m" in *visuals.🧪️tests.🐍️integration) export SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_LANGUAGE=en;; esac
[ -n "$EXTRAENV" ] && export $EXTRAENV
export SEMIO_TEACHING_OUTPUT_ROOT="$(pwd)/$G/o-$safe"
mkdir -p "$SEMIO_TEACHING_OUTPUT_ROOT"
s=$(date +%s)
timeout "${TMO:-1200}" uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest -v "$m" > "$log" 2>&1
rc=$?
echo "$rc $(( $(date +%s)-s ))s $m" >> "$G/${RES:-results.txt}"

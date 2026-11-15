#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
m="$1"
tmo="${TMO:-900}"
safe=$(echo "$m" | sed 's/^🎓️teaching\.🏛️architecture\.⚡️energy\.//' | md5sum | cut -c1-8)
n=$(echo "$m" | sed 's/^🎓️teaching\.🏛️architecture\.⚡️energy\.//;s/[^[:alnum:]_.-]//g' | cut -c1-60)
log="$T/🗑️generated/tests-out/logs/$n-$safe.log"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1 SEMIO_TEACHING_PROFILE=final SEMIO_TEACHING_LANGUAGE=en
export SEMIO_TEACHING_OUTPUT_ROOT="$(pwd)/$T/🗑️generated/tests-out/o-$safe"
mkdir -p "$SEMIO_TEACHING_OUTPUT_ROOT"
s=$(date +%s)
timeout "$tmo" uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest "$m" > "$log" 2>&1
rc=$?
echo "$rc $(( $(date +%s)-s ))s $m" >> "$T/🗑️generated/tests-out/${RES:-results.txt}"

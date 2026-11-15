#!/usr/bin/env bash
cd /c/git/semio
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
export SEMIO_TEST_MANIM=1 SEMIO_TEACHING_OUTPUT_ROOT="$TK/🗑️generated/roadmap-out" PYTHONUTF8=1 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEACHING_LANGUAGE=en
mkdir -p "$TK/🗑️generated/roadmap-out"
uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest "$@" > "$TK/🗑️generated/roadmap-out/test-$(echo "$1" | md5sum | cut -c1-6).log" 2>&1
echo "exit $?" >> "$TK/🗑️generated/roadmap-out/test-$(echo "$1" | md5sum | cut -c1-6).log"

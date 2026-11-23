#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1 SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_LANGUAGE=en
export SEMIO_TEACHING_OUTPUT_ROOT="$(pwd)/$T/🗑️generated/fix-last/test-out$$"
mkdir -p "$SEMIO_TEACHING_OUTPUT_ROOT"
timeout 1500 uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest "$@" 2>&1 | grep -v "^\s*$" | grep -v "INFO\|^ \{20,\}\|segment\]\|Animation\|manim version\|upgrading" | cut -c1-400 | tail -40

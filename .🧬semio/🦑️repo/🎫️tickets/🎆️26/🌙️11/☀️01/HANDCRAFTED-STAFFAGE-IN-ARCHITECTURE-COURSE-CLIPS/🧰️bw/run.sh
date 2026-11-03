#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
O="$T/🗑️generated/bw/$1"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
mkdir -p "$O"
cd "$W" && PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$W/.🧬semio/🎓️teaching/⚡️cache/python" \
  SEMIO_TEACHING_OUTPUT_ROOT=.🧬semio/🎓️teaching/⚡️cache/tests SEMIO_TEST_MANIM=1 \
  SEMIO_TEACHING_CACHE_DIR=.🧬semio/🎓️teaching/⚡️cache/fonts \
  timeout 7200 uv run --project "$P" --python 3.12 --group test python -u "$T/🧰️bw/fnrun.py" "$2" \
  </dev/null >"$O/run.log" 2>&1
echo "exit $?" >>"$O/life.log"

#!/usr/bin/env bash
# R3: run a python script/module in the teaching env: r3_run.sh <args to python>
W=/c/git/semio
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="$W/.🧬semio/🎓️teaching/⚡️cache"
cd "$W"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  SEMIO_TEACHING_OUTPUT_ROOT="$CACHE/tests" \
  uv run --project "$P" --python 3.12 python "$@"

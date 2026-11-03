#!/usr/bin/env bash
W=/c/git/semio
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
cd "$W" && PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 UV_PROJECT_ENVIRONMENT="$W/.🧬semio/🎓️teaching/⚡️cache/python" SEMIO_TEACHING_OUTPUT_ROOT=.🧬semio/🎓️teaching/⚡️cache/tests \
  uv run --project "$P" --python 3.12 --group test python "$@"

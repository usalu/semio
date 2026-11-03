#!/usr/bin/env bash
set -euo pipefail
W="$(git rev-parse --show-toplevel)"
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE"
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
B="🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals"
if [ "$#" -eq 0 ]; then set -- "$B.🧪️tests.🐍️staffage_only" "$B.🧍️staffage.🧪️tests.🐍️"; fi
cd "$W"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 \
  UV_PROJECT_ENVIRONMENT="$W/.🧬semio/🎓️teaching/⚡️cache/python" \
  SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/course" SEMIO_TEST_MANIM=1 \
  SEMIO_TEACHING_CACHE_DIR="$W/.🧬semio/🎓️teaching/⚡️cache/fonts" \
  uv run --project "$P" --python 3.12 --group test python -m unittest "$@"

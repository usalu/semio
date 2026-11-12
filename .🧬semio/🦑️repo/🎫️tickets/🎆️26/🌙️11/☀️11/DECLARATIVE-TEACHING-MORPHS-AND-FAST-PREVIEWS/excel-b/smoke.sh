#!/usr/bin/env bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️11/DECLARATIVE-TEACHING-MORPHS-AND-FAST-PREVIEWS"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1 UV_PROJECT_ENVIRONMENT="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/python" SEMIO_TEACHING_CACHE_DIR="/c/git/semio/.🧬semio/🎓️teaching/⚡️cache/fonts" SEMIO_TEACHING_OUTPUT_ROOT="/c/git/semio/$T/🗑️generated/excel-b-smoke"
mkdir -p "$SEMIO_TEACHING_OUTPUT_ROOT"
exec uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -u "$T/excel-b/smoke.py" "$@"

#!/usr/bin/env bash
# Usage: run.sh <python script> [args...]  (runs inside the teaching venv with the repo root importable)
T="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
cd "$ROOT" || exit 2
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1 PYTHONPATH="$ROOT"
export UV_PROJECT_ENVIRONMENT="$ROOT/.🧬semio/🎓️teaching/⚡️cache/python"
export SEMIO_TEACHING_CACHE_DIR="$ROOT/.🧬semio/🎓️teaching/⚡️cache/fonts"
exec uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -u "$@"

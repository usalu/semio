#!/usr/bin/env bash
# Usage: RUN_NAME=<worker> run-tests.sh <unittest module> [more modules...]
# Example: RUN_NAME=ana run-tests.sh '🎓️teaching.🏛️architecture.🔀️morph.🧪️tests.🐍️'
# Extra unittest flags may follow via UNITTEST_ARGS (e.g. UNITTEST_ARGS="-v -k terminal").
set -u
T="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
NAME="${RUN_NAME:-shared}"
[ "$#" -gt 0 ] || { echo "usage: RUN_NAME=<worker> $0 <module>..." >&2; exit 2; }
cd "$ROOT" || exit 2
OUT="$T/🗑️generated/tests-$NAME"
mkdir -p "$OUT"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1
export UV_PROJECT_ENVIRONMENT="$ROOT/.🧬semio/🎓️teaching/⚡️cache/python"
export SEMIO_TEACHING_CACHE_DIR="$ROOT/.🧬semio/🎓️teaching/⚡️cache/fonts"
export SEMIO_TEACHING_OUTPUT_ROOT="$OUT"
# shellcheck disable=SC2086
exec uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest ${UNITTEST_ARGS:-} "$@" 2>&1 | tee "$OUT/last.log"

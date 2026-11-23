#!/usr/bin/env bash
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
ENERGY="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring"
OUT="$HERE/../🗑️generated/fix-last/pyout"
mkdir -p "$OUT"
cd "$ROOT" || exit 2
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1
export SEMIO_TEACHING_OUTPUT_ROOT="$OUT"
export SEMIO_TEACHING_LANGUAGE="${SEMIO_TEACHING_LANGUAGE:-de}"
export UV_PROJECT_ENVIRONMENT="$ROOT/.🧬semio/🎓️teaching/⚡️cache/python"
exec uv run --project "$ENERGY/📦️packages/🐍️python" --python 3.12 --group test python "$@"

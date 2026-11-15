#!/usr/bin/env bash
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TK="$(cd "$HERE/../.." && pwd)"
ROOT="$(cd "$TK/../../../../../../.." && pwd)"
OUT="$TK/🗑️generated/consistency-out/units"
cd "$ROOT" || exit 2
ENERGY="🎓️teaching/🏛️architecture/⚡️energy"
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 SEMIO_TEST_MANIM=1
export SEMIO_TEACHING_OUTPUT_ROOT="$OUT" SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_PROFILE=preview
export UV_PROJECT_ENVIRONMENT="$ROOT/.🧬semio/🎓️teaching/⚡️cache/python"
mods=()
for rel in "$@"; do
  mods+=("$(echo "🎓️teaching/🏛️architecture/⚡️energy/${rel%.py}" | tr '/' '.')")
done
exec uv run --project "$ENERGY/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest "${mods[@]}"

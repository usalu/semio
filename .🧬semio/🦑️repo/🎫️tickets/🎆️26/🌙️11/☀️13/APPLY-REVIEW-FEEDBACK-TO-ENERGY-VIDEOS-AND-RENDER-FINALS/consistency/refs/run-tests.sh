#!/usr/bin/env bash
# Usage: run-tests.sh <dotted.module> [unittest args]   e.g. run-tests.sh "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🧪️tests.🐍️"
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TK="$(cd "$HERE/../.." && pwd)"
ROOT="$(cd "$TK/../../../../../../.." && pwd)"
ENERGY="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring"
OUT="$TK/🗑️generated/consistency-out/refs"
mkdir -p "$OUT"
cd "$ROOT" || exit 2
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1
export SEMIO_TEST_MANIM=1
export SEMIO_TEACHING_PROFILE="${SEMIO_TEACHING_PROFILE:-preview}"
export SEMIO_TEACHING_OUTPUT_ROOT="$OUT"
export SEMIO_TEACHING_LANGUAGE="${SEMIO_TEACHING_LANGUAGE:-en}"
export UV_PROJECT_ENVIRONMENT="${UV_PROJECT_ENVIRONMENT:-$ROOT/.🧬semio/🎓️teaching/⚡️cache/python}"
MODULE="$1"
shift
exec uv run --project "$ENERGY/📦️packages/🐍️python" --python 3.12 --group test python -m unittest "$MODULE" "$@"

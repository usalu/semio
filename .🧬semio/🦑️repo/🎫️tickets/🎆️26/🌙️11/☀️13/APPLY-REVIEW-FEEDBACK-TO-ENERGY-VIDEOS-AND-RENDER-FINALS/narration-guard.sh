#!/usr/bin/env bash
# Runs only the narration guard: every clause of every energy clip scene must write symbols as speakable $...$ TeX.
# Usage: narration-guard.sh                      all five videos
#        NARRATION_GUARD_FILTER=heating narration-guard.sh      only scripts whose path contains "heating"
#        NARRATION_GUARD_FILTER=heat-flow,volume narration-guard.sh   several substrings, comma separated
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../.." && pwd)"
ENERGY="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring"
OUT="$HERE/🗑️generated/speech-out"
mkdir -p "$OUT"
cd "$ROOT" || exit 2
export PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1
export SEMIO_TEST_MANIM=1
export SEMIO_TEACHING_OUTPUT_ROOT="${SEMIO_TEACHING_OUTPUT_ROOT:-$OUT}"
export SEMIO_TEACHING_LANGUAGE="${SEMIO_TEACHING_LANGUAGE:-en}"
export UV_PROJECT_ENVIRONMENT="${UV_PROJECT_ENVIRONMENT:-$ROOT/.🧬semio/🎓️teaching/⚡️cache/python}"
exec uv run --project "$ENERGY/📦️packages/🐍️python" --python 3.12 --group test \
  python -m unittest "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧪️tests.🛡️guard.🐍️" "$@"

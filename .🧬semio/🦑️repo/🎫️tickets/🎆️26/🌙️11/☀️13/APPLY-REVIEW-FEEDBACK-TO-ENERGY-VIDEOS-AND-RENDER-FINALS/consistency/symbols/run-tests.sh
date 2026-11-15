#!/usr/bin/env bash
# Runs every family test module of the symbols pass on its own and writes a one-line verdict per module.
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../../🗑️generated/consistency-out/symbols"
mkdir -p "$OUT/mods"
cd "$HERE/../../../../../../../../.." || exit 2
export PYTHONUTF8=1 SEMIO_TEST_MANIM=1 SEMIO_TEACHING_PROFILE=preview SEMIO_TEACHING_OUTPUT_ROOT="$OUT/o3" PYTHONDONTWRITEBYTECODE=1
export UV_PROJECT_ENVIRONMENT="$PWD/.🧬semio/🎓️teaching/⚡️cache/python"
mkdir -p "$SEMIO_TEACHING_OUTPUT_ROOT"
: > "$OUT/verdict.txt"
while read -r mod; do
  [ -z "$mod" ] && continue
  short="${mod#🎓️teaching.🏛️architecture.⚡️energy.}"
  log="$OUT/mods/$(echo "$short" | tr -c 'A-Za-z0-9' '_').txt"
  timeout 900 uv run --project "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python" --python 3.12 --group test python -m unittest "$mod" > "$log" 2>&1
  code=$?
  echo "$code $short $(tr '\r' '\n' < "$log" | grep -E '^(OK|FAILED|Ran )' | tr '\n' ' ')" >> "$OUT/verdict.txt"
done < "$OUT/mods.txt"
echo done >> "$OUT/verdict.txt"

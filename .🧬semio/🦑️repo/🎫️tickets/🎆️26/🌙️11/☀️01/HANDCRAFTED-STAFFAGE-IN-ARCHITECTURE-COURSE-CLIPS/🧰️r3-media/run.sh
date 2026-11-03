#!/usr/bin/env bash
# run.sh <script.py> [args...]: run a media pipeline script in the teaching Python environment (cross-platform, uv).
W=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
while [ ! -d "$W/🧰️framework" ] && [ "$W" != "/" ]; do W=$(dirname "$W"); done
P="$W/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python"
CACHE="${SEMIO_TEACHING_CACHE_ROOT:-$W/.🧬semio/🎓️teaching/⚡️cache}"
H=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
S="$1"; shift
case "$S" in /*|?:*) ;; *) S="$H/$S" ;; esac
cd "$W"
PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1 UV_PROJECT_ENVIRONMENT="$CACHE/python" \
  uv run --project "$P" --python 3.12 python "$S" "$@"

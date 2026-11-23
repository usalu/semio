#!/bin/sh
# 🏗️ Builds PW-15 assets with their generators. The drawings are unions of projected mesh triangles (shapely), so the generators run in the hash-locked `geometry` oracle environment of the staffage tests through the pinned uv of the repo. Usage: sh pw15-build.sh [--candidate] <id> [<id> ...]   (--candidate writes 🗑️generated/PW-15/cand/<id>/🔣️.json and leaves catalog and row alone)
HERE="$(cd "$(dirname "$0")" && pwd)"
R="$(cd "$HERE/../../../../../../../../.." && pwd)"
O="$R/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🐍️oracles"
if [ "$1" = "--candidate" ]; then PW15_CANDIDATE=1; export PW15_CANDIDATE; shift; fi
for ID in "$@"; do
  UV_PYTHON_DOWNLOADS=never UV_PROJECT_ENVIRONMENT="$HOME/Library/Caches/semio/staffage-oracles/geometry" repo tools exec uv -- run --quiet --project "$O" --locked --only-group geometry python "$HERE/pw15-$ID.py" 2>&1 | grep -v "^\[tools\]\|^\[capacity\]"
done

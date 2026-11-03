#!/usr/bin/env bash
# [DEBUG] A1 loop: build the asset views, verify (all poses and clips), render sheets.
# usage: run.sh <asset-module> <scales-to-dump comma list> [check]
set -u
export PYTHONIOENCODING=utf-8
HERE="$(cd "$(dirname "$0")" && pwd)"
cd "$HERE"
MOD="$1"; SCALES="${2:-50,100,200,500}"
ASSET="${MOD//_/-}"
python build.py "$MOD"
cd "$HERE/../../../../../../../.."
repo tools exec bun -- "$HERE/verify.ts" "$ASSET" "$SCALES"
cd "$HERE"
for sc in ${SCALES//,/ }; do python render.py "$ASSET" "$sc" "${@:4}"; done
if [ "${3:-}" = "check" ]; then
  cd "$HERE/../../../../../../../.."
  repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset "$ASSET"
fi

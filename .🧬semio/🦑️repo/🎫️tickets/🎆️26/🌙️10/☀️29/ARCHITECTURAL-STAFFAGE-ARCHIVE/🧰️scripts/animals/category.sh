#!/usr/bin/env bash
# [scratch] wraps the assembled assets into the animals CategoryDocument
set -euo pipefail
D="$(cd "$(dirname "$0")" && pwd)"
out="$1"; shift
{
  printf '{\n  "$schema": "../../🧬️schema/🔣️.json#/$defs/CategoryDocument",\n  "version": "semio.staffage/v1",\n  "category": "animals",\n  "assets": [\n'
  first=1
  for a in "$@"; do
    if [ $first -eq 0 ]; then printf ',\n'; fi
    first=0
    bash "$D/assemble.sh" "$a"
  done
  printf '\n  ]\n}\n'
} > "$out"

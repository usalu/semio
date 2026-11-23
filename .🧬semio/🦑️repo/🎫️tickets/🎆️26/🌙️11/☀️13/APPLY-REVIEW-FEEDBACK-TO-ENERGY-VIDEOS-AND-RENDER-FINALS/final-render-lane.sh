#!/usr/bin/env bash
# Usage: final-render-lane.sh <language>:<video-id>:<root-name>...; renders the given finals one after another and keeps going when one fails.
TK="$(dirname "$0")"
for job in "$@"; do
  IFS=: read -r language video root <<< "$job"
  bash "$TK/final-render-lang.sh" "$language" "$video" "$root"
done

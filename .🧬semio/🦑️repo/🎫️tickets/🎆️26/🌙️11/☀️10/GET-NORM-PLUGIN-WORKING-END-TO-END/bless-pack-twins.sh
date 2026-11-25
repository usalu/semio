#!/usr/bin/env bash
# 🗜️ Regenerates the stale packs of the given families from their DSL with the family's own pack encoder (the pack-twins test under SEMIO_UPDATE_GOLDEN=1), through the ticket lane.
T="$(cd "$(dirname "$0")" && pwd)"
for f in "$@"; do
  SEMIO_UPDATE_GOLDEN=1 python3 "$T/lane.py" "$T/test-family-examples.sh" "$f"
done

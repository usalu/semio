#!/usr/bin/env bash
# 🗜️ Runs the family examples tests one family at a time through the ticket lane.
T="$(cd "$(dirname "$0")" && pwd)"
for f in "$@"; do
  python3 "$T/lane.py" "$T/test-family-examples.sh" "$f" 
done

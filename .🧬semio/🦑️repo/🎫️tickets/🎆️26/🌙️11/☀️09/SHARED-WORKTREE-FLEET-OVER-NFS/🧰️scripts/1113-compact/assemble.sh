#!/bin/bash
# usage: assemble.sh <module root> <source of the new member and its tests: the draft or src dir>
# applies every anchored patch to <module root> (the live module under the lock, or a copy of the base for a dry run) and copies the new files
set -e
cd "$(dirname "$0")"
M="$1"; SRC="$2"
python3 patch-journal.py "$M"
python3 patch-replica.py "$M"
python3 patch-topology.py "$M"
python3 patch-readme.py "$M"
mkdir -p "$M/🗜️compaction/🧬️schema" "$M/🗜️compaction/🧪️tests"
for f in "🗜️compaction/🟦️.ts" "🗜️compaction/🧬️schema/🔣️.json" "🗜️compaction/🧪️tests/🟦️.ts" "🗜️compaction/🧪️tests/💥️killpoints.ts" "🗜️compaction/🧪️tests/🐍️.py"; do cp "$SRC/$f" "$M/$f"; done

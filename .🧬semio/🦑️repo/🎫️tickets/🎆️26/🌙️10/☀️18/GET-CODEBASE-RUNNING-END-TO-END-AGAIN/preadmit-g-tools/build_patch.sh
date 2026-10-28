#!/bin/sh
# 🧩️ Writes staged/preadmit.diff: the whole change of preadmit-g as one unified diff between the baseline overlay (live tree + phase-g batch 4) and the working overlay, and staged/files.txt (every path it touches, relative to the repository root). Usage: sh build_patch.sh
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad
F="🧰️framework"
LIB="$F/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
cd "$S"
: > "$HERE/staged/preadmit.diff"
for part in "$F/🔨️modules/🏃️process/🏗️build/🌡️host-capacity" "$LIB/🌡️host-capacity" "$LIB/🔧️tools/🏗️bazel/🌡️host-capacity" "$LIB/🔣️policy.json" "$LIB/🔧️tools/🏗️bazel/📜️script.ts"; do
  diff -ruN -x node_modules -x .DS_Store "preadmit-g-base/$part" "preadmit-g-ov/$part" >> "$HERE/staged/preadmit.diff" || true
done
grep '^+++ ' "$HERE/staged/preadmit.diff" | sed 's#^+++ preadmit-g-ov/##; s#\t.*##' | sort > "$HERE/staged/files.txt"
wc -l "$HERE/staged/files.txt" "$HERE/staged/preadmit.diff"

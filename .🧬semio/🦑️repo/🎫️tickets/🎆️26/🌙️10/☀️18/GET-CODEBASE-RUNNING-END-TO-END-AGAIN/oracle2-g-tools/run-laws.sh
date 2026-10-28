#!/bin/sh
# oracle2-g: runs the staged read-oracle laws graph-free in an overlay that has the library's relative layout (symlinks to the live framework modules and packages, the staged module copied in).
# usage: sh run-laws.sh [bun test args...]    overlay: $ORACLE2_OVERLAY (default: scratch o2/overlay)
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
W=/Users/ueli/Documents/semio
O="${ORACLE2_OVERLAY:-/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/o2/overlay}"
M="$O/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle"
mkdir -p "$O/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs"
ln -sfn "$W/🧰️framework/🔨️modules" "$O/🧰️framework/🔨️modules"
ln -sfn "$W/🧰️framework/📦️packages" "$O/🧰️framework/📦️packages"
ln -sfn "$W/node_modules" "$O/node_modules"
rm -rf "$M" && cp -R "$HERE/src/🔭️read-oracle" "$M"
cd "$M" && nice bun test ${ORACLE2_FILES:-./🧪️tests/🟦️.ts} "$@"

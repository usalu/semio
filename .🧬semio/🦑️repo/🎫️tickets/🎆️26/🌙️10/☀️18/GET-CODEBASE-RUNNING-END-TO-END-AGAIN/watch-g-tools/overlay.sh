#!/bin/sh
# Copies the staged modules to their FINAL relative paths below an overlay root outside the repository and runs the tests there against the live repository (watcher, protocol policy, churn peer, node_modules).
# usage: overlay.sh OVERLAY_DIR [bun test args...]
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../../../../../../../.." && pwd)
OV=$1; shift
OBS="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes"
RES="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal"
rm -rf "$OV/$OBS" "$OV/$RES"
mkdir -p "$OV/$OBS" "$OV/$RES"
cp -R "$HERE/staged/changes/." "$OV/$OBS/"
cp -R "$HERE/staged/journal/." "$OV/$RES/"
ln -sfn "$REPO/node_modules" "$OV/node_modules"
LIVE="$REPO/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident"
for d in 🔭️watch 📨️protocol ⌨️command; do ln -sfn "$LIVE/$d" "$OV/$RES/../$d"; done
cd "$OV/$RES"
export SEMIO_FUZZ_REPOSITORY="$REPO"
if [ "$1" = "run" ]; then shift; exec bun "$@"; fi
exec bun test "$@"

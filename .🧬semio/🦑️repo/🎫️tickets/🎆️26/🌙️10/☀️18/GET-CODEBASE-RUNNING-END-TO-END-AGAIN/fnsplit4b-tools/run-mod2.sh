#!/bin/zsh
# usage: run-mod2.sh <tag> <realFile> <snapshot> old|new : like run-mod.sh, but the old snapshot is copied next to the real file (same directory, so import.meta.url and relative imports agree) and removed afterwards
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd /Users/ueli/Documents/semio
mkdir -p "$C/🗑️generated"
REAL="/Users/ueli/Documents/semio/$2"; OLD="${REAL:h}/🕰️old-snapshot.${REAL:e}"
if [ "$4" = old ]; then cp "$C/fnsplit4b-tools/orig/$3" "$OLD"; fi
export REG_MODE="$4" REG_REAL_FILE="$REAL" REG_OLD_FILE="$OLD" REG_OUT="$C/🗑️generated/fnsplit4b-reg-$1-$4.json"
SEMIO_TEST_ARTIFACT_DIR=/Users/ueli/.cache/c6fin2/artifacts NODE_PATH=/Users/ueli/.cache/c6fin2/nm/node_modules perl -e "alarm 300; exec @ARGV" -- bun --tsconfig-override=/Users/ueli/.cache/c6fin2/nm/tsconfig.override.json test "$C/fnsplit4b-tools/mod-trace.test.ts" 2>&1 | head -30
rm -f "$OLD"

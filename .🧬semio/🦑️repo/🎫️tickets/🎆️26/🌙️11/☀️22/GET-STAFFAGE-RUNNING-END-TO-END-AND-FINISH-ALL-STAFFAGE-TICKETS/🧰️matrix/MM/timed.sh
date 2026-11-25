#!/bin/sh
# ⏱️ The matrix of single assets on the landed engine with the peak of the largest process (BSD `time -l`): timed.sh <run folder> <asset id> …; to be run inside a capacity grant (final.sh cmd:<label> with CMD="sh timed.sh …").
# Each asset runs alone (--jobs 1); the maximum resident set size and the real seconds of every one go to stdout as `[DEBUG] <id> maxrss <MB> MB real <s> s`.
set -u
R=/Users/ueli/Documents/semio
T="$R/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
out=$1; shift
cd "$R" || exit 1
for id in "$@"; do
  /usr/bin/time -l -o "$out.$id.time" "$BUN" "$T/📊️matrix.ts" --language en --out "$out" --jobs 1 --timeout-s "${LIMIT_S:-14400}" --assets "$id"
  awk -v id="$id" '/maximum resident set size/ { mb = $1 / 1048576 } /real/ { real = $1 } END { printf "[DEBUG] %s maxrss %.0f MB real %s s\n", id, mb, real }' "$out.$id.time"
done

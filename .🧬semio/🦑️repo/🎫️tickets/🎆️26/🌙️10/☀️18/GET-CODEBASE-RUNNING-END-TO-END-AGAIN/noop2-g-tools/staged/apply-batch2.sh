#!/bin/sh
# apply-batch2.sh: AFTER 11:30 only. One compile-atomic step: watch-g's journal, my v2 modules (named environment diff, ignored per-run variables, journal wiring), the snapshot executable-bit fix.
set -e
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/noop2-g-tools/staged"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T="$B/🔧️tools/🏗️bazel"
python3 "$C/watch-g-tools/apply.py" >/dev/null
python3 "$S/apply-script-v2.py" "$T/📜️script.ts" "$C/🗑️generated/noop2-g/script.v2.ts"
python3 "$S/snapshot/apply.py" "$T/📸️snapshot/🟦️.ts" "$C/🗑️generated/noop2-g/snapshot.v2.ts"
python3 "$C/watch-g-tools/apply.py" --write
cp -R "$S/v2/🔭️witness/." "$B/🔭️witness/"
cp -R "$S/v2/♻️plan-reuse/." "$T/♻️plan-reuse/"
cp "$C/🗑️generated/noop2-g/script.v2.ts" "$T/📜️script.ts"
cp "$C/🗑️generated/noop2-g/snapshot.v2.ts" "$T/📸️snapshot/🟦️.ts"
mkdir -p "$T/📸️snapshot/🧪️tests/🔐️mode"
cp "$S/snapshot/🔐️mode/🟦️.ts" "$T/📸️snapshot/🧪️tests/🔐️mode/🟦️.ts"
echo landed batch2

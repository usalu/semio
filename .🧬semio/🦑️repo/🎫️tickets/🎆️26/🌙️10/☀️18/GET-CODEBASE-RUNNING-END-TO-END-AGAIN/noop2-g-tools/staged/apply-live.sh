#!/bin/sh
# apply-live.sh: lands the plan-reuse batch in ONE step (new modules first, then the patch of the Bazel adapter script); every anchor is checked, nothing is written on a mismatch
set -e
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="$C/noop2-g-tools/staged"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
T="$B/🔧️tools/🏗️bazel/📜️script.ts"
python3 "$S/apply-script.py" "$T" "$C/🗑️generated/noop2-g/script.patched.ts"
[ -e "$B/🔭️witness" ] && { echo "witness module exists"; exit 1; }
[ -e "$B/🔧️tools/🏗️bazel/♻️plan-reuse" ] && { echo "plan-reuse module exists"; exit 1; }
cp -R "$S/🔭️witness" "$B/🔭️witness"
cp -R "$S/♻️plan-reuse" "$B/🔧️tools/🏗️bazel/♻️plan-reuse"
cp "$C/🗑️generated/noop2-g/script.patched.ts" "$T"
echo landed

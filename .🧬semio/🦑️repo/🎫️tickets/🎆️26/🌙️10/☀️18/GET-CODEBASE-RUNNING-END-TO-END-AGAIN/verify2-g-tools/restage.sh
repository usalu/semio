#!/bin/sh
# Rebuilds every staged batch of verify2-g from `work/` (new) and the live tree (base). Run only while the live files of the batches are untouched; refuses when a live file is newer than its work copy.
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && while [ ! -d 🧰️framework ] && [ "$PWD" != / ]; do cd ..; done; pwd)
cd "$ROOT" || exit 1
set -e
batch() { name=$1; shift; rm -rf "$HERE/staged/$name"; mkdir -p "$HERE/staged/$name"; for rel in "$@"; do if [ -e "$rel" ] && [ "$rel" -nt "$HERE/work/$rel" ]; then echo "live file newer than its work copy: $rel" >&2; exit 2; fi; if [ ${#rel} -gt 250 ]; then echo "path too long: $rel" >&2; exit 2; fi; done; python3 -I "$HERE/mkbatch.py" "$HERE/staged/$name" "$HERE/work" "$@" >/dev/null; cp "$HERE/verify-$name.sh" "$HERE/staged/$name/verify.sh"; echo "staged $name ($#)"; }
N="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"
J="$B/🏡️resident/📜️journal"
G="$L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
batch journal-fix \
  "$N/🟦️.ts" "$N/🧬️schema/🔣️.json" "$J/🟦️.ts" "$J/🧪️tests/🟦️.ts" "$J/🧪️tests/🌊️burst/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts"
batch vouch \
  "$N/🪪️vouch/🟦️.ts" "$N/🪪️vouch/🧬️schema/🔣️.json" "$N/🪪️vouch/🧪️tests/🟦️.ts" "$N/🪪️vouch/🧪️tests/🐍️.py" \
  "$J/🪪️vouch/🟦️.ts" "$G/🟦️.ts" "$G/🧬️schema/🔣️.json" "$G/💾️storage/🟦️.ts" "$G/🧪️tests/🟦️.ts" "$B/🚀️bootstrap/🟦️.ts"

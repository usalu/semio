#!/bin/sh
# 🗝️ (Re)builds the staged batch `keyparts` from the CURRENT live files plus keyparts/edit.py and keyparts/tests-ts.txt: sh make-keyparts.sh
set -e
HERE=$(cd "$(dirname "$0")" && pwd); LIVE=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$HERE")
B='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'
P="$B/🔧️tools/🏗️bazel/♻️plan-reuse"
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
python3 -I "$HERE/keyparts/edit.py" "$T"
mkdir -p "$T/$P/🧪️tests/🗝️parts" && cp "$HERE/keyparts/tests-ts.txt" "$T/$P/🧪️tests/🗝️parts/🟦️.ts"
mkdir -p "$HERE/staged/keyparts"
python3 -I "$HERE/lib.py" "$HERE/staged/keyparts" "$LIVE" "$T" '🗝️ Staged batch `keyparts` (lane hotpath2): SEMIO_REPO_PLAN_KEY_PARTS=<file> appends the digests of every part of the Bazel plan key per reuse decision, so a replan "selection, graph, implementation or environment changed" can be traced to its part.' "$P/🟦️.ts" "$B/🔧️tools/🏗️bazel/📜️script.ts" "$B/🔭️witness/🧬️schema/🔣️.json" "$P/🧪️tests/🗝️parts/🟦️.ts"

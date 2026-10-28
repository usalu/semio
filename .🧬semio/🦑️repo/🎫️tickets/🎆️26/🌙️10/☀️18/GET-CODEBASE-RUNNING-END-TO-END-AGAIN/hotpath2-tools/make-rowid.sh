#!/bin/sh
# 🔏️ (Re)builds the staged batch `rowid` from the mirror's new files and the CURRENT live script.ts: sh make-rowid.sh <mirror-root>
set -e
HERE=$(cd "$(dirname "$0")" && pwd); MIRROR=${1:?mirror root}; LIVE=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$HERE")
E='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🌿️environment/🔏️identity'
S='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts'
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
mkdir -p "$T/$E" && cp -R "$MIRROR/$E/." "$T/$E/" && mkdir -p "$(dirname "$T/$S")" && cp "$LIVE/$S" "$T/$S"
python3 -I "$HERE/rowid/edit_script.py" "$T"
mkdir -p "$HERE/staged/rowid"
set -- "$E/🟦️.ts" "$E/🧬️schema/🔣️.json" "$E/🧪️tests/🟦️.ts" "$E/🧪️tests/🧫️fixtures/🔣️.json" "$E/🧪️tests/🧮️resources/🐍️oracle/🐍️.py" "$S"
python3 -I "$HERE/lib.py" "$HERE/staged/rowid" "$LIVE" "$T" '🔏️ Staged batch `rowid` (lane hotpath2): a hot run answers the `runtime` rows of the plan witness from the identity of the executables, without starting the three `repo` doors.' "$@"
cp "$HERE/rowid-bench.ts" "$HERE/staged/rowid/bench.ts"

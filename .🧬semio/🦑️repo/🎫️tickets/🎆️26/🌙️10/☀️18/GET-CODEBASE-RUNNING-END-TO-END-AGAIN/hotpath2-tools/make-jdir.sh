#!/bin/sh
# 📍️ (Re)builds the staged batch `jdir` from the mirror's new test files and the CURRENT live files plus jdir/edit.py: sh make-jdir.sh <mirror-root>
set -e
HERE=$(cd "$(dirname "$0")" && pwd); MIRROR=${1:?mirror root}; LIVE=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$HERE")
B='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build'
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
D="$B/🏡️resident/📜️journal/🧪️tests/📍️directory"
set -- "$B/📥️caller/🛰️service/🔑️session/🟨️.mjs" "$B/📥️caller/🛰️service/🔑️session/🟨️.d.mts" "$B/🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs" "$B/🏡️resident/📨️protocol/📍️endpoint/🟨️.d.mts" "$B/🏡️resident/📜️journal/🟦️.ts"
for rel in "$@"; do mkdir -p "$(dirname "$T/$rel")"; cp "$LIVE/$rel" "$T/$rel"; done
python3 -I "$HERE/jdir/edit.py" "$T"
for rel in "$D/🟦️.ts" "$D/🐍️oracle/🐍️.py" "$D/🧫️fixtures/🔣️.json"; do mkdir -p "$(dirname "$T/$rel")"; cp "$MIRROR/$rel" "$T/$rel"; done
mkdir -p "$HERE/staged/jdir"
python3 -I "$HERE/lib.py" "$HERE/staged/jdir" "$LIVE" "$T" '📍️ Staged batch `jdir` (lane hotpath2): the change journal directory no longer depends on TMPDIR, so a nested `repo` inside a Bazel action reaches the journal (and the graph memo) of its parent.' "$@" "$D/🟦️.ts" "$D/🐍️oracle/🐍️.py" "$D/🧫️fixtures/🔣️.json"

#!/bin/sh
# 📦 (Re)builds the staged batch `memoout` from the CURRENT live memo files plus memoout/edit.py: sh make-memoout.sh   (edits run in a temp root)
set -e
HERE=$(cd "$(dirname "$0")" && pwd); LIVE=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$HERE")
G='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo'
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
python3 -I "$HERE/memoout/edit.py" "$T"
mkdir -p "$HERE/staged/memoout"
python3 -I "$HERE/lib.py" "$HERE/staged/memoout" "$LIVE" "$T" '📦 Staged batch `memoout` (lane hotpath2): the engine re-checks the listing of the directory a build writes its own results into (`.🧾️store`) on every lookup, so lock directory, temp file and rename of the build no longer make the graph memo miss.' "$G/🧬️schema/🔣️.json" "$G/🟦️.ts" "$G/📤️relevance/🟦️.ts" "$G/🧪️tests/🟦️.ts"

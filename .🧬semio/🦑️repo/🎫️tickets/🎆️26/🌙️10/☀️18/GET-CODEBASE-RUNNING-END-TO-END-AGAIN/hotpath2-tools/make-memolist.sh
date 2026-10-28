#!/bin/sh
# 🪧️ (Re)builds the staged batch `memolist` from the CURRENT live memo files plus memolist/edit_src.py and edit_tests.py: sh make-memolist.sh   (no mirror needed; the edits run in a temp root)
set -e
HERE=$(cd "$(dirname "$0")" && pwd); LIVE=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$HERE")
G='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo'
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
for rel in 🧬️schema/🔣️.json 🟦️.ts 📤️relevance/🟦️.ts 🧪️tests/🟦️.ts 🧪️tests/🐍️.py; do mkdir -p "$(dirname "$T/$G/$rel")"; done
python3 -I "$HERE/memolist/edit_src.py" "$T"
python3 -I "$HERE/memolist/edit_tests.py" "$T"
mkdir -p "$HERE/staged/memolist"
python3 -I "$HERE/lib.py" "$HERE/staged/memolist" "$LIVE" "$T" '🪧️ Staged batch `memolist` (lane hotpath2): the graph memo settles a content edit of an unread sibling in a directory of the discovery frontier with the stamp the generation captured, instead of replanning the graph.' "$G/🧬️schema/🔣️.json" "$G/🟦️.ts" "$G/📤️relevance/🟦️.ts" "$G/🧪️tests/🟦️.ts" "$G/🧪️tests/🐍️.py"

#!/bin/sh
# 🧪 Runs a suite list in mirror brf (sealed, guarded) with junit output: run-suites.sh <suites.txt> <label>; writes and prints the failure set (C/🗑️generated/bootreg/<label>.set)
HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/.." && pwd); G="$C/🗑️generated/bootreg"; REL="${G#/Users/ueli/Documents/semio/}"; mkdir -p "$G/art-$2"
cd /Users/ueli/Documents/semio && SEMIO_TEST_ARTIFACT_DIR="$G/art-$2" python3 -I "$C/mirror-tools/mirror.py" run brf --peers-ok --seal-allow "$REL" -- sh -c "bun test --timeout 200000 --reporter=junit --reporter-outfile='$G/$2.xml' $(tr '\n' ' ' < "$1") > '$G/$2.out' 2>&1; echo exit \$?" 2>&1 | grep -E '^exit|GUARD'
python3 -I "$C/cargo2-tools/junit-set.py" "$G/$2.xml" > "$G/$2.set"; head -1 "$G/$2.set"

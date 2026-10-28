#!/bin/sh
# 🧪 Runs the selnear suite list in mirror brw (sealed, guarded) with junit output: suites.sh <label>; prints the failure set (output under C/🗑️generated/bootreg)
HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../.." && pwd); G="$C/🗑️generated/bootreg"; REL="${G#/Users/ueli/Documents/semio/}"; mkdir -p "$G/art-$1"
cd /Users/ueli/Documents/semio && SEMIO_TEST_ARTIFACT_DIR="$G/art-$1" python3 -I "$C/mirror-tools/mirror.py" run brw --peers-ok --seal-allow "$REL" -- sh -c "bun test --timeout 120000 --reporter=junit --reporter-outfile='$G/$1.xml' $(tr '\n' ' ' < "$HERE/suites.txt") > '$G/$1.out' 2>&1; echo exit \$?" 2>&1 | grep -E '^exit|GUARD'
python3 -I "$C/cargo2-tools/junit-set.py" "$G/$1.xml" > "$G/$1.set"; head -1 "$G/$1.set"

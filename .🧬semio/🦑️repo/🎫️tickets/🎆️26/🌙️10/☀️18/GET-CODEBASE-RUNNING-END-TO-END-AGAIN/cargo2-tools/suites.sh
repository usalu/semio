#!/bin/sh
# 🧪 Runs a suite list in mirror c2 with junit output: suites.sh <list.txt> <label>   (output under C/🗑️generated/cargo2)
C=$(cd "$(dirname "$0")/.." && pwd); G="$C/🗑️generated/cargo2"; REL="${G#/Users/ueli/Documents/semio/}"; mkdir -p "$G/art-$2"
cd /Users/ueli/Documents/semio && SEMIO_TEST_ARTIFACT_DIR="$G/art-$2" python3 -I "$C/mirror-tools/mirror.py" run c2 --peers-ok --seal-allow "$REL" -- sh -c "bun test --timeout 120000 --reporter=junit --reporter-outfile='$G/$2.xml' $(tr '\n' ' ' < "$1") > '$G/$2.out' 2>&1; echo exit \$?" 2>&1 | grep -E '^exit|GUARD'
python3 -I "$C/cargo2-tools/junit-set.py" "$G/$2.xml"

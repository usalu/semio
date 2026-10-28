#!/bin/sh
# 🧪 Runs a suite list in mirror cfx with junit output: suites.sh <list.txt> <label>   (output under C/🗑️generated/cargofix)
C=$(cd "$(dirname "$0")/.." && pwd); G="$C/🗑️generated/cargofix"; REL="${G#/Users/ueli/Documents/semio/}"; mkdir -p "$G/art-$2"
cd /Users/ueli/Documents/semio && SEMIO_TEST_ARTIFACT_DIR="$G/art-$2" python3 -I "$C/mirror-tools/mirror.py" run cfx --peers-ok --seal-allow "$REL" -- sh -c "bun test --timeout 120000 --reporter=junit --reporter-outfile='$G/$2.xml' $(tr '\n' ' ' < "$1") > '$G/$2.out' 2>&1; echo exit \$?" 2>&1 | grep -E '^exit|guard'
python3 -I "$C/cargofix-tools/junit-set.py" "$G/$2.xml"

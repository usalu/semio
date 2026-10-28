#!/bin/sh
# 🧪 Unit-cache suites in mirror c2 WITHOUT SEMIO_TEST_ARTIFACT_DIR (their scratch workspaces must stay outside every checkout): suites-uc.sh <list> <label>
C=$(cd "$(dirname "$0")/.." && pwd); G="$C/🗑️generated/cargo2"; REL="${G#/Users/ueli/Documents/semio/}"; mkdir -p "$G"
cd /Users/ueli/Documents/semio && python3 -I "$C/mirror-tools/mirror.py" run c2 --peers-ok --seal-allow "$REL" -- sh -c "bun test --timeout 300000 --reporter=junit --reporter-outfile='$G/$2.xml' $(tr '\n' ' ' < "$1") > '$G/$2.out' 2>&1; echo exit \$?" 2>&1 | grep -E '^exit|GUARD'
python3 -I "$C/cargo2-tools/junit-set.py" "$G/$2.xml" | head -1

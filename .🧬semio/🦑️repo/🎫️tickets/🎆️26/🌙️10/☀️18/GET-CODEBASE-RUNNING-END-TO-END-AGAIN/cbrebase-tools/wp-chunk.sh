#!/bin/sh
# 🧮 Runs lines A..B of witpub's suites.txt on mirror cbrebase (junit under C/🗑️generated/cbrebase): sh wp-chunk.sh <from> <to>
here=$(cd "$(dirname "$0")" && pwd); C=$(cd "$here/.." && pwd); R=/Users/ueli/Documents/semio; art="$C/🗑️generated/cbrebase"; rel=${art#$R/}
list=$(sed -n "$1,$2p" "$C/edit30-tools/staged/witpub/suites.txt" | tr '\n' ' ')
B=$(ls $R/.cache/b/t/m/bazel/bin/*/bazel 2>/dev/null | head -1)
SEMIO_MIRROR_HOME=/private/tmp/m SEMIO_TEST_BAZEL_BINARY=$B SEMIO_TEST_ARTIFACT_DIR="$art" python3 -I "$C/mirror-tools/mirror.py" run cbrebase --peers-ok --seal-allow "$rel" -- sh -c "bun test --timeout 600000 --reporter=junit --reporter-outfile='$art/[DEBUG]-wp-$1.xml' $list" > "$art/[DEBUG]-wp-$1.out" 2>&1
python3 -I "$C/edit30-tools/staged/freshmtime/junit-set.py" "$art/[DEBUG]-wp-$1.xml" > "$art/[DEBUG]-wp-$1.set"; head -1 "$art/[DEBUG]-wp-$1.set"

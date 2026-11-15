#!/bin/bash
# usage: apply-all.sh <work-dir> ; the work dir is a fresh mirror of the live tree
set -e
H=$(cd "$(dirname "$0")" && pwd)
W=$1
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
SRC=$H/work/$M
python3 $H/reapply-attest.py "$W/$M/🩺️attest/🟦️.ts"
python3 $H/reapply-attest2.py "$W/$M/🩺️attest/🟦️.ts"
python3 $H/reapply-attest-schema.py "$W/$M"
python3 $H/patch-attest-test.py "$W/$M"
python3 $H/patch-replica-live.py "$W"
python3 $H/patch-replica-live2.py "$W"
python3 $H/patch-replica-live3.py "$W"
python3 $H/reapply-journal.py "$W/$M/📒️journal/🟦️.ts"
python3 $H/reapply-command.py "$W/$M/🔁️replica/⌨️command/🟦️.ts"
python3 $H/reapply-command2.py "$W/$M/🔁️replica/⌨️command/🟦️.ts"
python3 $H/reapply-schema.py "$W/$M"
python3 $H/reapply-schema2.py "$W/$M"
cp -R "$SRC"/*resilience "$W/$M/"
cp "$H/work2/$M/🔁️replica/📜️script.ts" "$W/$M/🔁️replica/📜️script.ts"
cp "$SRC/🔁️replica/🧪️tests/🛡️resilience.ts" "$SRC/🔁️replica/🧪️tests/💥️killpoints.ts" "$W/$M/🔁️replica/🧪️tests/"
echo APPLIED

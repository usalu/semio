#!/bin/sh
# two Bun-bundled artifact packages :test concurrently (run-many --parallel=2); every line timestamped
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
(nice -n 10 bun repo --tool=semio run-many --parallel="$1" --targets=test --projects=@semio-tech/norm-din16798,@semio-tech/norm-din18599 2>&1; echo "rc=$?") | python3 "$C/cap-c-tools/ts.py" > "$C/🗑️generated/slot-d/real-pair-p$1.log"

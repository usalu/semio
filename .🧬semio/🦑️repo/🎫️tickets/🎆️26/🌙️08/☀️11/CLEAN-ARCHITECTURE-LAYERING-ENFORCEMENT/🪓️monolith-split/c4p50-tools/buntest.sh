#!/bin/zsh
# usage: buntest.sh <label> <file>... -> G/c4p50-bun-<label>.xml/.log/.set (bun test with junit reporter; set = suite/name/status)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p50-tools/env.sh
LABEL="$1"; shift
cd "$R"
perl -e 'alarm 500; exec @ARGV' -- nice -n 10 bun test --reporter=junit --reporter-outfile="$G/c4p50-bun-$LABEL.xml" "$@" > "$G/c4p50-bun-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/c4p50-bun-$LABEL.log"
python3 "$T/junitset.py" "$G/c4p50-bun-$LABEL.xml" "$G/c4p50-bun-$LABEL.set"
tail -4 "$G/c4p50-bun-$LABEL.log"

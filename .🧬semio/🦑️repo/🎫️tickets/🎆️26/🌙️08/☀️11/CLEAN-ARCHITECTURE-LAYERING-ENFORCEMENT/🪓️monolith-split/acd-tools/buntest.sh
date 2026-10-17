#!/bin/zsh
# usage: buntest.sh <label> <file>... -> G/acd-bun-<label>.xml/.log/.set (bun test with junit reporter; set = suite/name/status)
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/acd-tools/env.sh
LABEL="$1"; shift
cd "$R"
perl -e 'alarm 500; exec @ARGV' -- nice -n 10 bun test --reporter=junit --reporter-outfile="$G/acd-bun-$LABEL.xml" "$@" > "$G/acd-bun-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/acd-bun-$LABEL.log"
python3 "$T/junitset.py" "$G/acd-bun-$LABEL.xml" "$G/acd-bun-$LABEL.set"
tail -4 "$G/acd-bun-$LABEL.log"

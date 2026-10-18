#!/bin/zsh
# usage: buntest.sh <label> <file>... -> G/tso-bun-<label>.xml/.log/.set
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tso-tools/env.sh
LABEL="$1"; shift
cd "$R"
perl -e 'alarm 1200; exec @ARGV' -- nice -n 10 bun test --reporter=junit --reporter-outfile="$G/tso-bun-$LABEL.xml" "$@" > "$G/tso-bun-$LABEL.log" 2>&1
echo "exit=$?" >> "$G/tso-bun-$LABEL.log"
python3 "$T/junitset.py" "$G/tso-bun-$LABEL.xml" "$G/tso-bun-$LABEL.set"
tail -4 "$G/tso-bun-$LABEL.log"

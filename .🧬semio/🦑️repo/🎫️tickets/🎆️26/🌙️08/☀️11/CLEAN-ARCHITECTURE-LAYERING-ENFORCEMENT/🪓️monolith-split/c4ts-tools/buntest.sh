#!/bin/zsh
# usage: buntest.sh <label> <cwd> <test-file>... : bun test (junit) with a caller-owned artifact dir; writes G/c4ts-bun-<label>.log, .xml and G/c4ts-bunset-<label>.txt (sorted "status<TAB>suite<TAB>name")
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4ts-tools/env.sh
label="$1"; cwd="$2"; shift; shift
mkdir -p "$G/c4ts-artifact/$label"
cd "$cwd"
SEMIO_TEST_ARTIFACT_DIR="$G/c4ts-artifact/$label" nice -n 10 perl -e 'alarm 900; exec @ARGV' -- bun test --timeout 120000 --reporter=junit --reporter-outfile="$G/c4ts-bun-$label.xml" "$@" > "$G/c4ts-bun-$label.log" 2>&1
echo "exit=$?" >> "$G/c4ts-bun-$label.log"
python3 "$T/junitset.py" "$G/c4ts-bun-$label.xml" "$G/c4ts-bunset-$label.txt"
tail -4 "$G/c4ts-bun-$label.log"

#!/bin/zsh
# usage: buntest.sh <label> <cwd> <test-file>... : bun test (junit) with a caller-owned artifact dir; writes G/tsx-bun-<label>.log, .xml and G/tsx-bunset-<label>.txt (sorted "status<TAB>suite<TAB>name")
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tsx-tools/env.sh
label="$1"; cwd="$2"; shift; shift
mkdir -p "$G/tsx-artifact/$label"
cd "$cwd"
SEMIO_TEST_ARTIFACT_DIR="$G/tsx-artifact/$label" nice -n 10 perl -e 'alarm 580; exec @ARGV' -- bun test --timeout 120000 --reporter=junit --reporter-outfile="$G/tsx-bun-$label.xml" "$@" > "$G/tsx-bun-$label.log" 2>&1
echo "exit=$?" >> "$G/tsx-bun-$label.log"
python3 "$T/junitset.py" "$G/tsx-bun-$label.xml" "$G/tsx-bunset-$label.txt"
tail -4 "$G/tsx-bun-$label.log"

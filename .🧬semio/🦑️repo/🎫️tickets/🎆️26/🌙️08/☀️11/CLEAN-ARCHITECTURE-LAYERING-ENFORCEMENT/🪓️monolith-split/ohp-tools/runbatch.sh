#!/bin/zsh
# usage: runbatch.sh <label> <fam>...   runs run.sh for each family sequentially
label=$1; shift
for f in "$@"; do /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ohp-tools/run.sh $f $label; done

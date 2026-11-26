#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
repo capacity run --memory=3600 --cpu=2 -- $T/tt1-cargo.sh $1 test -j 2 --no-fail-fast -- "${@:2}"

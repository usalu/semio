#!/bin/zsh
# usage: sweep.sh <list> <out.tsv> [parallel] [cap]
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
tr '\n' '\0' < "$1" | xargs -0 -n1 -P "${3:-6}" "$F/bd11-tools/one.sh" "${4:-150}" > "$2"

#!/bin/zsh
# usage: ctc-fix.sh OWNER FEATURE CONSUMER... : declares FEATURE in OWNER and enables it in each consumer's dev-dependencies
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ctc-tools
owner=$1; feat=$2; shift 2
om=$($T/mf $owner); [[ -z "$om" ]] && { echo "NOOWNER $owner"; exit 1; }
bun $T/ctc-ownerfeat.ts "$om" $feat 2>&1 | tail -1
for c in "$@"; do
  cm=$($T/mf $c); [[ -z "$cm" ]] && { echo "NOCONSUMER $c"; continue; }
  bun $T/ctc-devfeat.ts "$cm" $owner $feat 2>&1 | tail -1
done

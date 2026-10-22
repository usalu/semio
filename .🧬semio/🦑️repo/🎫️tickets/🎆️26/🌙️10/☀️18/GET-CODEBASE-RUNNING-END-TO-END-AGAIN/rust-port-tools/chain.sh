#!/bin/sh
# usage: chain.sh LOGPREFIX  -- checks the rust-port package list (lock refresh first when "update" is passed as $2)
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/rust-port-logs"
[ "$2" = update ] && sh "$C/rust-health-tools/run.sh" "$L/$1-update.log" update --workspace --offline
sh "$C/rust-health-tools/run.sh" "$L/$1.log" check $(cat "$L/pk.txt") --all-targets --keep-going --message-format=short

#!/bin/bash
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1030"
while [ ! -f "$G/mutderive-native.rc" ]; do sleep 20; done
rm -f "$G/mutderive-law.rc"
"$C/1030-tools/mutderive/law-test.sh"
"$C/1030-tools/mutderive/check-hub.sh"
echo done > "$G/mutderive-chain.rc"

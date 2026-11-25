#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
until [ -f "$T/🗑️generated/round6.done" ]; do sleep 20; done
"$T/run-round7-chain.sh" e1992
cp "$T/🗑️generated/round7.done" "$T/🗑️generated/round7a.done"; rm -f "$T/🗑️generated/round7.done"
"$T/run-round7-chain.sh" din

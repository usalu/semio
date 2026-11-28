#!/bin/sh
# usage: publish-base-after-chain.sh   (waits for the running publish-chain.sh, installs publish-chain.sh.next if present, then republishes base and energy and reruns the strict sweep)
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
while pgrep -f "sh $T/publish-chain.sh" >/dev/null || pgrep -f "sh .🧬semio.*publish-chain.sh$" >/dev/null; do sleep 30; done
[ -f "$T/publish-chain.sh.next" ] && mv "$T/publish-chain.sh.next" "$T/publish-chain.sh"
SEMIO_CHAIN_ONLY="base energy sweep" exec sh "$T/publish-chain.sh"

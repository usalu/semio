#!/bin/sh
# usage: resume-chain-after-drain.sh   (waits until the job-context check and test logs carry rc= or 90 min pass, then relaunches publish-chain.sh)
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
i=0
while [ $i -lt 540 ]; do
  grep -qa '^rc=' "$T/🗑️generated/verify-test3.log" 2>/dev/null && grep -qa '^rc=' "$T/🗑️generated/verify-check3.log" 2>/dev/null && break
  i=$((i+1)); sleep 10
done
exec sh "$T/publish-chain.sh"

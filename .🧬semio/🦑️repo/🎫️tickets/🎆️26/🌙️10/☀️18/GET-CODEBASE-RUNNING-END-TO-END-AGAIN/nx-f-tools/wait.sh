#!/bin/sh
# wait.sh <name> [max-iterations-of-10s]: stop waiting at rc= or at an Nx verdict line
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
F="$C/🗑️generated/nx-f/$1.raw"
for i in $(seq 1 ${2:-52}); do
  grep -q "^rc=" "$F" && break
  grep -q -E '^ NX   (Successfully ran|Ran target|Running target .* failed|Failed|Ran)|Run duration:|^ NX  ' "$F" && break
  sleep 10
done
grep -v -E '^\[script\]|nx2 |running elapsedMs|capacity\] waiting|\[nx\] ' "$F" | tail -n ${3:-25} | cut -c1-300

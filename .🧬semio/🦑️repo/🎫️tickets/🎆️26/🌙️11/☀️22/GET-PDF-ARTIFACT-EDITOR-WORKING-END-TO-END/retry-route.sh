#!/bin/sh
# usage: retry-route.sh <log prefix> <test filter> [attempts]  (slice 12: reruns one registered real-world route until it compiles and finishes; logs <prefix>N in 🗑️generated)
cd "$(dirname "$0")"
p=$1; f=$2; tries=${3:-6}
for n in $(seq 1 $tries); do
  rm -f 🗑️generated/$p$n.done
  sh ./run-s4.sh $p$n test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- $f --test-threads=1 --nocapture
  grep -aq "test result" 🗑️generated/$p$n.log && break
  sleep 300
done

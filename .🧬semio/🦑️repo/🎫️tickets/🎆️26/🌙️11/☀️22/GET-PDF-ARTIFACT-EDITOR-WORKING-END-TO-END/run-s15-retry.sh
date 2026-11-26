#!/bin/sh
# usage: run-s15-retry.sh <log prefix> <cargo test args..>  (slice 15: reruns one gated test run while the compiler is killed without a diagnostic; logs <prefix>N in 🗑️generated)
cd "$(dirname "$0")"
p=$1; shift
for n in 1 2 3 4; do
  sh ./run-s15.sh $p$n "$@"
  grep -aq "test result\|error\[E" 🗑️generated/$p$n.log && break
  sleep 120
done
echo "exit" > 🗑️generated/$p.done

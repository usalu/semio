#!/bin/sh
# usage: run-s16-thesis.sh  (slice 16: the thesis route, retried every 10 minutes while a foreign manifest or crate does not build; logs s16t-* in 🗑️generated)
cd "$(dirname "$0")"
rm -f 🗑️generated/s16t-all.done
for n in 1 2 3 4 5 6 7 8 9 10 11 12; do
  sh ./run-s4.sh s16t$n test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- registered_bachelor_thesis --test-threads=1 --nocapture
  grep -aq "could not compile\|cyclic package dependency\|failed to load manifest" 🗑️generated/s16t$n.log || break
  sleep 600
done
echo finished > 🗑️generated/s16t-all.done

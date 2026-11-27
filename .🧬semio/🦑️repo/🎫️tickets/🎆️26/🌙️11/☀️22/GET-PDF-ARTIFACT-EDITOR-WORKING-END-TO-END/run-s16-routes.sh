#!/bin/sh
# usage: run-s16-routes.sh  (slice 16: institution then thesis route, each retried every 10 minutes while a foreign crate does not compile; logs s16r-* in 🗑️generated)
cd "$(dirname "$0")"
rm -f 🗑️generated/s16r-all.done
for route in "s16r-inst registered_real_world_institution" "s16r-thesis registered_bachelor_thesis"; do
  set -- $route
  for n in 1 2 3 4 5 6 7 8 9 10 11 12; do
    sh ./run-s4.sh $1$n test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- $2 --test-threads=1 --nocapture
    grep -aq "could not compile" 🗑️generated/$1$n.log || break
    sleep 600
  done
done
echo finished > 🗑️generated/s16r-all.done

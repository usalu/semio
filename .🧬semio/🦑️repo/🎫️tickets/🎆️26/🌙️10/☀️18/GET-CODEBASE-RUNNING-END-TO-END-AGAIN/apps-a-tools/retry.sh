#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
log="$1"; shift
for t in 1 2 3 4 5 6 7 8 9 10; do
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$log" 2>&1; rc=$?
  [ $rc = 0 ] && exit 0
  /usr/bin/grep -qE 'could not compile|error(\[E|: aborting due)|^error: .* due to [0-9]+ previous' "$log" && exit $rc
  sleep 45
done
exit $rc

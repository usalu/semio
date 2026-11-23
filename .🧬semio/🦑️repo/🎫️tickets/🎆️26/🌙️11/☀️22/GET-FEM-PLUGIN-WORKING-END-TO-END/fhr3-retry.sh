#!/usr/bin/env bash
# usage: fhr3-retry.sh <2d|3d> <tag> <test-target|--lib> [filter...]  reruns until the log holds test results or the failure is not a foreign compile break
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
dim="$1"; tag="$2"
for n in 1 2 3 4 5 6 7 8; do
  "$T/fhr3-test.sh" "$@"
  log="$T/🗑️generated/fhr3-$dim-$tag.log"
  if grep -q "^test result\|^running " "$log"; then break; fi
  if ! grep -q "could not compile" "$log"; then break; fi
  cp "$log" "$T/🗑️generated/fhr3-$dim-$tag.try$n.log"
  sleep 240
done
echo "retry-done" >> "$T/🗑️generated/fhr3-$dim-$tag.log"

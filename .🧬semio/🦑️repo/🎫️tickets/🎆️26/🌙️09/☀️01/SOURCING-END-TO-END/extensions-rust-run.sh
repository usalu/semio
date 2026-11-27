#!/bin/sh
# Host-target check + test of the sourcing extensions, oracle and hub composition, one build at a time.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; export OUT="$T/🗑️generated"
N="$T/native-cargo.sh"
for e in beams slabs windows oracle hub; do
  case $e in
  beams) d="✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams";; slabs) d="✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs";; windows) d="✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows";;
  oracle) d="✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧪️tests/🔬️unit/🔮️oracle/📦️packages/🦀️rust";; hub) d="🌎️hub/🧩️compositions/🪵️sourcing/📦️packages/🦀️rust";;
  esac
  l=ext-$e
  sh "$N" "$l-test" "$d" test --all-targets -j 4 -- --test-threads=1
done
echo finished > "$OUT/ext-finished"

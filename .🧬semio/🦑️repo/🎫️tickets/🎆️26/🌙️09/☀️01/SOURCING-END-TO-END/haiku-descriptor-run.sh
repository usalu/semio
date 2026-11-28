#!/bin/sh
# 🧪 Sequential native-or-repo test runs for the descriptor freshness check (one build at a time).
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; OUT="$T/🗑️generated"
S="✏️s/🔌️plugins/🪵️sourcing/🧩️extensions"
run() {
  crate=$1; ws=$2
  OUT="$OUT" sh "$T/native-cargo.sh" "haiku-descriptor-$crate" "$ws" test -p "$crate" -j 4 > "$OUT/haiku-descriptor-$crate.status" 2>&1
  mv "$OUT/haiku-descriptor-$crate.txt" "$OUT/haiku-descriptor-$crate.log"
  echo "$crate $(cat "$OUT/haiku-descriptor-$crate.status")" >> "$OUT/haiku-descriptor-driver.log"
}
run semio-s-plugin-sourcing-beams-component "$S/🪵️beams/🧩️component/📦️packages/🦀️rust"
run semio-s-plugin-sourcing-slabs-component "$S/🧱️slabs/🧩️component/📦️packages/🦀️rust"
run semio-s-plugin-sourcing-windows-component "$S/🪟️windows/🧩️component/📦️packages/🦀️rust"
run semio-hub-sourcing-component "🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/📦️packages/🦀️rust"
echo finished >> "$OUT/haiku-descriptor-driver.log"

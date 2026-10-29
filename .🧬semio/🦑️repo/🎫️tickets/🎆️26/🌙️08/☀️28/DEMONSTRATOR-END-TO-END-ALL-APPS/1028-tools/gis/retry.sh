#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028"
for n in $(seq "$1" "$(( $1 + 9 ))"); do
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml" --target wasm32-wasip2 --keep-going -p semio-hub-gis-component > "$C/gis-chk$n.txt" 2>&1
  rc=$?
  if /usr/bin/grep -q "^error\[E\|could not compile\|Finished" "$C/gis-chk$n.txt" || [ $rc -eq 0 ]; then echo $n > "$C/gis-final.n"; echo $rc > "$C/gis-final.rc"; exit; fi
  sleep 20
done
echo 0 > "$C/gis-final.n"; echo 99 > "$C/gis-final.rc"

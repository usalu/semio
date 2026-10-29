#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
wait_space(){ while [ "$(df -k '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m' | tail -1 | awk '{print $4}')" -lt 6000000 ]; do sleep 60; done; }
for attempt in 1 2 3 4; do
  wait_space
  rm -f "$C/🗑️generated/1029/semiodsl-chk$N.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path '🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml' --target wasm32-wasip2 --keep-going -p semio-hub-energy-component > "$C/🗑️generated/1029/semiodsl-chk$N.txt" 2>&1
  echo $? > "$C/🗑️generated/1029/semiodsl-chk$N.rc"
  grep -qE 'No space left on device|ENOSPC|compiler image (ran out|is still full)' "$C/🗑️generated/1029/semiodsl-chk$N.txt" || break
  cp "$C/🗑️generated/1029/semiodsl-chk$N.txt" "$C/🗑️generated/1029/semiodsl-chk$N.enospc$attempt.txt"; sleep 300
done

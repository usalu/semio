#!/bin/bash
# usage: retry.sh N [cargo args]  -- hub check, retried while the log shows ENOSPC (compiler image saturated by the fleet); max 12 attempts
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
M=".🧬semio/🦑️repo/⚡️cache/rust/fixed/m"
rm -f "$C/🗑️generated/1029/energy-chk$N.rc" "$C/🗑️generated/1029/energy-chk$N.done"
for attempt in $(seq 1 12); do
  while :; do a=$(df -g "$M" 2>/dev/null | tail -1 | awk '{print $4}'); [ "${a:-0}" -ge 14 ] && break; sleep 10; done
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path '🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml' --target wasm32-wasip2 --keep-going -p semio-hub-energy-component "$@" > "$C/🗑️generated/1029/energy-chk$N.txt" 2>&1
  rc=$?
  if /usr/bin/grep -q "ENOSPC\|No space left on device" "$C/🗑️generated/1029/energy-chk$N.txt" && ! /usr/bin/grep -q "^error\[E" "$C/🗑️generated/1029/energy-chk$N.txt"; then cp "$C/🗑️generated/1029/energy-chk$N.txt" "$C/🗑️generated/1029/energy-chk$N.enospc$attempt.txt"; sleep 30; continue; fi
  echo $rc > "$C/🗑️generated/1029/energy-chk$N.rc"; exit 0
done
echo 99 > "$C/🗑️generated/1029/energy-chk$N.rc"

#!/bin/bash
# usage: chk.sh N [extra cargo args...]
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
rm -f "$C/🗑️generated/1029/energy-chk$N.rc"
nohup bash -c "SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path '🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml' --target wasm32-wasip2 --keep-going -p semio-hub-energy-component $* > '$C/🗑️generated/1029/energy-chk$N.txt' 2>&1; echo \$? > '$C/🗑️generated/1029/energy-chk$N.rc'" >/dev/null 2>&1 &
echo $! > "$C/🗑️generated/1029/energy-chk$N.pid"

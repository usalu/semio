#!/bin/bash
# usage: mchk.sh N crate [crate...]  -> native check in the energy model workspace
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
P=""; for c in "$@"; do P="$P -p $c"; done
rm -f "$C/🗑️generated/1029/energy-m$N.rc"
nohup bash -c "SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path '✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml' --keep-going $P > '$C/🗑️generated/1029/energy-m$N.txt' 2>&1; echo \$? > '$C/🗑️generated/1029/energy-m$N.rc'" >/dev/null 2>&1 &
echo $! > "$C/🗑️generated/1029/energy-m$N.pid"

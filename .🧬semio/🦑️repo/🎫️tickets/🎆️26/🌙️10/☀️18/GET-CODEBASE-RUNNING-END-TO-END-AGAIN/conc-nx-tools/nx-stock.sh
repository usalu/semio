#!/bin/sh
label=$1; shift
cd /private/tmp/conc-nx/fx
s=$(date +%s)
NX_DAEMON=false NX_TUI=false NX_NO_CLOUD=true NX_ISOLATE_PLUGINS=false node node_modules/nx/dist/bin/nx.js "$@" --output-style=static > /private/tmp/conc-nx/work/$label.log 2>&1
echo "$label exit=$? seconds=$(( $(date +%s) - s ))"

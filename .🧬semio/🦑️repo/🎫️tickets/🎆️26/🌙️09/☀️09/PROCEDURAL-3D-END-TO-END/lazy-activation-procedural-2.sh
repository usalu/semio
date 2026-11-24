#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
TF=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
until grep -q "end semio-hub-flow-component" "$TF/🗑️generated/lazy-activation-components.txt"; do sleep 20; done
echo "$(date +%T) start procedural-2" >> "$TF/🗑️generated/lazy-activation-components.txt"
repo run semio-hub-procedural-component:component -c prototype --exclude-dependencies > "$TF/🗑️generated/lazy-activation-procedural-2.txt" 2>&1
echo "$(date +%T) end procedural-2 exit $?" >> "$TF/🗑️generated/lazy-activation-components.txt"

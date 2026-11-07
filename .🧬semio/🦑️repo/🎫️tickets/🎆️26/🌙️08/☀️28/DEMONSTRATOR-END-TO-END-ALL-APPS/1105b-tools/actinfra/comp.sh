#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1105b/actinfra-comp"; rm -f "$O.rc"
"$C/1105b-tools/slot.sh" W-actinfra-comp env SEMIO_LANGUAGE=en repo run semio-s-plugin-flow-list-component:component --configuration=prototype --output=stream > "$O.txt" 2>&1
echo $? > "$O.rc"

#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1030"
M='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📦️packages/🦀️rust/Cargo.toml'
rm -f "$O"/stdiofix-t5.*
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-stdio-semio --lib -- record_pack_binary > "$O/stdiofix-t5.txt" 2>&1
echo $? > "$O/stdiofix-t5.rc"

#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1030"
M='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📦️packages/🦀️rust/Cargo.toml'
rm -f "$O"/stdiofix-t4.*
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-stdio-semio -p semio-s-artifact-stdio-semio-sqlite-laws-1 -p semio-s-artifact-stdio-semio-sqlite-laws-2 -p semio-s-artifact-stdio-semio-sqlite-laws-3 -p semio-s-artifact-stdio-semio-sqlite-laws-4 -p semio-s-artifact-stdio-semio-sqlite-laws-5 --lib --tests --no-fail-fast -- record_pack_binary last_snapshot shared_snapshot sqlite_snapshot_semio > "$O/stdiofix-t4.txt" 2>&1
echo $? > "$O/stdiofix-t4.rc"

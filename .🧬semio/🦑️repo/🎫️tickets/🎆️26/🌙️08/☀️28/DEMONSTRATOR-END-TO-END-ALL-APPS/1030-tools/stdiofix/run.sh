#!/bin/bash
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
O="$C/🗑️generated/1030"
M='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📦️packages/🦀️rust/Cargo.toml'
rm -f "$O"/stdiofix-*.rc
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-stdio-semio --lib --no-fail-fast -- record_pack_binary last_snapshot shared_snapshot > "$O/stdiofix-t1.txt" 2>&1
echo $? > "$O/stdiofix-t1.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-stdio-semio --test record_spec_law > "$O/stdiofix-spec.txt" 2>&1
echo $? > "$O/stdiofix-spec.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-stdio-semio-brep -p semio-s-artifact-stdio-semio-table --lib --no-fail-fast -- sqlite_snapshot_semio > "$O/stdiofix-t3.txt" 2>&1
echo $? > "$O/stdiofix-t3.rc"

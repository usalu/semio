#!/bin/bash
# usage: spec.sh <n> [write]   (runs the record_spec_law test; "write" prints RECORD-SPEC lines instead of asserting)
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
M='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📦️packages/🦀️rust/Cargo.toml'
rm -f "$C/🗑️generated/1029/semiocodec-spec$N.rc"
if [ "$2" = "write" ]; then export SEMIO_RECORD_SPEC_WRITE=1; fi
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-s-artifact-stdio-semio --test record_spec_law -- --nocapture > "$C/🗑️generated/1029/semiocodec-spec$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/semiocodec-spec$N.rc"

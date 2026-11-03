#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1102"; S="$C/1102-tools/slot.sh"
while [ ! -f $G/b3c-energy.rc ]; do sleep 20; done
rm -f $G/b3e-*.rc
SEMIO_LANGUAGE=en $S W-b3e-law repo cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml -p semio-framework-plugin-tests-04-2 -- registered_construction_publishes > $G/b3e-law.txt 2>&1; echo $? > $G/b3e-law.rc

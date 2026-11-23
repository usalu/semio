#!/bin/bash
set -u
cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991
REGEN_EN1991_ASSETS=1 repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1991-examples --offline -- regen_example_assets_when_env_set --nocapture > "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/en1991-regen.log" 2>&1
echo "[DEBUG] en1991 exit $?" >> "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/en1991-regen.log"

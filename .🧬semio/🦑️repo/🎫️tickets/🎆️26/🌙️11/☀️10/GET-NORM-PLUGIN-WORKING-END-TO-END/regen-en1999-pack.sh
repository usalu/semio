#!/bin/bash
set -u
cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999
REGEN_EN1999_ASSETS=1 repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1999 --offline -- regen_example_assets_when_env_set --nocapture > /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/regen-en1999-pack.log 2>&1
echo "[DEBUG] regen exit $?" >> /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/regen-en1999-pack.log

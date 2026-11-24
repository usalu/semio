#!/usr/bin/env bash
# 🔗️ Run the serde-schema law checks one cargo at a time.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"; A="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
while pgrep -f "repo cargo.*norm-din16798 --offline" >/dev/null; do sleep 10; done
(cd "$A/🌍️en1997/📦️packages/🦀️rust" && repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1997 --offline --lib example_snapshot_validates) > "$T/🗑️generated/chain-en1997.log" 2>&1
(cd "$A/🔩️en1993/📦️packages/🦀️rust" && repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1993 --offline --lib python_oracle_and_jsonschema) > "$T/🗑️generated/chain-en1993.log" 2>&1
"$T/test-family-examples.sh" 🪶️en1999 🌬️din16798 🌍️en1997 🔩️en1993 > "$T/🗑️generated/chain-examples.log" 2>&1
echo done > "$T/🗑️generated/chain-done"

#!/bin/zsh
OV=/private/tmp/gk/ov; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; D="$OV/$L/🏗️build/📥️inputs/🦀️resolution"
mkdir -p "$D/🧬️schema" "$D/🧪️tests/🧬️schema" "$D/🧪️tests/🐍️oracle"
cp /private/tmp/gk/resolution.ts "$D/🟦️.ts"; cp /private/tmp/gk/schema.json "$D/🧬️schema/🔣️.json"; cp /private/tmp/gk/policy.json "$D/🔣️.json"
[ -f /private/tmp/gk/test.ts ] && cp /private/tmp/gk/test.ts "$D/🧪️tests/🟦️.ts"
[ -f /private/tmp/gk/fixture.json ] && cp /private/tmp/gk/fixture.json "$D/🧪️tests/🔣️.json"
[ -f /private/tmp/gk/fixture-schema.json ] && cp /private/tmp/gk/fixture-schema.json "$D/🧪️tests/🧬️schema/🔣️.json"
[ -f /private/tmp/gk/oracle.py ] && cp /private/tmp/gk/oracle.py "$D/🧪️tests/🐍️oracle/🐍️.py"
echo synced

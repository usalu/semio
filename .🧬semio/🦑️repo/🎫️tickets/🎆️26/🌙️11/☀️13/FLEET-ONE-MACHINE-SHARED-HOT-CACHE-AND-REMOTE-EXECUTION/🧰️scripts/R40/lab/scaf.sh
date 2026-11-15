#!/bin/zsh
ov=$1; L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
cp /Users/Shared/semio/MODULE.bazel /Users/Shared/semio/📋️build-owners.json $ov/
mkdir -p $ov/node_modules/@iarna $ov/$L/📦️packages/🟦️typescript $ov/$L/🏗️build/📥️inputs/🧮️digest/📥️read
cp -R /Users/Shared/semio/node_modules/@iarna/toml $ov/node_modules/@iarna/
cp "/Users/Shared/semio/$L/📦️packages/🟦️typescript/📋️project.json" "$ov/$L/📦️packages/🟦️typescript/"
cp -Rn "/Users/Shared/semio/$L/🏗️build/📥️inputs/🧮️digest/📥️read/🧬️schema" "$ov/$L/🏗️build/📥️inputs/🧮️digest/📥️read/"

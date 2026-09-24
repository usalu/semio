#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t2/target CARGO_INCREMENTAL=0
for o in 🔗️graphql 🪝️hooks 🎫️tickets ⌨️cli 📐️model 🧩️providers 🗣️languages 🎛️dashboard 🔌️mcp; do
  bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" parity exhaustive --owner "🧰️framework/🛍️products/🦑️repo/🔨️modules/$o" > ".tmp-ticket/wp-t2/generated/parity-$o.txt" 2>&1
  echo "done $o" >> .tmp-ticket/wp-t2/generated/parity-owners.log
done
echo ALLDONE >> .tmp-ticket/wp-t2/generated/parity-owners.log

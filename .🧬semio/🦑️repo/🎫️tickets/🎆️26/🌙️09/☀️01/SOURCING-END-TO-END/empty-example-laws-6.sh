#!/bin/zsh
# 🧪️ The remodeling law is mounted in the remodeling package's `editor` test target.
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
until grep -q "^done5" $T/🗑️generated/verify/scoped-all5.txt; do sleep 20; done
$T/verify-cargo.sh scoped-remodel-editor "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling" test -p semio-s-artifact-remodel-remodeling --test editor empty
echo done6

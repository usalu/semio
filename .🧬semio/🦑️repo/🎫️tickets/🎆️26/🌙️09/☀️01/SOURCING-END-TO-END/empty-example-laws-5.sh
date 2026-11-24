#!/bin/zsh
# 🧪️ The note law is mirrored into the note v1 package.
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
until grep -q "^done4" $T/🗑️generated/verify/scoped-all4.txt; do sleep 20; done
$T/verify-cargo.sh scoped-note "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note" test -p semio-s-artifact-note-note-v1 --lib empty
echo done5

#!/bin/zsh
# 🧪️ Reruns rewriting and note empty-example laws after the first sequential pass.
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
until grep -q "^done" $T/🗑️generated/verify/empty-all.txt; do sleep 20; done
JOBS=4 $T/verify-cargo.sh empty-rewriting "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting" test --lib set_active_example
JOBS=4 $T/verify-cargo.sh empty-note "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note" test --lib set_active_example
echo done2

#!/bin/zsh
# 🧪️ Runs the empty-example laws per crate sequentially (shared target dir).
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; P=$R/✏️s/🔌️plugins
while pgrep -f "verify-cargo.sh note-ex" >/dev/null; do sleep 10; done
for spec in "rewriting|🔱️trinity/🗿️artifacts/♻️rewriting|set_active_example" "jack|🔱️trinity/🗿️artifacts/🔌️jack|set_active_example" "remodel|📸️remodel/🗿️artifacts/📸️remodeling|set_active_example" "raster|🖨️raster/🗿️artifacts/🖨️raster|empty_id" "block2d|🧱️block/🗿️artifacts/◻️2d|set_active_example" "block3d|🧱️block/🗿️artifacts/🧊️3d|set_active_example" "block5d|🧱️block/🗿️artifacts/🖐️5d|set_active_example"; do
  IFS='|' read l d f <<< "$spec"; JOBS=4 $T/verify-cargo.sh "empty-$l" "✏️s/🔌️plugins/$d" test --lib $f
done
echo done

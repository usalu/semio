#!/bin/zsh
# 🧪️ Runs each empty-example law scoped to its owning package, so unrelated stale crates in the workspace do not break the build.
R=/Users/ueli/Documents/semio; T="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; P=✏️s/🔌️plugins
for spec in "rewriting|$P/🔱️trinity/🗿️artifacts/♻️rewriting|semio-s-artifact-trinity-rewriting-editor" "remodel|$P/📸️remodel/🗿️artifacts/📸️remodeling|semio-s-artifact-remodel-remodeling-editor" "block2d|$P/🧱️block/🗿️artifacts/◻️2d|semio-s-artifact-block-2d-editor" "block3d|$P/🧱️block/🗿️artifacts/🧊️3d|semio-s-artifact-block-3d-editor" "block5d|$P/🧱️block/🗿️artifacts/🖐️5d|semio-s-artifact-block-5d-editor" "note|$P/🗒️note/🗿️artifacts/🗒️note|semio-s-artifact-note-note-unit-tests"; do
  IFS='|' read l d k <<< "$spec"; $T/verify-cargo.sh "scoped-$l" "$d" test -p $k --lib empty
done
echo done3

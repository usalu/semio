#!/bin/sh
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out="$here/🗑️generated/onebased"
mkdir -p "$out"
export PATH="$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH"
sh "$here/native-cargo.sh" test -p semio-s-artifact-stdio-pdf-page-editor-authoring-tests --lib > "$out/authoring.txt" 2>&1
sh "$here/native-cargo.sh" test -p semio-s-artifact-stdio-pdf-presence-lane --lib > "$out/presence.txt" 2>&1
sh "$here/native-cargo.sh" test -p semio-s-artifact-stdio-pdf14-canvas --lib > "$out/pdf14-canvas.txt" 2>&1
echo finished > "$out/done"

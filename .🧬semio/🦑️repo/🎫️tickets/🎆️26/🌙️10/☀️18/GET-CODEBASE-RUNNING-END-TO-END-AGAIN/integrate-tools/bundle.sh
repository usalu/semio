#!/bin/bash
# usage: bundle.sh <overlay-root> <entry-rel> [<out.js>]   bundles an entry with bun inside the overlay; bare packages stay external (they resolve through node_modules, no set touches them); every relative import must resolve.
ov="$1"; entry="$2"; out="${3:-/private/tmp/ig/bundle.js}"
cd "$ov" || exit 2
bun build --target=bun --packages=external "./$entry" --outfile="$out" 2>&1 | tail -${LINES_TAIL:-30}
exit ${PIPESTATUS[0]}

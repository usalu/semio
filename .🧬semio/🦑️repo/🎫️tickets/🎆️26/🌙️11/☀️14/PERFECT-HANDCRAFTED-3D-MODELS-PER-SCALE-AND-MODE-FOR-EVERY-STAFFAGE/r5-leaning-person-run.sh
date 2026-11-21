#!/bin/sh
# 🔁 Rebuilds the candidate of humans/leaning-person: poses fitted to the wall, volumes source, 2D shapes, candidate document, validation.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-leaning-person-roots.ts > /dev/null && "$BUN" r5-leaning-person-build.ts "$@" > /dev/null && "$BUN" r3-leaning-person-roots.ts --fit | grep "fit" && "$BUN" r5-leaning-person-build.ts "$@" | tail -4
"$BUN" r3-leaning-person-draw2d.ts ${DRAW:-} | grep -v "^ *\[DEBUG\] .*residual 0.000"
"$BUN" r3-leaning-person-roots.ts --with-volumes > /dev/null && "$BUN" r3-validate.ts --file "🗑️generated/r3-leaning-person/🔣️.json" | tail -6

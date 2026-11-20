#!/bin/sh
# 🔁 Rebuilds the volumes source and the candidate document of humans/runner-with-blade-prosthesis, then renders the requested sheets: `sh r3-runner-run.sh [sheet args...]`.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-runner-figure.ts && "$BUN" r3-runner-roots.ts --volumes > /dev/null || exit 1
if [ $# -gt 0 ]; then "$BUN" r3-runner-sheet.ts "$@"; fi

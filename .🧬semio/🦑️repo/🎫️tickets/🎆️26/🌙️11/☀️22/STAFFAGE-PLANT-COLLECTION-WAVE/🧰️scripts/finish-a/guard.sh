#!/bin/sh
# 🛡️ Finisher A guard run: the catalog silhouette measures of the given catalog plants (or of one file with `--file <json>`), one tool process at a time.
# usage: guard.sh <id>... | guard.sh --file <asset document json> [scales]
HERE="$(cd "$(dirname "$0")" && pwd)"
S="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
if [ "$1" = "--file" ]; then "$BUN" "$HERE/guard.ts" "$2" $3; exit; fi
for id in "$@"; do "$BUN" "$HERE/guard.ts" "$S/🗂️catalog/🌳️vegetation/$id/🔣️.json" | grep -e "^!" -e "!" -e GUARD; done

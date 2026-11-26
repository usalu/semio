#!/bin/zsh
# 📏️ The guard lines of one scale for several copies inside one grant: `guard50.sh <scale> <file.json>…` prints the line of `../MR/measure.ts --scales <scale>` per file.
source "${0:A:h}/env.sh"
s="$1"; shift; cd "$S/📦️packages/🟦️typescript"
for f in "$@"; do echo "${f:h:t}/${f:t} | $(timeout 900 "$B" "$M/measure.ts" "$f" --scales "$s" 2>&1 | grep bbox | sed 's/.*bbox [^ ]* [^ ]* [^ ]* *//' | cut -c1-200)"; done

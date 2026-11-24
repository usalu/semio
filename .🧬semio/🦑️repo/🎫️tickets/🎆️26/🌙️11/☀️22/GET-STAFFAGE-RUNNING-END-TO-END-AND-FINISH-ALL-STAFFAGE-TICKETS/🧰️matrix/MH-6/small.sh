#!/bin/zsh
# 🔍️ Renders the sheet of an asset (look.sh of MR) and writes a copy 1200 px wide beside it for a quick look: small.sh <id> <scale>…   (TAG, POSE as in look.sh)
export BATCH=MH-6; source "${0:A:h}/../MR/env.sh"
a="$1"; shift
for s in "$@"; do f="$("$M/look.sh" "$a" "$s" | tail -1)"; [[ -f "$f" ]] && sips -Z 1200 "$f" --out "${f%.png}.small.png" >/dev/null 2>&1 && echo "${f%.png}.small.png" || echo "$f"; done

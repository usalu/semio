#!/bin/zsh
# 📏️ `shift.ts` of backup against the loft trial copy of several assets in one grant: shift.sh <id>… (MM=<limit>)
BATCH=MA source "${0:A:h}/../MR/env.sh"; cd "$T"
for a in "$@"; do echo "== $a"; "$B" "$W/shift.ts" "$GEN/backup/$a.json" "$GEN/loft/$a.json" --mm ${MM:-6} 2>&1 | cut -c1-170; done

#!/bin/zsh
# 🗺️ `where.ts` of backup against the loft trial copy: where.sh <id> <scale>…   (CELL, MIN in mm)
BATCH=MA source "${0:A:h}/../MR/env.sh"; cd "$T"; a="$1"; shift
for s in "$@"; do "$B" "$W/where.ts" "$GEN/backup/$a.json" "$GEN/loft/$a.json" --scale $s --cell ${CELL:-2} --min ${MIN:-12} 2>&1 | cut -c1-260; done

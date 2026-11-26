#!/bin/zsh
# 🧭️ Where the guard sees its distance on a copy, inside one grant: `where.sh <file.json> <scale>:<view>…` runs `../MX-3/guard-where.ts` per item (cells of CELL metres, default 0.01) and prints its last lines.
source "${0:A:h}/env.sh"
f="$1"; shift; cd "$T"
for item in "$@"; do echo "-- where 1:${item%%:*} ${item#*:}"; timeout 1500 "$B" "$U/🧰️matrix/MX-3/guard-where.ts" "$f" "${item%%:*}" "${item#*:}" ${CELL:-0.01} 2>&1 | tail -${TAIL:-14} | cut -c1-240; done

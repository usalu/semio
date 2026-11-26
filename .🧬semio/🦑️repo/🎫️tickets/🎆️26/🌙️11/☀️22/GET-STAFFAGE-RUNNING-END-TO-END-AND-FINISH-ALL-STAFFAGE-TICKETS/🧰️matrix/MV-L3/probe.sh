#!/bin/zsh
# 🔬️ Probes of round MV-L3 inside one grant: `probe.sh <file.json> <scale>:<view>:<pose>…` prints the pieces of `pieces.ts` per sheet.
source "${0:A:h}/env.sh"
f="$1"; shift; cd "$T"
for item in "$@"; do p=("${(@s/:/)item}"); echo "== 1:${p[1]} ${p[2]} ${p[3]}"; timeout 900 "$B" "$W/pieces.ts" "$f" "${p[1]}" "${p[2]}" "${p[3]}" ${=PIECES} 2>&1 | cut -c1-300; done

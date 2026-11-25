#!/bin/zsh
# 🪴️ The multiview repair of `hosta-clump`, the last step of its chain (after `🧰️silhouette/R4-P/make/hosta-clump.sh`, whose slabs of 60 mm it replaces by thinner ones): blades, petioles and shoot necks are drawn again from the body of their scale with the redraw tool of R4-P, in slabs of 40 mm at 1:50 and 1:100 and of 30 mm at 1:200 (the thickest that keep the three views within the tolerance of 19.7 mm in every pose, clip instant and transition; see the report), and the decision texts name that depth (`texts/hosta-clump.json`). Idempotent. make-hosta-clump.sh [--in file] [--out file]
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
cd "$S/📦️packages/🟦️typescript"
timeout 2400 "$B" "$U/🧰️silhouette/R4-P/redraw.ts" hosta-clump --slab 0.04 --scales 50,100 "$@" | tail -1 | cut -c1-200
[[ "$*" == *--out* ]] && set -- --in "${@[-1]}" --out "${@[-1]}"
timeout 2400 "$B" "$U/🧰️silhouette/R4-P/redraw.ts" hosta-clump --slab 0.03 --scales 200 "$@" | tail -1 | cut -c1-200
timeout 600 "$B" "$W/texts.ts" hosta-clump "$@" | cut -c1-200

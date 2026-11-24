#!/bin/zsh
# 🌴️ The last step of the chain of `date-palm` (after `author-01-date-palm.ts`, `author-53-palm.ts`, `digital-date-palm-rachis.py` of the 3D ticket, the `datepalm-*` fixes of the silhouette-guard ticket and the frond ends of batch SV-1): stem, collar, boot, spear leaves, fronds and bunches of every scale are drawn from the body of their scale (redraw.ts, slabs of 0.5 m), and the decision texts say so. Idempotent. make/date-palm.sh [--in file] [--out file]
source "${0:A:h:h}/env.sh"
"$W/t.sh" redraw.ts date-palm --slab 0.5 "$@" | cut -c1-200
[[ "$*" == *--out* ]] && set -- --in "${@[-1]}" --out "${@[-1]}"
"$W/t.sh" texts.ts date-palm "$@" | cut -c1-200

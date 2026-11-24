#!/bin/zsh
# 🪴️ The last step of the chain of `hosta-clump` (after `author-01-hosta-clump.ts`, `author-54-hosta*.ts`, `digital-hosta-*.py|ts` of the 3D ticket and the fixes of the silhouette-guard ticket): blades, petioles and shoot necks of 1:50, 1:100 and 1:200 are drawn from the body of their scale (redraw.ts, slabs of 60 mm: the thickest that keeps the three views one object in every pose and clip instant), and the decision texts say so. Idempotent. make/hosta-clump.sh [--in file] [--out file]
source "${0:A:h:h}/env.sh"
"$W/t.sh" redraw.ts hosta-clump --slab 0.06 "$@" | cut -c1-200
[[ "$*" == *--out* ]] && set -- --in "${@[-1]}" --out "${@[-1]}"
"$W/t.sh" texts.ts hosta-clump "$@" | cut -c1-200

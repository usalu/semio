#!/bin/zsh
# 🌵️ The last step of the chain of `agave` (after `author-01-agave.ts`, `author-54-agave*.ts`, `agave-botanical-fit.ts` of the 3D ticket, the fixes of the silhouette-guard ticket and the declared dimensions of batch SV-1): leaves, spear and stem of every scale are drawn from the body of their scale (redraw.ts, slabs of 0.3 m), and the decision texts say so. Idempotent. make/agave.sh [--in file] [--out file]
source "${0:A:h:h}/env.sh"
"$W/t.sh" redraw.ts agave --slab 0.3 "$@" | cut -c1-200
[[ "$*" == *--out* ]] && set -- --in "${@[-1]}" --out "${@[-1]}"
"$W/t.sh" texts.ts agave "$@" | cut -c1-200

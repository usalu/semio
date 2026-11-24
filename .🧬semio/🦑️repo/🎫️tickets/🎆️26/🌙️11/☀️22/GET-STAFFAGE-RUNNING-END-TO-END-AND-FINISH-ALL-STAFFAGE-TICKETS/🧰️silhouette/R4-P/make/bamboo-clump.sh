#!/bin/zsh
# 🎋️ The last step of the chain of `bamboo-clump` (after `author-01-bamboo-clump.ts`, `author-54-bamboo-*.ts`, `bamboo-botanical-fit.ts` of the 3D ticket, the fixes of the silhouette-guard ticket and the declared dimensions of batch SV-1): at every scale the foliage of each bone and light (sunlit, shaded) is one flat outline drawn from the leaf, spray, tuft or plume solids of that scale, culms, branches and branchlets are strokes along the centre lines of their slender solids at their true width (at least the 0.13 mm pen), the bed and the barrier of 1:50 and 1:100 are drawn from their solids (at 1:200 the barrier stays the outline it was: its 36 mm are below the print minimum as a filled shape), the hairlines between touching leaves are closed as the guard closes them, and the decision texts say so. Idempotent. make/bamboo-clump.sh [--in file] [--out file]
source "${0:A:h:h}/env.sh"
args=("$@")
run() { "$W/t.sh" "$@" "${args[@]}" | grep -vE "^bamboo-clump 1:" | cut -c1-200; [[ "${args[*]}" == *--out* ]] && args=(--in "${args[-1]}" --out "${args[-1]}"); }
T_TIMEOUT=3400 run redraw.ts bamboo-clump --tones --strokes --flat --tol 0 --digits 8 --scales 50,100,500
T_TIMEOUT=3400 run redraw.ts bamboo-clump --tones --strokes --flat --tol 0 --digits 8 --scales 200 --skip '^(barrier|bed)$'
run seal.ts bamboo-clump
run texts.ts bamboo-clump

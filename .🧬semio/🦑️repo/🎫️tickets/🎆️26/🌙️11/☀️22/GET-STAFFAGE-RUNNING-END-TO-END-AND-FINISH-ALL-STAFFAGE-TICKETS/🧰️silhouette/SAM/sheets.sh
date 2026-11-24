#!/bin/zsh
# 👀️ One picture per asset to look at: the solid-view drawings of 1:50 now over those of the coarsest authored scale now: sheets.sh <id>…   (SCALES="50 100" chooses the scales; BEFORE=1 puts 1:50 of the backup in 🗑️generated/silhouette/SAM/before on top)
source "${0:A:h}/env.sh"; cd "$W"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
for id in "$@"; do
  file="$(ls "$S/🗂️catalog"/*/"$id"/🔣️.json)"
  coarse=$(python3 -c "import json,sys;a=json.load(open(sys.argv[1]))['asset'];print(max(int(k) for k,v in a['representations'].items() if v['status']=='authored'))" "$file")
  parts=()
  [[ -n "$BEFORE" ]] && parts+=("$(./look.sh before "$id" 50 "$GEN/before/$id.json")")
  for s in ${=SCALES:-50 $coarse}; do parts+=("$(./look.sh after "$id" $s)"); done
  (cd "$T14" && "$B" "$W/montage.ts" "$GEN/sheets/$id.png" 1500 $parts)
done

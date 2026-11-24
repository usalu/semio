#!/bin/zsh
# 👀️ Renders the product 2D sheet (side, front, plan; line and solid view) of plants at the given scales with the r3-2d tool of the 3D ticket and moves the PNGs to 🗑️generated/silhouette/SV-2/look/<tag>.<id>.s<scale>.png: look.sh <tag> "<scales>" <slug>…   (tag `before` renders the state before the repair)
source "${0:A:h}/env.sh"
T14="$U/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
tag="$1"; scales="$2"; shift 2; mkdir -p "$GEN/look"
for a in "$@"; do
  work="$GEN/look/work/$a"; mkdir -p "$work"
  if [[ "$tag" == before ]]; then cp "$GEN/backup/$a.json" "$work/🔣️.json"; else cp "$CAT/$a/🔣️.json" "$work/🔣️.json"; fi
  for s in ${=scales}; do
    png=$(cd "$T14" && timeout 300 "$B" r3-2d.ts --file "$work/🔣️.json" --scale "$s" 2>&1 | tail -1)
    if [[ -f "$png" ]]; then mv "$png" "$GEN/look/$tag.$a.s$s.png"; echo "$GEN/look/$tag.$a.s$s.png"; else echo "no sheet for $a 1:$s: $png"; fi
  done
  rm -rf "$GEN/look/work"
done

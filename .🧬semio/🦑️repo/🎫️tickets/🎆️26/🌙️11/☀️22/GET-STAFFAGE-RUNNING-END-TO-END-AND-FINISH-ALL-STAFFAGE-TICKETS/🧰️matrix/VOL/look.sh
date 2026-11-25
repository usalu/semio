#!/bin/zsh
# 👀️ Small contact sheet of one three-quarter (turn 50 tilt 28) body view per entry <id>:<scale> of the catalog (or of $GEN/cand with CAND=1), joined in a grid into $GEN/sheets/<tag>.jpg, in one queue grant: look.sh <tag> <id>:<scale>…   (LIST=…, CUT=1, COLS=4)
me="${0:A}"; source "${0:A:h}/env.sh"
if [[ "$1" != "--inside" ]]; then cd "$R" && SEMIO_LANGUAGE=en exec repo capacity run --memory=800 -- timeout 1800 "$me" --inside "$@"; fi
shift; tag="$1"; shift; list="${LIST:-body}"; mkdir -p "$GEN/sheets"; parts=(); cut=(); [[ -n "$CUT" ]] && cut=(--cut)
for item in "$@"; do a="${item%%:*}"; s="${item##*:}"; d="$GEN/look/$a"; mkdir -p "$d"
  if [[ -n "$CAND" ]]; then cp "$GEN/cand/$a/🔣️.json" "$d/🔣️.json"; else cp "$(asset "$a")" "$d/🔣️.json"; fi
  f="$(cd "$T14" && timeout 120 "$B" r3-sheet.ts --file "$d/🔣️.json" --scale "$s" --list "$list" --only "${VIEW:-three-quarter}" --no-rig --no-labels $cut 2>&1 | grep -o '/.*\.png\|🗑️generated.*\.png\|NOT DRAWN.*\|rror.*' | tail -1)"; [[ -f "$f" ]] || f="$T14/$f"
  [[ -f "$f" ]] && { mv "$f" "$GEN/look/$a.$s.png"; parts+=("$GEN/look/$a.$s.png"); } || echo "no sheet $item $f"
done
"$B" "$W/grid.ts" "$GEN/sheets/$tag.jpg" "${COLS:-4}" $parts

#!/bin/sh
# 🎞️ Renders the review sheets of candidates one tool process at a time and arranges them into two contact sheets per asset under 🗑️generated/contact/:
#   <slug>.3d.png     three-quarter view of every authored scale x body/thermal/structure, then at 1:50 of every non-default pose and of two instants of every clip
#   <slug>.2d.png     the 2D drawings (line and solid rows; elevation, front, plan) of every authored scale
# Usage: sh sheets.sh <slug> [<slug> …]
K=$(cd "$(dirname "$0")" && pwd)
T="$K/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
OUT="$K/🗑️generated/contact"
mkdir -p "$OUT"
cd "$T" || exit 1
for slug in "$@"; do
  C="$K/assets/$slug/🔣️.json"
  rm -f "$T/🗑️generated/r3-sheet/file/$slug".*.png "$T/🗑️generated/r3-2d/file/$slug".*.png
  scales=$(python3 -c "import json,sys;a=json.load(open(sys.argv[1]))['asset'];print(' '.join(s for s in ('50','100','200','500') if a['representations'][s]['status']=='authored'))" "$C")
  poses=$(python3 -c "import json,sys;a=json.load(open(sys.argv[1]))['asset'];print(' '.join(p['id'] for p in a['poses'] if p['id']!=a['presentation']['defaultPose']))" "$C")
  clips=$(python3 -c "import json,sys;a=json.load(open(sys.argv[1]))['asset'];print(' '.join('%s:%.2f %s:%.2f'%(c['id'],c['durationS']/3,c['id'],c['durationS']*2/3) for c in a['clips']))" "$C")
  set --
  for list in body thermal structure; do for s in $scales; do
    "$BUN" r3-sheet.ts --file "$C" --scale "$s" --list "$list" --only three-quarter > /dev/null 2>"$OUT/$slug.err.txt" || cat "$OUT/$slug.err.txt"
    set -- "$@" "$(ls -t "$T/🗑️generated/r3-sheet/file/$slug".*.png | head -1)"
  done; done
  for p in $poses; do
    "$BUN" r3-sheet.ts --file "$C" --scale 50 --pose "$p" --only three-quarter > /dev/null 2>"$OUT/$slug.err.txt" || cat "$OUT/$slug.err.txt"
    set -- "$@" "$(ls -t "$T/🗑️generated/r3-sheet/file/$slug".*.png | head -1)"
  done
  for ct in $clips; do
    "$BUN" r3-sheet.ts --file "$C" --scale 50 --clip "${ct%%:*}" --t "${ct##*:}" --only three-quarter > /dev/null 2>"$OUT/$slug.err.txt" || cat "$OUT/$slug.err.txt"
    set -- "$@" "$(ls -t "$T/🗑️generated/r3-sheet/file/$slug".*.png | head -1)"
  done
  "$BUN" "$K/contact.ts" "$OUT/$slug.3d.png" 5 320 300 "$@"
  set --
  for s in $scales; do
    "$BUN" r3-2d.ts --file "$C" --scale "$s" > /dev/null 2>"$OUT/$slug.err.txt" || cat "$OUT/$slug.err.txt"
    set -- "$@" "$(ls -t "$T/🗑️generated/r3-2d/file/$slug".*.png | head -1)"
  done
  "$BUN" "$K/contact.ts" "$OUT/$slug.2d.png" 2 800 540 "$@"
  rm -f "$T/🗑️generated/r3-sheet/file/$slug".*.png "$T/🗑️generated/r3-2d/file/$slug".*.png "$OUT/$slug.err.txt"
done

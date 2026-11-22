#!/bin/zsh
# 🖼️ Multiview sheets (plan, side, front) of an asset document at the given scales and poses: sheet.sh <file.json> <out-dir> <view> <pose|clip:id@t> <scale…>
source "${0:A:h}/env.sh"
f=$1; o=$2; view=$3; p=$4; shift 4
mkdir -p "$o"
for s in "$@"; do
  if [[ $p == clip:* ]]; then c=${p#clip:}; q=(--clip ${c%@*} --time ${c#*@}); else q=(--pose $p); fi
  (cd "$S" && $B "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts" --asset x --file "$f" $q --scale $s --views elevation,front,plan --projection third --view $view --no-png --flow none --out "$o" >/dev/null 2>&1) || echo "[DEBUG] preview failed $p $s"
done
(cd "$PK" && $B "$W/png.ts" --dpi ${DPI:-110} "$o"/*.svg >/dev/null && rm "$o"/*.svg)
ls "$o"

#!/bin/zsh
# 👀️ Shaded axonometric views of the body list of a catalog asset with the render tool of the 3D ticket (`volumes-render.ts`), inside the grant of its caller: looks.sh <category>/<id> <scale>… moves each picture to `$GEN/sheets/<id>.s<scale>.body.png`.
source "${0:A:h}/env.sh"
a="$1"; shift; mkdir -p "$GEN/sheets"; cd "$T14"
for s in "$@"; do
  timeout 900 "$B" volumes-render.ts "$a" --scale "$s" --list body 2>&1 | tail -1
  png="$T14/🗑️generated/volumes-tools/render/$a.s$s.body.png"; [[ -f "$png" ]] && mv "$png" "$GEN/sheets/${a:t}.s$s.body.png" && echo "$GEN/sheets/${a:t}.s$s.body.png"
done

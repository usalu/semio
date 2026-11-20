#!/bin/sh
# 🔎 Draws the product's 2D drawings of the catalog asset humans/standing-phone-user at one scale and pose and enlarges the three line drawings (3000 px each) into 🗑️generated/r3-standing-phone-user/svg/.
# Usage: ./r3-standing-phone-user-look2d.sh 50 [pose]
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
O="🗑️generated/r3-standing-phone-user/svg"
mkdir -p "$O"
if [ -n "$2" ]; then P="--pose $2"; T=".$2"; else P=""; T=""; fi
"$BUN" r3-2d.ts humans/standing-phone-user --scale "$1" $P --views light-line,light-solid --svg >/dev/null || true
for v in elevation front plan; do
  for m in light-line light-solid; do
    qlmanage -t -s 3000 -o "$O" "🗑️generated/r3-2d/humans/standing-phone-user.s$1$T.$v.$m.svg" >/dev/null 2>&1
    sips -c 2000 1500 --cropOffset 100 150 "$O/standing-phone-user.s$1$T.$v.$m.svg.png" --out "$O/s$1$T.$v.$m.png" >/dev/null
  done
done
ls "$O" | grep "^s$1$T\."

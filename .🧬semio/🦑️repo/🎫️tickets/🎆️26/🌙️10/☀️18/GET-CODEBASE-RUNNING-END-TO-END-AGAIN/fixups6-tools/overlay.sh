#!/bin/sh
set -e
T=$(cd "$(dirname "$0")" && pwd)
MODE=${1:?orig|split}
OV=${2:-/private/tmp/claude-501/fixups6-ov-$MODE}
ROOT=/Users/ueli/Documents/semio
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph"
"$T/detach.sh" "$OV"; rm -rf "$OV"; mkdir -p "$OV"
link_siblings() {
  src=$1; dst=$2; skip=$3
  for e in "$src"/* "$src"/.[!.]*; do
    [ -e "$e" ] || [ -L "$e" ] || continue
    n=$(basename "$e")
    [ "$n" = "$skip" ] && continue
    [ "$n" = ".cache" ] && continue
    ln -s "$e" "$dst/$n"
  done
}
cur=$ROOT; dst=$OV
set -- "🧰️framework" "🛍️products" "🦑️repo" "🔨️modules" "📚️library" "🏗️build" "🧪️tests" "🕸️graph"
while [ $# -gt 0 ]; do
  link_siblings "$cur" "$dst" "$1"
  cur="$cur/$1"; dst="$dst/$1"; mkdir -p "$dst"; shift
done
if [ "$MODE" = orig ]; then
  for e in "$cur"/*; do n=$(basename "$e"); [ "$n" = "🟦️.ts" ] || ln -s "$e" "$dst/$n"; done
  cp "$cur/🟦️.ts" "$dst/🟦️.ts"
else
  for e in "$cur"/*; do n=$(basename "$e"); [ "$n" = "🟦️.ts" ] || ln -s "$e" "$dst/$n"; done
  cp -R "$T/staged/$G/." "$dst/"
fi
for f in package.json 📋️project.json tsconfig.json; do rm -f "$OV/$f"; cp "$ROOT/$f" "$OV/$f"; done
python3 -I - "$dst/🟦️.ts" <<'PY'
import re,sys
p=sys.argv[1];s=open(p,encoding="utf8").read().split("\n")
open(p,"w",encoding="utf8").write("\n".join(l for l in s if not re.match(r'^import\s*"(?:\.\./|\./🔄️)[^"]+";?$',l))+("" if s[-1]=="" else "\n"))
PY
mkdir -p "$OV/.cache/b/c"
echo "$OV"

#!/bin/zsh
# usage: invall.sh <tag>
cd "$(dirname "$0")"
G="$(pwd)/../🗑️generated"
TAG="$1"
for scope in "✏️s/🔌️plugins/🗄️stdio" "✏️s/🔌️plugins/🌀️procedural" "✏️s/🔌️plugins/📐️cad" "✏️s/🔌️plugins/🧱️block"; do
  name=$(echo "$scope" | sed -E 's#.*/##' | python3 -c "import sys,unicodedata;print(''.join(c for c in sys.stdin.read().strip() if c.isascii()) or 'x')")
  perl -e 'alarm 2400; exec @ARGV' -- bun inv.ts "$scope" "$G/hr2-inv-$name-$TAG.json" > "$G/hr2-inv-$name-$TAG.log" 2>&1
done
echo done > "$G/hr2-inv-$TAG.done"

#!/bin/zsh
# usage: checkapplied.sh <label> ; runs the layout gate for both touched artifact packages, one after the other
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad
P='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts'
L=$1
cd $R
for a in 5d:🖐️5d 3d:🧊️3d; do
  n=${a%%:*}; d=${a##*:}
  rm -rf "$SC/c4p6-ca-$n-$L"
  perl -e 'alarm 3400; exec @ARGV' -- bun ./📜️script.ts new crates "$P/$d/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SC/c4p6-ca-$n-$L" > "$F/🗑️generated/c4p6-checkapplied-$n-$L.log" 2>&1
  echo "exit $?" >> "$F/🗑️generated/c4p6-checkapplied-$n-$L.log"
done

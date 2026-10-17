#!/bin/zsh
# usage: checkapplied.sh <label> ; runs the layout gate (new crates <pkg> --check-applied --base=f856dff0813) for the four touched artifact packages, one after the other
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad
P='✏️s/🔌️plugins'
L=$1
cd $R
for a in gen3d:🌀️procedural/🗿️artifacts/🧊️generation3d gis:🌍️gis/🗿️artifacts/🗺️gismap vcs:🌿️vcs/🗿️artifacts/🌿️vcs anim:🎞️animate/🗿️artifacts/🎬️presentation; do
  n=${a%%:*}; d=${a##*:}
  rm -rf "$SC/c4p18-ca-$n-$L"
  perl -e 'alarm 3400; exec @ARGV' -- bun ./📜️script.ts new crates "$P/$d/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SC/c4p18-ca-$n-$L" > "$F/🗑️generated/c4p18-checkapplied-$n-$L.log" 2>&1
  echo "exit $?" >> "$F/🗑️generated/c4p18-checkapplied-$n-$L.log"
done
echo done > "$F/🗑️generated/c4p18-checkapplied-$L.done"

#!/bin/zsh
# usage: checkapplied.sh <label> <tag:artifact-dir-relative-to-plugins>... ; runs the layout gate (new crates --check-applied --base=f856dff0813) for each package, in parallel
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SC=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p7-work
L=$1; shift
cd $R
for a in "$@"; do
  n=${a%%:*}; d=${a##*:}
  rm -rf "$SC/ca-$n-$L"
  ( nice perl -e 'alarm 3400; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/$d/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SC/ca-$n-$L" > "$F/🗑️generated/c4p7-checkapplied-$n-$L.log" 2>&1; echo "exit $?" >> "$F/🗑️generated/c4p7-checkapplied-$n-$L.log" ) &
done
wait

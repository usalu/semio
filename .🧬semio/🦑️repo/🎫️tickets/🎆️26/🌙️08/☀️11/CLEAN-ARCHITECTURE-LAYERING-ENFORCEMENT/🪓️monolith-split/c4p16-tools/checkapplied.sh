#!/bin/zsh
# usage: checkapplied.sh <label> <tag>...   runs the layout gate (new crates --check-applied --base=f856dff0813) for each stdio artifact package, one after the other
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SC=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/c4p16-work
P='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
L=$1; shift
cd $R
typeset -A DIR
DIR=(las '☁️las' zip '🎒️zip' mp4 '🎥️mp4' bcf '💬️bcf' step '📐️step' xlsx '📕️xlsx')
for n in "$@"; do
  d=${DIR[$n]}
  rm -rf "$SC/ca-$n-$L"
  nice perl -e 'alarm 3500; exec @ARGV' -- bun ./📜️script.ts new crates "$P/$d/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SC/ca-$n-$L" > "$F/🗑️generated/c4p16-checkapplied-$n-$L.log" 2>&1
  echo "exit $?" >> "$F/🗑️generated/c4p16-checkapplied-$n-$L.log"
done
echo "all done $L" >> "$F/🗑️generated/c4p16-checkapplied.exit"

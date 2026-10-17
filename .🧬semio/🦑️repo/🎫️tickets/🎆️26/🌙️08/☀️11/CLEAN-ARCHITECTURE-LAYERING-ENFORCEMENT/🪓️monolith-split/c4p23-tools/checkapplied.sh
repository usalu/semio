#!/bin/zsh
# usage: checkapplied.sh <label> <tag:dir-relative-to-repo>... ; runs the layout gate (new crates --check-applied --base=f856dff0813) for each directory, in parallel
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/c4p23-ca
export LC_ALL=en_US.UTF-8
ulimit -n 65536
L=$1; shift
cd $R
for a in "$@"; do
  n=${a%%:*}; d=${a##*:}
  rm -rf "$SC/ca-$n-$L"
  ( nice perl -e 'alarm 3400; exec @ARGV' -- bun ./📜️script.ts new crates "$d" --check-applied --base=f856dff0813 --sandbox="$SC/ca-$n-$L" > "$F/🗑️generated/c4p23-checkapplied-$n-$L.log" 2>&1; echo "exit $?" >> "$F/🗑️generated/c4p23-checkapplied-$n-$L.log" ) &
done
wait

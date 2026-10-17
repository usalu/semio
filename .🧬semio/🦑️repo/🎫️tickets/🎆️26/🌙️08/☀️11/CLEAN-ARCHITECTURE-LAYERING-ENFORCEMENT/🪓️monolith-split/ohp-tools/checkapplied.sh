#!/bin/zsh
# usage: checkapplied.sh <label> <plugin-dir-name>   runs the layout gate for one package (new crates --check-applied --base=f856dff0813); log F/🗑️generated/ohp-checkapplied-<plugin>-<label>.log
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
export LC_ALL=en_US.UTF-8
ulimit -n 65536
L=$1; d=$2; pkg=${3:-}
cd $R
SC=/Users/ueli/.cache/ohp/ca-$L
rm -rf "$SC"
nice -n 10 perl -e 'alarm 3400; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/$d/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SC" > "$F/🗑️generated/ohp-checkapplied-$L.log" 2>&1
echo "exit $?" >> "$F/🗑️generated/ohp-checkapplied-$L.log"

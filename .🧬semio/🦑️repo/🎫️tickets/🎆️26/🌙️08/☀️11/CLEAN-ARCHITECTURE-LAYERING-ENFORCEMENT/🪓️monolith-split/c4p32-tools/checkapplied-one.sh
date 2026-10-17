#!/bin/zsh
# usage: checkapplied-one.sh <label> <name>   `new crates <artifact>/📦️packages/🦀️rust --check-applied` for one c4p32 artifact package (shooting, fem3d, bim)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/c4p32
label=$1; n=$2
typeset -A P
P=(shooting "🎥️shooting/🗿️artifacts/🎥️shooting" fem3d "🏗️fem/🗿️artifacts/🧊️3d" bim "🏙️bim/🗿️artifacts/🏢️model")
export LC_ALL=en_US.UTF-8
cd /Users/ueli/Documents/semio || exit 2
rm -rf "$SCR/ca-$label-$n"; mkdir -p "$SCR/ca-$label-$n"
perl -e 'alarm 3000; exec @ARGV' -- bun ./📜️script.ts new crates "✏️s/🔌️plugins/${P[$n]}/📦️packages/🦀️rust" --check-applied --base=f856dff0813 --sandbox="$SCR/ca-$label-$n" ${C4P32_KEEP:+--keep} > "$F/🗑️generated/c4p32-checkapplied-$label-$n.log" 2>&1
echo "$n exit=$?" >> "$F/🗑️generated/c4p32-checkapplied-$label.exit"

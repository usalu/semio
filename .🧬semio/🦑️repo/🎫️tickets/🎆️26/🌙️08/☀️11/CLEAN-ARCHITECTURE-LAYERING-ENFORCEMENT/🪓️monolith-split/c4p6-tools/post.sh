#!/bin/zsh
# usage: post.sh <label> ; post-install checks: census, lit, modowners (cheap ones first), then the typeck oracle
R=/Users/ueli/Documents/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G="$F/🗑️generated"
L=$1
cd $R
P='✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts'
E='🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
bun "$F/pfb-tools/census.ts" "$P/🧊️3d/$E/⏳️precompute/📐️geometry/📦️packages/🦀️rust/Cargo.toml" "$P/🧊️3d/$E/⏳️precompute/🖌️brush/📦️packages/🦀️rust/Cargo.toml" "$P/🖐️5d/Cargo.toml" > "$G/c4p6-census-$L.log" 2>&1
python3 "$F/rpv-tools/lit.py" "$F/c4p6-tools/families/c4p6.json" > "$G/c4p6-lit-$L.log" 2>&1
bun "$F/dm2-tools/modowners.ts" "$G" "c4p6-modowners-$L.tsv" > "$G/c4p6-modowners-$L.log" 2>&1
cd "$F/pos-tools" && POS_PREFIX=c4p6 perl -e 'alarm 3000; exec @ARGV' -- python3 pos.py run ../c4p6-tools/families/c4p6.json --cfgs lib,test --label $L > "$G/c4p6-pos-$L.log" 2>&1
echo done > "$G/c4p6-post-$L.done"

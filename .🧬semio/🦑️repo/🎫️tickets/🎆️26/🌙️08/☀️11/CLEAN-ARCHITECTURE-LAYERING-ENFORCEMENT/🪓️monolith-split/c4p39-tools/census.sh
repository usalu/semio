#!/bin/zsh
# usage: census.sh <label>   canonical test-closure census (c4p39-tools/census.ts, copy of rtb-census.ts) of the six Shell test bins that mount the six split laws
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu"
M=""
for d in 🌅️shell-tests-chrome-3 🌴️shell-tests-chrome-4 🥧️shell-tests-docs-1 🥪️shell-tests-docs-2 🍎️shell-tests-input-1 🏙️shell-tests-windows-1; do M="$M;$W/$d/📦️packages/🦀️rust/Cargo.toml"; done
cd "$F/c4p39-tools" || exit 2
label=$1
MANIFESTS="${M#;}" DUMP_INPUTS=1 perl -e 'alarm 900; exec @ARGV' -- bun census.ts > "$F/🗑️generated/c4p39-census-$label.log" 2>&1
echo "exit $?"
cp "$F/🗑️generated/c4p39-census-units.tsv" "$F/🗑️generated/c4p39-census-$label-units.tsv"
cp "$F/🗑️generated/c4p39-census-overlimit-test-units.tsv" "$F/🗑️generated/c4p39-census-$label-overlimit.tsv" 2>/dev/null
python3 - "$F/🗑️generated/c4p39-census-$label-units.tsv" <<'PY'
import csv,sys
rows=list(csv.DictReader(open(sys.argv[1],encoding='utf-8'),delimiter='\t'))
print(len(rows),'rows; columns',list(rows[0].keys()))
for r in rows:
    print(r.get('crate','')[-34:],r.get('kind',''),r.get('label',''),r.get('files',''),r.get('bytes',''),r.get('lines',''),r.get('ratio',''),r.get('binding',''),r.get('error',''))
PY

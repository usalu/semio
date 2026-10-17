#!/bin/zsh
# usage: census.sh <label> <manifest;manifest...>   canonical closure census (copy of the c4p39 rtb census, outputs c4pp-census-*) for the given repo-relative Cargo.toml manifests
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/c4pp-tools" || exit 2
label=$1
MANIFESTS="$2" perl -e 'alarm 900; exec @ARGV' -- bun census-m.ts > "$F/🗑️generated/c4pp-census-$label.log" 2>&1
echo "exit $?"
cp "$F/🗑️generated/c4pp-census-units.tsv" "$F/🗑️generated/c4pp-census-$label-units.tsv"
python3 - "$F/🗑️generated/c4pp-census-$label-units.tsv" <<'PY'
import csv,sys
rows=list(csv.DictReader(open(sys.argv[1],encoding='utf-8'),delimiter='\t'))
print(len(rows),'rows; columns',list(rows[0].keys()))
for r in rows:
    print(r.get('crate','')[-52:],r.get('kind',''),r.get('label',''),r.get('files',''),r.get('bytes',''),r.get('lines',''),r.get('ratio',''),r.get('binding',''),r.get('error',''))
PY

#!/bin/zsh
# 🧰 Shell helpers of the group D silhouette repairs. source this file.
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
O=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET
G=$T/🗑️generated
TS=/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript
hullop() { (cd $TS && $B $T/humansb-D-addhull.ts "$@"); }
cl() { (cd $TS && $B $T/humansb-D-clusters.ts "$@"); }
try() { (cd $TS && $B $O/humans-try.ts "$@"); }
merge() { python3 -c "
import json,sys
o=[]
for f in sys.argv[2:]: o+=json.load(open(f))
json.dump(o,open(sys.argv[1],'w'))" "$@"; }
tri() { (cd $TS && $B $O/silhouette-triage.ts current $G/humansb-D-$1.jsonl "${@:2}" >/dev/null 2>&1; python3 $T/humansb-summary.py $G/humansb-D-$1.jsonl); }
hulls() { local a=$1 s=$2 out=$3; shift 3; local fs=(); local i=0; for spec in "$@"; do i=$((i+1)); local f=$G/D-h$i.json; hullop $a $s ${=spec} > $f; fs+=($f); done; merge $out $fs; }
build() { local a=$1 s=$2 spec=$3 out=$4; local i=0; echo '[]' > $G/D-acc.json; while IFS= read -r line; do [[ -z "$line" || "$line" == \#* ]] && continue; i=$((i+1)); local f=$G/D-b$i.json; local mode=${line%% *}; local rest=${line#* }; if [[ $mode == T ]]; then PATCH=$G/D-acc.json TRACE=1 hullop $a $s ${=rest} > $f; else PATCH=$G/D-acc.json hullop $a $s ${=rest} > $f; fi; merge $G/D-acc2.json $G/D-acc.json $f; mv $G/D-acc2.json $G/D-acc.json; done < $spec; cp $G/D-acc.json $out; }
earfix() { python3 -c "
import json,sys
f=sys.argv[1]; o=json.load(open(f)); o+=json.loads(sys.argv[2]); json.dump(o,open(f,'w'))" "$@"; }

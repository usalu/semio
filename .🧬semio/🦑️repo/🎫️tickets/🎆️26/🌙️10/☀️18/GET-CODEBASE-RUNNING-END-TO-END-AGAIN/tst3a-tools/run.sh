#!/bin/zsh
# usage: run.sh <tag> <list> ; 2 shards, resumable
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio
tag=$1; list=${2:-$C/tst3a-tools/list.txt}
mkdir -p $C/🗑️generated/tst3a-out
if [ ! -f $C/tst3a-tools/shard-$tag-a ]; then
python3 - "$list" "$C" "$tag" <<'PY'
import sys
L=[l for l in open(sys.argv[1],encoding="utf-8").read().split("\n") if l]
for i,p in enumerate("ab"):
    open(sys.argv[2]+"/🗑️generated/tst3a-out/shard-"+sys.argv[3]+"-"+p,"w",encoding="utf-8").write("\n".join(L[i::2])+"\n")
PY
fi
for p in a b; do TST2_HOIST=1 python3 $C/tst3a-tools/runbun.py $C/🗑️generated/tst3a-out/shard-$tag-$p $C/🗑️generated/tst3a-out/$tag-$p.jsonl 270 295 60000 > $C/🗑️generated/tst3a-out/run-$tag-$p.txt 2>&1 & done
wait
cat $C/🗑️generated/tst3a-out/run-$tag-*.txt
python3 $C/tst3a-tools/sum.py $tag

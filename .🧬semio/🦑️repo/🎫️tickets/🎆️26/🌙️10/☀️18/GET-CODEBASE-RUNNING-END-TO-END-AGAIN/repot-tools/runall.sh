#!/bin/zsh
# usage: runall.sh <tag> [list]  (4 parallel shards, ~10 min)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio
tag=$1; list=${2:-$C/repot-tools/list.txt}
python3 - "$list" "$C" <<'PY'
import sys
L=[l for l in open(sys.argv[1],encoding="utf-8").read().split("\n") if l]
for i,p in enumerate("abcd"):
    open(sys.argv[2]+"/repot-tools/shard-"+p,"w",encoding="utf-8").write("\n".join(L[i::4])+"\n")
PY
rm -f $C/🗑️generated/repot-out/$tag-*.jsonl
for p in a b c d; do TST2_HOIST=1 python3 $C/repot-tools/runbun.py $C/repot-tools/shard-$p $C/🗑️generated/repot-out/$tag-$p.jsonl 570 280 60000 > $C/🗑️generated/repot-out/run-$tag-$p.txt 2>&1 & done
wait
cat $C/🗑️generated/repot-out/run-$tag-*.txt
python3 $C/repot-tools/sum.py $tag

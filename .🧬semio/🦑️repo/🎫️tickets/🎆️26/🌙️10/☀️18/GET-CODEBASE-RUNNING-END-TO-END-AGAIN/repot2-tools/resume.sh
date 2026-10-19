#!/bin/zsh
# usage: resume.sh <tag>  (continues the 4 shards of runall.sh)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio
tag=$1
for p in a b c d; do TST2_HOIST=1 python3 $C/repot2-tools/runbun.py $C/repot2-tools/shard-$p $C/🗑️generated/repot2-out/$tag-$p.jsonl 570 280 60000 > $C/🗑️generated/repot2-out/run2-$tag-$p.txt 2>&1 & done
wait
python3 $C/repot2-tools/sum.py $tag

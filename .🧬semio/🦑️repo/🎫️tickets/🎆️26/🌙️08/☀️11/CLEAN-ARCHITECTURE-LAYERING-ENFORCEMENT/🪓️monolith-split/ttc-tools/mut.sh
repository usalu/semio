#!/bin/zsh
# usage: mut.sh <family> <label> <mutation>  -> seeded sensitivity run on the test cfg
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
T="$F/ttc-tools"
perl -e 'alarm 1500; exec @ARGV' -- python3 "$T/ttc.py" oracle "$T/families/$1.json" --cfgs test --label $2 --iters 4 --mutate "$3" > "$F/🗑️generated/ttc-run-$1-$2.log" 2>&1
python3 "$T/priv.py" "$F/🗑️generated/ttc-pos-$1-$2-result.json" test | cut -c1-200 | head -8

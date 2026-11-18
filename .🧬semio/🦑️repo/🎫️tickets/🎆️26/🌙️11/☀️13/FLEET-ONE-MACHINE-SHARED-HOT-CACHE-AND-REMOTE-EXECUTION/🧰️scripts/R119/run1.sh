#!/bin/bash
# usage: run1.sh <label> <owners-csv|""> [N]   env: R119_PRE=1 patched, R119_VERDICTS=dir
cd /Users/ueli/Documents/semio || exit 1
BUN=$PWD/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
D=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R119"
S=$PWD/$D/accept.ts
SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R119
export R119_STORE=${R119_STORE-$PWD/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/ce}
PRE=(); [ -n "$R119_PRE" ] && PRE=(--preload $PWD/$D/preload.ts)
R119_OWNERS="$2" R119_N=${3:-5} nice -n 5 $BUN "${PRE[@]}" "$S" > $SP/$1.out 2>&1
echo done >> $SP/$1.out

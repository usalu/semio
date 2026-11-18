#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
BUN=.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R200
D1="✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust"; D2="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust"
for run in fix1 base1 base2; do
 nf=1; [ $run = fix1 ] && nf=0
 echo "== $run"
 NAMES_OUT=$S/names-$run NOFIX=$nf LATE=0 SEMIO_WORKSPACE_ROOT=$PWD BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 $BUN --preload $S/fix2.ts $S/prof2.ts "$D1" "$D2" 2>&1 | grep -a "^{\|CALLS\|rror" | cut -c1-500
done
echo ALLDONE

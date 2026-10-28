#!/bin/zsh
# [derive-g] runs the environment-move scenario twice back to back on the same base: NEW (the command walk of the testing domain as a derived fact) and OLDWALK (OLDWALK=1: the walk read directly, its ~20 thousand rows in the capture), then compares the two resulting generations owner by owner: the tests must publish the same fragments. usage: pair3.sh <tag>
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; S=/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/derive-g; O="$C/🗑️generated/derive-g"; T=$1
for v in derived oldwalk; do
 rm -rf $S/store $S/run-$T-$v; cp -R ${BASE:-$S/gen547-move} $S/run-$T-$v
 if [ $v = oldwalk ]; then X="OLDWALK=1"; else X="OLDWALK=0"; fi
 env $X SLICE=0 TOUCH_KINDS=digest TOUCH="✏️s/Cargo.toml" RSS_CAP_MB=9000 STORE=$S/store nice -n 10 /usr/bin/time -l bun "$C/derive-g-tools/delta.mjs" $S/run-$T-$v @semio-tech/print-rs test > "$O/pair-$T-$v.log" 2>&1
done
bun "$C/asm-g-tools/cmp-gens.mjs" $S/run-$T-derived $S/run-$T-oldwalk > "$O/pair-$T.cmp" 2>&1
echo done > "$O/pair-$T.done"

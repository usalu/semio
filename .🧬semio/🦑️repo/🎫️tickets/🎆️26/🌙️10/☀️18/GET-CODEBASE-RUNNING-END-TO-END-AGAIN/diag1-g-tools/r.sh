#!/bin/zsh
# usage: r.sh <label> <index-list-name>
cd /Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/diag1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
LABEL=$1
PN="$L/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"; OB="$L/👁️observation/🟨️.mjs"; SI="$L/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"; GN="$L/🕸️graph/🕰️generations/🟨️.mjs"; GR="$L/🕸️graph/🟦️.ts"
case $2 in
 none) IDX="";;
 all) IDX="$PN,$OB,$SI,$GN,$GR";;
 allbutgraph) IDX="$PN,$OB,$SI,$GN";;
 allbutgen) IDX="$PN,$OB,$SI,$GR";;
esac
rm -rf $S/g-$LABEL $S/store-$LABEL; cp -R $S/g544 $S/g-$LABEL; mkdir -p $S/store-$LABEL
export DIAG_INDEX="$IDX" STORE=$S/store-$LABEL RSS_CAP_MB=${RSS_CAP_MB:-5200} OVERLAY_EXT=old RUNS=${RUNS:-1} TOUCH="🧰️framework/📦️packages/🟦️typescript/📋️project.json" TOUCH2="${TOUCH2_OVERRIDE:-🧰️framework/📦️packages/🟦️typescript/📋️project.json}"
/usr/bin/time -l $BUN $C/diag1-g-tools/delta-diag.mjs $S/g-$LABEL @semio-tech/framework typecheck > $S/out-$LABEL.log 2>&1
echo "exit $?" >> $S/out-$LABEL.log

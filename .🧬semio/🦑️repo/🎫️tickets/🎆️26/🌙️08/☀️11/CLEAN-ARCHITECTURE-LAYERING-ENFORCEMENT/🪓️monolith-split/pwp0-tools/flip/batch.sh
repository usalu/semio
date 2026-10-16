#!/bin/zsh
# usage: batch.sh build  <tag> <keys>           -- fb.py: live tree -> OUT (scratch pi1/out-<tag>)
#        batch.sh oracle <tag> <keys> [uo2 args] -- overlays OUT on a copy of the live tree and runs the crate-scoped union oracle (A only)
#        batch.sh widen  <tag> <config>          -- widens the definitions the last oracle run reported private (E0603/E0616/E0624/E0451) in OUT
set -e
CMD=$1; TAG=$2; KEYS=$3; shift 3 2>/dev/null || shift $#
HERE=${0:A:h}
WS=/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/pi1
PLUGREL='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
LIVE=/Users/ueli/Documents/semio/$PLUGREL
GEN=$HERE/../../🗑️generated
REAL=${REAL:-abi,actor-host,identity,interaction,operation,window-config,window-transient,window-kits,window-kits-rich,decl-views,decl-emission,decl-app,app-builder,decl-surface,decl-registry,plugin-program}
case $CMD in
build)
  rm -rf $WS/out-$TAG
  python3 $HERE/fb.py $LIVE $WS/out-$TAG $REAL $KEYS > $GEN/pi1-fb-$TAG.log 2>&1 || { tail -30 $GEN/pi1-fb-$TAG.log; exit 1; }
  head -3 $GEN/pi1-fb-$TAG.log ;;
oracle)
  rm -rf $WS/sbx-$TAG; mkdir -p $WS/sbx-$TAG/$PLUGREL
  rsync -a --exclude node_modules --exclude target --exclude .cache --exclude '📦️packages' --exclude '🖥️host' --exclude '🧠️interpreter' --exclude '🏇️mounted-owner' --exclude '🏛️ownership' --exclude '🧬️guest-contract' --exclude '🖨️describe' --exclude '🌐️browser-bundle' $LIVE/ $WS/sbx-$TAG/$PLUGREL/
  rsync -a $WS/out-$TAG/ $WS/sbx-$TAG/$PLUGREL/
  python3 $HERE/uo2.py $TAG --a $WS/sbx-$TAG --aflip "$REAL,$KEYS" --mode crates --only-a "$@" ;;
widen)
  python3 $HERE/widen.py $WS/pwp0-oracle-pi1${TAG}-A-${KEYS}.jsonl $WS/sb-$TAG/A $WS/out-$TAG $LIVE ;;
esac

#!/bin/zsh
G="$(cd "$(dirname "$0")" && pwd)"; SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/caf759d9-ccba-4796-9909-827125acd710/scratchpad/gr16/ab2
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
cd /Users/ueli/Documents/semio
rm -rf $SP; mkdir -p $SP; STORE_DIR=$SP/store; mkdir -p $STORE_DIR
i=0
for name in dag-dag draw-js cad-js-module-aec-building-structure cad-js compiler-syntax-ecma-ts canvas-react-renderer; do
  i=$((i+1))
  env WAVE=$SP/../wave UNITS=@semio-tech/$name $B --preload "$G/preload.mjs" "$G/one-owner-v4.mjs" > $SP/old-$i-$name.json 2> $SP/old-$i.err
  echo "old $i $name $(date +%T)" >> $SP/log
  env WAVE=$SP/../wave UNITS=@semio-tech/$name STORE=$STORE_DIR $B --preload "$G/preload.mjs" "$G/one-owner-v4.mjs" > $SP/new-$i-$name.json 2> $SP/new-$i.err
  echo "new $i $name $(date +%T)" >> $SP/log
done
echo all-done >> $SP/log

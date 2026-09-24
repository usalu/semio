#!/bin/zsh
# ⏭️ W2: waits for both stage chains to finish, re-stages puzzle (its [DEBUG] strip landed after its first stage), then runs w2-final.sh.
set -u
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-w2/generated
until grep -q '^\[w2-stage\] DONE' "$OUT/stage-chain-free.txt" && grep -q '^\[w2-stage\] DONE' "$OUT/stage-chain-52.txt"; do sleep 30; done
echo "[w2-after] stage chains done $(date '+%F %T')"
rm -f "$OUT/stage-puzzle-plugin.ok"
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-w2/w2-stage-all.sh @semio-tech/puzzle-plugin
failed=$(for f in $(cat "$OUT/stage-order-52.txt") @semio-tech/cad-extension-aec-building-energy-rust @semio-tech/cad-extension-aec-building-structure-rust @semio-tech/cad-extension-spatial-shape-rust @semio-tech/imperative-extension-control-rust @semio-tech/imperative-extension-effect-rust @semio-tech/imperative-extension-logic-rust @semio-tech/imperative-extension-math-rust @semio-tech/imperative-extension-text-rust; do n=${f#@semio-tech/}; [ -f "$OUT/stage-$n.ok" ] || echo $n; done)
if [ -n "$failed" ]; then echo "[w2-after] NOT STAGED: $failed — final chain not started"; exit 1; fi
zsh /Users/ueli/Documents/semio/.tmp-ticket/wp-w2/w2-final.sh

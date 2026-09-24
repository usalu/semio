#!/bin/zsh
cd /Users/ueli/Documents/semio
for c in semio-s-plugin-cad semio-s-plugin-lowpoly semio-s-plugin-dag semio-s-plugin-sequence semio-s-plugin-forms semio-s-plugin-mathematical semio-s-plugin-animate semio-s-plugin-imperative semio-s-plugin-reasoning semio-s-plugin-trinity semio-s-plugin-raster semio-s-plugin-playbook semio-s-plugin-energy semio-s-plugin-layout semio-s-plugin-norm semio-s-plugin-procedural semio-s-plugin-wfc semio-s-plugin-shooting semio-s-plugin-flow semio-s-plugin-writer semio-s-plugin-puzzle; do
  out=$(CARGO_INCREMENTAL=0 cargo check -p $c --target wasm32-wasip2 2>&1 | /usr/bin/grep -E "^error|could not compile|Finished" | head -4 | tr '\n' ' ')
  echo "$c: $out"
done
echo WASM-CHECK-DONE

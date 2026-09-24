#!/bin/zsh
cd /Users/ueli/Documents/semio
for c in semio-s-artifact-process-process3d semio-s-plugin-process semio-s-artifact-writer-writer semio-s-artifact-cad-cad semio-s-artifact-lowpoly-lowpoly semio-s-artifact-sequence-sequence semio-s-artifact-forms-forms semio-s-artifact-animate-presentation semio-s-artifact-playbook-playbook semio-s-artifact-layout-layout semio-s-artifact-norm-en1990 semio-s-artifact-norm-din18599 semio-s-artifact-mathematical-equation semio-s-artifact-imperative-procedure semio-s-artifact-reasoning-wires semio-s-artifact-trinity-jack semio-s-artifact-raster-raster semio-s-artifact-energy-model; do
  echo "== $c"
  CARGO_INCREMENTAL=0 cargo check -p $c --target wasm32-wasip2 2>&1 | grep -E "^error|Finished" | head -20
done

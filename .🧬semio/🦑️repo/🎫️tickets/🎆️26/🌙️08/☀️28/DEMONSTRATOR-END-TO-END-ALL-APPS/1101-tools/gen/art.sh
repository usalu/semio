#!/bin/bash
# usage: art.sh <N> — wasm check of the procedural artifact crates (no flow-extension closure)
D="$(dirname "$0")"; N=$1; shift
P=""; for c in generation2d generation3d 2d-editor 2d-viewer 2d-host 2d-io 2d-preview-eval 3d-editor 3d-viewer 3d-host 3d-io 3d-preview-eval 3d-io-geometry 3d-geometry-service 3d-envelope; do case $c in generation*) P="$P -p semio-s-artifact-procedural-$c";; *) P="$P -p semio-s-artifact-procedural-$c";; esac; done
exec "$D/../chk.sh" "gen-art$N" "🌎️hub/Cargo.toml" wasm $P "$@"

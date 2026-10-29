#!/bin/bash
D="$(dirname "$0")"
CRATES="object-editor value-mutations model-mutations flow-mutations document-mutations mesh-mutations audio-mutations animation-schema-4 cad-editor table-mutations drawing-editor brep-viewer image-editor text-mutations presentation-mutations video-mutations kit-mutations graph-editor mesh-editor brep-io flow-snapshot object-examples kit-examples"
"$D/retry.sh" "$1" TOP $CRATES
"$D/retry.sh" "$2" --target wasm32-wasip2 TOP $CRATES

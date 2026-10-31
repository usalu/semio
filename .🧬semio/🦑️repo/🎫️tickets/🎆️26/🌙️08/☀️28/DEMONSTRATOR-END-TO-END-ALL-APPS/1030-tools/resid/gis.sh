#!/bin/bash
D="$(dirname "$0")"
"$D/chk.sh" "$1" "🌎️hub/Cargo.toml" native -p semio-hub-gis -p semio-hub-inference-runtime -p semio-hub-inference-binding
"$D/chk.sh" "$2" "Cargo.toml" native -p semio-framework-os

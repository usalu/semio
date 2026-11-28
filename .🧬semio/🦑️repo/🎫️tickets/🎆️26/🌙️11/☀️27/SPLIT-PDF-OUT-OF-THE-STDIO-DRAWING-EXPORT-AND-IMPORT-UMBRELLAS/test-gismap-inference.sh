#!/usr/bin/env bash
# 🔎️ The gismap lib's whole-map inference tests alone, so their panic messages survive the window-ownership stack overflow.
cd "$(git rev-parse --show-toplevel)"
exec "$(dirname "$0")/cargo-lane.sh" 4 test --manifest-path "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" --lib --message-format short -p semio-s-artifact-gis-gismap -- tests:: --test-threads=1

#!/bin/bash
# usage: smoke.sh <round> — fixture lane: harness proof on the four pane smokes moved to the example door, one at a time, W- labels
D="$(cd "$(dirname "$0")/.." && pwd)"; R=${1:-101}; T="$D/runtime/tst.sh"; G="$D/../🗑️generated/1105b"; rm -f "$G/fixture-$R.done"
P="✏️s/🔌️plugins"
"$T" W-fixture-$R "$P/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml" native -p semio-s-artifact-cad-cad --test cad_pane_smoke -- --nocapture --test-threads=1
"$T" W-fixture-$((R+1)) "$P/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml" native -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke -- --nocapture --test-threads=1
"$T" W-fixture-$((R+2)) "$P/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen --nocapture --test-threads=1
"$T" W-fixture-$((R+3)) "$P/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.toml" native -p semio-s-artifact-sourcing-curation --test aussuchen_pane_smoke -- aussuchen_pane --nocapture --test-threads=1
touch "$G/fixture-$R.done"

#!/bin/bash
# usage: panes.sh <round> — five native pane smokes, one at a time, W- labels
D="$(cd "$(dirname "$0")" && pwd)"; R=${1:-1}; T="$D/runtime/tst.sh"; G="$D/../🗑️generated/1105b"; rm -f "$G/panes-$R.done"
"$T" W-pane-cad-$R "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/Cargo.toml" native -p semio-s-artifact-cad-cad --test cad_pane_smoke -- --nocapture --test-threads=1
"$T" W-pane-process3d-$R "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml" native -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke -- --nocapture --test-threads=1
"$T" W-pane-energy-$R "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" native -p semio-s-artifact-energy-model-editor-mounted-tests -- --nocapture --test-threads=1
"$T" W-pane-fem3d-$R "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" native -p semio-s-artifact-fem-3d-editor --test statik_pane_smoke -- statik_pane --nocapture --test-threads=1
"$T" W-pane-gis-$R "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen --nocapture --test-threads=1
touch "$G/panes-$R.done"

#!/bin/bash
R=/Users/ueli/Documents/semio
C="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
K="$C/1101-tools/chk.sh"
P="$R/✏️s/🔌️plugins"
"$K" closure-verfolgen "$P/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" native -p semio-s-artifact-gis-gismap-gis2d-unit-tests --tests
"$K" closure-statik "$P/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" native -p semio-s-artifact-fem-3d-editor --tests
"$K" closure-aussuchen "$P/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.toml" native -p semio-s-artifact-sourcing-curation --tests
"$K" closure-bearbeiten "$P/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml" native -p semio-s-artifact-process-process3d-editor --tests
"$K" closure-koordinator "$P/📐️cad/🗿️artifacts/📐️cad/Cargo.toml" native -p semio-s-artifact-cad-cad --tests
"$K" closure-generator "$R/✏️s/🧑‍💻dev/🧩️composition/Cargo.toml" native -p semio-s-composition-laws --tests
echo done > "$C/🗑️generated/1101/closure-seq.rc"

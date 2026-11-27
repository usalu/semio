#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
R="$T/pdfcycle-run.sh"; S="/Users/ueli/Documents/semio/✏️s/🔌️plugins"
"$R" gismap check --manifest-path "$S/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" -p semio-s-artifact-gis-gismap-gis2d -p semio-s-artifact-gis-gismap -p semio-s-artifact-gis-gismap-gismap --tests -j 3
"$R" pdf check --manifest-path "$S/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.toml" -p semio-s-artifact-stdio-pdf-page-editor --tests -j 3
echo done > "$T/🗑️generated/pdfcycle-all.done"

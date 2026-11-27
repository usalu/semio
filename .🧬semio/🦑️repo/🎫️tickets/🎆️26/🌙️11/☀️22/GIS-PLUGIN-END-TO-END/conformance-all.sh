#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
R="$T/conformance-run.sh"
"$R" gis2d check --manifest-path "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" -p semio-s-artifact-gis-gismap-gis2d --tests -j 3
"$R" gismap check --manifest-path "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" -p semio-s-artifact-gis-gismap --tests -j 3
"$R" gismap-viewer check --manifest-path "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" -p semio-s-artifact-gis-gismap-gismap --tests -j 3
"$R" gt-editor check --manifest-path "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml" -p semio-s-artifact-gis-gisterrain-editor --tests -j 3
"$R" gt check --manifest-path "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml" -p semio-s-artifact-gis-gisterrain --tests -j 3
echo done > "$T/🗑️generated/conformance-all.done"

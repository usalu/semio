#!/usr/bin/env bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
R="$T/gismap2-run.sh"
M="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml"
C=(repo cargo --lang=en --store=shared --configuration=prototype --)
"$R" check-all "${C[@]}" check -j 3 --manifest-path "$M" -p semio-s-artifact-gis-gismap-gis2d -p semio-s-artifact-gis-gismap -p semio-s-artifact-gis-gismap-gismap -p semio-s-artifact-gis-gismap-config --tests
echo done > "$T/🗑️generated/gismap2-all.done"

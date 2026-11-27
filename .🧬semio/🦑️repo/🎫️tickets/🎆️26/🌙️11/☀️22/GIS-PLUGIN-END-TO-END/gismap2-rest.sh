#!/usr/bin/env bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
until [ -f "$T/🗑️generated/gismap2-all.done" ]; do sleep 30; done
R="$T/gismap2-run-6.sh"
G="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis"
M="$G/🗿️artifacts/🗺️gismap/Cargo.toml"
C=(repo cargo --lang=en --store=shared --configuration=prototype --)
"$R" lib "${C[@]}" test -j 3 --manifest-path "$M" -p semio-s-artifact-gis-gismap --lib -- --test-threads=2
"$R" tiles "${C[@]}" test -j 3 --manifest-path "$M" -p semio-s-artifact-gis-gismap-tile-coordinates -- --test-threads=2
"$R" mvt "${C[@]}" test -j 3 -p semio-framework-surface-tiled-map-mvt vector_label_anchor_uses_line_midpoint_for_transportation_names -- --test-threads=2
"$R" hub "${C[@]}" test -j 3 -p semio-hub-gis --lib gis_manifest_publishes_one_editor_viewer_pair_per_artifact_dialect -- --test-threads=2
"$R" bridge "${C[@]}" test -j 3 --manifest-path "$G/🏭️bridge/Cargo.toml" -- --test-threads=2
"$R" gates bash "$T/../../../🌙️09/☀️16/GIS-2D-END-TO-END-BUILD/📜️verify-gis2d-envelope-gates.sh"
"$R" cold-map repo run @semio-tech/gis-plugin:component-cold-map-patch-native-check --lang=en
echo done > "$T/🗑️generated/gismap2-rest.done"

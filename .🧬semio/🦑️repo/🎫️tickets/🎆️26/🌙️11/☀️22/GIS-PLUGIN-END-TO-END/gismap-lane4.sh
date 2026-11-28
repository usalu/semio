#!/usr/bin/env bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
L="$T/🗑️generated/compile.lock2"
R=/Users/ueli/Documents/semio
G=$R/✏️s/🔌️plugins/🌍️gis
M="$G/🗿️artifacts/🗺️gismap/Cargo.toml"
cd $R
gate(){ while [ -e "$T/🗑️generated/exclusive-plan.active" ]; do sleep 30; done; }
step(){ name="$1"; shift
  for attempt in 1 2 3 4 5 6; do
    gate
    until mkdir "$L" 2>/dev/null; do
      if [ $(( $(date +%s) - $(stat -f %m "$L" 2>/dev/null || date +%s) )) -gt 7200 ]; then rmdir "$L" 2>/dev/null; fi
      sleep 20
    done
    echo gismap-lane > "$L/owner"
    gate
    until [ "$(memory_pressure | awk '/free percentage/{gsub("%","",$5);print $5}')" -ge 30 ]; do sleep 20; done
    env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute "$@" > "$T/🗑️generated/gismap4-$name.log" 2>&1
    rc=$?; rm -rf "$L"
    grep -q "host warden cancelled\|Wächter\] der Build kann nicht enden\|critically short\|SIGTERM\|No space left\|disk is full\|EFAULT\|Bounded physical Source revision\|Source image identity" "$T/🗑️generated/gismap4-$name.log" || break
    sleep 60
  done
  echo "exit $rc" >> "$T/🗑️generated/gismap4-$name.log"; }
C=(repo cargo --lang=en --store=shared --configuration=prototype --)
sleep 180
rm -rf "$L"
step tiles "${C[@]}" test -j 8 --manifest-path "$M" -p semio-s-artifact-gis-gismap-tile-coordinates -- --test-threads=2
step config "${C[@]}" test -j 8 --manifest-path "$M" -p semio-s-artifact-gis-gismap-config -- --test-threads=2
step lib "${C[@]}" test -j 8 --manifest-path "$M" -p semio-s-artifact-gis-gismap --lib -- --test-threads=2
step check "${C[@]}" test --no-run -j 8 --manifest-path "$M" -p semio-s-artifact-gis-gismap-gis2d -p semio-s-artifact-gis-gismap -p semio-s-artifact-gis-gismap-gismap -p semio-s-artifact-gis-gismap-config
step mvt "${C[@]}" test -j 8 --manifest-path "$R/🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧱️vector-tiles/📦️mvt-decoder/📦️packages/🦀️rust/Cargo.toml" -p semio-framework-surface-tiled-map-mvt vector_label_anchor_uses_line_midpoint_for_transportation_names -- --test-threads=2
step hub "${C[@]}" test -j 8 --manifest-path "$R/🌎️hub/🧩️compositions/🌍️gis/📦️packages/🦀️rust/Cargo.toml" -p semio-hub-gis --lib gis_manifest_publishes_one_editor_viewer_pair_per_artifact_dialect -- --test-threads=2
step bridge "${C[@]}" test -j 8 --manifest-path "$G/🏭️bridge/Cargo.toml" -- --test-threads=2
step gates bash "$T/../../../🌙️09/☀️16/GIS-2D-END-TO-END-BUILD/📜️verify-gis2d-envelope-gates.sh"
step cold-map repo run @semio-tech/gis-plugin:component-cold-map-patch-native-check --lang=en
echo done > "$T/🗑️generated/gismap4.done"

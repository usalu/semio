#!/bin/bash
# usage: seq.sh <tag> <pane...>   runs the listed panes one native cargo test at a time; done marker rtpanes-seq-<tag>.rc
D=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$D/../.." && pwd)
tag=$1; shift
for pane in "$@"; do
  case $pane in
    bearbeiten) "$D/run.sh" bearbeiten "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" -p semio-s-artifact-process-process3d-editor --test process3d_editor_unit -- bearbeiten_pane ;;
    aussuchen) "$D/run.sh" aussuchen "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation" -p semio-s-artifact-sourcing-curation --lib -- aussuchen_pane ;;
    verfolgen) "$D/run.sh" verfolgen "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap" -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen_pane ;;
    koordinator) "$D/run.sh" koordinator "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad" -p semio-s-artifact-cad-cad --test cad_editor_unit -- koordinator_pane ;;
    aggregator) "$D/run.sh" aggregator "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d" -p semio-s-artifact-puzzle-3d-editor --test editor-gestures -- aggregator_pane ;;
    statik) "$D/run.sh" statik "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" -p semio-s-artifact-fem-3d-editor --test editor_app -- statik_pane ;;
    generator) "$D/run.sh" generator "✏️s/🧑‍💻dev/🧩️composition" -p semio-s-composition-laws --test generation3d-app-laws-editor-unit -- generator_pane ;;
    energie) "$D/run.sh" energie "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model" -p semio-s-artifact-energy-model-editor-mounted-tests --lib ;;
  esac
done
echo done > "$C/🗑️generated/1101/rtpanes-seq-$tag.rc"

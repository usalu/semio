#!/usr/bin/env bash
ROOT=/home/deck/semio
A="$ROOT/✏️s/🔌️plugins/🗄️stdio"
OUT="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
run() {
  dir="$1"; pkg="$2"; filter="$3"
  ( cd "$dir" && repo cargo --store=shared --configuration=prototype -- test -p "$pkg" --lib "$filter" ) > "$OUT/e-$pkg-$filter.log" 2>&1
  echo "$pkg $filter exit $?" >> "$OUT/e-batch-summary.log"
}
run "$A" semio-s-artifact-stdio-contract-editing transform
run "$A/🗿️artifacts/🗽️obj" semio-s-artifact-stdio-obj editor::obj
run "$A/🗿️artifacts/🔺️stl" semio-s-artifact-stdio-stl editor::stl
run "$A/🗿️artifacts/🧱️ply" semio-s-artifact-stdio-ply editor::ply
run "$A/🗿️artifacts/☁️las" semio-s-artifact-stdio-las editor::las
run "$A/🗿️artifacts/🧊️gltf" semio-s-artifact-stdio-gltf-editor editor::gltf
run "$A/🗿️artifacts/🧊️gltf" semio-s-artifact-stdio-gltf-editor edit_rules
run "$A/🗿️artifacts/📐️step" semio-s-artifact-stdio-step-base-editor editor::step_any
run "$A/🗿️artifacts/📐️step" semio-s-artifact-stdio-step-mutations edit_rules
run "$A" semio-s-artifact-stdio-contract-window-kit mesh_scene
run "$A/🗿️artifacts/📐️step" semio-s-artifact-stdio-step-ladder ladder
for n in 1 2 3 4 5 6; do run "$A/🗿️artifacts/📐️step" "semio-s-artifact-stdio-step-cc$n" edit_rules; done
for n in 1 2 3 4 5 6; do run "$A/🗿️artifacts/📐️step" "semio-s-artifact-stdio-step-cc$n-editor" editor::step_cc$n; done

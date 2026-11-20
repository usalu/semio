#!/usr/bin/env bash
# Builds the remaining demonstrator runtime modules in three parallel combined runs, then runs the demonstrator check and the site build. Usage: native-tri.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; M="/Users/ueli/Documents/semio/.cache/b/p/m/browser/release/modules"
A=(gis flow process cad-extension-aec-building cad-extension-aec-building-energy cad-extension-aec-building-structure cad-extension-spatial-shape flow-extension-brep)
B=(flow-extension-dictionary flow-extension-draw flow-extension-list flow-extension-logic flow-extension-math flow-extension-primitive flow-extension-text playbook-module-procedural)
C=(process-extension-concrete process-extension-metal process-extension-robotic process-extension-wood sourcing-module-beams sourcing-module-slabs sourcing-module-windows)
part() { local name="$1"; shift; rm -f "$G/done/browser-$name"; "$T/native-lanes.sh" step "browser-$name" "" repo ./📜️script.ts components describe "$@" --facet browser || grep -q " 0 failure(s) in " "$G/browser-$name.log"; }
part partA "${A[@]}" & part partB "${B[@]}" & part partC "${C[@]}" & wait
missing=0; for d in "$M"/*; do :; done
for id in "${A[@]}" "${B[@]}" "${C[@]}"; do ls "$M" | grep -q -- "$id\$" || { echo "MISSING $id"; missing=1; }; done
[ $missing -eq 0 ] || exit 1
rm -f "$G/done/demonstrator-prepare"
"$T/native-lanes.sh" step demonstrator-prepare "♻️mit-bestand/🧺️demonstrator" repo ./🔨️modules/🧩️runtime/📜️script.ts prepare || exit 2
MEM=4096 bash "$T/native-wasm.sh" site

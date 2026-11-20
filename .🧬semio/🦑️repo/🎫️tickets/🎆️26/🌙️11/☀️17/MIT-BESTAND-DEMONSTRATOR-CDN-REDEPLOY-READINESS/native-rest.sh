#!/usr/bin/env bash
# Builds the remaining demonstrator runtime modules in one combined run (falling back to two at a time), waits for cad and gis, then runs the demonstrator check and the site build. Usage: native-rest.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
R=(gis flow process puzzle sourcing cad-extension-aec-building cad-extension-aec-building-energy cad-extension-aec-building-structure cad-extension-spatial-shape flow-extension-bim flow-extension-brep flow-extension-dictionary flow-extension-draw flow-extension-list flow-extension-logic flow-extension-math flow-extension-primitive flow-extension-text playbook-module-procedural process-extension-concrete process-extension-metal process-extension-robotic process-extension-wood sourcing-module-beams sourcing-module-slabs sourcing-module-windows)
ok() { [ -f "$G/done/browser-$1" ] || grep -qs " 0 failure(s) in " "$G/browser-$1.log"; }
rm -f "$G/done/browser-rest"
P=(playbook flow)
if "$T/native-lanes.sh" step browser-rest "" repo ./📜️script.ts components describe "${R[@]}" --facet browser || grep -q " 0 failure(s) in " "$G/browser-rest.log"; then for id in "${R[@]}"; do touch "$G/done/browser-$id"; done; else "$T/native-modules.sh" 2 "${R[@]}" || exit 1; fi
until ok cad && ok playbook; do sleep 30; done
rm -f "$G/done/demonstrator-prepare"
"$T/native-lanes.sh" step demonstrator-prepare "♻️mit-bestand/🧺️demonstrator" repo ./🔨️modules/🧩️runtime/📜️script.ts prepare || exit 2
MEM=4096 bash "$T/native-wasm.sh" site

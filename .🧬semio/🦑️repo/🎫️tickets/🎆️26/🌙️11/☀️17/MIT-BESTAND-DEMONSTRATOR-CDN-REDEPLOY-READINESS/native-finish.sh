#!/usr/bin/env bash
# Regenerates the catalog from the rebuilt descriptors, runs the nine session and lane preparation steps, the demonstrator release check and the static site build. Usage: native-finish.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
VARIANTS="aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen"
rc=0
"$T/native-lanes.sh" catalog || exit 1
for v in $VARIANTS; do "$T/native-lanes.sh" session "$v" || rc=1; done
for v in $VARIANTS; do "$T/native-lanes.sh" prepare "$v" || rc=1; done
for t in surface editor node-graph puzzle flow canvas; do MEM=4096 bash "$T/native-wasm.sh" "$t" || rc=1; done
"$T/native-lanes.sh" step demonstrator-prepare "♻️mit-bestand/🧺️demonstrator" repo ./🔨️modules/🧩️runtime/📜️script.ts prepare || rc=1
MEM=4096 bash "$T/native-wasm.sh" site || rc=1
exit $rc

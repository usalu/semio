#!/usr/bin/env bash
# Runs the nine session and lane preparation steps, the demonstrator release check and the static site build. Usage: native-finish.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
VARIANTS="aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen"
rc=0
for v in $VARIANTS; do "$T/native-lanes.sh" session "$v" || rc=1; done
for v in $VARIANTS; do "$T/native-lanes.sh" prepare "$v" || rc=1; done
"$T/native-lanes.sh" step demonstrator-prepare "♻️mit-bestand/🧺️demonstrator" repo ./🔨️modules/🧩️runtime/📜️script.ts prepare || rc=1
MEM=4096 bash "$T/native-wasm.sh" site || rc=1
exit $rc

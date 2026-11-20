#!/usr/bin/env bash
# Builds energy for the browser beside the running procedural build, waits until all four components are staged, then runs the finishing sequence. Usage: native-join.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
ok() { [ -f "$G/done/browser-$1" ] || grep -qs " 0 failure(s) in " "$G/browser-$1.log"; }
ok energy || "$T/native-lanes.sh" step browser-energy "" repo ./📜️script.ts components describe energy --facet browser || ok energy || exit 1
until ok procedural; do grep -qs "^\[FAIL [0-9]*\] browser-procedural" "$G/par.log" && ! ok procedural && exit 2; sleep 30; done
for c in demonstrator fem procedural energy; do touch "$G/done/browser-$c"; done
for c in 🎪️demonstrator 🔋️energy 🏗️fem 🌀️procedural; do touch "$G/done/component-$c"; done
for v in aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen; do rm -f "$G/done/session-$v" "$G/done/prepare-$v"; done
rm -f "$G/done/demonstrator-prepare"
"$T/native-finish.sh"

#!/usr/bin/env bash
# Builds, describes (writing the committed pairs) and stages the four release components for the browser, then regenerates the catalog and runs the finishing sequence. Usage: native-all.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
for c in demonstrator fem procedural energy; do
  rm -f "$G/done/browser-$c"
  "$T/native-lanes.sh" step "browser-$c" "" repo ./📜️script.ts components describe "$c" --facet browser || { grep -q " 0 failure(s) in " "$G/browser-$c.log" && touch "$G/done/browser-$c" || exit 1; }
done
for c in 🎪️demonstrator 🔋️energy 🏗️fem 🌀️procedural; do touch "$G/done/component-$c"; done
for v in aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen; do rm -f "$G/done/session-$v" "$G/done/prepare-$v"; done
rm -f "$G/done/demonstrator-prepare"
"$T/native-finish.sh"

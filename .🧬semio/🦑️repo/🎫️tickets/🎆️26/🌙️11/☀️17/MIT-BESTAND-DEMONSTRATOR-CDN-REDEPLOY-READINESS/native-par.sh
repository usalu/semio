#!/usr/bin/env bash
# Builds fem, procedural and energy for the browser while the demonstrator finishes, then runs the finishing sequence once all four are staged. Usage: native-par.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
for c in fem procedural energy; do
  [ -f "$G/done/browser-$c" ] && continue
  "$T/native-lanes.sh" step "browser-$c" "" repo ./📜️script.ts components describe "$c" --facet browser || { grep -q " 0 failure(s) in " "$G/browser-$c.log" && touch "$G/done/browser-$c" || exit 1; }
done
until [ -f "$G/done/browser-demonstrator" ] || grep -qs " 0 failure(s) in " "$G/browser-demonstrator.log"; do grep -qs "^\[FAIL" "$G/all.log" && exit 3; sleep 30; done
touch "$G/done/browser-demonstrator"
for c in 🎪️demonstrator 🔋️energy 🏗️fem 🌀️procedural; do touch "$G/done/component-$c"; done
for v in aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen; do rm -f "$G/done/session-$v" "$G/done/prepare-$v"; done
rm -f "$G/done/demonstrator-prepare"
"$T/native-finish.sh"

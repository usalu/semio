#!/usr/bin/env bash
# Builds each release component and writes its committed semantic descriptor pair, then regenerates the catalog and runs the finishing sequence. Usage: native-describe-write.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
for c in demonstrator fem procedural energy; do "$T/native-lanes.sh" step "describe-write-$c" "" repo ./📜️script.ts components describe "$c" || exit 1; done
for c in 🎪️demonstrator 🔋️energy 🏗️fem 🌀️procedural; do touch "$G/done/component-$c"; done
rm -f "$G/done/catalog-generate"; "$T/native-lanes.sh" catalog || exit 2
"$T/native-finish.sh"

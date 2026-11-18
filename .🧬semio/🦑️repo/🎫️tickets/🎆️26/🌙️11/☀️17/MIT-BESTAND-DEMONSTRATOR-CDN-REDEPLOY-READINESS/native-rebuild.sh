#!/usr/bin/env bash
# Rebuilds the descriptor compiler, then the four release components one at a time, then the catalog. Usage: native-rebuild.sh
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
rm -f "$G/done/describe-build"
"$T/native-lanes.sh" prereqs || exit 1
for c in 🎪️demonstrator 🔋️energy 🏗️fem 🌀️procedural; do "$T/native-lanes.sh" component "$c"; done
for c in 🎪️demonstrator 🔋️energy 🏗️fem 🌀️procedural; do [ -f "$G/done/component-$c" ] || exit 2; done
rm -f "$G/done/catalog-generate"
"$T/native-lanes.sh" catalog

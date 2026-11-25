#!/usr/bin/env bash
# Runs the scene tests of one norm family once: run-one-scene.sh <log-name> <family-dir> <pkg> [cargo test args after --]
set -u
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
name="$1"; family="$2"; pkg="$3"; shift 3
dir="$ROOT/$family/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust"
cd "$dir" && python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- test -p "$pkg" --offline --no-fail-fast "$@" > "$T/🗑️generated/$name.log" 2>&1
echo "exit=$?" >> "$T/🗑️generated/$name.log"

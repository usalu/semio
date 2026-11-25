#!/usr/bin/env bash
# Runs `cargo test` for the given packages through the ticket lane: run-lane-pkgs.sh <log-name> <family-dir> <pkg>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
name="$1"; family="$2"; shift 2
args=(); for p in "$@"; do args+=(-p "$p"); done
cd "$ROOT/$family/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust" && python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- test "${args[@]}" --offline --no-fail-fast > "$T/🗑️generated/$name.log" 2>&1
echo "exit=$?" >> "$T/🗑️generated/$name.log"

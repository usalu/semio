#!/bin/zsh
# usage: fmtcheck.sh <sandbox-dir> <root-rel> -> prints count of "Diff in" hunks
SC=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad
cd "$1" || exit 1
rustfmt +nightly-2026-07-20 --edition 2021 --config-path /Users/ueli/Documents/semio/rustfmt.toml --check "$2" 2>&1 | /usr/bin/grep -c "^Diff in"

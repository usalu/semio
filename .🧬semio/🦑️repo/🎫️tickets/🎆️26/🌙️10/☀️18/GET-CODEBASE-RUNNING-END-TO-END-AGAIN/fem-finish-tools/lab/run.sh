#!/bin/sh
# Lab: type-checks and runs the new fem owners against the REAL PagedList, HeapSortCursor, control-contract and fixtures.
# usage: lab/run.sh [SRC]   SRC = tree holding the new sources (default: the live repo after apply; use /private/tmp/fem-finish/ov before apply)
# One nice'd cargo process, offline, private target dir; touches nothing in the repo.
set -e
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=${REPO:-/Users/ueli/Documents/semio}
SRC=${1:-$REPO}
RUN=${FF_LAB_DIR:-/private/tmp/fem-finish/lab-run}
rm -rf "$RUN"; mkdir -p "$RUN"; cp -R "$HERE"/. "$RUN"/; rm -f "$RUN/run.sh"
LISTDIR="$RUN/vtree/🧰️framework/🔨️modules/🌱️value/📋️list"; mkdir -p "$LISTDIR"
cp "$SRC/🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs" "$LISTDIR/🦀️.rs"
ln -s "$REPO/🧰️framework/🔨️modules/🌱️value/📋️list/🎮️edit" "$LISTDIR/🎮️edit"
find "$RUN" -type f \( -name '*.rs' -o -name 'Cargo.toml' \) -print0 | xargs -0 sed -i '' -e "s|@LAB@|$RUN|g" -e "s|@SRC@|$SRC|g" -e "s|@REPO@|$REPO|g"
cd "$RUN"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/private/tmp/fem-finish/target}
nice -n 10 cargo generate-lockfile --offline >/dev/null 2>&1
nice -n 10 cargo test --offline -j1 -p semio-s-fem-sparse-lab --test triplet_csr --test stress --test paged_csr_steps
nice -n 10 cargo test --offline -j1 -p semio-s-fem-analyses-model-lab --test dofmap
nice -n 10 cargo test --offline -j1 -p semio-framework-value-paged-list --test depth

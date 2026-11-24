#!/bin/sh
# Reruns the scene crates touched by the incremental-key fixes, one cargo test at a time.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
export SEMIO_PYTHON=/Users/ueli/Documents/semio/.cache/b/d/m/python/environments/workspace-3b97251b6599d245/bin/python
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
run() { name="$1"; dir="$2"; pkg="$3"; (cd "$dir" && repo cargo --store=shared --configuration=prototype -- test -p "$pkg" --offline) > "$T/🗑️generated/rerun-$name.log" 2>&1; }
run en1997 "$R/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust" semio-s-artifact-norm-en1997-scene
run en1990 "$(dirname "$(find "$R/⚖️en1990" -path '*🎬️scene/📦️packages/🦀️rust/Cargo.toml' | head -1)")" semio-s-artifact-norm-en1990-scene
run en1993 "$(dirname "$(find "$R/🔩️en1993" -path '*🎬️scene/📦️packages/🦀️rust/Cargo.toml' | head -1)")" semio-s-artifact-norm-en1993-scene
run structural "$(dirname "$(find "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🎬️structural-scene" -maxdepth 4 -path '*📦️packages/🦀️rust/Cargo.toml' | head -1)")" semio-s-plugin-norm-structural-scene

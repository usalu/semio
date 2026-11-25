#!/bin/bash
# 🔗 Empty-collection insert: build through the ticket lane, run the live-core insert tests, the en1998 editor lib and integration, then en1997 editor and scene.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"
mkdir -p "$G/bin"
build() { # dir pkg name
  cd "$1" || return
  python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- test -p "$2" --offline --no-run > "$G/ei-build-$3.log" 2>&1
  rm -f "$G/bin/ei-$3-"*; local n=0
  for exe in $(grep -o "(/[^)]*)" "$G/ei-build-$3.log" | tr -d '()'); do [ -f "$exe" ] && [ -x "$exe" ] || continue; n=$((n+1)); cp "$exe" "$G/bin/ei-$3-$n"; done
}
runall() { # dir name timeout args...
  local dir="$1" name="$2" to="$3"; shift 3
  for b in "$G/bin/ei-$name-"*; do [ -f "$b" ] || continue; ( cd "$dir" && RUST_MIN_STACK=33554432 timeout "$to" "$b" "$@" > "$G/ei-$name-$(basename "$b" | sed "s/^ei-$name-//").log" 2>&1; echo "exit $?" >> "$G/ei-$name-$(basename "$b" | sed "s/^ei-$name-//").log" ); done
}
L="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧲️live-core/📦️packages/🦀️rust"
E1="$N/🫨️en1998/$A/✏️editor/📦️packages/🦀️rust"; E7="$N/🌍️en1997/$A/✏️editor/📦️packages/🦀️rust"; S7="$N/🌍️en1997/$A/🎬️scene/📦️packages/🦀️rust"
build "$L" semio-s-plugin-norm-app-surface-live-core livecore; runall "$L" livecore 600 insert
build "$E1" semio-s-artifact-norm-en1998-editor e1998ed; runall "$E1" e1998ed 1740 --test-threads=4 --skip latency_bench
build "$E7" semio-s-artifact-norm-en1997-editor e1997ed; runall "$E7" e1997ed 1740 --test-threads=4 --skip latency_bench
build "$S7" semio-s-artifact-norm-en1997-scene e1997sc; runall "$S7" e1997sc 1740 --test-threads=4
echo done > "$G/ei.done"

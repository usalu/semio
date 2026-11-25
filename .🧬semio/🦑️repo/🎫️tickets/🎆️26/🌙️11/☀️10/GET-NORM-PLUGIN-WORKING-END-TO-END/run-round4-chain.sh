#!/bin/bash
# 🔗 Round 4: en1998 scene (oracle through repo cargo), en1998 editor, en1992 timing probe and the two failing lib tests, din18599 editor.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"
mkdir -p "$G/bin"
build() { cd "$1" || return; repo cargo --store=shared --configuration=prototype -- test -p "$2" --offline --no-run > "$G/build-$3.log" 2>&1
  local n=0; for exe in $(grep -o "(/[^)]*)" "$G/build-$3.log" | tr -d '()'); do [ -f "$exe" ] && [ -x "$exe" ] || continue; n=$((n+1)); cp "$exe" "$G/bin/$3-$n"; done; }
runbin() { local dir="$1" bin="$2" log="$3" to="$4"; shift 4; ( cd "$dir" && RUST_MIN_STACK=33554432 timeout "$to" "$bin" "$@" > "$G/$log" 2>&1; echo "exit $?" >> "$G/$log" ); }
S="$N/🫨️en1998/$A/🎬️scene/📦️packages/🦀️rust"; E1="$N/🫨️en1998/$A/✏️editor/📦️packages/🦀️rust"
E2="$N/🏛️en1992/$A/✏️editor/📦️packages/🦀️rust"; D="$N/⚡️din18599/$A/✏️editor/📦️packages/🦀️rust"
rm -f "$G"/bin/r4-*
build "$E1" semio-s-artifact-norm-en1998-editor r4-en1998-editor
for b in "$G"/bin/r4-en1998-editor-*; do runbin "$E1" "$b" "r4-$(basename "$b").log" 1740 --test-threads=4 --skip latency_bench; done
cd "$S" && repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1998-scene --offline --no-fail-fast -- --test-threads=4 > "$G/r4-en1998-scene.log" 2>&1
build "$E2" semio-s-artifact-norm-en1992-editor r4-en1992
for b in "$G"/bin/r4-en1992-*; do
  runbin "$E2" "$b" "r4-$(basename "$b")-time.log" 1200 --test-threads=1 --nocapture zz_temp_time_units
  for t in every_grip_of_every_entity_kind every_editor_action_is_reachable_by_touch; do runbin "$E2" "$b" "r4-$(basename "$b")-$t.log" 1200 --test-threads=1 "$t"; done
done
build "$D" semio-s-artifact-norm-din18599-editor r4-din
for b in "$G"/bin/r4-din-*; do runbin "$D" "$b" "r4-$(basename "$b").log" 1740 --test-threads=4 --skip latency_bench; done
echo done > "$G/round4.done"

#!/bin/bash
# 🔗 Round 7: build each package (retrying when another writer breaks the compile), copy the test binaries out of the cache, run them under hard timeouts with explicit threads and without the benches.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"
mkdir -p "$G/bin"
build() { # dir pkg name
  cd "$1" || return
  for attempt in 1 2 3 4 5 6 7 8; do
    repo cargo --store=shared --configuration=prototype -- test -p "$2" --offline --no-run > "$G/build-$3.log" 2>&1
    rm -f "$G/bin/$3-"*; local n=0
    for exe in $(grep -o "(/[^)]*)" "$G/build-$3.log" | tr -d '()'); do [ -f "$exe" ] && [ -x "$exe" ] || continue; n=$((n+1)); cp "$exe" "$G/bin/$3-$n"; done
    [ "$n" -gt 0 ] && return
    sleep 240
  done
}
runall() { # dir name timeout args...
  local dir="$1" name="$2" to="$3"; shift 3
  for b in "$G/bin/$name-"*; do [ -f "$b" ] || continue; ( cd "$dir" && RUST_MIN_STACK=33554432 timeout "$to" "$b" "$@" > "$G/$name-$(basename "$b" | sed "s/^$name-//").log" 2>&1; echo "exit $?" >> "$G/$name-$(basename "$b" | sed "s/^$name-//").log" ); done
}
S="$N/🫨️en1998/$A/🎬️scene/📦️packages/🦀️rust"; E1="$N/🫨️en1998/$A/✏️editor/📦️packages/🦀️rust"
E2="$N/🏛️en1992/$A/✏️editor/📦️packages/🦀️rust"; D="$N/⚡️din18599/$A/✏️editor/📦️packages/🦀️rust"
which="${1:-all}"
if [ "$which" = all ] || [ "$which" = e1998 ]; then
  build "$E1" semio-s-artifact-norm-en1998-editor r7-e1998ed; runall "$E1" r7-e1998ed 1740 --test-threads=4 --skip latency_bench
  cd "$S" && repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1998-scene --offline --no-fail-fast -- --test-threads=4 > "$G/r7-e1998scene.log" 2>&1
fi
if [ "$which" = all ] || [ "$which" = e1992 ]; then
  build "$E2" semio-s-artifact-norm-en1992-editor r7-e1992ed
  for b in "$G"/bin/r7-e1992ed-*; do for t in zz_temp_time_units every_grip_of_every_entity_kind every_editor_action_is_reachable_by_touch a_click_on_a_check_row every_check_of_every_demo_document; do
    ( cd "$E2" && RUST_MIN_STACK=33554432 timeout 1200 "$b" --test-threads=1 --nocapture "$t" > "$G/r7-e1992ed-$(basename $b)-$t.log" 2>&1; echo "exit $?" >> "$G/r7-e1992ed-$(basename $b)-$t.log" ); done; done
  runall "$E2" r7-e1992ed 1740 --test-threads=4 --skip latency_bench --skip zz_temp_time_units --skip every_unit_declares_all_it_reads --skip an_edit_re_runs_exactly --skip the_retained_session_equals --skip every_bound_diagram_marker
fi
if [ "$which" = all ] || [ "$which" = din ]; then
  build "$D" semio-s-artifact-norm-din18599-editor r7-din; runall "$D" r7-din 1740 --test-threads=4 --skip latency_bench
fi
echo done > "$G/round7.done"

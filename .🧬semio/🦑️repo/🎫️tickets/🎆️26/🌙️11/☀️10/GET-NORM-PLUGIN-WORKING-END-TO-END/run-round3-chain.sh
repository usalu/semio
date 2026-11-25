#!/bin/bash
# 🔗 Round 3: en1998 scene through repo cargo (oracle), en1998 editor, en1992 editor (heavy tests one by one), din18599 editor; binaries are copied out of the cache before they run.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"
mkdir -p "$G/bin"
build() { # dir pkg name -> copies executables to $G/bin/<name>-<n>
  cd "$1" || return
  repo cargo --store=shared --configuration=prototype -- test -p "$2" --offline --no-run > "$G/build-$3.log" 2>&1
  local n=0
  for exe in $(grep -o "(/[^)]*)" "$G/build-$3.log" | tr -d '()'); do
    [ -f "$exe" ] && [ -x "$exe" ] || continue
    n=$((n+1)); cp "$exe" "$G/bin/$3-$n"
  done
}
runbin() { # dir bin log timeout args...
  local dir="$1" bin="$2" log="$3" to="$4"; shift 4
  ( cd "$dir" && RUST_MIN_STACK=33554432 timeout "$to" "$bin" "$@" > "$G/$log" 2>&1; echo "exit $?" >> "$G/$log" )
}
S="$N/🫨️en1998/$A/🎬️scene/📦️packages/🦀️rust"; E1="$N/🫨️en1998/$A/✏️editor/📦️packages/🦀️rust"
E2="$N/🏛️en1992/$A/✏️editor/📦️packages/🦀️rust"; D="$N/⚡️din18599/$A/✏️editor/📦️packages/🦀️rust"
cd "$S" && repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1998-scene --offline --no-fail-fast -- --test-threads=4 > "$G/r3-en1998-scene.log" 2>&1
build "$E1" semio-s-artifact-norm-en1998-editor r3-en1998-editor
for b in "$G"/bin/r3-en1998-editor-*; do runbin "$E1" "$b" "r3-en1998-editor-$(basename "$b").log" 1740 --test-threads=4 --skip latency_bench; done
build "$E2" semio-s-artifact-norm-en1992-editor r3-en1992
for b in "$G"/bin/r3-en1992-*; do
  for t in a_click_on_a_check_row every_check_of_every_demo_document every_unit_declares_all_it_reads an_edit_re_runs_exactly the_retained_session_equals every_bound_diagram_marker; do
    runbin "$E2" "$b" "r3-en1992-$(basename "$b")-$t.log" 1200 --test-threads=1 "$t"
  done
  runbin "$E2" "$b" "r3-en1992-$(basename "$b")-rest.log" 1740 --test-threads=4 --skip latency_bench --skip a_click_on_a_check_row --skip every_check_of_every_demo_document --skip every_unit_declares_all_it_reads --skip an_edit_re_runs_exactly --skip the_retained_session_equals --skip every_bound_diagram_marker
done
build "$D" semio-s-artifact-norm-din18599-editor r3-din
for b in "$G"/bin/r3-din-*; do runbin "$D" "$b" "r3-din-$(basename "$b").log" 1740 --test-threads=4 --skip latency_bench; done
echo done > "$G/round3.done"

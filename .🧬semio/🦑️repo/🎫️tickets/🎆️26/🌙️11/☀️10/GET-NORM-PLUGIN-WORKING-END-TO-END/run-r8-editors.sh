#!/bin/bash
# 🔗 Round 8: build one editor through the ticket lane, copy the test binaries out of the cache, run them under hard timeouts.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; N="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"; A="🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust"
case "$1" in e1992) D="$N/🏛️en1992/$A"; P=semio-s-artifact-norm-en1992-editor;; din) D="$N/⚡️din18599/$A"; P=semio-s-artifact-norm-din18599-editor;; esac
NAME="r8-$1"; mkdir -p "$G/bin"; cd "$D" || exit 1
for attempt in 1 2 3 4 5 6; do
  python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- test -p "$P" --offline --no-run > "$G/build-$NAME.log" 2>&1
  rm -f "$G/bin/$NAME-"*; n=0
  for exe in $(grep -o "(/[^)]*)" "$G/build-$NAME.log" | tr -d '()'); do [ -f "$exe" ] && [ -x "$exe" ] || continue; n=$((n+1)); cp "$exe" "$G/bin/$NAME-$n"; done
  [ "$n" -gt 0 ] && break; sleep 120
done
for b in "$G/bin/$NAME-"*; do [ -f "$b" ] || continue; k=$(basename "$b")
  RUST_MIN_STACK=33554432 timeout 1500 "$b" --test-threads=4 --skip latency_bench > "$G/$k.log" 2>&1; echo "exit $?" >> "$G/$k.log"; done
echo done > "$G/$NAME.done"

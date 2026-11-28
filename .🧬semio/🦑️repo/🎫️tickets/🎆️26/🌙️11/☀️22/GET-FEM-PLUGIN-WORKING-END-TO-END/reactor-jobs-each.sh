#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/rj"; mkdir -p "$G"
EXE=$(ls -t /c/t/rj/debug/deps/semio_plugin_reactor_jobs-*.exe | head -1)
export RUST_MIN_STACK=134217728
out="$G/each-$1.log"; : > "$out"
for t in $("$EXE" --list 2>/dev/null | grep ': test' | sed 's/: test//'); do
  r=$(timeout 300 "$EXE" --exact "$t" --nocapture 2>&1); rc=$?
  echo "== $t rc=$rc" >> "$out"
  [ $rc -ne 0 ] && echo "$r" | grep -v "^$" | grep -n "panicked\|assert\|left\|right\|Err\|failed\|message\|fatal" | head -8 | cut -c1-500 >> "$out"
done
echo done >> "$out"
